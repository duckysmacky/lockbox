//! Contains common functions and constatants for running tests.

pub mod commands;

use std::{env, fs, io};
use std::path::{Path, PathBuf};

pub const PASSWORD: &str = "test1234"; // just a password for testing
pub const ORIGINAL_DIR: &str = "files/original";
pub const TEST_DIR: &str = "files/test";

/// Global test environment setup (must be run before each test)
pub fn setup() {
    if let Err(err) = copy_original() {
        panic!("Unable to copy original test files: {}", err)
    }
}

/// Global test environment cleanup (must be run after each test)
pub fn cleanup() {
    if let Err(err) = copy_original() {
        panic!("Unable to copy original test files: {}", err)
    }
}

/// Copies original test files for use in tests
fn copy_original() -> io::Result<()> {
    let test_dir = Path::new(TEST_DIR);

    if test_dir.exists() {
        fs::remove_dir_all(test_dir)?;
    }
    fs::create_dir(test_dir)?;

    for entry in fs::read_dir(ORIGINAL_DIR)? {
        let original_file = entry?.path();

        if original_file.is_file() {
            let file_name = original_file.file_name().unwrap();
            let test_file = test_dir.join(file_name);

            fs::copy(&original_file, &test_file)?;
        }
    }

    Ok(())
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