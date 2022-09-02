use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct TokenDataResponse {
    pub token: String,
    pub expiry_time: u128,
}


#[derive(Serialize)]
pub struct CheckResponse {
    pub valid: bool,
    pub expiry_time: Option<u128>,
}
