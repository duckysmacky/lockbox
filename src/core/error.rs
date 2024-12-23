//! Custom error types used within all the core functions

use std::{fmt, io};

/// Custom error type which should be used throughout the codebase for consistency. Provided custom
/// error types should cover most of the possible program errors
pub type Result<T> = std::result::Result<T, Error>;

/// Custom Lockbox error type
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Error related to anything to do with user's profile or one's creation
    ProfileError(String),
    /// Error related to accessing, reading or writing files
    IOError(String),
    /// Error related to encryption, decryption, hashing and anything to do with cypher
    CipherError(String),
    /// Error related to incorrect data being provided, read or failed parsing
    InvalidData(String),
    /// Error related to user's provided input being invalid
    InvalidInput(String),
    /// Error related to failing authentication (e.g. invalid password being provided)
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