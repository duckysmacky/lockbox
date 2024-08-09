use std::{fs::File, io, io::Write};
use std::io::BufReader;

use serde::{Deserialize, Serialize};

use crate::{log_debug, log_error, log_fatal, log_success};
use crate::encryption::cipher::{self, Key};
use crate::storage::get_data_dir;

#[derive(Serialize, Deserialize, Debug)]
struct KeyData {
    key: Option<Key>
}

pub fn get_key() -> Key {
    log_debug!("Getting key");

    let key = match get_key_data() {
        Ok(key_data) => key_data.key,
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound {
                log_error!("Key doesn't exist");
                generate_new_key();
                return get_key()
            }
            log_fatal!("An error occurred while trying to get key: {}", err);
        }
    };

    if key == None {
        log_error!("Encryption key doesn't exist");
        generate_new_key();
        return get_key()
    }
    key.unwrap()
}

pub fn save_key(key: Key) {
    log_debug!("Saving key");
    let key_data = KeyData {
        key: Some(key)
    };

    match write_key_data(key_data) {
        Ok(_) => log_success!("Successfully saved key"),
        Err(err) => {
            log_fatal!("An error occurred while trying to save key: {}", err);
        }
    }
}

#[allow(dead_code)]
pub fn delete_key() {
    log_debug!("Deleting key");

    let mut key_data = get_key_data().unwrap_or_else(|_| {
        log_error!("Encryption key doesn't exist");
        KeyData { key: None }
    });
    key_data.key = None;

    match write_key_data(key_data) {
        Ok(_) => log_success!("Successfully deleted key"),
        Err(err) => log_fatal!("An error occurred while trying to delete key: {}", err)
    }
}

fn generate_new_key() {
    log_debug!("Generating new key");
    let key = cipher::generate_key();
    save_key(key);
    log_success!("Successfully generated a new key");
}

fn write_key_data(key_data: KeyData) -> io::Result<()> {
    log_debug!("Writing saved key data: {:?}", key_data);

    let mut keys_path = get_data_dir();
    keys_path.push("keys.json");

    let mut file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(keys_path)?;

    serde_json::to_writer(&file, &key_data)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Unable to serialize key data"))?;

    file.flush()?;
    Ok(())
}

fn get_key_data() -> io::Result<KeyData> {
    log_debug!("Getting saved key data");

    let mut keys_path = get_data_dir();
    keys_path.push("keys.json");

    let file = match File::open(keys_path) {
        Ok(f) => f,
        Err(_) => {
            log_error!("Encryption key doesn't exist");
            generate_new_key();
            return get_key_data()
        }
    };
    let reader = BufReader::new(file);

    let key_data = serde_json::from_reader(reader)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Unable to deserialize key data"))?;

    log_debug!("Got key data: {:?}", key_data);
    Ok(key_data)
}