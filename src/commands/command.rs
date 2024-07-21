use std::path::{Path, PathBuf};
use std::{fs, io};
use clap::ArgMatches;
use crate::encryption::{cipher, file, parser, storage};
use crate::{log_debug, log_info, log_success, log_warn};

pub fn encrypt_box(args: &ArgMatches, input_path: &Path) -> io::Result<()> {
    if input_path.extension().unwrap() == "box" {
        log_warn!("\"{:?}\" is already encrypted! Skipping", input_path.file_name().unwrap().to_os_string());
        return Ok(());
    }
    log_success!("Boxing file: \"{:?}\"", input_path.file_name().unwrap().to_os_string());
    let mut path_buffer = PathBuf::from(input_path);

    // get needed pathes
    let file_path = path_buffer.as_path();

    // get needed data
    let file_data = file::read_bytes(file_path)?;
    let checksum = parser::generate_checksum(&file_data);
    let key = storage::get_key()?;
    let nonce = cipher::generate_nonce();
    let header = parser::generate_header(file_path, checksum, nonce).expect("Error generating header");

    log_info!("Saving keys");
    storage::save_key(&key)?;

    log_info!("Changing file");
    // change the file to be .box instead
    fs::remove_file(file_path)?;
    if let Some(new_name) = args.get_one::<String>("custom-name") {
        path_buffer.set_file_name(new_name);
    } else if !args.get_flag("keep-name") {
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

pub fn decrypt_box(_args: &ArgMatches, input_path: &Path) -> io::Result<()> {
    if input_path.extension().unwrap() != "box" {
        log_warn!("\"{:?}\" is not encrypted! Skipping", input_path.file_name().unwrap().to_os_string());
        return Ok(());
    }
    log_success!("Unboxing file: \"{:?}\"", input_path.file_name().unwrap().to_os_string());
    let mut path_buffer = PathBuf::from(input_path);

    // get needed pathes
    let file_path = path_buffer.as_path();

    log_info!("Reading data");
    let key = storage::get_key()?;
    let (header, body) = parser::parse_file(file_path, key)?;
    log_debug!("Got header: {:?}", header);

    log_info!("Validating checksum");
    let new_checksum = parser::generate_checksum(&body);
    if new_checksum != header.checksum {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Checksum verification failed")
        );
    }

    log_info!("Changing file");
    // change the file to its original form
    fs::remove_file(file_path)?;
    path_buffer.set_file_name(&header.original_filename);
    path_buffer.set_extension(&header.original_extension);
    let file_path = path_buffer.as_path();

    log_info!("Writing data");
    file::write_bytes(&file_path, &body)?;

    Ok(())
}