//!
//! This example showcases the Github OAuth2 process for requesting access to the user's public repos and
//! email address.
//!
//! Before running it, you'll need to generate your own Github OAuth2 credentials.
//!
//! In order to run the example call:
//!
//! ```sh
//! CASDOOR_CLIENT_ID=xxx CASDOOR_CLIENT_SECRET=yyy cargo run --example github
//! ```
//!
//! ...and follow the instructions.
//!

use actix_web::{web, HttpResponse, Responder};

use oauth2::basic::{BasicClient, BasicErrorResponseType, BasicTokenType};
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use oauth2::{
    Client, EmptyExtraTokenFields, PkceCodeChallenge, PkceCodeVerifier,
    RevocationErrorResponseType, StandardErrorResponse, StandardRevocableToken,
    StandardTokenIntrospectionResponse, StandardTokenResponse,
};
use redis::AsyncCommands as _;
use serde::{Deserialize, Serialize};
use url::Url;

use std::env;

use libre_core::database::{postgres::PostgresPool, redis::RedisMultiplexClient};
use libre_core::jsonwebtoken;

use super::{Error, LoginResponse};
use crate::env::{FRONTEND_URL, HOST_URL};
use crate::models;

type OAuthClient = Client<
    StandardErrorResponse<BasicErrorResponseType>,
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
    oauth2::EndpointSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointSet,
>;

const CASDOOR_CALLBACK_PATH: &str = "/api/callback/casdoor";
const CASDOOR_AUTH_URL: &str = "/login/oauth/authorize";
const CASDOOR_TOKEN_URL: &str = "/api/login/oauth/access_token";
const CASDOOR_USER_API_URL: &str = "/api/userinfo";

lazy_static::lazy_static!{
    static ref CASDOOR_BASE_URL: String = std::env::var("CASDOOR_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
}

// Available scopes
// openid (no scope)	sub (user's id), iss (issuer), and aud (audience)
// profile	        user profile info, including name, displayName, and avatar
// email	        user's email address
// {
//     "address": "string",
//     "aud": "string",
//     "email": "string",
//     "email_verified": true,
//     "groups": [
//       "string"
//     ],
//     "iss": "string",
//     "name": "string",
//     "phone": "string",
//     "picture": "string",
//     "preferred_username": "string",
//     "sub": "string"
// }
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct CasdoorUser {
    pub address: Option<String>,
    pub aud: String,
    pub email: Option<String>, // email
    pub email_verified: Option<bool>,
    pub groups: Option<Vec<String>>,
    pub iss: String,
    pub name: Option<String>, // login
    pub phone: Option<String>,
    pub picture: Option<String>,
    pub preferred_username: Option<String>, // name
    pub sub: String, // casdoor_id
}

// impl From<CasdoorUser> for models::User {
//     fn from(user: CasdoorUser) -> Self {
//         Self {
//             login: user.name.,
//             name: user.preferred_username.unwrap_or(user.name),
//             email: user.email.unwrap_or_default(),
//             avatar: user.picture,
//             casdoor_id: Some(user.sub),
//             ..Default::default()
//         }
//     }
// }

impl TryFrom<CasdoorUser> for models::User {
    type Error = crate::oauth::Error;

    fn try_from(user: CasdoorUser) -> Result<Self, Self::Error> {
        let username = user.name.ok_or(Error::Other("Name is required"))?;
        Ok(
            Self {
                login: username.clone(),
                name: user.preferred_username,
                email: user.email,
                avatar: user.picture,
                casdoor_id: Some(user.sub),
                ..Default::default()
            }
        )
    }
}

pub fn casdoor_config(cfg: &mut web::ServiceConfig) {
    let client_id = env::var("CASDOOR_CLIENT_ID");
    let client_secret = env::var("CASDOOR_CLIENT_SECRET");
    if client_id.is_err() || client_secret.is_err() {
        log::info!("CASDOOR environments are not set. Start without casdoor auth");
        return;
    }

    let client = BasicClient::new(ClientId::new(client_id.unwrap()))
        .set_client_secret(ClientSecret::new(client_secret.unwrap()))
        .set_auth_uri(AuthUrl::new(CASDOOR_BASE_URL.clone() + CASDOOR_AUTH_URL).unwrap())
        .set_token_uri(TokenUrl::new(CASDOOR_BASE_URL.clone() + CASDOOR_TOKEN_URL).unwrap())
        .set_redirect_uri(
            RedirectUrl::new(FRONTEND_URL.to_string() + CASDOOR_CALLBACK_PATH).unwrap(),
        );
    cfg.service(
        web::scope("/casdoor")
            .app_data(web::Data::new(client))
            .route("", web::get().to(auth))
            .route("/callback", web::get().to(callback)),
    );
}

async fn auth(
    oauth_client: web::Data<OAuthClient>,
    redis: web::Data<RedisMultiplexClient>,
) -> Result<HttpResponse, actix_web::Error> {
    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    // Create an authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();
    // Save the CSRF state to the Redis database.
    let csrf_state = csrf_state.secret();
    let pkce_verifier = pkce_verifier.secret();
    let mut redis = redis.get().await?;
    let _ = redis.set::<_, _, ()>(csrf_state, pkce_verifier).await;
    // Return the CSRF token to the client
    Ok(HttpResponse::Found()
        .append_header(("Location", authorize_url.as_str()))
        .append_header(("X-CSRF-Token", csrf_state.as_str()))
        .finish())
}

async fn callback(
    query: web::Query<super::CallbackQuery>,
    oauth_client: web::Data<OAuthClient>,
    jwt: web::Data<jsonwebtoken::TokenEncoder>,
    redis_pool: web::Data<RedisMultiplexClient>,
    postgres_pool: web::Data<PostgresPool>,
) -> Result<HttpResponse, actix_web::Error> {
    use redis::AsyncCommands;
    let query = query.into_inner();
    let mut redis_conn = redis_pool.get().await?;

    let pkce_verifier = PkceCodeVerifier::new(
        redis_conn
            .get(query.state.secret())
            .await
            .map_err(actix_web::error::ErrorBadRequest)?,
    );

    let http_client = reqwest::ClientBuilder::new()
        // Following redirects opens the client up to SSRF vulnerabilities.
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    let token = oauth_client
        .exchange_code(query.code)
        .set_pkce_verifier(pkce_verifier)
        .request_async(&http_client)
        .await
        .map_err(|err| {
            log::debug!("Failed to exchange code: {:?}", err);
            match err {
                oauth2::RequestTokenError::ServerResponse(response) => {
                    actix_web::error::ErrorBadRequest(response.to_string())
                }
                _ => actix_web::error::ErrorInternalServerError(err),
            }
        })?;

    let casdoor_user = get_user_info_from_casdoor(&token).await?;
    let mut conn = postgres_pool.get().await?;

    let find_result = models::User::find_by_casdoor_id(&casdoor_user.sub, &mut conn).await;

    // Following are sign-in or sign-up logic
    let libre_user = match find_result {
        Ok(user) => user,
        Err(models::Error::NotFound) => {
            // Create a new user ==> sign-up
            models::User::try_from(casdoor_user)?.create(&mut conn).await?
        }
        Err(err) => return Err(err.into()),
    };

    // generate a JWT token
    let jwt = jsonwebtoken::Claims::from(&libre_user)
        .expiration(chrono::Duration::hours(1))
        .generate_jwt(&jwt)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let body = LoginResponse {
        user: libre_user,
        token: jwt,
    };

    Ok(HttpResponse::Ok().json(body))
}

async fn get_user_info_from_casdoor(
    token: &StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
) -> Result<CasdoorUser, Error> {
    let scopes = if let Some(scopes_vec) = token.scopes() {
        scopes_vec
            .iter()
            .flat_map(|comma_separated| comma_separated.split(','))
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    log::debug!("Casdoor returned the following scopes:\n{scopes:?}\n");
    log::debug!("Token type: {:?}\n", token.token_type());

    let response = match token.token_type() {
        BasicTokenType::Bearer => reqwest::Client::new()
            .get(CASDOOR_BASE_URL.clone() + CASDOOR_USER_API_URL)
            .header(
                "Authorization",
                format!("Bearer {}", token.access_token().secret()),
            )
            .send()
            .await
            .map_err(|_| Error::Other("Failed to Request user info from Casdoor"))?,
        _ => {
            return Err(Error::Other("Unsupported token type"));
        }
    };

    match response.status() {
        reqwest::StatusCode::OK => {}
        reqwest::StatusCode::UNAUTHORIZED => {
            return Err(Error::Authentication);
        }
        status => {
            log::debug!("Casdoor returned status: {:?}\n", status);
            return Err(Error::Other("Failed to Recieve user info from Casdoor"));
        }
    }

    let user_info = response
        .json::<CasdoorUser>()
        .await
        .map_err(|_| Error::Other("Failed to parse user info from Casdoor"))?;

    log::debug!("Casdoor return info: {:?}\n", user_info);

    Ok(user_info)
}
