use std::{env, fs};
use std::path::{Path, PathBuf};

pub const PASSWORD: &str = "test123"; // just a password for testing
#[allow(dead_code)]
pub const ORIGINAL_DIR: &str = "test-files/original";
#[allow(dead_code)]
pub const TEST_DIR: &str = "test-files/test";

#[allow(dead_code)]
pub fn setup() {
    let test_path = Path::new(TEST_DIR);
    let original_path = Path::new(ORIGINAL_DIR);

    if !test_path.exists() {
        fs::create_dir_all(test_path).unwrap();
    }

    for entry in fs::read_dir(&original_path).unwrap() {
        let entry = entry.unwrap();
        let original_path = entry.path();
        if original_path.is_file() {
            let file_name = original_path.file_name().unwrap();
            let test_path = test_path.join(file_name);
            fs::copy(&original_path, &test_path).unwrap();
        }
    }
}

#[allow(dead_code)]
pub fn cleanup() {
    let test_path = Path::new(TEST_DIR);
    if test_path.exists() {
        fs::remove_dir_all(test_path).unwrap();
    }

    let data_path = get_data_dir();
    if data_path.as_path().exists() {
        fs::remove_dir_all(data_path).unwrap();
    }
}

fn get_data_dir() -> PathBuf {
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
    } else { // Assuming Linux or other Unix-like OS
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