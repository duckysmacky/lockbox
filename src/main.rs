mod commands;
mod encryption;
mod logger;
mod storage;

use std::io::{self};
use clap::{command, Arg, ArgAction, Command};

fn main() -> io::Result<()> {
    let args = get_command().get_matches();

    logger::configure_logger(&args);

    // BOX
    if let Some(args) = args.subcommand_matches("box") {
        match commands::box_files(args) {
            Ok(_) => log_success!("Encryption finished"),
            Err(err) => panic!("Error has occurred while trying to encrypt data: {}", err.to_string()),
        }
    }

    // UNBOX
    if let Some(args) = args.subcommand_matches("unbox") {
        match commands::unbox_files(args) {
            Ok(_) => log_success!("Decryption finished"),
            Err(err) => panic!("Error has occurred while trying to decrypt data: {}", err.to_string()),
        }
    }
    
    Ok(())
}

fn get_command() -> Command {
    command!()
        .arg(Arg::new("debug")
            .short('d')
            .long("debug")
            .action(ArgAction::SetTrue)
            .help("Turns on debug mode")
        )
        .arg(Arg::new("verbose")
            .short('v')
            .long("verbose")
            .help("Use verbose output (extra information)")
            .action(ArgAction::SetTrue)
            .conflicts_with("quiet")
        )
        .arg(Arg::new("quiet")
            .short('q')
            .long("quiet")
            .help("Do not print any log messages")
            .action(ArgAction::SetTrue)
            .conflicts_with("verbose")
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
            .arg(Arg::new("keep-name")
                .short('k')
                .long("keep")
                .help("Keep original file name for the encrypted file")
                .action(ArgAction::SetTrue)
                .conflicts_with("custom-name")
            )
            .arg(Arg::new("custom-name")
                .short('n')
                .long("name")
                .help("Specify file name for encrypted file")
                .action(ArgAction::Set)
                .conflicts_with("keep-name")
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
}