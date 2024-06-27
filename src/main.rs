mod encryption;

use std::io;
use chacha20poly1305::Nonce;
use clap::{command, Arg, ArgAction, Command};

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

        println!("File: {}", path);

        let text = encryption::file::read_file(path)?;
        let key = encryption::encryption::generate_key();
        let nonce: Nonce = encryption::encryption::generate_nonce();

        encryption::file::write_file("key.txt", &key)?;
        encryption::file::write_file("nonce.txt", &nonce)?;

        match encryption::encryption::encrypt(&key, &nonce, &text) {
            Ok(encrypted_text) => {
                encryption::file::write_file(path, &encrypted_text)?;
                println!("Encrypted {}", path);
            },
            Err(err) => panic!("{}", err.to_string()),
        }
    }

    if let Some(args) = args.subcommand_matches("decrypt") {
        let path = args.get_one::<String>("filepath").expect("No path to the file was provided");

        println!("File: {}", path);

        let ciphertext: &[u8] = &encryption::file::read_file(path)?;
        let key: &[u8] = &encryption::file::read_file("key.txt")?;
        let nonce: &[u8] = &encryption::file::read_file("nonce.txt")?;

        match encryption::encryption::decrypt(key, nonce.into(), ciphertext) {
            Ok(decrypted_text) => {
                encryption::file::write_file(path, &decrypted_text)?;
                println!("Decrypted {}", path);
            },
            Err(err) => panic!("{}", err.to_string()),
        }
    }
    
    // assert_eq!(&plaintext, b"plaintext message");
    Ok(())
}