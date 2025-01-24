//! Contains everything related to outside data manipulations, filesystem and operating system
//! interactions

use crate::{Result, log_debug};
use profile::LockboxProfiles;
use config::LockboxConfig;

pub mod profile;
pub mod keys;
pub mod config;
pub mod os;
pub mod io;
mod auth;

/// Fetches the Lockbox profiles by importing it from the file on the disk. Will return an error in
/// case of the operation failing
pub fn get_profiles() -> Result<LockboxProfiles> {
    log_debug!("Getting Lockbox profiles");
    let data_directory = os::get_data_dir()?;
    LockboxProfiles::import(data_directory)
}

/// Fetches the Lockbox config by importing it from the file on the disk. Will return an error in
/// case of the operation failing
#[allow(dead_code)]
pub fn get_config() -> Result<LockboxConfig> {
    log_debug!("Getting Lockbox profiles");
    let config_directory = os::get_config_dir()?;
    LockboxConfig::import(config_directory)
}
