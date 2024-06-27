mod encryption;

use std::io::{self, Error};
use chacha20poly1305::Nonce;
use clap::{command, Arg, ArgAction, Command};

use encryption::{cipher, file};

fn main() -> io::Result<()> {
    let args = command!()
        .arg(Arg::new("debug")
            .short('d')
            .long("debug")
            .action(ArgAction::SetTrue)
            .help("Turns on debug output")
        )
        .subcommand(Command::new("encrypt")
            .about("Encrypt specified files")
            .alias("box")
            .arg(Arg::new("filepath")
                .help("Path to the file to be encrypted")
                .required(true)
            )
        )
        .subcommand(Command::new("decrypt")
            .about("Decrypt specified files")
            .alias("unbox")
            .arg(Arg::new("filepath")
                .help("Path to the file to be decrypted")
                .required(true)
            )
        )
        .get_matches();

    println!("Debug: {}", args.get_flag("debug"));

    if let Some(args) = args.subcommand_matches("encrypt") {
        let path = args.get_one::<String>("filepath").expect("No path to the file was provided");

        match encrypt(path) {
            Ok(bytes) => println!("Successfully encrypted {} bytes", bytes.len()),
            Err(err) => panic!("Error has occured while trying to encrypt data: {}", err.to_string()),
        }
    }

    if let Some(args) = args.subcommand_matches("decrypt") {
        let path = args.get_one::<String>("filepath").expect("No path to the file was provided");

        match decrypt(path) {
            Ok(bytes) => println!("Successfully decrypted {} bytes", bytes.len()),
            Err(err) => panic!("Error has occured while trying to decrypt data: {}", err.to_string()),
        }
    }
    
    Ok(())
}

fn encrypt(path: &str) -> Result<Vec<u8>, Error> {
    println!("Encrypting {}...", path);

    let plaintext = file::read_bytes(path)?;
    let key = cipher::generate_key();
    let nonce: Nonce = cipher::generate_nonce();

    file::write_bytes("key.txt", &key)?;
    file::write_bytes("nonce.txt", &nonce)?;

    match cipher::encrypt(&key, &nonce, &plaintext) {
        Ok(encrypted_text) => file::write_bytes(path, &encrypted_text),
        Err(err) => panic!("Error has occured while trying to encrypt data: {}", err.to_string()),
    }
}

fn decrypt(path: &str) -> Result<Vec<u8>, Error> {
    println!("File: {}", path);

    let ciphertext = file::read_bytes(path)?;
    let key = file::read_bytes("key.txt")?;
    let nonce = file::read_bytes("nonce.txt")?;

    match encryption::cipher::decrypt(&key, &nonce, &ciphertext) {
        Ok(decrypted_text) => file::write_bytes(path, &decrypted_text),
        Err(err) => panic!("Error has occured while trying to decrypt data: {}", err.to_string()),
    }
}