pub mod keys;

use std::{path::PathBuf, env, fs};

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
        fs::create_dir_all(&path).expect("Failed to create Data directory");
    }

    path
}

#[allow(dead_code)]
fn get_config_dir() -> PathBuf {
    let path = if cfg!(target_os = "windows") {
        let mut path = PathBuf::from(env::var("APPDATA").expect("Could not retrieve APPDATA environment variable"));
        path.push("Lockbox");
        path.push("Config");
        path
    } else if cfg!(target_os = "macos") {
        let mut path = PathBuf::from(env::var("HOME").expect("Could not retrieve HOME environment variable"));
        path.push("Library");
        path.push("Preferences");
        path.push("Lockbox");
        path
    } else { // Assuming Linux or other Unix-like OS
        let mut path = PathBuf::from(env::var("HOME").expect("Could not retrieve HOME environment variable"));
        path.push(".config");
        path.push("lockbox");
        path
    };

    if !path.exists() {
        fs::create_dir_all(&path).expect("Failed to create Config directory");
    }

    path
}