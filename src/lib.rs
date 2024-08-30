//! # Lockbox API
//!
//! This library functions like an API between the CLI and GUI wrappers and the rest of the core
//! program code. It contains all the main functions related to the Lockbox's functionality, like
//! encryption, decryption, key and profile management.

use std::fs;
use std::path::{Path, PathBuf};
use crate::encryption::{checksum, cipher};
use crate::file::{header, io, parser};
use crate::data::{auth, keys, profiles};

pub use error::{Result, Error};

pub mod cli;
mod encryption;
mod data;
mod file;
mod error;
mod utils;

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
pub fn encrypt(password: &str, input_path: &Path, opts: &mut options::EncryptionOptions) -> Result<()> {
    let profile = profiles::get_current_profile()?;

    if !auth::verify_password(password, profile) {
        return Err(Error::AuthError("Invalid password entered".to_string()))
    }

    if let Some(extension) = input_path.extension() {
        if extension == "box" {
            return Err(Error::InvalidInput("This file is already encrypted".to_string()))
        }
    }

    let mut path_buffer = PathBuf::from(input_path);
    let file_path = path_buffer.as_path();

    // get needed data
    let file_data = io::read_bytes(file_path).map_err(Error::from)?;
    let key = keys::get_key()?;
    let nonce = cipher::generate_nonce();
    let header = header::generate_header(file_path, &file_data, &nonce)?;

    // change the file to be .box instead
    fs::remove_file(file_path)?;

    if let Some(ref mut output_paths) = opts.output_paths {
        log_debug!("Output paths given: {:?}", output_paths);
        if let Some(output_path) = output_paths.pop_front() {
            log_debug!("Writing output to: {:?}", output_path);
            path_buffer = output_path;

            if path_buffer.file_name() == None {
                path_buffer.set_file_name(uuid::Uuid::new_v4().to_string());
            }
        }
    } else if !opts.keep_original_name {
        path_buffer.set_file_name(uuid::Uuid::new_v4().to_string());
    }

    path_buffer.set_extension("box");
    let file_path = path_buffer.as_path();

    let body = cipher::encrypt(&key, &nonce, &file_data)?;
    parser::write_file(file_path, header, body)?;

    Ok(())
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
pub fn decrypt(password: &str, input_path: &Path, opts: &mut options::DecryptionOptions) -> Result<()> {
    let profile = profiles::get_current_profile()?;

    if !auth::verify_password(password, profile) {
        return Err(Error::AuthError("Invalid password entered".to_string()))
    }

    if let Some(extension) = input_path.extension() {
        if extension != "box" {
            return Err(Error::InvalidInput("This file is not encrypted".to_string()))
        }
    } else {
        return Err(Error::InvalidInput("This file is not encrypted".to_string()))
    }

    let mut path_buffer = PathBuf::from(input_path);
    let file_path = path_buffer.as_path();

    let key = keys::get_key()?;
    let box_file = parser::parse_file(file_path)?;
    let header = box_file.header;
    let body = cipher::decrypt(&key, &header.nonce, &box_file.body)?;

    log_debug!("Validating checksum");
    let new_checksum = checksum::generate_checksum(&body);
    if new_checksum != header.checksum {
        return Err(Error::InvalidData("Checksum verification failed (data was probably tampered with)".to_string()));
    }

    fs::remove_file(file_path)?;
    if let Some(ref mut output_paths) = opts.output_paths {
        log_debug!("Output paths given: {:?}", output_paths);
        if let Some(output_path) = output_paths.pop_front() {
            log_debug!("Writing output to: {:?}", output_path);
            path_buffer = output_path;

            if path_buffer.file_name() == None {
                path_buffer.set_file_name(&header.original_filename);
                path_buffer.set_extension(&header.original_extension);
            }

            if path_buffer.extension() == None {
                path_buffer.set_extension(&header.original_extension);
            }
        }
    } else {
        path_buffer.set_file_name(&header.original_filename);
        path_buffer.set_extension(&header.original_extension);
    }

    let file_path = path_buffer.as_path();
    io::write_bytes(&file_path, &body).map_err(Error::from)?;

    Ok(())
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
    log_info!("Creating a new profile with name \"{}\"", profile_name);
    profiles::create_new_profile(profile_name, password)
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
    let profile = profiles::get_profile(profile_name)?;

    if !auth::verify_password(password, profile) {
        return Err(Error::AuthError("Invalid password entered".to_string()))
    }

    log_info!("Deleting profile \"{}\"", profile_name);
    profiles::delete_profile(profile_name)
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
    let profile = profiles::get_profile(profile_name)?;

    if !auth::verify_password(password, profile) {
        return Err(Error::AuthError("Invalid password entered".to_string()))
    }

    if profile_name == profiles::get_current_profile()?.name {
        return Err(Error::InvalidInput(format!("Current profile is already set to \"{}\"", profile_name)))
    }

    log_info!("Switching profile to \"{}\"", profile_name);
    profiles::set_current_profile(profile_name)
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
    log_info!("Getting current profile");

    let profile = profiles::get_current_profile()?;
    Ok(profile.name)
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
    log_info!("Listing all available profiles");

    let profiles = profiles::get_profiles()?.iter()
        .map(|p| p.name.to_string())
        .collect::<Vec<String>>();
    Ok(profiles)
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
    let profile = profiles::get_current_profile()?;

    if !auth::verify_password(password, profile) {
        return Err(Error::AuthError("Invalid password entered".to_string()))
    }

    log_info!("Generating a new encryption key for current profile");
    keys::generate_new_key()
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
pub fn get_key(password: &str, opts: options::GetKeyOptions) -> Result<String> {
    let profile = profiles::get_current_profile()?;

    if !auth::verify_password(password, profile) {
        return Err(Error::AuthError("Invalid password entered".to_string()))
    }

    log_info!("Retrieving the encryption key from the current profile");
    let key = keys::get_key()?;
    if !opts.byte_format {
        return Ok(utils::hex::key_to_hex_string(key));
    }
    Ok(format!("{:?}", key))
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
	let profile = profiles::get_current_profile()?;

    if !auth::verify_password(password, profile) {
        return Err(Error::AuthError("Invalid password entered".to_string()))
    }

    log_info!("Setting the encryption key from the current profile");
    let new_key = utils::hex::hex_string_to_key(new_key.to_string())?;
	keys::set_key(new_key)?;

	Ok(())
}