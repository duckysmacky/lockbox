use std::{fs::File, io, io::Write};
use std::io::BufReader;

use serde::{Deserialize, Serialize};
use crate::{log_debug, log_error, log_fatal, log_info, log_warn};
use crate::encryption::cipher::{self, Key};
use crate::storage::get_data_dir;

#[derive(Serialize, Deserialize, Debug)]
struct KeysFile {
    key_data: Option<KeyData>
}

#[derive(Serialize, Deserialize, Debug)]
struct KeyData {
    key: Key
}

pub fn get_key() -> Key {
    log_debug!("Getting key");

    match get_keys_file() {
        Ok(keys_file) => {
            return if let Some(key_data) = keys_file.key_data {
                key_data.key
            } else {
                log_error!("Key doesn't exist");
                generate_new_key();
                get_key()
            }
        },
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound {
                log_error!("Key doesn't exist");
                generate_new_key();
                return get_key()
            }
            log_fatal!("An error occurred while trying to get key: {}", err);
        }
    };
}

pub fn delete_key() {
    log_debug!("Deleting key data");

    if let Err(_) = get_keys_file() {
        log_warn!("Encryption key doesn't exist");
    }
    let empty_keys_file = KeysFile { key_data: None };

    match write_keys_file(empty_keys_file) {
        Ok(_) => log_info!("Deleted key data"),
        Err(err) => log_fatal!("An error occurred while trying to delete key data: {}", err)
    }
}

pub fn generate_new_key() {
    log_debug!("Generating new key");
    let key = cipher::generate_key();

    let key_data = KeyData { key };
    let keys_file = KeysFile {
        key_data: Some(key_data)
    };

    match write_keys_file(keys_file) {
        Ok(_) => log_info!("Saved generated key"),
        Err(err) => {
            log_fatal!("An error occurred while trying to save key: {}", err);
        }
    }
}

fn write_keys_file(keys_file: KeysFile) -> io::Result<()> {
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

fn get_keys_file() -> io::Result<KeysFile> {
    log_debug!("Getting keys file data");

    let mut keys_path = get_data_dir();
    keys_path.push("keys.json");

    let file = File::open(keys_path)?;
    let reader = BufReader::new(file);

    let keys_file: KeysFile = serde_json::from_reader(reader)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Unable to deserialize key data"))?;

    log_debug!("Got keys file data: {:?}", keys_file);
    Ok(keys_file)
}