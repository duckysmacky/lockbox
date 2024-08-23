use crate::{Result, Error, log_debug};
use crate::data::profiles::{get_current_profile, save_profile};
use crate::encryption::cipher::{self, Key};

pub fn get_key() -> Result<Key> {
    log_debug!("Getting encryption key from current profile");

    let profile_data = get_current_profile()?;
    Ok(profile_data.key)
}

pub fn generate_new_key() -> Result<()> {
    log_debug!("Generating new encryption key for current profile");

    let mut profile = get_current_profile()?;
    profile.key = cipher::generate_key();

    save_profile(profile)
        .map_err(|err| Error::from(err))
}