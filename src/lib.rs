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

pub type Key = [u8; 32];
pub type Nonce = [u8; 12];
pub type Checksum = [u8; 32];

/// Contains extra options for each API function
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

pub fn decrypt(password: &str, input_path: &Path, opts: &mut options::DecryptionOptions) -> Result<()> {
    let profile = profiles::get_current_profile()?;

    if !auth::verify_password(password, profile) {
        return Err(Error::AuthError("Invalid password entered".to_string()))
    }

    if let Some(extension) = input_path.extension() {
        if extension != "box" {
            return Err(Error::InvalidInput("This file is not encrypted".to_string()))
        }
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

pub fn create_profile(password: &str, profile_name: &str) -> Result<()> {
    log_info!("Creating a new profile with name \"{}\"", profile_name);
    profiles::create_new_profile(profile_name, password)
}

pub fn delete_profile(password: &str, profile_name: &str) -> Result<()> {
    let profile = profiles::get_profile(profile_name)?;

    if !auth::verify_password(password, profile) {
        return Err(Error::AuthError("Invalid password entered".to_string()))
    }

    log_info!("Deleting profile \"{}\"", profile_name);
    profiles::delete_profile(profile_name)
}

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

pub fn get_profile() -> Result<String> {
    log_info!("Getting current profile");

    let profile = profiles::get_current_profile()?;
    Ok(profile.name)
}

pub fn get_profiles() -> Result<Vec<String>> {
    log_info!("Listing all available profiles");

    let profiles = profiles::get_profiles()?.iter()
        .map(|p| p.name.to_string())
        .collect::<Vec<String>>();
    Ok(profiles)
}

pub fn new_key(password: &str) -> Result<()> {
    let profile = profiles::get_current_profile()?;

    if !auth::verify_password(password, profile) {
        return Err(Error::AuthError("Invalid password entered".to_string()))
    }

    log_info!("Generating a new encryption key for current profile");
    keys::generate_new_key()
}

pub fn get_key(password: &str, opts: options::GetKeyOptions) -> Result<String> {
    let profile = profiles::get_current_profile()?;

    if !auth::verify_password(password, profile) {
        return Err(Error::AuthError("Invalid password entered".to_string()))
    }

    log_info!("Retrieving the encryption key from the current profile");
    let key = keys::get_key()?;
    if !opts.byte_format {
        return match std::str::from_utf8(&key) {
            Ok(str_key) => Ok(str_key.to_string()),
            Err(_) => Err(Error::InvalidData("Unable to convert encryption key into a UTF-8 string format".to_string())),
        };
    }
    Ok(format!("{:?}", key))
}