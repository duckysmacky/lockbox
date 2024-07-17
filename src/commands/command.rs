use std::path::{Path, PathBuf};
use std::{fs, io};

use crate::encryption::{cipher, file, parser, storage};

pub fn encrypt_box(input_path: &Path) -> io::Result<()> {
    if input_path.extension().unwrap() == "box" {
        println!("{} is already encrypted! Skipping", input_path.display());
        return Ok(());
    }
    println!("Boxing {}", input_path.display());
    let mut path_buffer = PathBuf::from(input_path);

    // get needed pathes
    let file_path = path_buffer.as_path();

    // get needed data
    let file_data = file::read_bytes(file_path)?;
    let checksum = parser::generate_checksum(&file_data);
    let key = storage::get_key()?;
    let nonce = cipher::generate_nonce();
    let header = parser::generate_header(file_path, checksum, nonce).expect("Error generating header");

    println!("Saving keys...");
    storage::save_key(&key)?;

    println!("Changing file...");
    // change the file to be .box instead
    fs::remove_file(file_path)?;
    path_buffer.set_file_name(uuid::Uuid::new_v4().to_string());
    path_buffer.set_extension("box");
    let file_path = path_buffer.as_path();

    println!("Encrypting data...");
    let body = cipher::encrypt(&key, &nonce, &file_data).expect("Error encrypting file");

    println!("Writing data...");
    parser::write_file(file_path, header, body)?;

    Ok(())
}

pub fn decrypt_box(input_path: &Path) -> io::Result<()> {
    if input_path.extension().unwrap() != "box" {
        println!("{} is not encrypted! Skipping", input_path.display());
        return Ok(());
    }
    println!("Unboxing {}", input_path.display());
    let mut path_buffer = PathBuf::from(input_path);

    // get needed pathes
    let file_path = path_buffer.as_path();

    println!("Reading data...");
    let key = storage::get_key()?;
    let (header, body) = parser::parse_file(file_path, key)?;
    println!("Got header: {:?}", header);

    println!("Validating checksum...");
    let new_checksum = parser::generate_checksum(&body);
    if new_checksum != header.checksum {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Checksum verification failed")
        );
    }

    println!("Changing file...");
    // change the file to its original form
    fs::remove_file(file_path)?;
    path_buffer.set_file_name(&header.original_filename);
    path_buffer.set_extension(&header.original_extension);
    let file_path = path_buffer.as_path();

    println!("Writing data...");
    file::write_bytes(&file_path, &body)?;

    Ok(())
}