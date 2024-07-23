use std::{fs, io, path::Path, ffi::OsString};

use crate::encryption::parser;
use crate::{log_error, log_info};
use crate::commands::BoxOptions;

type CallbackFunction = fn(&Path, &BoxOptions) -> io::Result<()>;

pub struct PathOptions<'a> {
    pub paths: Vec<&'a Path>,
    pub recursive: bool
}

pub fn parse_path(opts: &PathOptions, callback: CallbackFunction, callback_options: BoxOptions) -> io::Result<()> {
    let paths = &opts.paths;

    for path in paths {
        if path.is_dir() {
            read_dir(path, opts, callback, &callback_options)?;
        } else if path.is_file() {
            callback(path, &callback_options)?;
        } else if !path.exists() {
            let target_name = path.file_stem().unwrap().to_os_string();

            match search_for_original(path.parent().unwrap(), target_name) {
                Ok(box_path) => callback(Path::new(&box_path), &callback_options)?,
                Err(_) => {
                    log_error!("Path \"{}\" doesn't exist!", path.display());
                    continue;
                }
            }
        }
    }

    Ok(())
}

fn read_dir(dir: &Path, opts: &PathOptions, callback: CallbackFunction, callback_options: &BoxOptions) -> io::Result<()> {
    let recursive = opts.recursive;

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && recursive {
            read_dir(&path, opts, callback, callback_options)?;
        } else if path.is_file() {
            callback(&path, callback_options)?;
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
            log_info!("Found an encrypted (.box) file with the same original name: {}", path.display());
            return Ok(path.as_os_str().to_os_string())
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Given file name is not present within original encrypted file names")
    )
}