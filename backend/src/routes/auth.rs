use std::time::{Duration, UNIX_EPOCH};

use actix_web::{delete, get, HttpRequest, post, web};
use actix_web::web::{Json, scope};
use log::{info, warn};
use serde::{Deserialize, Serialize};

use crate::middleware::auth::TOKEN_HEADER;
use crate::models::errors::{AuthError, server_error};
use crate::stores::auth::AuthStoreData;
use crate::utils::JsonResult;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            scope("/auth")
                .service(auth)
                .service(check_auth)
                .service(delete_token)
        );
}

type AuthResult<T> = JsonResult<T, AuthError>;

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
) -> AuthResult<TokenDataResponse> {
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


#[derive(Serialize)]
pub struct CheckResponse {
    valid: bool,
    expiry_time: Option<u128>,
}

#[get("")]
pub async fn check_auth(
    req: HttpRequest,
    auth_store: AuthStoreData,
) -> AuthResult<CheckResponse> {
    let token_header = req.headers().get(TOKEN_HEADER)
        .ok_or(AuthError::MissingToken)?;

    let token = token_header.to_str()
        .map_err(server_error)?
        .to_string();

    let auth_store = auth_store.lock()
        .map_err(server_error)?;

    let expiry_time =
        auth_store.get_token_expiry(&token)?
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

#[delete("")]
pub async fn delete_token(
    req: HttpRequest,
    auth_store: AuthStoreData
) -> AuthResult<()> {
    let token_header = req.headers().get(TOKEN_HEADER)
        .ok_or(AuthError::MissingToken)?;

    let token = token_header.to_str()
        .map_err(server_error)?
        .to_string();

    let mut auth_store = auth_store.lock()
        .map_err(server_error)?;

    auth_store.remove_token(&token)
        .map_err(server_error)?;
    if let Some(address) = req.peer_addr() {
        info!("Deleted authentication token {} for {}", token, address.ip().to_string());
    }
    Ok(Json(()))
}