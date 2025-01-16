//! Core API wrapper functions which handle the CLI input

use std::{collections::VecDeque, path::PathBuf, ffi::OsStr};
use std::process::exit;
use clap::ArgMatches;
use crate::core::utils::path;
use crate::{Error, options, log_error, log_success, log_warn, log_info};
use crate::core::error::ProfileErrorKind;
use super::prompts;

pub fn handle_box(g_args: &ArgMatches, args: &ArgMatches) -> (u32, u32) {
    let mut total_files: u32 = 0;
    let mut error_files: u32 = 0;

    let file_paths: Vec<PathBuf> = {
        let input_paths = get_path_vec(args, "PATH").expect("File path is required");
        let recursive = args.get_flag("RECURSIVE-SEARCH");

        path::parse_paths(input_paths, recursive)
    };

    // options for encryption
    let mut options = options::EncryptionOptions {
        keep_original_name: args.get_flag("KEEP-ORIGINAL-NAME"),
        output_paths: get_path_deque(args, "OUTPUT-PATH")
    };

    let password = match g_args.get_one::<String>("PASSWORD") {
        None => prompts::prompt_password("Please enter the password for the current profile:"),
        Some(password) => password.to_string()
    };

    // encrypt each file and handle errors accordingly
    for path in file_paths {
        total_files += 1;
        let file_name = match args.get_flag("SHOW-FULL-PATH") {
            true => path.as_os_str().to_os_string(),
            false => path.file_name().unwrap_or(OsStr::new("<unknown file name>")).to_os_string()
        };

        log_info!("Encrypting {:?}", file_name);
        match crate::encrypt(&password, path.as_path(), &mut options) {
            Ok(_) => log_success!("Successfully encrypted {:?}", file_name),
            Err(err) => {
                log_error!("Unable to encrypt \"{}\"", file_name.to_string_lossy());
                handle_error(err);
                error_files += 1;
            }
        }
    }

    (total_files, error_files)
}

pub fn handle_unbox(g_args: &ArgMatches, args: &ArgMatches) -> (u32, u32) {
    let mut total_files: u32 = 0;
    let mut error_files: u32 = 0;

    let file_paths: Vec<PathBuf> = {
        let input_paths = get_path_vec(args, "PATH").expect("File path is required");
        let recursive = args.get_flag("RECURSIVE-SEARCH");

        path::parse_paths(input_paths, recursive)
    };

    // options for decryption
    let mut options = options::DecryptionOptions {
        output_paths: get_path_deque(args, "OUTPUT-PATH")
    };

    let password = match g_args.get_one::<String>("PASSWORD") {
        None => prompts::prompt_password("Please enter the password for the current profile:"),
        Some(password) => password.to_string()
    };

    // decrypt each file and handle errors accordingly
    for path in file_paths {
        total_files += 1;
        let file_name = match args.get_flag("SHOW-FULL-PATH") {
            true => path.as_os_str().to_os_string(),
            false => path.file_name().unwrap_or(OsStr::new("<unknown file name>")).to_os_string()
        };

        log_info!("Decrypting {:?}", file_name);
        match crate::decrypt(&password, path.as_path(), &mut options) {
            Ok(_) => log_success!("Successfully decrypted {:?}", path.file_name().unwrap().to_os_string()),
            Err(err) => {
                log_error!("Unable to encrypt \"{}\"", file_name.to_string_lossy());
                handle_error(err);
                error_files += 1;
            }
        }
    }

    (total_files, error_files)
}

pub fn handle_profile_create(g_args: &ArgMatches, args: &ArgMatches) {
    let password = match g_args.get_one::<String>("PASSWORD") {
        None => prompts::prompt_password("Please enter a password for the new profile:"),
        Some(password) => password.to_string()
    };

    let name = args.get_one::<String>("NAME").expect("Profile name is required");

    match crate::create_profile(&password, name) {
        Ok(_) => log_success!("Successfully created new profile \"{}\"", name),
        Err(err) => {
            log_error!("Unable to create a new profile named \"{}\"", name);
            handle_error(err);
        }
    }
}

pub fn handle_profile_delete(g_args: &ArgMatches, args: &ArgMatches) {
    let name = args.get_one::<String>("NAME").expect("Profile name is required");

    let password = match g_args.get_one::<String>("PASSWORD") {
        None => prompts::prompt_password(&format!("Please enter the password for {}", name)),
        Some(password) => password.to_string()
    };

    match crate::delete_profile(&password, name) {
        Ok(_) => log_success!("Successfully deleted profile \"{}\"", name),
        Err(err) => {
            log_error!("Unable to delete profile \"{}\"", name);
            handle_error(err);
        }
    }
}

pub fn handle_profile_set(g_args: &ArgMatches, args: &ArgMatches) {
    let name = args.get_one::<String>("NAME").expect("Profile name is required");

    let password = match g_args.get_one::<String>("PASSWORD") {
        None => prompts::prompt_password(&format!("Please enter the password for {}", name)),
        Some(password) => password.to_string()
    };

    match crate::select_profile(&password, name) {
        Ok(_) => log_success!("Successfully set current profile to \"{}\"", name),
        Err(err) => {
            log_error!("Unable to switch to profile \"{}\"", name);
            handle_error(err);
        }
    }
}

pub fn handle_profile_get(_g_args: &ArgMatches, _args: &ArgMatches) {
    match crate::get_profile() {
        Ok(name) => log_success!("Currently selected profile: {}", name),
        Err(err) => {
            log_error!("Unable to get currently selected profile");
            handle_error(err);
        }
    }
}

pub fn handle_profile_list(_g_args: &ArgMatches, _args: &ArgMatches) {
    let profiles = crate::get_profiles();

    let profiles = profiles.unwrap_or_else(|err| {
        log_error!("Unable to get a list of all profiles");
        handle_error(err);
        vec![]
    });
    let count = profiles.len();

    if count == 0 {
        log_warn!("No profiles found");
        log_warn!("New profile can be created with \"lockbox profile new\"");
    } else {
        if count > 1 {log_success!("There are {} profiles found:", count);}
        else {log_success!("There is {} profile found:", count);}
        for name in profiles {
            println!("\t- {}", name)
        }
    }
}

pub fn handle_key_new(g_args: &ArgMatches, _args: &ArgMatches) {
    let password = match g_args.get_one::<String>("PASSWORD") {
        None => prompts::prompt_password("Please enter the password for the current profile:"),
        Some(password) => password.to_string()
    };

    match crate::new_key(&password) {
        Ok(_) => log_success!("Successfully generated new encryption key for the current profile"),
        Err(err) => {
            log_error!("Unable to generate a new encryption key");
            handle_error(err);
        }
    }
}

pub fn handle_key_get(g_args: &ArgMatches, args: &ArgMatches) {
    let password = match g_args.get_one::<String>("PASSWORD") {
        None => prompts::prompt_password("Please enter the password for the current profile:"),
        Some(password) => password.to_string()
    };

    let options = options::GetKeyOptions {
        byte_format: args.get_flag("BYTE-FORMAT"),
    };

    match crate::get_key(&password, options) {
        Ok(key) => {
            // TODO: add current profile name
            log_success!("Encryption key for the current profile:\n    {}", key);
        }
        Err(err) => {
            log_error!("Unable to get an encryption key for the current profile");
            handle_error(err);
        }
    }
}

pub fn handle_key_set(g_args: &ArgMatches, args: &ArgMatches) {
	let password = match g_args.get_one::<String>("PASSWORD") {
        None => prompts::prompt_password("Please enter the password for the current profile:"),
        Some(password) => password.to_string()
    };

	let new_key = args.get_one::<String>("KEY").expect("Key is required");

    match crate::set_key(&password, &new_key) {
        Ok(_) => log_success!("Successfully set a new encryption key for the current profile"),
        Err(err) => {
            log_error!("Unable to set an encryption key for the current profile");
            handle_error(err);
        }
    }
}

/// Converts from the passed arguments strings to vector of paths
fn get_path_vec(args: &ArgMatches, arg_id: &str) -> Option<Vec<PathBuf>> {
    if let Some(strings) = args.get_many::<String>(arg_id) {
        return Some(strings
            .map(|s| PathBuf::from(s))
            .collect::<Vec<PathBuf>>()
        )
    }
    None
}

/// Converts from the passed arguments strings to deque of paths
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

/// Handles the error and acts according to each error
fn handle_error(err: Error) {
    log_error!("{}", err);
    match &err {
        Error::ProfileError(kind) => {
            if let ProfileErrorKind::AuthenticationFailed = kind {
                log_warn!("Try again or use a different profile")
            } else {
                log_warn!("New profile can be created with \"lockbox profile new\"");
            }
        },
        Error::ConfigError(_) => {
            log_warn!("Please check the config file for any mistakes and try again");
        }
        _ => {}
    }

    if err.should_exit() {
        exit(err.exit_code());
    }
}