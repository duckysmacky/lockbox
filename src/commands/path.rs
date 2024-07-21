use std::{fs, io, path::Path};
use std::ffi::OsString;

use clap::ArgMatches;

use crate::encryption::parser;

pub fn parse_path(args: &ArgMatches, callback: fn(&ArgMatches, &Path) -> io::Result<()>) -> io::Result<()> {
    let paths = match args.get_many::<String>("path") {
        Some(p) => p.map(|s| Path::new(s.as_str())).collect::<Vec<&Path>>(),
        None => vec![Path::new(".")],
    };

    for path in paths {
        if path.is_dir() {
            read_dir(path, args, callback)?;
        } else if path.is_file() {
            callback(args, path)?;
        } else if !path.exists() {
            let target_name = path.file_stem().unwrap().to_os_string();

            match search_for_original(path.parent().unwrap(), target_name) {
                Ok(box_path) => callback(args, Path::new(&box_path))?,
                Err(_) => {
                    println!("Path \"{}\" doesn't exist!", path.display());
                    continue;
                }
            }
        }
    }

    Ok(())
}

fn read_dir(dir: &Path, args: &ArgMatches, callback: fn(&ArgMatches, &Path) -> io::Result<()>) -> io::Result<()> {
    let recursive = args.get_flag("recursive");
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && recursive {
            read_dir(&path, args, callback)?;
        } else if path.is_file() {
            callback(args, &path)?;
        }
    }

    Ok(())
}

fn search_for_original(dir: &Path, target_name: OsString) -> io::Result<OsString> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if !path.is_file() || path.extension().unwrap() != "box" {
            continue;
        }

        let original_name = parser::get_header(path.as_path())?.original_filename;
        if target_name == original_name {
            println!("Found an encrypted (.box) file with the same original name: {}", path.display());
            return Ok(path.as_os_str().to_os_string())
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Given file name is not present within original encrypted file names")
    )
}