use actix_web::web;
use oauth2::{AuthorizationCode, CsrfToken};
use serde::{Deserialize, Serialize};

mod casdoor;
mod error;
// mod github;
pub use error::Error;

use crate::{env::FRONTEND_URL, models};

pub fn init(cfg: &mut web::ServiceConfig) {
    let cors = actix_cors::Cors::default()
        .allowed_origin(&FRONTEND_URL)
        .allow_any_method()
        .allow_any_header()
        .max_age(3600);
    cfg.service(
        web::scope("/oauth")
            .wrap(cors)
            // .configure(github::github_config)
            .configure(casdoor::casdoor_config),
    );
}

#[derive(Deserialize)]
pub struct CallbackQuery {
    code: AuthorizationCode,
    state: CsrfToken,
}

#[derive(Serialize)]
struct LoginResponse {
    user: models::User,
    token: String,
}

// #[derive(Deserialize, Debug)]
// pub struct BaseOauthUser {
//     pub id: String,
//     pub login: String,
//     pub name: Option<String>,
//     pub email: Option<String>,
//     pub avatar_url: String,
// }

// impl From<github::GitHubUser> for BaseOauthUser {
//     fn from(user: github::GitHubUser) -> Self {
//         Self {
//             id: user.id.to_string(),
//             login: user.login,
//             name: user.name,
//             email: user.email,
//             avatar_url: user.avatar_url.to_string(),
//         }
//     }
// }
