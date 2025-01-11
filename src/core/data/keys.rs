//! Contains wrapper functions above profiles to get and set current profile's key

use super::profile;
use crate::log_debug;
use crate::{Key, Result};

/// Gets the key for the current profile
pub fn get_key() -> Result<Key> {
    log_debug!("Getting encryption key from current profile");
    let mut profiles = profile::get_profiles();
    let profile = profiles.get_current_profile()?;
    Ok(profile.key)
}

/// Sets the key for the current profile
pub fn set_key(new_key: Key) -> Result<()> {
    log_debug!("Setting a new encryption key for current profile");
    let mut profiles = profile::get_profiles();
    let profile = profiles.get_current_profile()?;
    profile.set_key(new_key);
    profiles.save()
}