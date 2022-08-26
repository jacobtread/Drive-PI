use std::fmt;
use std::fmt::{Display, Formatter};
use std::time::UNIX_EPOCH;

use actix_web::{HttpResponse, post, ResponseError};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use derive_more::Error;
use serde::{Deserialize, Serialize};

use crate::stores::auth::AuthStoreData;
use crate::utils::JsonResult;

#[derive(Deserialize)]
pub struct AuthRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    token: String,
    expiry_time: u128,
}

#[derive(Debug, Error)]
pub enum AuthError {
    InvalidCredentials,
    InternalServerError,
}

impl Display for AuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::InvalidCredentials => f.write_str("invalid credentials"),
            AuthError::InternalServerError => f.write_str("internal server error")
        }
    }
}

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            AuthError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}

#[post("/api/auth")]
pub async fn auth(body: Json<AuthRequest>, auth_store: AuthStoreData) -> JsonResult<AuthResponse, AuthError> {
    let mut auth_store = auth_store.lock()
        .map_err(|_| AuthError::InternalServerError)?;

    let is_credentials = auth_store.is_credentials(&body.username, &body.password);

    if is_credentials {
        let token_data = auth_store.create_token()
            .map_err(|_| AuthError::InternalServerError)?;
        let time_elapsed = token_data
            .expiry_time
            .duration_since(UNIX_EPOCH)
            .map_err(|_| AuthError::InternalServerError)?;

        Ok(Json(AuthResponse {
            token: token_data.token,
            expiry_time: time_elapsed.as_millis(),
        }))
    } else {
        Err(AuthError::InvalidCredentials)
    }
}
