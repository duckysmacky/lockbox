//! Contains everything related to outside data manipulations, filesystem and operating system
//! interactions

use crate::{Result, log_debug};
use profile::DataboxerProfiles;
use config::DataboxerConfig;

pub mod profile;
pub mod keys;
pub mod config;
pub mod os;
pub mod io;
mod auth;

/// Fetches the Databoxer profiles by importing it from the file on the disk. Will return an error in
/// case of the operation failing
pub fn get_profiles() -> Result<DataboxerProfiles> {
    log_debug!("Getting Databoxer profiles");
    let data_directory = os::get_data_dir()?;
    DataboxerProfiles::import(data_directory)
}

/// Fetches the Databoxer config by importing it from the file on the disk. Will return an error in
/// case of the operation failing
#[allow(dead_code)]
pub fn get_config() -> Result<DataboxerConfig> {
    log_debug!("Getting Databoxer profiles");
    let config_directory = os::get_config_dir()?;
    DataboxerConfig::import(config_directory)
}
