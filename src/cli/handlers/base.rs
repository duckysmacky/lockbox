//! Contains handlers for subcommands under the base `databoxer` command

use clap::ArgMatches;
use std::path::PathBuf;
use std::ffi::OsStr;
use crate::cli::{handlers, prompts};
use crate::core::utils::path;
use crate::{exits_on, log_error, log_info, log_success, options};

pub fn handle_box(args: &ArgMatches) -> (u32, u32) {
    let mut total_files: u32 = 0;
    let mut error_files: u32 = 0;

    let file_paths: Vec<PathBuf> = {
        let input_paths = handlers::get_path_vec(args, "PATH").expect("File path is required");
        let recursive = args.get_flag("RECURSIVE");

        path::parse_paths(input_paths, recursive)
    };

    let mut options = options::EncryptionOptions {
        keep_original_name: args.get_flag("KEEP_NAME"),
        output_paths: handlers::get_path_deque(args, "OUTPUT")
    };

    let password = match args.get_one::<String>("PASSWORD") {
        None => prompts::prompt_password("Please enter the password for the current profile:"),
        Some(password) => password.to_string()
    };

    // encrypt each file and handle errors accordingly
    for path in file_paths {
        total_files += 1;
        let file_name = match args.get_flag("SHOW_FULL_PATH") {
            true => path.as_os_str().to_os_string(),
            false => path.file_name().unwrap_or(OsStr::new("<unknown file name>")).to_os_string()
        };

        log_info!("Encrypting {:?}", file_name);
        match crate::encrypt(path.as_path(), &password, &mut options) {
            Ok(_) => log_success!("Successfully encrypted {:?}", file_name),
            Err(err) => {
                log_error!("Unable to encrypt \"{}\"", file_name.to_string_lossy());
                exits_on!(err; IOError false; InvalidInput false);
                error_files += 1;
            }
        }
    }

    (total_files, error_files)
}

pub fn handle_unbox(args: &ArgMatches) -> (u32, u32) {
    let mut total_files: u32 = 0;
    let mut error_files: u32 = 0;

    let file_paths: Vec<PathBuf> = {
        let input_paths = handlers::get_path_vec(args, "PATH").expect("File path is required");
        let recursive = args.get_flag("RECURSIVE");

        path::parse_paths(input_paths, recursive)
    };

    // options for decryption
    let mut options = options::DecryptionOptions {
        output_paths: handlers::get_path_deque(args, "OUTPUT")
    };

    let password = match args.get_one::<String>("PASSWORD") {
        None => prompts::prompt_password("Please enter the password for the current profile:"),
        Some(password) => password.to_string()
    };

    // decrypt each file and handle errors accordingly
    for path in file_paths {
        total_files += 1;
        let file_name = match args.get_flag("SHOW_FULL_PATH") {
            true => path.as_os_str().to_os_string(),
            false => path.file_name().unwrap_or(OsStr::new("<unknown file name>")).to_os_string()
        };

        log_info!("Decrypting {:?}", file_name);
        match crate::decrypt(path.as_path(), &password, &mut options) {
            Ok(_) => log_success!("Successfully decrypted {:?}", path.file_name().unwrap().to_os_string()),
            Err(err) => {
                log_error!("Unable to decrypt \"{}\"", file_name.to_string_lossy());
                exits_on!(err; IOError false; InvalidInput false);
                error_files += 1;
            }
        }
    }

    (total_files, error_files)
}