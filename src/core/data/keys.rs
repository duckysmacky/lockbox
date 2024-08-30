use crate::{Error, Result, Key};
use crate::log_debug;
use crate::core::encryption::cipher;
use super::profiles;

pub fn get_key() -> Result<Key> {
    log_debug!("Getting encryption key from current profile");

    let profile_data = profiles::get_current_profile()?;
    Ok(profile_data.key)
}

pub fn set_key(new_key: Key) -> Result<()> {
    log_debug!("Setting a new encryption key for current profile");

    let mut profile = profiles::get_current_profile()?;
    profile.key = new_key;

    profiles::save_profile(profile)
        .map_err(|err| Error::IOError(format!("Unable to save profile data: {}", err)))
}

pub fn generate_new_key() -> Result<()> {
    log_debug!("Generating new encryption key for current profile");

    let mut profile = profiles::get_current_profile()?;
    profile.key = cipher::generate_key();

    profiles::save_profile(profile)
        .map_err(|err| Error::IOError(format!("Unable to save profile data: {}", err)))
}