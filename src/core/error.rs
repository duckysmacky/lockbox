//! Custom error implementations which are used throughout the whole codebase. Contains a custom
//! result type, error types and their implementations for Display and conversion from other errors.
//! Consult every error's doc for more details

use std::{fmt, io};
use std::ffi::OsString;
use std::fmt::{Display, Formatter};
use std::io::ErrorKind;

/// Custom result type which should be used throughout the codebase for consistency and better
/// error handling
pub type Result<T> = std::result::Result<T, Error>;

/// Custom Lockbox error type. Contains different kinds of errors for each category, both simple
/// errors with a single message and complex enum errors with different kinds. These custom error
/// types should cover most of the possible program errors
///
/// Not every error is process-ending, but instead can just be handled as a warning or a
/// notification for the user
// TODO: implement a system to check whether the error should end the process
#[derive(Debug)]
pub enum Error {
    /// Error related to anything to do with user's profile. This could mean profile not
    /// being found, no profile being currently selected or any other kind
    ProfileError(ProfileErrorKind),
    /// Error related to program's configuration
    ConfigError(ConfigErrorKind),
    /// Error related to the user's operating system. Any failed operation which was
    /// based on underlying OS will result in this error. This could be failed retrieval of an
    /// environment variable, unable to access native toolchain or similar
    OSError(OSErrorKind),
    /// Error related to encryption decryption failure, hashing and everything else to do
    /// with the encryption process
    EncryptionError(EncryptionErrorKind),
    /// Error related to incorrect data being provided to the program. This could mean an error on
    /// the user's side, wrong type of the file being supplied or any other process related to
    /// incorrect passing data to the program
    InvalidData(InvalidDataKind),
    /// Error related to serializing and deserializing. Failed parsing of profile and config files
    /// along with any other process related to data serialization results in this error
    SerializeError(SerializeErrorKind),
    /// Error related to accessing, reading or writing files. Wrapper for the std::Error and is
    /// converted from it
    IOError(ErrorKind, String),
}

impl Error {
    /// Returns whether the given error should exit upon reaching it
    pub fn should_exit(&self) -> bool {
        match self {
            Error::ProfileError(_) => true,
            Error::ConfigError(_) => true,
            Error::OSError(_) => true,
            Error::EncryptionError(_) => false,
            Error::InvalidData(_) => false,
            Error::SerializeError(_) => false,
            Error::IOError(_, _) => false,
        }
    }
    
    /// Returns an error-specific exit code
    pub fn exit_code(&self) -> i32 {
        match self {
            Error::ProfileError(_) => 6,
            Error::ConfigError(_) => 7,
            Error::OSError(_) => 2,
            Error::EncryptionError(_) => 4,
            Error::InvalidData(_) => 3,
            Error::SerializeError(_) => 5,
            Error::IOError(_, _) => 1,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::ProfileError(e) => write!(f, "Profile error: {}", e),
            Error::ConfigError(e) => write!(f, "Configuration error: {}", e),
            Error::OSError(e) => write!(f, "OS error: {}", e),
            Error::EncryptionError(e) => write!(f, "Encryption error: {}", e),
            Error::InvalidData(e) => write!(f, "Invalid data: {}", e),
            Error::SerializeError(e) => write!(f, "Serialization error: {}", e),
            Error::IOError(k, s) => write!(f, "IO error ({}): {}", k, s),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IOError(err.kind(), err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerializeError(SerializeErrorKind::JSONParseError(err.to_string(), err.line(), err.column()))    
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Error::SerializeError(SerializeErrorKind::TOMLParseError(err.to_string()))
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::SerializeError(SerializeErrorKind::TOMLParseError(err.to_string()))
    }
}
#[derive(Debug)]
pub enum ProfileErrorKind {
    NotFound(String),
    NotSelected,
    AlreadySelected(String),
    AlreadyExists(String),
    AuthenticationFailed,
    MismatchedProfile,
}

impl Display for ProfileErrorKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ProfileErrorKind::NotFound(s) => write!(f, "Profile \"{}\" not found", s),
            ProfileErrorKind::NotSelected => write!(f, "No profile is currently selected"),
            ProfileErrorKind::AlreadySelected(s) => write!(f, "Profile \"{}\" is already selected", s),
            ProfileErrorKind::AlreadyExists(s) => write!(f, "Profile \"{}\" already exists", s),
            ProfileErrorKind::AuthenticationFailed => write!(f, "Authentication failed. Invalid profile password provided"),
            ProfileErrorKind::MismatchedProfile => write!(f, "Mismatched profile. File seems to be encrypted with a different one."),
        }
    }
}

#[derive(Debug)]
pub enum ConfigErrorKind {}

impl Display for ConfigErrorKind {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(Debug)]
pub enum OSErrorKind {
    EnvVariableUnavailable(String),
}

impl Display for OSErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self { OSErrorKind::EnvVariableUnavailable(s) => write!(f, "Unable to get the \"{}\" environment variable", s), }
    }
}

#[derive(Debug)]
pub enum EncryptionErrorKind {
    CipherError(String),
    HashError(String),
}

impl Display for EncryptionErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            EncryptionErrorKind::CipherError(s) => write!(f, "Unable to apply cipher ({})", s),
            EncryptionErrorKind::HashError(s) => write!(f, "Unable to generate a hash ({})", s),
        }
    }
}

#[derive(Debug)]
pub enum InvalidDataKind {
    FileAlreadyEncrypted(OsString),
    FileNotEncrypted(OsString),
    FileNotSupported(OsString),
    InvalidHexNumber(String),
}

impl Display for InvalidDataKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            InvalidDataKind::FileAlreadyEncrypted(s) => write!(f, "File {:?} is already encrypted", s),
            InvalidDataKind::FileNotEncrypted(s) => write!(f, "File {:?} is not encrypted", s),
            InvalidDataKind::FileNotSupported(s) => write!(f, "File {:?} is not supported", s),
            InvalidDataKind::InvalidHexNumber(s) => write!(f, "Invalid hex number provided ({})", s),
        }
    }
}

#[derive(Debug)]
pub enum SerializeErrorKind {
    JSONParseError(String, usize, usize),
    TOMLParseError(String),
    BOXParseError(String),
}

impl Display for SerializeErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SerializeErrorKind::JSONParseError(s, l, c) => write!(f, "Unable to parse a JSON file (line {}, column {}):\n{}", l, c, s),
            SerializeErrorKind::TOMLParseError(s) => write!(f, "Unable to parse a TOML file:\n{}", s),
            SerializeErrorKind::BOXParseError(s) => write!(f, "Unable to parse a BOX file:\n{}", s),
        }
    }
}

/// Macro used as a shortcut for creating a new Lockbox Error.
/// 
/// A new error is generated by providing the error *type* (`<Name>Error`) and an error *kind*
/// (`<Name>ErrorKind`) after the `:`. An error message should also be supplied if the error *kind* 
/// requires one. If the error requires an `OsString` instead of a basic `String`, append `os`
/// token before the error message (e.g. `new_err!(<Error>: <ErrorKind>, os <message>)`)
#[macro_export]
macro_rules! new_err {
    ($err:ident: $kind:ident) => {
        {
            use paste::paste;
            paste! {
                use crate::core::error::{Error, [<$err Kind>]};
                Error::$err([<$err Kind>]::$kind)
            }
        }
    };
    ($err:ident: $kind:ident, $msg:expr) => {
        {
            use paste::paste;
            paste! {
                use crate::core::error::{Error, [<$err Kind>]};
                Error::$err([<$err Kind>]::$kind($msg.to_string()))
            }
        }
    };
    ($err:ident: $kind:ident, os $msg:expr) => {
        {
            use paste::paste;
            paste! {
                use crate::core::error::{Error, [<$err Kind>]};
                Error::$err([<$err Kind>]::$kind($msg.to_os_string()))
            }
        }
    };
}

/// Macro used as a shortcut for comparing errors by their *type* and *kind*. 
/// 
/// Comparison can be done in two ways: by *type* only (e.g. `err_cmp!(err, ProfileError)`) or by
/// *type* and *kind* (e.g. `err_cmp!(err, ProfileError, NotSelected)`). If the error *kind* has a
/// value, `()` should be added at the end of the *kind* specification (e.g. `err_cmp!(err,
/// ProfileError, NotFound())`)
#[macro_export]
macro_rules! err_cmp {
    ($err:expr, $err_type:ident) => {
        {
            use crate::core::error::Error;
            if let Error::$err_type(_) = &$err {
                true
            } else {
                false
            }
        }
    };
    ($err:expr, $err_type:ident, $err_kind:ident) => {
        {
            use paste::paste;
            use crate::core::error::Error;
            if let Error::$err_type(kind) = &$err {
                paste! {
                    use crate::core::error::[<$err_type Kind>];
                    if let [<$err_type Kind>]::$err_kind = kind {
                        true
                    } else {
                        false
                    }
                }
            } else {
                false
            }
        }
    };
    ($err:expr, $err_type:ident, $err_kind:ident()) => {
        {
            use paste::paste;
            use crate::core::error::Error;
            if let Error::$err_type(kind) = &$err {
                paste! {
                    use crate::core::error::[<$err_type Kind>];
                    if let [<$err_type Kind>]::$err_kind(_) = kind {
                        true
                    } else {
                        false
                    }
                }
            } else {
                false
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use super::*;

    #[test]
    fn test_err_cmp() {
        let err = Error::ProfileError(ProfileErrorKind::AuthenticationFailed);
        
        let compare_type = err_cmp!(err, ProfileError);
        assert!(compare_type);
        
        let compare_kind = err_cmp!(err, ProfileError, AuthenticationFailed);
        assert!(compare_kind);
        
        let failed_compare_type = err_cmp!(err, OSError);
        assert_ne!(failed_compare_type, true);
        
        let failed_compare_kind = err_cmp!(err, ProfileError, MismatchedProfile);
        assert_ne!(failed_compare_kind, true);
    }
    
    #[test]
    fn test_new_err_macro() {
        let err = new_err!(ProfileError: NotSelected);
        assert!(err_cmp!(err, ProfileError, NotSelected));
        
        let str_err = new_err!(InvalidData: InvalidHexNumber, "placeholder");
        assert!(err_cmp!(str_err, InvalidData, InvalidHexNumber()));
        
        let os_str_err = new_err!(InvalidData: FileNotSupported, os OsStr::new("filename"));
        assert!(err_cmp!(os_str_err, InvalidData, FileNotSupported()));
    }
}