use std::{fs::File, io, io::Write};
use std::io::BufReader;

use serde::{Deserialize, Serialize};
use crate::{log_debug, log_error, log_fatal, log_info, log_success, log_warn};
use crate::encryption::cipher::{self, Key};
use crate::storage::{auth, get_data_dir};

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyFile {
    pub key_data: Option<KeyData>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyData {
    pub key: Key,
    pub password_hash: String,
}

pub fn get_key_data(password: &str) -> KeyData {
    log_debug!("Getting key data");

    match get_keys_file() {
        Ok(key_file) => {
            return if let Some(key_data) = key_file.key_data {
                key_data
            } else {
                create_new_key(password)
            }
        }
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound {
                create_new_key(password)
            } else {
                log_fatal!("An error occurred while trying to get key file data: {}", err);
            }
        }
    }
}

pub fn delete_key() {
    log_debug!("Deleting key data");

    if let Err(_) = get_keys_file() {
        log_warn!("Encryption key doesn't exist");
    }
    let empty_keys_file = KeyFile { key_data: None };

    match write_keys_file(empty_keys_file) {
        Ok(_) => log_info!("Deleted key data"),
        Err(err) => log_fatal!("An error occurred while trying to delete key data: {}", err)
    }
}

pub fn generate_key(password: &str) {
    log_debug!("Generating a new encryption key");
    let key = cipher::generate_key();
    let (hash, _salt) = auth::hash_password(password);

    let key_file = KeyFile {
        key_data: Some(KeyData {
            key,
            password_hash: hash,
        })
    };

    match write_keys_file(key_file) {
        Ok(_) => log_info!("Saved the new key"),
        Err(err) => {
            log_fatal!("An error occurred while trying to save the new key: {}", err);
        }
    }
}

pub fn create_new_key(password: &str) -> KeyData {
    log_error!("Encryption key doesn't exist");
    log_success!("Creating a new encryption key with given password");
    generate_key(password);
    get_key_data(password)
}

fn write_keys_file(keys_file: KeyFile) -> io::Result<()> {
    log_debug!("Writing data to keys file: {:?}", keys_file);

    let mut keys_path = get_data_dir();
    keys_path.push("keys.json");

    let mut file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(keys_path)?;

    serde_json::to_writer(&file, &keys_file)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Unable to serialize keys file"))?;

    file.flush()?;
    Ok(())
}

fn get_keys_file() -> io::Result<KeyFile> {
    log_debug!("Getting keys file data");

    let mut keys_path = get_data_dir();
    keys_path.push("keys.json");

    let file = File::open(keys_path)?;
    let reader = BufReader::new(file);

    let keys_file: KeyFile = serde_json::from_reader(reader)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Unable to deserialize key data"))?;

    log_debug!("Got keys file data: {:?}", keys_file);
    Ok(keys_file)
}