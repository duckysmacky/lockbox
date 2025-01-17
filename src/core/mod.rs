//! Contains the core functionality of the program

use std::path::{Path, PathBuf};
use std::fs;
use crate::core::data::{io, keys};
use crate::core::encryption::{boxfile, checksum, cipher};
use crate::{Result, log_debug, log_info, log_warn, new_err, options};
use crate::core::data::profile::Profile;

pub mod utils;
pub mod error;
mod data;
mod encryption;

/// Encrypts the file at provided path using current profile's key. Password is required to verify
/// and get access to current profile. Additional options can be supplied to change the encryption
/// process
pub fn encrypt(password: &str, input_path: &Path, opts: &mut options::EncryptionOptions) -> Result<()> {
    let mut profiles = data::get_profiles()?;
    let profile = profiles.get_current_profile()?;

    if !profile.verify_password(password) {
        return Err(new_err!(ProfileError: AuthenticationFailed))
    }

    if let Some(extension) = input_path.extension() {
        if extension == "box" {
            return Err(new_err!(InvalidInput: FileAlreadyEncrypted, os input_path.file_name().unwrap()))
        }
    }

    let mut path_buffer = PathBuf::from(input_path);
    let file_path = path_buffer.as_path();

    // get needed data
    let file_data = io::read_bytes(file_path).map_err(crate::Error::from)?;
    let key = keys::get_key()?;
    let nonce = cipher::generate_nonce();
    let header = boxfile::generate_header(file_path, &file_data, &nonce)?;

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
    boxfile::write_file(file_path, header, body)?;

    Ok(())
}

/// Decryption the file at provided path using current profile's key. Password is required to
/// verify and get access to current profile. Additional options can be supplied to change the
/// decryption process
pub fn decrypt(password: &str, input_path: &Path, opts: &mut options::DecryptionOptions) -> Result<()> {
    let mut profiles = data::get_profiles()?;
    let profile = profiles.get_current_profile()?;

    if !profile.verify_password(password) {
        return Err(new_err!(ProfileError: AuthenticationFailed))
    }

    if let Some(extension) = input_path.extension() {
        if extension != "box" {
            return Err(new_err!(InvalidInput: FileNotSupported, os input_path.file_name().unwrap()))
        }
    } else {
        return Err(new_err!(InvalidInput: FileNotSupported, os input_path.file_name().unwrap()))
    }

    let mut path_buffer = PathBuf::from(input_path);
    let file_path = path_buffer.as_path();

    let key = keys::get_key()?;
    let box_file = boxfile::parse_file(file_path)?;
    let header = box_file.header;
    let body = cipher::decrypt(&key, &header.nonce, &box_file.body)?;

    log_debug!("Validating checksum");
    let new_checksum = checksum::generate_checksum(&body);
    if new_checksum != header.checksum {
        log_warn!("Checksum verification failed. Data seems to be tampered with");
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
    io::write_bytes(&file_path, &body, true)?;

    Ok(())
}

pub fn create_profile(password: &str, profile_name: &str) -> Result<()> {
    log_info!("Creating a new profile with name \"{}\"", profile_name);
    let mut profiles = data::get_profiles()?;
    profiles.new_profile(Profile::new(profile_name, password)?)?;
    Ok(())
}

pub fn delete_profile(password: &str, profile_name: &str) -> Result<()> {
    let mut profiles = data::get_profiles()?;
    let profile = profiles.get_current_profile()?;

    if !profile.verify_password(password) {
        return Err(new_err!(ProfileError: AuthenticationFailed))
    }

    log_info!("Deleting profile \"{}\"", profile_name);
    profiles.delete_profile(profile_name)?;
    Ok(())
}

pub fn select_profile(password: &str, profile_name: &str) -> Result<()> {
    let mut profiles = data::get_profiles()?;
    let profile = profiles.find_profile(profile_name)?;

    if !profile.verify_password(password) {
        return Err(new_err!(ProfileError: AuthenticationFailed))
    }

    if let Ok(profile) = profiles.get_current_profile() {
        if profile_name == profile.name {
            return Err(new_err!(ProfileError: AlreadySelected, profile_name))
        }
    }

    log_info!("Switching profile to \"{}\"", profile_name);
    profiles.set_current(profile_name)?;
    Ok(())
}

pub fn get_profile() -> Result<String> {
    log_info!("Getting current profile");
    let mut profiles = data::get_profiles()?;
    let profile = profiles.get_current_profile()?;
    Ok(profile.name.to_string())
}

pub fn get_profiles() -> Result<Vec<String>> {
    log_info!("Listing all available profiles");

    let profiles = data::get_profiles()?;
    let profile_list = profiles.get_profiles().into_iter()
        .map(|p| p.name.to_string())
        .collect::<Vec<String>>();
    Ok(profile_list)
}

pub fn new_key(password: &str) -> Result<()> {
    let mut profiles = data::get_profiles()?;
    let profile = profiles.get_current_profile()?;

    if !profile.verify_password(password) {
        return Err(new_err!(ProfileError: AuthenticationFailed))
    }

    log_info!("Generating a new encryption key for current profile");
    keys::set_key(cipher::generate_key())?;
    Ok(())
}

pub fn get_key(password: &str, opts: options::GetKeyOptions) -> Result<String> {
    let mut profiles = data::get_profiles()?;
    let profile = profiles.get_current_profile()?;

    if !profile.verify_password(password) {
        return Err(new_err!(ProfileError: AuthenticationFailed))
    }

    log_info!("Retrieving the encryption key from the current profile");
    let key = keys::get_key()?;
    if !opts.byte_format {
        return Ok(utils::hex::key_to_hex_string(key));
    }

    Ok(format!("{:?}", key))
}

pub fn set_key(password: &str, new_key: &str) -> Result<()> {
    let mut profiles = data::get_profiles()?;
    let profile = profiles.get_current_profile()?;

    if !profile.verify_password(password) {
        return Err(new_err!(ProfileError: AuthenticationFailed))
    }

    log_info!("Setting the encryption key from the current profile");
    let new_key = utils::hex::hex_string_to_key(new_key.to_string())?;
    keys::set_key(new_key)?;
    Ok(())
}

