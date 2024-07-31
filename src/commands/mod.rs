mod encrypt;
mod decrypt;
mod path;

use std::{path::PathBuf, io};
use clap::ArgMatches;

pub fn encrypt(args: &ArgMatches) -> io::Result<()> {
    let mut file_paths: Vec<PathBuf> = Vec::new();

    let options = path::PathOptions {
        input_paths: get_paths(args, "path").expect("file path is required"),
        recursive: args.get_flag("recursive")
    };
    path::parse_paths(&mut file_paths, options)?;

    let options = encrypt::EncryptionOptions {
        keep_name: args.get_flag("keep-name"),
        output_paths: get_paths(args, "output-path")
    };

    for path in file_paths {
        encrypt::encrypt(path.as_path(), &options)?;
    }

    Ok(())
}

pub fn decrypt(args: &ArgMatches) -> io::Result<()> {
    let mut file_paths: Vec<PathBuf> = Vec::new();

    let options = path::PathOptions {
        input_paths: get_paths(args, "path").expect("file path is required"),
        recursive: args.get_flag("recursive")
    };
    path::parse_paths(&mut file_paths, options)?;

    let options = decrypt::DecryptionOptions {
        output_paths: get_paths(args, "output-path")
    };

    for path in file_paths {
        decrypt::decrypt(path.as_path(), &options)?;
    }

    Ok(())
}


fn get_paths(args: &ArgMatches, arg_id: &str) -> Option<Vec<PathBuf>> {
    if let Some(strings) = args.get_many::<String>(arg_id) {
        return Some(strings
            .map(|s| PathBuf::from(s))
            .collect::<Vec<PathBuf>>()
        )
    }

    None
}