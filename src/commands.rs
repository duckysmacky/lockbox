use std::path::{Path, PathBuf};
use std::{fs, io::Error};

use chacha20poly1305::Nonce;
use clap::ArgMatches;

use crate::encryption::cipher;
use crate::stream::file;

pub fn encrypt(path: &str, args: &ArgMatches) -> Result<Vec<u8>, Error> {
    let mut path = PathBuf::from(path);

    // TODO - (recursive) folder encryption
    let file_path = path.as_path();
    // TODO - key storage
    let key_path = Path::new("key.txt");
    let nonce_path = Path::new("nonce.txt");

    println!("Encrypting {}...", path.to_str().unwrap());

    let plaintext = file::read_bytes(file_path)?;
    let key = cipher::generate_key();
    let nonce: Nonce = cipher::generate_nonce();

    file::write_bytes(key_path, &key)?;
    file::write_bytes(nonce_path, &nonce)?;

    if args.get_flag("box") {
        fs::remove_file(file_path)?;
        path.set_extension("box");
    }
    let file_path = path.as_path();

    match cipher::encrypt(&key, &nonce, &plaintext) {
        Ok(encrypted_text) => file::write_bytes(file_path, &encrypted_text),
        Err(err) => panic!("Error has occured while trying to encrypt data: {}", err.to_string()),
    }
}

pub fn decrypt(path: &str, args: &ArgMatches) -> Result<Vec<u8>, Error> {
    let mut path = PathBuf::from(path);

    let file_path = path.as_path();
    let key_path = Path::new("key.txt");
    let nonce_path = Path::new("nonce.txt");

    println!("Decrypting {}...", path.to_str().unwrap());

    let ciphertext = file::read_bytes(file_path)?;
    let key = file::read_bytes(key_path)?;
    let nonce = file::read_bytes(nonce_path)?;

    if args.get_flag("box") {
        fs::remove_file(file_path)?;
        path.set_extension("txt");
    }
    let file_path = path.as_path();

    match cipher::decrypt(&key, &nonce, &ciphertext) {
        Ok(decrypted_text) => file::write_bytes(file_path, &decrypted_text),
        Err(err) => panic!("Error has occured while trying to decrypt data: {}", err.to_string()),
    }
}