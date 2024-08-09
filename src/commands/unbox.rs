use std::{path::{Path, PathBuf}, collections::VecDeque, fs, io};

use crate::{log_debug, log_success, log_warn, storage};
use crate::encryption::{file, parser};

pub struct DecryptionOptions {
    pub output_paths: Option<VecDeque<PathBuf>>
}

pub fn decrypt(input_path: &Path, opts: &mut DecryptionOptions) -> io::Result<()> {
    if input_path.extension().unwrap() != "box" {
        log_warn!("\"{:?}\" is not encrypted! Skipping", input_path.file_name().unwrap().to_os_string());
        return Ok(());
    }

    log_success!("Decrypting file: {:?}", input_path.file_name().unwrap().to_os_string());
    let mut path_buffer = PathBuf::from(input_path);

    // get needed paths
    let file_path = path_buffer.as_path();

    log_debug!("Reading data");
    let key = storage::keys::get_key();
    let (header, body) = parser::parse_file(file_path, key)?;
    log_debug!("Got header: {:?}", header);

    log_debug!("Validating checksum");
    let new_checksum = parser::generate_checksum(&body);
    if new_checksum != header.checksum {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Checksum verification failed")
        );
    }

    log_debug!("Changing file");
    // change the file to its original form
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