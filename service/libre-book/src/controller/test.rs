use crate::error::ServiceError;
use actix_web::{web, HttpMessage, HttpResponse};
use casdoor_rust_sdk::CasdoorUser;

async fn user_info(req: actix_web::HttpRequest) -> Result<HttpResponse, ServiceError> {
    let user = req
        .extensions()
        .get::<CasdoorUser>()
        .ok_or(ServiceError::Unauthorized)?
        .clone();
    Ok(HttpResponse::Ok().json(user))
}
