use std::{fmt, io};
use std::ffi::OsString;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    KeyNotFound,
    IOError(io::Error),
    InvalidChecksum(OsString),
    InvalidFile(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::KeyNotFound => write!(f, "Key not found"),
            Error::IOError(ref err) => write!(f, "{}", err),
            Error::InvalidChecksum(ref file_name) => write!(f, "Checksum verification failed for {:?}", file_name),
            Error::InvalidFile(ref msg) => write!(f, "{}", msg)
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IOError(err)
    }
}