use std::{path::{Path, PathBuf}, fs, io};

use crate::encryption::{cipher, file, parser};
use crate::{log_info, log_success, log_warn, storage};

pub struct EncryptionOptions {
    pub keep_name: bool,
    pub output_paths: Option<Vec<PathBuf>>
}

pub fn encrypt(input_path: &Path, opts: &EncryptionOptions) -> io::Result<()> {
    if input_path.extension().unwrap() == "box" {
        log_warn!("\"{:?}\" is already encrypted! Skipping", input_path.file_name().unwrap().to_os_string());
        return Ok(());
    }

    log_success!("Boxing file: \"{:?}\"", input_path.file_name().unwrap().to_os_string());
    let mut path_buffer = PathBuf::from(input_path);

    // get needed paths
    let file_path = path_buffer.as_path();

    // get needed data
    let file_data = file::read_bytes(file_path)?;
    let checksum = parser::generate_checksum(&file_data);
    let key = storage::get_key()?;
    let nonce = cipher::generate_nonce();
    let header = parser::generate_header(file_path, checksum, nonce).expect("Error generating header");

    log_info!("Saving keys");
    storage::save_key(&key)?;

    log_info!("Converting file");
    // change the file to be .box instead
    fs::remove_file(file_path)?;
    // if let Some(new_name) = opts.custom_name {
    //     path_buffer.set_file_name(new_name);
    // }
    if !opts.keep_name {
        path_buffer.set_file_name(uuid::Uuid::new_v4().to_string());
    }
    path_buffer.set_extension("box");
    let file_path = path_buffer.as_path();

    log_info!("Encrypting data");
    let body = cipher::encrypt(&key, &nonce, &file_data).expect("Error encrypting file");

    log_info!("Writing data");
    parser::write_file(file_path, header, body)?;

    Ok(())
}