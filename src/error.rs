use std::{fmt, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    ProfileError(String),
    IOError(String),
    CipherError(String),
    InvalidData(String),
    InvalidInput(String),
    AuthError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ProfileError(ref msg) => write!(f, "Profile error - {}", msg),
            Error::IOError(ref err) => write!(f, "{}", err),
            Error::CipherError(ref err) => write!(f, "{}", err),
            Error::InvalidData(ref msg) => write!(f, "{}", msg),
            Error::InvalidInput(ref msg) => write!(f, "{}", msg),
            Error::AuthError(ref msg) => write!(f, "Authentication failed - {}", msg)
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IOError(err.to_string())
    }
}