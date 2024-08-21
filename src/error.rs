use std::{fmt, io};
use std::ffi::OsString;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    ProfileError(String),
    KeyError(String),
    IOError(String),
    InvalidChecksum(OsString),
    InvalidFile(String),
    AuthError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ProfileError(ref err) => write!(f, "{}", err),
            Error::KeyError(ref err) => write!(f, "{}", err),
            Error::IOError(ref err) => write!(f, "{}", err),
            Error::InvalidChecksum(ref file_name) => write!(f, "Checksum verification failed for {:?}", file_name),
            Error::InvalidFile(ref msg) => write!(f, "{}", msg),
            Error::AuthError(ref msg) => write!(f, "Authentication failed: {}", msg)
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IOError(err.to_string())
    }
}