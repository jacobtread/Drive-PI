use std::fmt::{Debug, Display, Formatter};
use std::{fmt, io};

use actix_web::http::header::ToStrError;
use actix_web::http::StatusCode;
use actix_web::ResponseError;
use std::sync::PoisonError;
use std::time::SystemTimeError;

/// Generic enum error type for comm errors
#[derive(Debug)]
pub enum GenericError {
    ServerError,
}

impl Display for GenericError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GenericError::ServerError => f.write_str("internal server error"),
        }
    }
}

/// Error type for authentication errors like missing
/// tokens or invalid credentials. Allows generic errors
#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials,
    MissingToken,
    InvalidToken,
    GenericError(GenericError),
}

impl Display for AuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::InvalidCredentials => f.write_str("invalid credentials"),
            AuthError::MissingToken => f.write_str("missing token"),
            AuthError::InvalidToken => f.write_str("invalid token"),
            AuthError::GenericError(err) => Debug::fmt(err, f),
        }
    }
}

#[derive(Debug)]
pub enum DrivesError {
    ParseError,
    UnmountError,
    MountError,
    TargetBusy,
    IOError,
}

impl Display for DrivesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DrivesError::ParseError => f.write_str("parse error"),
            DrivesError::UnmountError => f.write_str("unmount error"),
            DrivesError::MountError => f.write_str("mount error"),
            DrivesError::TargetBusy => f.write_str("Target is busy cannot unmount"),
            DrivesError::IOError => f.write_str("io error"),
        }
    }
}

#[derive(Debug)]
pub enum FilesError {
    OutsideMountRoot,
    NotDirectory,
    IOError,
}

impl Display for FilesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FilesError::OutsideMountRoot => f.write_str("path outside mount root"),
            FilesError::NotDirectory => f.write_str("path was not a directory"),
            FilesError::IOError => f.write_str("io error"),
        }
    }
}

#[derive(Debug)]
pub enum HotspotError {
    NotActivated,
    CommandError,
    CommandOutputError,
}

impl Display for HotspotError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            HotspotError::NotActivated => f.write_str("failed to activate hotspot"),
            HotspotError::CommandError => f.write_str("failed to execute hotspot command"),
            HotspotError::CommandOutputError => {
                f.write_str("failed to parse output from hotspot command")
            }
        }
    }
}

impl From<io::Error> for FilesError {
    fn from(_: io::Error) -> Self {
        FilesError::IOError
    }
}

impl From<io::Error> for DrivesError {
    fn from(_: io::Error) -> Self {
        DrivesError::IOError
    }
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
            FilesError::NotDirectory | FilesError::OutsideMountRoot => StatusCode::BAD_REQUEST,
        }
    }
}

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::InvalidCredentials | AuthError::InvalidToken => StatusCode::UNAUTHORIZED,
            AuthError::MissingToken => StatusCode::BAD_REQUEST,
            AuthError::GenericError(err) => err.status_code(),
        }
    }
}
