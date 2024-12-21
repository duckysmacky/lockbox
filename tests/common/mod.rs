pub mod commands;

use std::{env, fs};
use std::path::{Path, PathBuf};

pub const PASSWORD: &str = "test1234"; // just a password for testing
pub const ORIGINAL_DIR: &str = "files/original";
pub const TEST_DIR: &str = "files/test";

pub fn setup() {
    copy_original()
}

pub fn cleanup() {
    copy_original()
}

fn copy_original() {
    let test_dir = Path::new(TEST_DIR);

    if test_dir.exists() {
        fs::remove_dir_all(test_dir).unwrap();
    }
    fs::create_dir(test_dir).unwrap();

    for entry in fs::read_dir(ORIGINAL_DIR).unwrap() {
        let original_file = entry.unwrap().path();

        if original_file.is_file() {
            let file_name = original_file.file_name().unwrap();
            let test_file = test_dir.join(file_name);

            fs::copy(&original_file, &test_file).unwrap();
        }
    }
}

fn _get_data_dir() -> PathBuf {
    let path = if cfg!(target_os = "windows") {
        let mut path = PathBuf::from(env::var("APPDATA").expect("Could not retrieve APPDATA environment variable"));
        path.push("Lockbox");
        path.push("Data");
        path
    } else if cfg!(target_os = "macos") {
        let mut path = PathBuf::from(env::var("HOME").expect("Could not retrieve HOME environment variable"));
        path.push("Library");
        path.push("Application Support");
        path.push("Lockbox");
        path
    } else {
        let mut path = PathBuf::from(env::var("HOME").expect("Could not retrieve HOME environment variable"));
        path.push(".local");
        path.push("share");
        path.push("lockbox");
        path
    };
    if !path.exists() {
        fs::create_dir_all(&path).expect("Failed to create lockbox data directory");
    }
    path
}