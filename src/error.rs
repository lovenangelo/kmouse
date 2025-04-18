//! Error handling for the Kmouse application

use std::fmt;

/// Custom error type for the application
#[derive(Debug)]
pub enum Error {
    /// X11 connection errors
    X11(String),
    /// Input device errors
    Input(String),
    /// UI errors
    Ui(String),
    /// IO errors
    Io(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::X11(msg) => write!(f, "X11 error: {}", msg),
            Error::Input(msg) => write!(f, "Input error: {}", msg),
            Error::Ui(msg) => write!(f, "UI error: {}", msg),
            Error::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<x11rb::errors::ConnectionError> for Error {
    fn from(err: x11rb::errors::ConnectionError) -> Self {
        Error::X11(err.to_string())
    }
}

impl From<x11rb::errors::ReplyError> for Error {
    fn from(err: x11rb::errors::ReplyError) -> Self {
        Error::X11(err.to_string())
    }
}

impl From<enigo::InputError> for Error {
    fn from(err: enigo::InputError) -> Self {
        Error::Input(err.to_string())
    }
}

impl From<rdev::ListenError> for Error {
    fn from(err: rdev::ListenError) -> Self {
        Error::Input(format!("{:?}", err))
    }
}

/// Result type alias for the application
pub type Result<T> = std::result::Result<T, Error>;
