//! Contains everything which has to do with the encryption and decryption processes

use std::fs;
use std::path::{Path, PathBuf};
use crate::{Error, Result};
use crate::options;
use crate::log_debug;
use super::data::{io, keys, profile};
use super::file::{header, parser};

pub mod cipher;
pub mod checksum;

/// Encrypts the file at provided path using current profile's key. Password is required to verify
/// and get access to current profile. Additional options can be supplied to change the encryption
/// process
pub fn encrypt(password: &str, input_path: &Path, opts: &mut options::EncryptionOptions) -> Result<()> {
    let mut profiles = profile::get_profiles();
    let profile = profiles.get_current_profile()?;

    if !profile.verify_password(password) {
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
    let file_data = io::read_bytes(file_path).map_err(crate::Error::from)?;
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

/// Decryption the file at provided path using current profile's key. Password is required to
/// verify and get access to current profile. Additional options can be supplied to change the
/// decryption process
pub fn decrypt(password: &str, input_path: &Path, opts: &mut options::DecryptionOptions) -> Result<()> {
    let mut profiles = profile::get_profiles();
    let profile = profiles.get_current_profile()?;

    if !profile.verify_password(password) {
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
    io::write_bytes(&file_path, &body, true).map_err(Error::from)?;

    Ok(())
}