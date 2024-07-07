use std::{
    fs::{self, File}, io::{Read, Result, Write}
};

use chacha20poly1305::Key;

// TODO - add an actual storage
const KEY_PATH: &str = "temp/key.txt";

pub fn save_key(key: &Key) -> Result<()> {
    let mut file = File::create(KEY_PATH)?;

    file.write_all(key)?;

    file.flush()?;
    Ok(())
}

pub fn get_key() -> Result<Vec<u8>> {
    let mut file = File::open(KEY_PATH).expect("Key not found");
    let metadata = fs::metadata(KEY_PATH)?;
    let mut key = vec![0u8; metadata.len() as usize];

    file.read(&mut key)?;

    file.flush()?;
    Ok(key)
}