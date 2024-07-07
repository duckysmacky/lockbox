use std::{
    io::{Read, Result, Write},
    fs::File
};

use super::cipher::{self, Key};

// TODO - add an actual storage
const KEY_PATH: &str = "temp/key.txt";

pub fn save_key(key: &Key) -> Result<()> {
    let mut file = File::create(KEY_PATH)?;

    file.write_all(key)?;

    file.flush()?;
    Ok(())
}

pub fn get_key() -> Result<Key> {
    let mut file = match File::open(KEY_PATH) {
        Ok(f) => f,
        Err(_) => {
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