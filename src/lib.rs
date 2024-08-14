use std::fs;
use std::path::{Path, PathBuf};
use crate::encryption::{checksum, cipher};
use crate::file::{parser, io, header};
use crate::storage::keys;

pub use crate::error::{Error, Result};

pub mod cli;
mod encryption;
mod storage;
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
    let file_data = io::read_bytes(file_path).map_err(Error::from)?;
    let key = keys::get_key().key;
    let nonce = cipher::generate_nonce();
    let header = header::generate_header(file_path, &file_data, &nonce);

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
    let body = cipher::encrypt(&key, &nonce, &file_data);

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
    let key = keys::get_key().key;
    let box_file = parser::parse_file(file_path)?;
    let header = box_file.header;
    let body = cipher::decrypt(&key, &header.nonce, &box_file.body);

    log_debug!("Validating checksum");
    let new_checksum = checksum::generate_checksum(&body);
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
    io::write_bytes(&file_path, &body).map_err(Error::from)?;

    Ok(())
}

pub fn new_key(_options: &options::NewKeyOptions) -> Result<()> {
    log_success!("Generating a new encryption key");
    keys::create_new_key();
    Ok(())
}

pub fn delete_key(_options: &options::DeleteKeyOptions) -> Result<()> {
    log_success!("Deleting encryption key");
    keys::delete_key();
    Ok(())
}