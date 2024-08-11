use std::{
    collections::VecDeque, path::PathBuf,
    io
};
use clap::ArgMatches;
use crate::cli::path;
use crate::{decrypt, DecryptionOptions, delete_key, DeleteKeyOptions, encrypt, EncryptionOptions, KeyOptions, log_error, log_success, new_key, NewKeyOptions};

pub fn r#box(args: &ArgMatches) -> io::Result<u32> {
    let mut total_files: u32 = 0;
    let mut file_paths: Vec<PathBuf> = Vec::new();

    let options = path::PathOptions {
        input_paths: get_path_vec(args, "path").expect("file path is required"),
        recursive: args.get_flag("recursive")
    };
    path::parse_paths(&mut file_paths, options);

    let mut options = EncryptionOptions {
        keep_name: args.get_flag("keep-name"),
        output_paths: get_path_deque(args, "output-path")
    };

    for path in file_paths {
        match encrypt(path.as_path(), &mut options) {
            Ok(_) => {
                log_success!("Successfully encrypted {:?}", path.file_name().unwrap().to_os_string());
                total_files += 1;
            },
            Err(err) => log_error!("An error has occurred while trying to encrypt {:?}: {}", path.file_name().unwrap().to_os_string(), err),
        }
    }

    Ok(total_files)
}

pub fn unbox(args: &ArgMatches) -> io::Result<u32> {
    let mut total_files: u32 = 0;
    let mut file_paths: Vec<PathBuf> = Vec::new();

    let options = path::PathOptions {
        input_paths: get_path_vec(args, "path").expect("file path is required"),
        recursive: args.get_flag("recursive")
    };
    path::parse_paths(&mut file_paths, options);

    let mut options = DecryptionOptions {
        output_paths: get_path_deque(args, "output-path")
    };

    for path in file_paths {
        match decrypt(path.as_path(), &mut options) {
            Ok(_) => {
                log_success!("Successfully decrypted {:?}", path.file_name().unwrap().to_os_string());
                total_files += 1;
            },
            Err(err) => log_error!("An error has occurred while trying to decrypt {:?}: {}", path.file_name().unwrap().to_os_string(), err),
        }
    }

    Ok(total_files)
}

pub fn key(args: &ArgMatches) {
    /* NEW */
    if let Some(_args) = args.subcommand_matches("new") {
        let options = NewKeyOptions {
            key_options: KeyOptions {}
        };
        match new_key(&options) {
            Ok(_) => log_success!("Successfully generated a new encryption key"),
            Err(err) => log_error!("An error has occurred while trying to generate a new key: {}", err)
        }
    }
    /* DELETE */
    if let Some(_args) = args.subcommand_matches("delete") {
        let options = DeleteKeyOptions {
            key_options: KeyOptions {}
        };
        match delete_key(&options) {
            Ok(_) => log_success!("Successfully deleted encryption key"),
            Err(err) => log_error!("An error has occurred while trying to delete a key: {}", err)
        }
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