use std::fs;
use std::path::{Path, PathBuf};
use crate::encryption::{cipher, file, parser};
use crate::storage::keys;

pub use crate::error::{Error, Result};

pub mod cli;
mod encryption;
mod storage;
mod error;

pub mod options {
    use std::{collections::VecDeque, path::PathBuf};

    pub struct EncryptionOptions {
        pub keep_name: bool,
        pub output_paths: Option<VecDeque<PathBuf>>
    }

    pub struct DecryptionOptions {
        pub output_paths: Option<VecDeque<PathBuf>>
    }

    pub struct KeyOptions {
        // TODO
    }

    pub struct NewKeyOptions {
        // TODO
        pub key_options: KeyOptions
    }

    pub struct DeleteKeyOptions {
        // TODO
        pub key_options: KeyOptions
    }
}

pub fn encrypt(input_path: &Path, opts: &mut options::EncryptionOptions) -> Result<()> {
    if input_path.extension().unwrap() == "box" {
        return Err(Error::InvalidFile(format!("\"{}\" is already encrypted", input_path.display())))
    }

    log_success!("Encrypting file \"{:?}\"", input_path.file_name().unwrap().to_os_string());

    let mut path_buffer = PathBuf::from(input_path);
    let file_path = path_buffer.as_path();

    // get needed data
    let file_data = file::read_bytes(file_path).map_err(Error::from)?;
    let checksum = parser::generate_checksum(&file_data);
    let key = keys::get_key();
    let nonce = cipher::generate_nonce();
    let header = parser::generate_header(file_path, checksum, nonce);

    log_debug!("Converting file");
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

    log_debug!("Encrypting data");
    let body = cipher::encrypt(&key, &nonce, &file_data).expect("Error encrypting file");

    log_debug!("Writing data");
    parser::write_file(file_path, header, body)?;

    Ok(())
}

pub fn decrypt(input_path: &Path, opts: &mut options::DecryptionOptions) -> Result<()> {
    if input_path.extension().unwrap() != "box" {
        return Err(Error::InvalidFile(format!("\"{}\" cannot be decrypted", input_path.display())))
    }

    let file_name = input_path.file_name().unwrap().to_os_string();
    log_success!("Decrypting file: {:?}", file_name);

    let mut path_buffer = PathBuf::from(input_path);
    let file_path = path_buffer.as_path();

    log_debug!("Reading data");
    let key = keys::get_key();
    let (header, body) = parser::parse_file(file_path, key)?;

    log_debug!("Validating checksum");
    let new_checksum = parser::generate_checksum(&body);
    if new_checksum != header.checksum {
        return Err(Error::InvalidChecksum(file_name));
    }

    log_debug!("Changing file");
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

    log_debug!("Writing data");
    file::write_bytes(&file_path, &body)?;

    Ok(())
}

pub fn new_key(_options: &options::NewKeyOptions) -> Result<()> {
    log_success!("Generating a new encryption key");
    keys::generate_new_key();
    Ok(())
}

pub fn delete_key(_options: &options::DeleteKeyOptions) -> Result<()> {
    log_success!("Deleting encryption key");
    keys::delete_key();
    Ok(())
}