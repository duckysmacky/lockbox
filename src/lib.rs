//! # Lockbox API
//!
//! This library functions like an API between the CLI and GUI wrappers and the rest of the core
//! program code. It contains all the main functions related to the Lockbox's functionality, like
//! encryption, decryption, key and profile management.

pub use core::error::{Error, Result};
pub use core::utils;
// TODO: add a way to use the parser in utils without re-importing it

pub mod cli;
mod core;

/// Type representing a basic 32-byte encryption key
pub type Key = [u8; 32];
/// Type representing a 12-byte nonce used for encryption in combination with an encryption key
pub type Nonce = [u8; 12];
/// Type representing a 32-byte checksum hash used to validate data integrity
pub type Checksum = [u8; 32];
/// Contains extra options for some API functions
pub mod options {
    use std::{collections::VecDeque, path::PathBuf};

    pub struct EncryptionOptions {
        /// Don't replace the name with a random UUID for the encrypted file
        pub keep_original_name: bool,
        /// Contains an output path for each file
        pub output_paths: Option<VecDeque<PathBuf>>
    }

    pub struct DecryptionOptions {
        /// Contains an output path for each file
        pub output_paths: Option<VecDeque<PathBuf>>
    }

    pub struct GetKeyOptions {
        /// Format encryption key as list of bytes
        pub byte_format: bool
    }
}

/// Encrypts the file at the given path. Extra options can be provided to control the process
///
/// Requires the password for the current profile in order to authenticate user and start the
/// encryption process
///
/// # Errors
/// Most errors can be safely handled without an unsuccessful exit (e.g. file can just be skipped).
/// Although it is better to exit on errors related with user authentication and profiles, as the
/// program will simply not work without a user profile
pub fn encrypt(password: &str, file_path: &std::path::Path, options: &mut options::EncryptionOptions) -> Result<()> {
    core::encrypt(password, file_path, options)
}

/// Decrypts the file at the given path. Extra options can be provided to control the process.
/// Works similarly to the `encrypt` function just the other way around
///
/// Requires the password for the current profile in order to authenticate user and start the
/// decryption process
///
/// # Errors
/// Most errors can be safely handled without an unsuccessful exit (e.g. file can just be skipped).
/// Although it is better to exit on errors related with user authentication and profiles, as the
/// program will simply not work without a user profile
pub fn decrypt(password: &str, file_path: &std::path::Path, options: &mut options::DecryptionOptions) -> Result<()> {
    core::decrypt(password, file_path, options)
}

/// Creates a new profile with the provided password and profile name. Will **not** automatically
/// switch to the new profile
///
/// No user authentication needed, as it just creates a new profile
///
/// # Errors
/// Any error suggests that the function failed and should be the reason for an unsuccessful exit
///
/// * `IOError` - in case of failing to access or write to a `profiles.json` file
/// * `CipherError` - unsuccessful attempt to hash the password
pub fn create_profile(password: &str, profile_name: &str) -> Result<()> {
    core::create_profile(password, profile_name)
}

/// Deletes the profile with the corresponding name. After deletion will switch back to the first
/// profile in the list or if there are no profiles left set the current profile to `None`
///
/// Needs the target profile's password to authenticate
///
/// # Errors
/// Any error suggests that the function failed and should be the reason for an unsuccessful exit
///
/// * `AuthError` - invalid password for the target profile
/// * `ProfileError` - if the target profile is not found
/// * `IOError` - in case of failing to access or write to a `profiles.json` file
pub fn delete_profile(password: &str, profile_name: &str) -> Result<()> {
    core::delete_profile(password, profile_name)
}

/// Select (set as the current) the profile with the corresponding name
///
/// Needs the target profile's password to authenticate
///
/// # Errors
/// Any error suggests that the function failed and should be the reason for an unsuccessful exit
///
/// * `AuthError` - invalid password for the target profile
/// * `InvalidInput` - if the target profile is already set to be the current one
/// * `ProfileError` - if the target profile is not found
/// * `IOError` - in case of failing to access or write to a `profiles.json` file
pub fn select_profile(password: &str, profile_name: &str) -> Result<()> {
    core::select_profile(password, profile_name)
}

/// Returns the name of the currently selected profile
///
/// No authentication needed, as it just returns the name
///
/// # Errors
/// Any error suggests that the function failed and should be the reason for an unsuccessful exit
///
/// * `ProfileError` - if no profile is currently selected
/// * `IOError` - in case of failing to access or write to a `profiles.json` file
pub fn get_profile() -> Result<String> {
    core::get_profile()
}

/// Returns the names of all currently available profiles
///
/// No authentication needed, as it just returns the names
///
/// # Errors
/// Any error suggests that the function failed and should be the reason for an unsuccessful exit
///
/// * `ProfileError` - if no profile data is found (no profiles exist)
/// * `IOError` - in case of failing to access or write to a `profiles.json` file
pub fn get_profiles() -> Result<Vec<String>> {
    core::get_profiles()
}

/// Generates a new encryption key for the current profile
///
/// **Warning:** this will replace the current encryption key, meaning that currently encrypted
/// files will no longer be able to be decrypted due to a different key being used
///
/// Needs the current profile's password to authenticate
///
/// # Errors
/// Any error suggests that the function failed and should be the reason for an unsuccessful exit
///
/// * `AuthError` - invalid password for the current profile
/// * `ProfileError` - if there is no current profile or no profiles found in general
/// * `IOError` - in case of failing to access or write to a `profiles.json` file
pub fn new_key(password: &str) -> Result<()> {
    core::new_key(password)
}

/// Returns the encryption key being used by the current profile in a hex format
///
/// Needs the current profile's password to authenticate
///
/// # Errors
/// Any error suggests that the function failed and should be the reason for an unsuccessful exit
///
/// * `AuthError` - invalid password for the current profile
/// * `ProfileError` - if there is no current profile or no profiles found in general
/// * `IOError` - in case of failing to access or write to a `profiles.json` file
pub fn get_key(password: &str, options: options::GetKeyOptions) -> Result<String> {
    core::get_key(password, options)
}

/// Sets a new encryption key for the current profile. The input key has to be a valid 32-byte long
/// hex key for it to work (e.g. input key of `"0128AE1005..."` translates to `[1, 40, 174, 16, 5,
/// ...]`)
///
/// Needs the current profile's password to authenticate
///
/// # Errors
/// Any error suggests that the function failed and should be the reason for an unsuccessful exit
///
/// * `AuthError` - invalid password for the current profile
/// * `InvalidInput` - if the provided key is incorrect and cannot be parsed into a 32 byte array
/// * `ProfileError` - if there is no current profile or no profiles found in general
/// * `IOError` - in case of failing to access or write to a `profiles.json` file
pub fn set_key(password: &str, new_key: &str) -> Result<()> {
    core::set_key(password, new_key)
}