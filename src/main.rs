mod commands;
mod encryption;

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
        .subcommand(Command::new("box")
            .about("Encrypt specified files into a special file type")
            .arg(Arg::new("filepath")
                .help("Path to the file to be encrypted")
                .required(true)
            )
        )
        .subcommand(Command::new("unbox")
            .about("Decrypt specified files from a special file type")
            .arg(Arg::new("filepath")
                .help("Path to the file to be decrypted")
                .required(true)
            )
        )
        .subcommand(Command::new("encrypt")
            .about("Encrypt specified files")
            .arg(Arg::new("filepath")
                .help("Path to the file to be encrypted")
                .required(true)
            )
        )
        .subcommand(Command::new("decrypt")
            .about("Decrypt specified files")
            .arg(Arg::new("filepath")
                .help("Path to the file to be decrypted")
                .required(true)
            )
        )
        .get_matches();

    println!("Debug: {}", args.get_flag("debug"));

    if let Some(args) = args.subcommand_matches("box") {
        let path = args.get_one::<String>("filepath").expect("No path to the file was provided");

        match commands::encrypt_box(path) {
            Ok(_) => println!("Successfully boxed {}", path),
            Err(err) => panic!("Error has occured while trying to encrypt data: {}", err.to_string()),
        }
    }

    if let Some(args) = args.subcommand_matches("unbox") {
        let path = args.get_one::<String>("filepath").expect("No path to the file was provided");

        match commands::decrypt_box(path) {
            Ok(_) => println!("Successfully unboxed {}", path),
            Err(err) => panic!("Error has occured while trying to decrypt data: {}", err.to_string()),
        }
    }

    if let Some(args) = args.subcommand_matches("encrypt") {
        let path = args.get_one::<String>("filepath").expect("No path to the file was provided");

        match commands::encrypt(path) {
            Ok(_) => println!("Successfully encrypted {}", path),
            Err(err) => panic!("Error has occured while trying to encrypt data: {}", err.to_string()),
        }
    }

    if let Some(args) = args.subcommand_matches("decrypt") {
        let path = args.get_one::<String>("filepath").expect("No path to the file was provided");

        match commands::decrypt(path) {
            Ok(_) => println!("Successfully decrypted {}", path),
            Err(err) => panic!("Error has occured while trying to decrypt data: {}", err.to_string()),
        }
    }
    
    Ok(())
}