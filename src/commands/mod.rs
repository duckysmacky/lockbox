mod encrypt;
mod decrypt;
mod path;

use std::{path::Path, io};
use clap::ArgMatches;

struct BoxOptions<'a> {
    pub custom_name: Option<&'a String>,
    pub keep_name: bool
}

pub fn box_files(args: &ArgMatches) -> io::Result<()> {
    let paths = match args.get_many::<String>("path") {
        Some(p) => p.map(|s| Path::new(s.as_str())).collect::<Vec<&Path>>(),
        None => vec![Path::new(".")],
    };

    let path_options = path::PathOptions {
        paths,
        recursive: args.get_flag("recursive")
    };

    let options = BoxOptions {
        custom_name: args.get_one::<String>("custom-name"),
        keep_name: args.get_flag("keep-name")
    };

    path::parse_path(&path_options, encrypt::encrypt, options)
}

pub fn unbox_files(args: &ArgMatches) -> io::Result<()> {
    let paths = match args.get_many::<String>("path") {
        Some(p) => p.map(|s| Path::new(s.as_str())).collect::<Vec<&Path>>(),
        None => vec![Path::new(".")],
    };

    let path_options = path::PathOptions {
        paths,
        recursive: args.get_flag("recursive")
    };

    // TEMP CODE
    let temp_name = String::from("");
    let options = BoxOptions {
        custom_name: Some(&temp_name),
        keep_name: false
    };

    path::parse_path(&path_options, decrypt::decrypt, options)
}