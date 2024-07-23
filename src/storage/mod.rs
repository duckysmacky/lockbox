use std::{
    io::{Read, Result, Write},
    fs::File
};
use crate::{log_info, log_warn};
use crate::encryption::cipher;
use crate::encryption::cipher::Key;

// TODO - add an actual storage
const KEY_PATH: &str = "temp/key.txt";

pub fn save_key(key: &Key) -> Result<()> {
    let mut file = File::create(KEY_PATH)?;

    file.write_all(key)?;
    log_info!("Saved key");

    file.flush()?;
    Ok(())
}

pub fn get_key() -> Result<Key> {
    let mut file = match File::open(KEY_PATH) {
        Ok(f) => {
            log_info!("Found key");
            f
        },
        Err(_) => {
            log_warn!("Key not found!");
            log_info!("Generating new key");
            let key = cipher::generate_key();
            save_key(&key)?;
            File::open(KEY_PATH)?
        }
    };
    let mut key = [0u8; 32];

    file.read_exact(&mut key)?;

    file.flush()?;
    Ok(key)
}