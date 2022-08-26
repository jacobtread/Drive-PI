use std::fmt;
use std::fmt::{Display, Formatter, Write};
use std::ops::Add;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, SystemTimeError, UNIX_EPOCH};
use actix_web::{post, HttpResponse, Responder, ResponseError, web};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use derive_more::{Display, Error};
use log::info;
use serde::de::Unexpected::Str;
use crate::AuthStore;

#[derive(Deserialize)]
struct AuthRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct AuthResponse {
    token: String,
    expiry_time: u128,
}

#[derive(Debug, Error)]
enum AuthError {
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
pub async fn auth(body: Json<AuthRequest>, auth_store: web::Data<Arc<Mutex<AuthStore>>>) -> Result<Json<AuthResponse>, AuthError> {
    let mut auth_store = auth_store.lock()
        .expect("Failed to get mutable auth store ref");


    info!("Auth Request");

    let valid = auth_store.is_valid_credentials(
        &body.username,
        &body.password,
    );

    if valid {
        let create = auth_store.create_token();
        match create {
            Some((token, expiry_time)) => {
                let time_elapsed = expiry_time.duration_since(UNIX_EPOCH);
                match time_elapsed {
                    Ok(time_elapsed) => {
                        Ok(Json(AuthResponse {
                            token,
                            expiry_time: time_elapsed.as_millis(),
                        }))
                    }
                    Err(_) => Err(AuthError::InternalServerError)
                }
            }
            None => Err(AuthError::InternalServerError)
        }
    } else {
        Err(AuthError::InvalidCredentials)
    }
}
