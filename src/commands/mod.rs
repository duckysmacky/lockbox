mod encrypt;
mod decrypt;
mod path;

use std::{path::PathBuf, io};
use clap::ArgMatches;

struct BoxOptions<'a> {
    pub custom_name: Option<&'a String>,
    pub keep_name: bool
}

pub fn encrypt(args: &ArgMatches) -> io::Result<()> {
    let mut file_paths: Vec<PathBuf> = Vec::new();
    let input_paths: Vec<PathBuf> = args.get_many::<String>("path")
        .expect("file path is required")
        .map(|s| PathBuf::from(s))
        .collect();

    path::parse_paths(input_paths, &mut file_paths, args.get_flag("recursive"))?;

    let options = BoxOptions {
        custom_name: args.get_one::<String>("custom-name"),
        keep_name: args.get_flag("keep-name")
    };

    for path in file_paths {
        encrypt::encrypt(path.as_path(), &options)?;
    }

    Ok(())
}

pub fn decrypt(args: &ArgMatches) -> io::Result<()> {
    let mut file_paths: Vec<PathBuf> = Vec::new();
    let input_paths: Vec<PathBuf> = args.get_many::<String>("path")
        .expect("file path is required")
        .map(|s| PathBuf::from(s))
        .collect();

    path::parse_paths(input_paths, &mut file_paths, args.get_flag("recursive"))?;

    // TODO temporary code
    let temp_name = String::from("");
    let options = BoxOptions {
        custom_name: Some(&temp_name),
        keep_name: false
    };

    for path in file_paths {
        decrypt::decrypt(path.as_path(), &options)?;
    }

    Ok(())
}