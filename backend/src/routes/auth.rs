use std::time::{Duration, UNIX_EPOCH};

use actix_web::{HttpRequest, post, web};
use actix_web::web::{Json, scope};
use log::{info, warn};
use serde::{Deserialize, Serialize};

use crate::models::errors::{AuthError, server_error};
use crate::stores::auth::AuthStoreData;
use crate::utils::JsonResult;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            scope("/auth")
                .service(auth)
                .service(check_auth)
        );
}

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

#[post("")]
pub async fn auth(
    req: HttpRequest,
    body: Json<AuthRequest>,
    auth_store: AuthStoreData,
) -> JsonResult<TokenDataResponse, AuthError> {
    let mut auth_store = auth_store.lock()
        .map_err(server_error)?;


    let is_credentials = auth_store.is_credentials(&body.username, &body.password);

    if is_credentials {
        let token_data = auth_store.create_token()?;
        let time_elapsed = token_data
            .expiry_time
            .duration_since(UNIX_EPOCH)
            .map_err(server_error)?;

        if let Some(address) = req.peer_addr() {
            info!("Successful authentication attempt from: {}", address.ip().to_string());
        }

        Ok(Json(TokenDataResponse {
            token: token_data.token,
            expiry_time: time_elapsed.as_millis(),
        }))
    } else {
        if let Some(address) = req.peer_addr() {
            warn!("Invalid authentication attempt from: {}", address.ip().to_string());
        }
        Err(AuthError::InvalidCredentials)
    }
}


#[derive(Deserialize)]
pub struct CheckRequest {
    token: String,
}

#[derive(Serialize)]
pub struct CheckResponse {
    valid: bool,
    expiry_time: Option<u128>,
}

#[post("/check")]
pub async fn check_auth(
    body: Json<CheckRequest>,
    auth_store: AuthStoreData,
) -> JsonResult<CheckResponse, AuthError> {
    let mut auth_store = auth_store.lock()
        .map_err(server_error)?;

    let expiry_time =
        auth_store.get_token_expiry(&body.token)?
            .map(|token_expiry| {
                token_expiry
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or(Duration::from_millis(0))
                    .as_millis()
            });

    Ok(Json(CheckResponse {
        valid: expiry_time.is_some(),
        expiry_time,
    }))
}