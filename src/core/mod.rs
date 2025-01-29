//! Contains the core functionality of the program

use std::fs;
use std::path::Path;
use crate::core::data::{io, keys};
use crate::core::encryption::{boxfile, cipher};
use crate::{Result, log_debug, log_info, log_warn, new_err, options, Key};
use crate::core::data::profile::Profile;

pub mod utils;
pub mod error;
pub mod data;
pub mod encryption;

/// Encrypts the file at provided path using current profile's key. Password is required to verify
/// and get access to current profile. Additional options can be supplied to change the encryption
/// process
pub fn encrypt(password: &str, input_path: &Path, opts: &mut options::EncryptionOptions) -> Result<()> {
    log_info!("Starting encryption...");
    if let Some(extension) = input_path.extension() {
        if extension == "box" {
            return Err(new_err!(InvalidInput: InvalidFile, "Already encrypted"))
        }
    }

    let mut boxfile = boxfile::Boxfile::new(input_path)?;
    let key = keys::get_key(password)?;
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
    fs::remove_file(&input_path)?;

    Ok(())
}

/// Decryption the file at provided path using current profile's key. Password is required to
/// verify and get access to current profile. Additional options can be supplied to change the
/// decryption process
pub fn decrypt(password: &str, input_path: &Path, opts: &mut options::DecryptionOptions) -> Result<()> {
    log_info!("Starting decryption...");
    let mut boxfile = boxfile::Boxfile::parse(&input_path)?;
    let key = keys::get_key(password)?;
    boxfile.decrypt_data(&key)?;
    let (original_name, original_extension) = boxfile.file_info();
    let file_data = boxfile.file_data()?;

    log_info!("Validating checksum...");
    if boxfile.verify_checksum()? {
        log_info!("Checksum verification successful");
    } else {
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
                let mut path = input_path.to_path_buf();
                path.set_file_name(original_name);
                path.set_extension(original_extension);
                path
            }
        },
        None => {
            let mut path = input_path.to_path_buf();
            path.set_file_name(original_name);
            path.set_extension(original_extension);
            path
        }
    };
    
    io::write_bytes(&output_path, &file_data, true)?;
    fs::remove_file(&input_path)?;

    Ok(())
}

pub fn create_profile(password: &str, profile_name: &str) -> Result<()> {
    log_info!("Creating a new profile with name \"{}\"", profile_name);
    let mut profiles = data::get_profiles()?;
    profiles.new_profile(Profile::new(profile_name, password)?)?;
    Ok(())
}

pub fn delete_profile(password: &str, profile_name: &str) -> Result<()> {
    log_info!("Deleting profile \"{}\"", profile_name);
    let mut profiles = data::get_profiles()?;

    profiles.delete_profile(password, profile_name)?;
    Ok(())
}

pub fn select_profile(password: &str, profile_name: &str) -> Result<()> {
    log_info!("Switching profile to \"{}\"", profile_name);
    let mut profiles = data::get_profiles()?;

    if let Ok(profile) = profiles.get_current_profile() {
        if profile_name == profile.name {
            return Err(new_err!(ProfileError: AlreadySelected, profile_name))
        }
    }

    profiles.set_current(password, profile_name)?;
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
    log_info!("Generating a new encryption key for current profile");
    let key = cipher::generate_key();
    keys::set_key(password, key)?;
    Ok(())
}

pub fn get_key(password: &str, opts: options::GetKeyOptions) -> Result<String> {
    log_info!("Retrieving the encryption key from the current profile");
    let key = keys::get_key(password)?;
    
    if !opts.byte_format {
        return Ok(format!("{:?}", key))
    }
    Ok(utils::hex::bytes_to_string(&key))
}

pub fn set_key(password: &str, new_key: &str) -> Result<()> {
    log_info!("Setting the encryption key from the current profile");
    let new_key = utils::hex::string_to_bytes(new_key)?;
    
    if new_key.len() != 32 {
        return Err(new_err!(InvalidData: InvalidHex, "Provided hex is not a 32-byte key"))
    }
    
    let new_key = Key::try_from(&new_key[..32]).unwrap();
    keys::set_key(password, new_key)?;
    Ok(())
}

