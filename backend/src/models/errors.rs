use actix_web::{ResponseError};
use actix_web::http::StatusCode;
use derive_more::{Display, Error};

/// Generic enum error type for comm errors
#[derive(Debug, Display, Error)]
pub enum GenericError {
    #[display(fmt = "internal server error")]
    ServerError
}

pub fn server_error<E>(_: E) -> GenericError {
    return GenericError::ServerError;
}

#[derive(Debug, Display, Error)]
pub enum AuthError {
    #[display(fmt = "invalid credentials")]
    InvalidCredentials,
    #[display(fmt = "missing token")]
    MissingToken,
    #[display(fmt = "invalid token")]
    InvalidToken,
    #[display(fmt = "{}", .0)]
    GenericError(GenericError),
}

impl From<GenericError> for AuthError {
    fn from(value: GenericError) -> Self {
        AuthError::GenericError(value)
    }
}

impl ResponseError for GenericError {}

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::InvalidCredentials
            | AuthError::InvalidToken => StatusCode::UNAUTHORIZED,
            AuthError::MissingToken => StatusCode::BAD_REQUEST,
            AuthError::GenericError(err) => err.status_code()
        }
    }
}