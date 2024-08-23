use std::{collections::VecDeque, path::PathBuf};
use std::ffi::OsStr;
use clap::ArgMatches;
use crate::cli::path;
use crate::{create_profile, delete_profile, Error, get_key, log_warn, options};
use crate::{decrypt, encrypt, log_error, log_success, new_key};
use crate::cli::prompts;

pub fn r#box(g_args: &ArgMatches, args: &ArgMatches) -> (u32, u32) {
    let mut total_files: u32 = 0;
    let mut error_files: u32 = 0;
    let mut file_paths: Vec<PathBuf> = Vec::new();

    // options for path parsing
    let options = path::PathOptions {
        input_paths: get_path_vec(args, "PATH").expect("File path is required"),
        recursive: args.get_flag("RECURSIVE-SEARCH")
    };
    path::parse_paths(&mut file_paths, options);

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

        if let Err(err) = encrypt(path.as_path(), &password, &mut options) {
            match err {
                Error::ProfileError(_) => {
                    log_error!("{}", err);
                    log_error!("New profile can be created with \"lockbox profile new\"");
                    std::process::exit(1);
                },
                Error::AuthError(_) => {
                    log_error!("{}", err);
                    log_error!("Please try again");
                    std::process::exit(1);
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
                }
            }
        } else {
            log_success!("Successfully encrypted {:?}", file_name);
        }
    }

    (total_files, error_files)
}

pub fn unbox(g_args: &ArgMatches, args: &ArgMatches) -> (u32, u32) {
    let mut total_files: u32 = 0;
    let mut error_files: u32 = 0;
    let mut file_paths: Vec<PathBuf> = Vec::new();

    // options for path parsing
    let options = path::PathOptions {
        input_paths: get_path_vec(args, "PATH").expect("File path is required"),
        recursive: args.get_flag("RECURSIVE-SEARCH")
    };
    path::parse_paths(&mut file_paths, options);

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

        if let Err(err) = decrypt(path.as_path(), &password, &mut options) {
            match err {
                Error::ProfileError(_) => {
                    log_error!("{}", err);
                    log_error!("New profile can be created with \"lockbox profile new\"");
                    std::process::exit(1);
                },
                Error::AuthError(_) => {
                    log_error!("{}", err);
                    log_error!("Please try again");
                    std::process::exit(1);
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
                }
            }
        } else {
            log_success!("Successfully decrypted {:?}", path.file_name().unwrap().to_os_string());
        }
    }

    (total_files, error_files)
}

pub fn profile_create(g_args: &ArgMatches, args: &ArgMatches) {
    let password = match g_args.get_one::<String>("PASSWORD") {
        None => prompts::prompt_password("Please enter a password for the new profile:"),
        Some(password) => password.to_string()
    };

    let name = args.get_one::<String>("NAME").expect("Profile name is required");

    if let Err(err) = create_profile(name, &password) {
        log_error!("Unable to create a new profile: {}", err);
        std::process::exit(1);
    }

    log_success!("Successfully created new profile \"{}\"", name);
}

pub fn profile_delete(g_args: &ArgMatches, args: &ArgMatches) {
    let password = match g_args.get_one::<String>("PASSWORD") {
        None => prompts::prompt_password("Please enter the password for the target profile"),
        Some(password) => password.to_string()
    };

    let name = args.get_one::<String>("NAME").expect("Profile name is required");

    if let Err(err) = delete_profile(name, &password) {
        match err {
            Error::AuthError(_) => {
                log_error!("{}", err);
                log_error!("Please try again");
                std::process::exit(1);
            },
            _ => {
                log_error!("Unable to delete the profile: {}", err);
                std::process::exit(1);
            }
        }
    }

    log_success!("Successfully deleted profile \"{}\"", name);
}

pub fn key_new(g_args: &ArgMatches, _args: &ArgMatches) {
    let password = match g_args.get_one::<String>("PASSWORD") {
        None => prompts::prompt_password("Please enter the password for the current profile:"),
        Some(password) => password.to_string()
    };

    if let Err(err) = new_key(&password) {
        match err {
            Error::AuthError(_) => {
                log_error!("{}", err);
                log_error!("Please try again");
                std::process::exit(1);
            },
            Error::ProfileError(_) => {
                log_error!("{}", err);
                log_error!("New profile can be created with \"lockbox profile new\"");
                std::process::exit(1);
            },
            _ => {
                log_error!("Unable to generate new encryption key: {}", err);
                std::process::exit(1);
            }
        }
    }

    log_success!("Successfully generated new encryption key for the current profile");
}

pub fn key_get(g_args: &ArgMatches, _args: &ArgMatches) {
    let password = match g_args.get_one::<String>("PASSWORD") {
        None => prompts::prompt_password("Please enter the password for the current profile:"),
        Some(password) => password.to_string()
    };

    let key = get_key(&password);
    if let Err(err) = &key {
        match err {
            Error::AuthError(_) => {
                log_error!("{}", err);
                log_error!("Please try again");
                std::process::exit(1);
            },
            Error::ProfileError(_) => {
                log_error!("{}", err);
                log_error!("New profile can be created with \"lockbox profile new\"");
                std::process::exit(1);
            },
            _ => {
                log_error!("Unable to get key: {}", err);
                std::process::exit(1);
            }
        }
    }

    // TODO: add current profile name
    log_success!("Encryption key for the current profile:");
    log_success!("{}", key.unwrap());
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