mod r#box;
mod unbox;
mod path;

use std::{path::PathBuf, io};
use std::collections::VecDeque;
use clap::ArgMatches;

pub fn encrypt(args: &ArgMatches) -> io::Result<()> {
    let mut file_paths: Vec<PathBuf> = Vec::new();

    let options = path::PathOptions {
        input_paths: get_path_vec(args, "path").expect("file path is required"),
        recursive: args.get_flag("recursive")
    };
    path::parse_paths(&mut file_paths, options)?;

    let mut options = r#box::EncryptionOptions {
        keep_name: args.get_flag("keep-name"),
        output_paths: get_path_deque(args, "output-path")
    };

    for path in file_paths {
        r#box::encrypt(path.as_path(), &mut options)?;
    }

    Ok(())
}

pub fn decrypt(args: &ArgMatches) -> io::Result<()> {
    let mut file_paths: Vec<PathBuf> = Vec::new();

    let options = path::PathOptions {
        input_paths: get_path_vec(args, "path").expect("file path is required"),
        recursive: args.get_flag("recursive")
    };
    path::parse_paths(&mut file_paths, options)?;

    let mut options = unbox::DecryptionOptions {
        output_paths: get_path_deque(args, "output-path")
    };

    for path in file_paths {
        unbox::decrypt(path.as_path(), &mut options)?;
    }

    Ok(())
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