use std::io::ErrorKind;
use std::path::PathBuf;
use std::{
    io::{Error, Result},
    fs
};

use crate::encryption::{cipher, file, parser, storage};

// TODO - (recursive) folder encryption
pub fn encrypt_box(input_path: &str) -> Result<()> {
    println!("Boxing {}", input_path);
    let mut path_buffer = PathBuf::from(input_path);

    // get needed pathes
    let file_path = path_buffer.as_path();

    // get needed data
    let data = file::read_bytes(file_path)?;
    let checksum = parser::generate_checksum(&data);
    let key = cipher::generate_key();
    let nonce = cipher::generate_nonce();
    let header = parser::generate_header(file_path, checksum, nonce).expect("Error generating header");

    println!("Saving keys...");
    storage::save_key(&key)?;

    println!("Changing file...");
    // change the file to be .box instead
    fs::remove_file(file_path)?;
    path_buffer.set_extension("box");
    // TODO - generate random file name
    let file_path = path_buffer.as_path();

    println!("Encrypting data...");

    let body = cipher::encrypt(&key, &nonce, &data).expect("Error encrypting file");

    println!("Writing data...");

    parser::write_file(file_path, header, body)?;

    Ok(())
}

pub fn decrypt_box(input_path: &str) -> Result<()> {
    println!("Unboxing {}", input_path);
    let mut path_buffer = PathBuf::from(input_path);

    // get needed pathes
    let file_path = path_buffer.as_path();

    println!("Reading data...");

    // get needed data
    let key = storage::get_key()?;
    let (header, body) = parser::parse_file(file_path, &key)?;

    println!("Got header: {:?}", header);
    println!("Validating checksum...");

    let new_checksum = parser::generate_checksum(&body);
    if new_checksum != header.checksum {
        return Err(Error::new(ErrorKind::InvalidData, "Checksum verification failed"));
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