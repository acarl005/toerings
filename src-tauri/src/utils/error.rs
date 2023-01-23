use std::{borrow::Cow, result};

use thiserror::Error;

#[cfg(target_os = "linux")]
use procfs::ProcError;

/// A type alias for handling errors related to Bottom.
pub type Result<T> = result::Result<T, ToeError>;

/// An error that can occur while Bottom runs.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ToeError {
    /// An error when there is an IO exception.
    #[error("IO exception, {0}")]
    InvalidIo(String),
    /// An error when the heim library encounters a problem.
    #[error("Error caused by Heim, {0}")]
    InvalidHeim(String),
    /// An error when the Crossterm library encounters a problem.
    #[error("Error caused by Crossterm, {0}")]
    CrosstermError(String),
    /// An error to represent generic errors.
    #[error("Error, {0}")]
    GenericError(String),
    /// An error to represent errors with fern.
    #[error("Fern error, {0}")]
    FernError(String),
    /// An error to represent errors with the config.
    #[error("Configuration file error, {0}")]
    ConfigError(String),
    /// An error to represent errors with converting between data types.
    #[error("Conversion error, {0}")]
    ConversionError(String),
    /// An error to represent errors with querying.
    #[error("Query error, {0}")]
    QueryError(Cow<'static, str>),
    /// An error that just signifies something minor went wrong; no message.
    #[error("Minor error.")]
    MinorError,
    /// An error to represent errors with procfs
    #[cfg(target_os = "linux")]
    #[error("Procfs error, {0}")]
    ProcfsError(String),
}

impl From<std::io::Error> for ToeError {
    fn from(err: std::io::Error) -> Self {
        ToeError::InvalidIo(err.to_string())
    }
}

#[cfg(not(target_os = "freebsd"))]
impl From<heim::Error> for ToeError {
    fn from(err: heim::Error) -> Self {
        ToeError::InvalidHeim(err.to_string())
    }
}

impl From<std::num::ParseIntError> for ToeError {
    fn from(err: std::num::ParseIntError) -> Self {
        ToeError::ConfigError(err.to_string())
    }
}

impl From<std::string::String> for ToeError {
    fn from(err: std::string::String) -> Self {
        ToeError::GenericError(err)
    }
}

#[cfg(feature = "fern")]
impl From<fern::InitError> for ToeError {
    fn from(err: fern::InitError) -> Self {
        ToeError::FernError(err.to_string())
    }
}

impl From<std::str::Utf8Error> for ToeError {
    fn from(err: std::str::Utf8Error) -> Self {
        ToeError::ConversionError(err.to_string())
    }
}

impl From<std::string::FromUtf8Error> for ToeError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        ToeError::ConversionError(err.to_string())
    }
}

#[cfg(target_os = "linux")]
impl From<ProcError> for ToeError {
    fn from(err: ProcError) -> Self {
        match err {
            ProcError::PermissionDenied(p) => {
                ToeError::ProcfsError(format!("Permission denied for {:?}", p))
            }
            ProcError::NotFound(p) => ToeError::ProcfsError(format!("{:?} not found", p)),
            ProcError::Incomplete(p) => ToeError::ProcfsError(format!("{:?} incomplete", p)),
            ProcError::Io(e, p) => ToeError::ProcfsError(format!("io error: {:?} for {:?}", e, p)),
            ProcError::Other(s) => ToeError::ProcfsError(format!("Other procfs error: {}", s)),
            ProcError::InternalError(e) => {
                ToeError::ProcfsError(format!("procfs internal error: {:?}", e))
            }
        }
    }
}
