use std::{collections::VecDeque, path::PathBuf};
use clap::ArgMatches;
use crate::cli::path;
use crate::{Error, log_fatal, log_warn, options};
use crate::{decrypt, delete_key, encrypt, log_error, log_success, new_key};
use crate::cli::prompts;

pub fn r#box(args: &ArgMatches) -> (u32, u32) {
    let mut total_files: u32 = 0;
    let mut error_files: u32 = 0;
    let mut file_paths: Vec<PathBuf> = Vec::new();

    // options for path parsing
    let options = path::PathOptions {
        input_paths: get_path_vec(args, "path").expect("File path is required"),
        recursive: args.get_flag("recursive")
    };
    path::parse_paths(&mut file_paths, options);

    // options for encryption
    let mut options = options::EncryptionOptions {
        keep_name: args.get_flag("keep-name"),
        output_paths: get_path_deque(args, "output-path")
    };

    let password = prompts::prompt_password("Please enter your password to continue: ");

    // encrypt each file and handle errors accordingly
    for path in file_paths {
        total_files += 1;

        if let Err(err) = encrypt(path.as_path(), &password, &mut options) {
            let file_name = path.file_name().unwrap().to_os_string();

            if !handle_error(&err) {
                log_fatal!("A fatal error has occurred while trying to encrypt {:?}: {}", file_name, err)
            }

            log_warn!("Skipping file {:?}", file_name);
            error_files += 1;
        }

        log_success!("Successfully encrypted {:?}", path.file_name().unwrap().to_os_string());
    }

    (total_files, error_files)
}

pub fn unbox(args: &ArgMatches) -> (u32, u32) {
    let mut total_files: u32 = 0;
    let mut error_files: u32 = 0;
    let mut file_paths: Vec<PathBuf> = Vec::new();

    // options for path parsing
    let options = path::PathOptions {
        input_paths: get_path_vec(args, "path").expect("File path is required"),
        recursive: args.get_flag("recursive")
    };
    path::parse_paths(&mut file_paths, options);

    // options for decryption
    let mut options = options::DecryptionOptions {
        output_paths: get_path_deque(args, "output-path")
    };

    let password = prompts::prompt_password("Please enter your password to continue: ");

    // decrypt each file and handle errors accordingly
    for path in file_paths {
        total_files += 1;

        if let Err(err) = decrypt(path.as_path(), &password, &mut options) {
            let file_name = path.file_name().unwrap().to_os_string();

            if !handle_error(&err) {
                log_fatal!("A fatal error has occurred while trying to encrypt {:?}: {}", file_name, err)
            }

            log_warn!("Skipping file {:?}", file_name);
            error_files += 1;
        }

        log_success!("Successfully encrypted {:?}", path.file_name().unwrap().to_os_string());
    }

    (total_files, error_files)
}

pub fn key(args: &ArgMatches) {
    let password = prompts::prompt_password("Please enter your password to continue: ");

    /* NEW */
    if let Some(_args) = args.subcommand_matches("new") {
        let options = options::NewKeyOptions {
            key_options: options::KeyOptions {}
        };

        if let Err(err) = new_key(&password, &options) {
            if !handle_error(&err) {
                log_fatal!("A fatal error has occurred while trying to generate a new key: {}", err)
            }
        }

        log_success!("Successfully generated a new encryption key");
    }
    /* DELETE */
    if let Some(_args) = args.subcommand_matches("delete") {
        let options = options::DeleteKeyOptions {
            key_options: options::KeyOptions {}
        };

        if let Err(err) = delete_key(&password, &options) {
            if !handle_error(&err) {
                log_fatal!("A fatal error has occurred while trying to generate a new key: {}", err)
            }
        }

        log_success!("Successfully deleted the saved encryption key");
    }
}

fn get_path_vec(args: &ArgMatches, arg_id: &str) -> Option<Vec<PathBuf>> {
    if let Some(strings) = args.get_many::<String>(arg_id) {
        return Some(strings
            .map(|s| PathBuf::from(s))
            .collect::<Vec<PathBuf>>()
        )
    }

    None
}

fn get_path_deque(args: &ArgMatches, arg_id: &str) -> Option<VecDeque<PathBuf>> {
    if let Some(strings) = args.get_many::<String>(arg_id) {
        let mut deque = VecDeque::new();

        for s in strings {
            deque.push_back(PathBuf::from(s))
        }

        return Some(deque)
    }

    None
}

fn handle_error(err: &Error) -> bool {
    match err {
        Error::AuthenticationFailed(msg) => {
            log_error!("{}", msg);
            log_error!("Please try again");
            std::process::exit(1);
        },
        Error::InvalidFile(msg) => {
            log_error!("{}", msg);
            true
        },
        _ => false
    }
}