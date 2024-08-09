use std::{fs, io, path::{Path, PathBuf}, ffi::OsString};

use crate::encryption::parser;
use crate::{log_error, log_info};

pub struct PathOptions {
    pub input_paths: Vec<PathBuf>,
    pub recursive: bool
}

pub fn parse_paths(file_paths: &mut Vec<PathBuf>, opts: PathOptions) {
    for path in opts.input_paths {
        if path.is_dir() {
            if let Err(err) = read_dir(&path, file_paths, opts.recursive) {
                log_error!("Unable to read directory \"{}\": {}", path.display(), err);
                continue;
            }
        } else if path.is_file() {
            file_paths.push(path);
        } else if !path.exists() {
            let target_name = path.file_stem().unwrap().to_os_string();

            match search_for_original(path.parent().unwrap(), target_name) {
                Ok(box_path) => file_paths.push(box_path),
                Err(err) => {
                    log_error!("Unable to find \"{}\": {}", path.display(), err);
                    continue;
                }
            }
        }
    }
}

fn read_dir(dir_path: &Path, file_paths: &mut Vec<PathBuf>, recursive: bool) -> io::Result<()> {
    for entry in fs::read_dir(dir_path)? {
        let path = entry?.path();

        if path.is_dir() && recursive {
            read_dir(&path, file_paths, true)?;
        } else if path.is_file() {
            file_paths.push(path);
        }
    }

    Ok(())
}

fn search_for_original(dir_path: &Path, target_name: OsString) -> io::Result<PathBuf> {
    for entry in fs::read_dir(dir_path)? {
        let path = entry?.path();

        if !path.is_file() || path.extension().unwrap() != "box" { continue; }

        let original_name = parser::get_header(path.as_path())?.original_filename;

        if target_name == original_name {
            log_info!("Found an encrypted (.box) file with the same original name: {}", path.display());
            return Ok(path)
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Given file name is not present within original encrypted file names")
    )
}