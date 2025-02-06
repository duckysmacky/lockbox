//! Contains core logic for key manipulation subcommands

use crate::{log_info, new_err, Key};
use crate::core::utils;
use crate::core::data::keys;
use crate::core::encryption::cipher;
pub fn new(password: &str) -> crate::Result<()> {
    log_info!("Generating a new encryption key for current profile");
    let key = cipher::generate_key();
    keys::set_key(password, key)?;
    Ok(())
}

pub fn get(password: &str, as_byte_array: bool) -> crate::Result<String> {
    log_info!("Retrieving the encryption key from the current profile");
    let key = keys::get_key(password)?;
    
    if as_byte_array {
        return Ok(format!("{:?}", key))
    }
    Ok(utils::hex::bytes_to_string(&key))
}

pub fn set(password: &str, new_key: &str) -> crate::Result<()> {
    log_info!("Setting the encryption key from the current profile");
    let new_key = utils::hex::string_to_bytes(new_key)?;
    
    if new_key.len() != 32 {
        return Err(new_err!(InvalidData: InvalidHex, "Provided hex is not a 32-byte key"))
    }
    
    let new_key = Key::try_from(&new_key[..32]).unwrap();
    keys::set_key(password, new_key)?;
    Ok(())
}