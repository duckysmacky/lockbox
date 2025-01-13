//! Contains everything related to outside data manipulations, filesystem and operating system
//! interactions

use crate::core::data::profile::LockboxProfiles;
use crate::{log_debug, log_error};
use crate::core::data::config::LockboxConfig;

pub mod keys;
mod auth;
pub mod profile;
pub mod config;
pub mod os;
pub mod io;

/// Fetches the Lockbox profiles by importing it from the file on the disk. Will error and exit in
/// case of the operation failing
pub fn get_profiles() -> LockboxProfiles {
    log_debug!("Getting Lockbox profiles");

    match LockboxProfiles::import() {
        Ok(profiles) => profiles,
        Err(err) => {
            log_error!("Unable to import Lockbox profiles");
            log_error!("{}", err);
            std::process::exit(1);
        }
    }
}

/// Fetches the Lockbox config by importing it from the file on the disk. Will error and exit in
/// case of the operation failing
#[allow(dead_code)]
pub fn get_config() -> LockboxConfig {
    log_debug!("Getting Lockbox profiles");

    match LockboxConfig::import() {
        Ok(config) => config,
        Err(err) => {
            log_error!("Unable to import Lockbox config");
            log_error!("{}", err);
            std::process::exit(1);
        }
    }
}
