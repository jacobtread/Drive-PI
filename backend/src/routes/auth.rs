use std::time::UNIX_EPOCH;

use actix_web::{get, post, web};
use actix_web::web::{Json, scope};
use serde::{Deserialize, Serialize};

use crate::middleware::auth::AuthMiddleware;
use crate::models::errors::{AuthError, server_error};
use crate::stores::auth::{AuthStoreData, AuthStoreSafe};
use crate::utils::JsonResult;

#[derive(Deserialize)]
pub struct AuthRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct TokenDataResponse {
    token: String,
    expiry_time: u128,
}

#[post("/auth")]
pub async fn auth(body: Json<AuthRequest>, auth_store: AuthStoreData) -> JsonResult<TokenDataResponse, AuthError> {
    let mut auth_store = auth_store.lock()
        .map_err(server_error)?;

    let is_credentials = auth_store.is_credentials(&body.username, &body.password);

    if is_credentials {
        let token_data = auth_store.create_token()?;
        let time_elapsed = token_data
            .expiry_time
            .duration_since(UNIX_EPOCH)
            .map_err(server_error)?;

        Ok(Json(TokenDataResponse {
            token: token_data.token,
            expiry_time: time_elapsed.as_millis(),
        }))
    } else {
        Err(AuthError::InvalidCredentials)
    }
}

#[derive(Serialize)]
pub struct ProtectedResponse {
    message: String,
}

#[get("/test")]
pub async fn protected() -> JsonResult<ProtectedResponse, AuthError> {
    Ok(Json(ProtectedResponse {
        message: "Success Protected Route Hit".to_string()
    }))
}


pub fn init_routes(cfg: &mut web::ServiceConfig, auth_store: AuthStoreSafe) {
    cfg
        .service(auth)
        .service(
            scope("/protected")
                .wrap(AuthMiddleware::new(auth_store))
                .service(protected)
        );
}