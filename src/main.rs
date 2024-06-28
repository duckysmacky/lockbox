mod commands;
mod encryption;
mod stream;

use std::io::{self};
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

        match commands::encrypt(path, args) {
            Ok(bytes) => println!("Successfully encrypted {} bytes", bytes.len()),
            Err(err) => panic!("Error has occured while trying to encrypt data: {}", err.to_string()),
        }
    }

    if let Some(args) = args.subcommand_matches("decrypt") {
        let path = args.get_one::<String>("filepath").expect("No path to the file was provided");

        match commands::decrypt(path, args) {
            Ok(bytes) => println!("Successfully decrypted {} bytes", bytes.len()),
            Err(err) => panic!("Error has occured while trying to decrypt data: {}", err.to_string()),
        }
    }
    
    Ok(())
}