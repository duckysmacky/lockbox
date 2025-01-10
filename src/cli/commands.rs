//! Core API wrapper functions which handle the CLI input

use std::{collections::VecDeque, path::PathBuf, ffi::OsStr};
use std::process::exit;
use clap::ArgMatches;
use crate::core::utils::path;
use crate::{Error, options, log_error, log_success, log_warn};
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

        log_success!("Encrypting {:?}", file_name);

        if let Err(err) = crate::encrypt(&password, path.as_path(), &mut options) {
            match err {
                Error::ProfileError(_) => {
                    log_error!("{}", err);
                    log_error!("New profile can be created with \"lockbox profile new\"");
                    exit(1);
                },
                Error::AuthError(_) => {
                    log_error!("{}", err);
                    log_error!("Please try again");
                    exit(1);
                },
                Error::IOError(_) => {
                    log_error!("Unable to access {:?}: {}", file_name, err);
                    error_files += 1;
                },
                Error::CipherError(_) => {
                    log_error!("{}", err);
                    error_files += 1;
                },
                Error::InvalidData(_) => {
                    log_error!("Invalid file data in {:?}: {}", file_name, err);
                    error_files += 1;
                },
                Error::InvalidInput(_) => {
                    log_warn!("Skipping {:?}: {}", file_name, err);
                },
                _ => {
                    log_error!("{}", err);
                    exit(1);
                }
            }
        } else {
            log_success!("Successfully encrypted {:?}", file_name);
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
        log_success!("Decrypting {:?}", file_name);

        if let Err(err) = crate::decrypt(&password, path.as_path(), &mut options) {
            match err {
                Error::ProfileError(_) => {
                    log_error!("{}", err);
                    log_error!("New profile can be created with \"lockbox profile new\"");
                    exit(1);
                },
                Error::AuthError(_) => {
                    log_error!("{}", err);
                    log_error!("Please try again");
                    exit(1);
                },
                Error::IOError(_) => {
                    log_error!("Unable to access {:?}: {}", file_name, err);
                    error_files += 1;
                },
                Error::CipherError(_) => {
                    log_error!("{}", err);
                    error_files += 1;
                },
                Error::InvalidData(_) => {
                    log_error!("Invalid file data in {:?}: {}", file_name, err);
                    error_files += 1;
                },
                Error::InvalidInput(_) => {
                    log_warn!("Skipping {:?}: {}", file_name, err);
                },
                _ => {
                    log_error!("{}", err);
                    exit(1);
                }
            }
        } else {
            log_success!("Successfully decrypted {:?}", path.file_name().unwrap().to_os_string());
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

    if let Err(err) = crate::create_profile(&password, name) {
        log_error!("Unable to create a new profile: {}", err);
        exit(1);
    } else {
        log_success!("Successfully created new profile \"{}\"", name);
    }
}

pub fn handle_profile_delete(g_args: &ArgMatches, args: &ArgMatches) {
    let name = args.get_one::<String>("NAME").expect("Profile name is required");

    let password = match g_args.get_one::<String>("PASSWORD") {
        None => prompts::prompt_password(&format!("Please enter the password for {}", name)),
        Some(password) => password.to_string()
    };

    if let Err(err) = crate::delete_profile(&password, name) {
        match err {
            Error::ProfileError(_) => {
                log_error!("{}", err);
                log_error!("New profile can be created with \"lockbox profile new\"");
                exit(1);
            },
            Error::AuthError(_) => {
                log_error!("{}", err);
                log_error!("Please try again");
                exit(1);
            },
            _ => {
                log_error!("Unable to delete the profile: {}", err);
                exit(1);
            }
        }
    } else {
        log_success!("Successfully deleted profile \"{}\"", name);
    }
}

pub fn handle_profile_set(g_args: &ArgMatches, args: &ArgMatches) {
    let name = args.get_one::<String>("NAME").expect("Profile name is required");

    let password = match g_args.get_one::<String>("PASSWORD") {
        None => prompts::prompt_password(&format!("Please enter the password for {}", name)),
        Some(password) => password.to_string()
    };

    if let Err(err) = crate::select_profile(&password, name) {
        match err {
            Error::ProfileError(_) => {
                log_error!("{}", err);
                log_error!("New profile can be created with \"lockbox profile new\"");
                exit(1);
            },
            Error::AuthError(_) => {
                log_error!("{}", err);
                log_error!("Please try again");
                exit(1);
            },
            Error::InvalidInput(_) => {
                log_success!("Profile is already set to \"{}\"", name);
            }
            _ => {
                log_error!("Unable to set the profile: {}", err);
                exit(1);
            }
        }
    } else {
        log_success!("Successfully set profile to \"{}\"", name);
    }
}

pub fn handle_profile_get(_g_args: &ArgMatches, _args: &ArgMatches) {
    let profile_name = crate::get_profile();
    if let Err(err) = profile_name {
        match err {
            Error::ProfileError(_) => {
                log_error!("{}", err);
                log_error!("New profile can be created with \"lockbox profile new\"");
                exit(1);
            },
            _ => {
                log_error!("Unable to list profiles: {}", err);
                exit(1);
            }
        }
    } else {
        log_success!("Currently selected profile: {}", profile_name.unwrap());
    }
}

pub fn handle_profile_list(_g_args: &ArgMatches, _args: &ArgMatches) {
    let profiles = crate::get_profiles();
    if let Err(err) = profiles {
        match err {
            Error::ProfileError(_) => {
                log_error!("{}", err);
                log_error!("New profile can be created with \"lockbox profile new\"");
                exit(1);
            },
            _ => {
                log_error!("Unable to list profiles: {}", err);
                exit(1);
            }
        }
    }

    let profiles = profiles.unwrap();
    let count = profiles.len();

    if count == 0 {
        log_success!("No profiles found");
        log_success!("New profile can be created with \"lockbox profile new\"");
    } else {
        if count > 1 {log_success!("There are {} profiles found:", count);}
        else {log_success!("There is {} profile found:", count);}
        for name in profiles {
            println!("    - {}", name)
        }
    }
}

pub fn handle_key_new(g_args: &ArgMatches, _args: &ArgMatches) {
    let password = match g_args.get_one::<String>("PASSWORD") {
        None => prompts::prompt_password("Please enter the password for the current profile:"),
        Some(password) => password.to_string()
    };

    if let Err(err) = crate::new_key(&password) {
        match err {
            Error::ProfileError(_) => {
                log_error!("{}", err);
                log_error!("New profile can be created with \"lockbox profile new\"");
                exit(1);
            },
            Error::AuthError(_) => {
                log_error!("{}", err);
                log_error!("Please try again");
                exit(1);
            },
            _ => {
                log_error!("Unable to generate new encryption key: {}", err);
                exit(1);
            }
        }
    } else {
        log_success!("Successfully generated new encryption key for the current profile");
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

    let key = crate::get_key(&password, options);
    if let Err(err) = &key {
        match err {
            Error::ProfileError(_) => {
                log_error!("{}", err);
                log_error!("New profile can be created with \"lockbox profile new\"");
                exit(1);
            },
            Error::AuthError(_) => {
                log_error!("{}", err);
                log_error!("Please try again");
                exit(1);
            },
            _ => {
                log_error!("Unable to get the key: {}", err);
                exit(1);
            }
        }
    } else {
        // TODO: add current profile name
        log_success!("Encryption key for the current profile:\n    {}", key.unwrap());
    }
}

pub fn handle_key_set(g_args: &ArgMatches, args: &ArgMatches) {
	let password = match g_args.get_one::<String>("PASSWORD") {
        None => prompts::prompt_password("Please enter the password for the current profile:"),
        Some(password) => password.to_string()
    };

	let new_key = args.get_one::<String>("KEY").expect("Key is required");

    let key = crate::set_key(&password, &new_key);
    if let Err(err) = &key {
        match err {
            Error::ProfileError(_) => {
                log_error!("{}", err);
                log_error!("New profile can be created with \"lockbox profile new\"");
                exit(1);
            },
            Error::AuthError(_) => {
                log_error!("{}", err);
                log_error!("Please try again");
                exit(1);
            },
			Error::InvalidInput(_) => {
				log_error!("Invalid key provided: {}", err);
                exit(1);
			},
            _ => {
                log_error!("Unable to set a new key: {}", err);
                exit(1);
            }
        }
    } else {
        log_success!("Successfully set a new encryption key for the current profile");
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