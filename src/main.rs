mod commands;
mod encryption;

use std::io::{self};
use clap::{command, Arg, ArgAction, Command};

use crate::commands::{command, path};

fn main() -> io::Result<()> {
    let args = command!()
        .arg(Arg::new("debug")
            .short('d')
            .long("debug")
            .action(ArgAction::SetTrue)
            .help("Turns on debug output")
        )
        .subcommand(Command::new("box")
            .about("Encrypt specified files into a special file type")
            .arg(Arg::new("path")
                .help("Specify path to the file/directory to be encrypted")
                .action(ArgAction::Append)
            )
            .arg(Arg::new("recursive")
                .short('R')
                .long("recursive")
                .help("Recursively encrypt directory")
                .action(ArgAction::SetTrue)
            )
        )
        .subcommand(Command::new("unbox")
            .about("Decrypt specified files from a special file type")
            .arg(Arg::new("path")
                .help("Specify path to the file/directory to be decrypted")
                .action(ArgAction::Append)
            )
            .arg(Arg::new("recursive")
                .short('R')
                .long("recursive")
                .help("Recursively decrypt directory")
                .action(ArgAction::SetTrue)
            )
        )
        .get_matches();

    println!("Debug: {}", args.get_flag("debug"));

    // BOX
    if let Some(args) = args.subcommand_matches("box") {
        match path::parse_path(args, command::encrypt_box) {
            Ok(_) => println!("Encryption finished"),
            Err(err) => panic!("Error has occured while trying to encrypt data: {}", err.to_string()),
        }
    }

    // UNBOX
    if let Some(args) = args.subcommand_matches("unbox") {
        match path::parse_path(args, command::decrypt_box) {
            Ok(_) => println!("Decryption finished"),
            Err(err) => panic!("Error has occured while trying to decrypt data: {}", err.to_string()),
        }
    }
    
    Ok(())
}