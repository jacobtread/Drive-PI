use std::time::{Duration, UNIX_EPOCH};

use crate::define_routes;
use actix_web::web::{Data, Json};
use actix_web::{delete, get, post, web, HttpRequest};
use log::{info, warn};

use crate::middleware::auth::TOKEN_HEADER;
use crate::models::auth::{AuthRequest, CheckResponse, TokenDataResponse};
use crate::models::errors::AuthError;
use crate::stores::auth::AuthStore;
use crate::utils::{ok_json, ok_json_empty, JsonResult};

define_routes!(auth, check_auth, delete_token);

type AuthResult<T> = JsonResult<T, AuthError>;
type AuthResultEmpty = AuthResult<()>;

#[post("/auth")]
pub async fn auth(
    req: HttpRequest,
    body: Json<AuthRequest>,
    auth_store: Data<AuthStore>,
) -> AuthResult<TokenDataResponse> {
    let is_credentials = auth_store.is_credentials(&body.username, &body.password);

    if is_credentials {
        let token_data = auth_store.create_token().await;
        let time_elapsed = token_data.expiry_time.duration_since(UNIX_EPOCH)?;

        if let Some(address) = req.peer_addr() {
            info!(
                "Successful authentication attempt from: {}",
                address.ip().to_string()
            );
        }

        ok_json(TokenDataResponse {
            token: token_data.token,
            expiry_time: time_elapsed.as_millis(),
        })
    } else {
        if let Some(address) = req.peer_addr() {
            warn!(
                "Invalid authentication attempt from: {}",
                address.ip().to_string()
            );
        }
        Err(AuthError::InvalidCredentials)
    }
}

#[get("/auth")]
pub async fn check_auth(
    req: HttpRequest,
    auth_store: Data<AuthStore>,
) -> AuthResult<CheckResponse> {
    let token_header = req
        .headers()
        .get(TOKEN_HEADER)
        .ok_or(AuthError::MissingToken)?;

    let token = token_header.to_str()?;
    let expiry_time = auth_store
        .get_token_expiry(token)
        .await
        .map(|token_expiry| {
            token_expiry
                .duration_since(UNIX_EPOCH)
                .unwrap_or(Duration::from_millis(0))
                .as_millis()
        });

    ok_json(CheckResponse {
        valid: expiry_time.is_some(),
        expiry_time,
    })
}

#[delete("/auth")]
pub async fn delete_token(req: HttpRequest, auth_store: Data<AuthStore>) -> AuthResultEmpty {
    let token_header = req
        .headers()
        .get(TOKEN_HEADER)
        .ok_or(AuthError::MissingToken)?;
    let token = token_header.to_str()?;
    auth_store.remove_token(token).await;
    if let Some(address) = req.peer_addr() {
        info!(
            "Deleted authentication token {} for {}",
            token,
            address.ip().to_string()
        );
    }
    ok_json_empty()
}
