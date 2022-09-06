use serde::{Deserialize, Serialize};

/// Model for the body of requests going to POST /api/auth
#[derive(Deserialize)]
pub struct AuthRequest {
    // Username credentials
    pub username: String,
    // Password Credentials
    pub password: String,
}

/// Model for the response of successful requests to POST /api/auth
#[derive(Serialize)]
pub struct TokenDataResponse {
    // The token string to use for the X-Token header
    pub token: String,
    // Time in milliseconds for when the token will expire
    pub expiry_time: u128,
}

/// Model for the response of requests to GET /api/auth
#[derive(Serialize)]
pub struct CheckResponse {
    // Whether the token is valid
    pub valid: bool,
    // If the token is valid then the time in milliseconds to its expiry time
    pub expiry_time: Option<u128>,
}
