//! Contains the core functionality of the program

use std::path::Path;
use crate::core::data::{io, keys};
use crate::core::encryption::{boxfile, cipher};
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

    let mut boxfile = boxfile::Boxfile::new(input_path)?;
    let key = keys::get_key()?;
    boxfile.encrypt_data(&key)?;

    let mut output_path = match opts.output_paths {
        Some(ref mut paths) => {
            if let Some(mut path) = paths.pop_front() {
                log_debug!("Writing to custom output path: {:?}", path);

                if path.file_name() == None {
                    path.set_file_name(uuid::Uuid::new_v4().to_string());
                }
                path
            } else {
                input_path.to_path_buf()
            }
        },
        None => input_path.to_path_buf()
    };
    
    if !opts.keep_original_name {
        output_path.set_file_name(uuid::Uuid::new_v4().to_string());
    }

    output_path.set_extension("box");
    boxfile.save_to(&output_path)?;

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

    let mut boxfile = boxfile::Boxfile::parse(&input_path)?;
    let key = keys::get_key()?;
    let file_data = boxfile.decrypt_data(&key)?;
    let (original_name, original_extension) = boxfile.file_info();

    log_info!("Validating checksum");
    if !boxfile.verify_checksum()? {
        log_warn!("Checksum verification failed. Data seems to be tampered with");
    }

    let output_path = match opts.output_paths {
        Some(ref mut paths) => {
            if let Some(mut path) = paths.pop_front() {
                log_debug!("Writing to custom output path: {:?}", path);

                if path.file_name() == None {
                    path.set_file_name(original_name);
                    path.set_extension(original_extension);
                } else if path.extension() == None {
                    path.set_extension(original_extension);
                }
                path
            } else {
                let mut path = input_path.join(original_name);
                path.set_extension(original_extension);
                path
            }
        },
        None => {
            let mut path = input_path.join(original_name);
            path.set_extension(original_extension);
            path
        }
    };
    
    io::write_bytes(&output_path, &file_data, true)?;

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

