use std::{fs::File, io, io::Write};
use std::io::BufReader;

use serde::{Deserialize, Serialize};

use crate::{log_debug, log_error, log_info, log_success};
use crate::encryption::cipher::{self, Key};
use crate::storage::get_data_dir;

#[derive(Serialize, Deserialize, Debug)]
struct KeyData {
    key: Key
}

pub fn get() -> Key {
    log_debug!("Fetching saved key");

    match get_key_data() {
        Ok(key_data) => return key_data.key,
        Err(err) => {
            log_error!("An error occurred while trying to get key: {}", err);
            std::process::exit(1);
        }
    }
}

pub fn save(key: Key) {
    log_debug!("Saving key");
    let key_data = KeyData { key };

    match write_key_data(key_data) {
        Ok(_) => log_success!("Successfully saved key"),
        Err(err) => {
            log_error!("An error occurred while trying to save key: {}", err);
            std::process::exit(1);
        }
    }
}

fn generate_new_key() {
    log_info!("Generating new key");
    let key = cipher::generate_key();
    save(key);
}

fn write_key_data(key_data: KeyData) -> io::Result<()> {
    log_debug!("Writing saved key data");

    let mut key_path = get_data_dir();
    key_path.push("keys.json");

    let mut file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(key_path)?;

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

    Ok(key_data)
}