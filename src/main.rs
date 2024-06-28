mod encryption;
mod stream;

use std::{fs, io::{self, Error}};
use chacha20poly1305::Nonce;
use clap::{command, Arg, ArgAction, ArgMatches, Command};

use encryption::cipher;
use stream::file;

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
            .arg(Arg::new("box")
                .short('b')
                .long("box")
                .action(ArgAction::SetTrue)
                .help("Encrypts into a custom .box file")
            )
        )
        .subcommand(Command::new("decrypt")
            .about("Decrypt specified files")
            .alias("unbox")
            .arg(Arg::new("filepath")
                .help("Path to the file to be decrypted")
                .required(true)
            )
            .arg(Arg::new("box")
                .short('b')
                .long("box")
                .action(ArgAction::SetTrue)
                .help("Decrypts from a custom .box file")
        )
        )
        .get_matches();

    println!("Debug: {}", args.get_flag("debug"));

    if let Some(args) = args.subcommand_matches("encrypt") {
        let path = args.get_one::<String>("filepath").expect("No path to the file was provided");

        match encrypt(path, args) {
            Ok(bytes) => println!("Successfully encrypted {} bytes", bytes.len()),
            Err(err) => panic!("Error has occured while trying to encrypt data: {}", err.to_string()),
        }
    }

    if let Some(args) = args.subcommand_matches("decrypt") {
        let path = args.get_one::<String>("filepath").expect("No path to the file was provided");

        match decrypt(path, args) {
            Ok(bytes) => println!("Successfully decrypted {} bytes", bytes.len()),
            Err(err) => panic!("Error has occured while trying to decrypt data: {}", err.to_string()),
        }
    }
    
    Ok(())
}

fn encrypt(path: &str, args: &ArgMatches) -> Result<Vec<u8>, Error> {
    let filepath: String;
    if args.get_flag("box") {
        filepath = path.replace(path.split('.').collect::<Vec<&str>>().get(1).unwrap(), "box");
    } else {
        filepath = path.to_string();
    }

    println!("Encrypting {}...", filepath);

    let plaintext = file::read_bytes(path)?;
    let key = cipher::generate_key();
    let nonce: Nonce = cipher::generate_nonce();

    file::write_bytes("key.txt", &key)?;
    file::write_bytes("nonce.txt", &nonce)?;

    if args.get_flag("box") {fs::remove_file(path)?;}

    match cipher::encrypt(&key, &nonce, &plaintext) {
        Ok(encrypted_text) => file::write_bytes(&filepath, &encrypted_text),
        Err(err) => panic!("Error has occured while trying to encrypt data: {}", err.to_string()),
    }
}

fn decrypt(path: &str, args: &ArgMatches) -> Result<Vec<u8>, Error> {
    let filepath: String;
    if args.get_flag("box") {
        filepath = path.replace(path.split('.').collect::<Vec<&str>>().get(1).unwrap(), "txt");
    } else {
        filepath = path.to_string();
    }

    println!("File: {}", filepath);

    let ciphertext = file::read_bytes(path)?;
    let key = file::read_bytes("key.txt")?;
    let nonce = file::read_bytes("nonce.txt")?;

    if args.get_flag("box") {fs::remove_file(path)?;}

    match cipher::decrypt(&key, &nonce, &ciphertext) {
        Ok(decrypted_text) => file::write_bytes(&filepath, &decrypted_text),
        Err(err) => panic!("Error has occured while trying to decrypt data: {}", err.to_string()),
    }
}