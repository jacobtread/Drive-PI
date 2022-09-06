use std::io;
use std::io::Error;

use std::sync::PoisonError;
use std::time::SystemTimeError;
use actix_web::http::header::ToStrError;
use actix_web::http::StatusCode;
use actix_web::ResponseError;
use derive_more::{Display, Error};

/// Generic enum error type for comm errors
#[derive(Debug, Display, Error)]
pub enum GenericError {
    #[display(fmt = "internal server error")]
    ServerError
}

/// Error type for authentication errors like missing
/// tokens or invalid credentials. Allows generic errors
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

/// Error type for errors that occurred on the
/// auth store.
#[derive(Debug, Display, Error)]
pub enum AuthStoreError {
    #[display(fmt = "read failure")]
    ReadFailure,
    #[display(fmt = "add failure")]
    AddFailure,
    #[display(fmt = "remove failure")]
    RemoveFailure,
}

#[derive(Debug, Display, Error)]
pub enum DrivesError {
    #[display(fmt = "parse error")]
    ParseError,
    #[display(fmt = "unmount error")]
    UnmountError,
    #[display(fmt = "mount error")]
    MountError,
    #[display(fmt = "Target device is busy cannot unmount")]
    TargetBusy,
    #[display(fmt = "io error")]
    IOError,
}

#[derive(Debug, Display, Error)]
pub enum FilesError {
    #[display(fmt = "path outside mount root")]
    OutsideMountRoot,
    #[display(fmt = "path was not a directory")]
    NotDirectory,
    #[display(fmt = "io error")]
    IOError,
}

#[derive(Debug, Display, Error)]
pub enum HotspotError {
    #[display(fmt = "failed to activate hotspot")]
    NotActivated,
    #[display(fmt = "failed to execute hotspot command")]
    CommandError,
    #[display(fmt = "failed to parse output from hotspot command")]
    CommandOutputError,
}

impl From<io::Error> for FilesError {
    fn from(_: Error) -> Self { FilesError::IOError }
}

impl From<io::Error> for DrivesError {
    fn from(_: Error) -> Self { DrivesError::IOError }
}


/// Helper function to be passed into map_err to
/// provide a server error like:
///
/// ```
/// .map_err(server_error)?
/// ```
///
/// which is more readable than the alternative
/// which is:
///
/// ```
/// .map_err(|_|GenericError::ServerError)?
/// ```
pub fn server_error<E>(_: E) -> GenericError {
    return GenericError::ServerError;
}

/// From trait to allow generic errors to be turned into
/// auth errors.
impl From<GenericError> for AuthError {
    fn from(value: GenericError) -> Self {
        AuthError::GenericError(value)
    }
}

impl From<AuthStoreError> for AuthError {
    fn from(_: AuthStoreError) -> Self {
        AuthError::GenericError(GenericError::ServerError)
    }
}

impl From<ToStrError> for AuthError {
    fn from(_: ToStrError) -> Self {
        AuthError::GenericError(GenericError::ServerError)
    }
}

impl From<SystemTimeError> for AuthError {
    fn from(_: SystemTimeError) -> Self {
        AuthError::GenericError(GenericError::ServerError)
    }
}

impl<Guard> From<PoisonError<Guard>> for AuthError {
    fn from(_: PoisonError<Guard>) -> Self {
        AuthError::GenericError(GenericError::ServerError)
    }
}

impl ResponseError for GenericError {}

impl ResponseError for DrivesError {}

impl ResponseError for FilesError {
    fn status_code(&self) -> StatusCode {
        match self {
            FilesError::IOError => StatusCode::INTERNAL_SERVER_ERROR,
            FilesError::NotDirectory | FilesError::OutsideMountRoot => StatusCode::BAD_REQUEST
        }
    }
}

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::InvalidCredentials | AuthError::InvalidToken => StatusCode::UNAUTHORIZED,
            AuthError::MissingToken => StatusCode::BAD_REQUEST,
            AuthError::GenericError(err) => err.status_code()
        }
    }
}