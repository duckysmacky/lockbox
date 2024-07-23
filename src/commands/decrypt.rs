use std::{path::{Path, PathBuf}, fs, io};
use crate::{log_debug, log_info, log_success, log_warn, storage};
use crate::commands::BoxOptions;
use crate::encryption::{file, parser};

pub fn decrypt(input_path: &Path, _opts: &BoxOptions) -> io::Result<()> {
    if input_path.extension().unwrap() != "box" {
        log_warn!("\"{:?}\" is not encrypted! Skipping", input_path.file_name().unwrap().to_os_string());
        return Ok(());
    }

    log_success!("Unboxing file: \"{:?}\"", input_path.file_name().unwrap().to_os_string());
    let mut path_buffer = PathBuf::from(input_path);

    // get needed paths
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