use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::{
    io::{Error, Result},
    fs
};

use chacha20poly1305::{Key, Nonce};

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
    let header = parser::generate_header(file_path, checksum).expect("Error generating header");
    let key = cipher::generate_key();
    let nonce = cipher::generate_nonce();

    println!("Saving keys...");

    storage::save_key(&key)?;
    storage::save_nonce(&nonce)?;

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
    let nonce = storage::get_nonce()?;
    let (header, body) = parser::parse_file(file_path, &key, &nonce)?;

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

pub fn encrypt(input_path: &str) -> Result<()> {
    println!("Encrypting {}", input_path);
    let file_path = Path::new(input_path);

    // get needed data
    let data = file::read_bytes(file_path)?;
    let key: Key = cipher::generate_key();
    let nonce: Nonce = cipher::generate_nonce();

    println!("Saving keys...");

    storage::save_key(&key)?;
    storage::save_nonce(&nonce)?;

    println!("Encrypting data...");

    let data = cipher::encrypt(&key, &nonce, &data).expect("Error encrypting file");

    println!("Writing data...");

    file::write_bytes(file_path, &data)?;

    Ok(())
}

pub fn decrypt(input_path: &str) -> Result<()> {
    println!("Decrypting {}", input_path);
    let file_path = Path::new(input_path);

    println!("Reading data...");

    // get needed data
    let key = storage::get_key()?;
    let nonce = storage::get_nonce()?;
    let data = file::read_bytes(file_path)?;

    println!("Decrypting data...");

    let data = cipher::decrypt(&key, &nonce, &data).expect("Error decrypting file data");

    println!("Writing data...");

    file::write_bytes(&file_path, &data)?;

    Ok(())
}