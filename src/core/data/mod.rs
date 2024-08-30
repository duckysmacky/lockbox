//! Contains functions to access profiles, keys, config and other program data

pub mod keys;
pub mod auth;
pub mod profiles;

use std::{path::PathBuf, env, fs};
use serde::{Deserialize, Serialize};
use crate::Key;

#[derive(Serialize, Deserialize, Debug)]
struct ProfilesData {
    pub current_profile: Option<String>,
    pub profiles: Vec<Profile>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub name: String,
    pub key: Key,
    pub password_hash: String,
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
        fs::create_dir_all(&path).expect("Failed to create lockbox config directory");
    }

    path
}