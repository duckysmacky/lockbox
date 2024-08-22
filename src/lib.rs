use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use crate::encryption::{checksum, cipher};
use crate::file::{parser, io, header};
use crate::data::{auth, keys, profiles};

pub use crate::error::{Error, Result};

pub mod cli;
mod encryption;
mod data;
mod error;
mod file;

pub mod options {
    use std::{collections::VecDeque, path::PathBuf};

    pub struct EncryptionOptions {
        pub keep_name: bool,
        pub output_paths: Option<VecDeque<PathBuf>>
    }

    pub struct DecryptionOptions {
        pub output_paths: Option<VecDeque<PathBuf>>
    }
}

pub fn encrypt(input_path: &Path, password: &str, opts: &mut options::EncryptionOptions) -> Result<()> {
    let profile = profiles::get_current_profile()?;

    if !auth::verify_password(password, profile) {
        return Err(Error::AuthError("Invalid password entered".to_string()))
    }

    if let Some(extension) = input_path.extension() {
        if extension == "box" {
            return Err(Error::InvalidFile(format!("\"{}\" is already encrypted", input_path.display())))
        }
    }

    let file_name = input_path.file_name().unwrap_or(OsStr::new("unknown file name")).to_os_string();
    log_success!("Encrypting file {:?}", file_name);

    let mut path_buffer = PathBuf::from(input_path);
    let file_path = path_buffer.as_path();

    // get needed data
    let file_data = io::read_bytes(file_path).map_err(Error::from)?;
    let key = keys::get_key()?;
    let nonce = cipher::generate_nonce();
    let header = header::generate_header(file_path, &file_data, &nonce);

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
    } else if !opts.keep_name {
        path_buffer.set_file_name(uuid::Uuid::new_v4().to_string());
    }

    path_buffer.set_extension("box");
    let file_path = path_buffer.as_path();

    let body = cipher::encrypt(&key, &nonce, &file_data);
    parser::write_file(file_path, header, body)?;

    Ok(())
}

pub fn decrypt(input_path: &Path, password: &str, opts: &mut options::DecryptionOptions) -> Result<()> {
    let profile = profiles::get_current_profile()?;

    if !auth::verify_password(password, profile) {
        return Err(Error::AuthError("Invalid password entered".to_string()))
    }

    if let Some(extension) = input_path.extension() {
        if extension != "box" {
            return Err(Error::InvalidFile(format!("\"{}\" cannot be decrypted", input_path.display())))
        }
    }

    let file_name = input_path.file_name().unwrap_or(OsStr::new("unknown file name")).to_os_string();
    log_success!("Decrypting file {:?}", file_name);

    let mut path_buffer = PathBuf::from(input_path);
    let file_path = path_buffer.as_path();

    let key = keys::get_key()?;
    let box_file = parser::parse_file(file_path)?;
    let header = box_file.header;
    let body = cipher::decrypt(&key, &header.nonce, &box_file.body);

    log_debug!("Validating checksum");
    let new_checksum = checksum::generate_checksum(&body);
    if new_checksum != header.checksum {
        return Err(Error::InvalidChecksum(file_name));
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

pub fn create_profile(profile_name: &str, password: &str) -> Result<()> {
    log_info!("Creating a new profile with name \"{}\"", profile_name);
    profiles::create_new_profile(profile_name, password)
}

pub fn delete_profile(profile_name: &str, password: &str) -> Result<()> {
    let profile = profiles::get_profile(profile_name)?;

    if !auth::verify_password(password, profile) {
        return Err(Error::AuthError("Invalid password entered".to_string()))
    }

    log_info!("Deleting profile \"{}\"", profile_name);
    profiles::delete_profile(profile_name)
}

pub fn new_key(password: &str) -> Result<()> {
    let profile = profiles::get_current_profile()?;

    if !auth::verify_password(password, profile) {
        return Err(Error::AuthError("Invalid password entered".to_string()))
    }

    log_info!("Generating a new encryption key for current profile");
    keys::generate_new_key()
}

pub fn get_key(_password: &str) -> Result<String> {
    todo!()
}