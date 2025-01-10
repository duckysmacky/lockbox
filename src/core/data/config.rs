//! Contains functions for program config

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use crate::log_debug;
use crate::core::error::{Result, Error};
use crate::core::data::os::get_config_dir;

/// Name of the main configuration file
const CONFIG_FILE_NAME: &str = "lockbox.toml";

/// Struct representing a TOML Configuration file
#[derive(Serialize, Deserialize, Debug)]
pub struct LockboxConfig {
    pub general: GeneralConfig,
    pub encryption: EncryptionConfig,
    pub storage: StorageConfig
}

/// Struct containing general configuration for the program
#[derive(Serialize, Deserialize, Debug)]
pub struct GeneralConfig { }

/// Struct containing encryption configuration for the program
#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptionConfig { }

/// Struct containing storage configuration for the program
#[derive(Serialize, Deserialize, Debug)]
pub struct StorageConfig { }

impl LockboxConfig {
    pub fn default() -> LockboxConfig {
        LockboxConfig {
            general: GeneralConfig { },
            encryption: EncryptionConfig { },
            storage: StorageConfig { }
        }
    }
}

/// Fetches and returns config file data
#[allow(dead_code)]
pub fn get_config() -> Result<LockboxConfig> {
    log_debug!("Getting config data");

    let config_dir = get_config_dir()?;
    match read_config_file(&config_dir) {
        Ok(config) => Ok(config),
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound {
                Err(Error::ConfigError("No config file found".to_string()))
            } else {
                Err(Error::IOError(format!("Error reading config file: {}", err)))
            }
        }
    }
}

/// Saves the provided config file data
#[allow(dead_code)]
pub fn save_config(config: LockboxConfig) -> Result<()> {
    log_debug!("Saving config data");

    let config_dir = get_config_dir()?;
    write_config_file(config, &config_dir).map_err(|err| Error::from(err))
}

/// Writes to config file. Overwrites old data
fn write_config_file(config_data: LockboxConfig, config_directory: &PathBuf) -> io::Result<()> {
    log_debug!("Writing data to config file: {:?}", config_data);

    let config_file = config_directory.join(CONFIG_FILE_NAME);

    let mut file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(config_file)?;

    file.write(toml::to_string(&config_data).unwrap().as_bytes())
        .map_err(|_| io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid data was provided for the configuration file"
        ))?;

    file.flush()?;
    Ok(())
}

/// Reads and returns config data from file
fn read_config_file(config_directory: &PathBuf) -> io::Result<LockboxConfig> {
    log_debug!("Getting config file data");

    let config_file = config_directory.join(CONFIG_FILE_NAME);

    let config_data = match File::open(&config_file) {
        Ok(mut file) => {
            let mut file_data = String::new();
            file.read_to_string(&mut file_data)?;

            toml::from_str(&file_data)
                .map_err(|err| io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Config file contains invalid data: {}", err.to_string()))
                )?
        },
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound {
                LockboxConfig::default()
            } else {
                return Err(err)
            }
        }
    };

    log_debug!("Got config file data: {:?}", config_data);
    Ok(config_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn write_default_config() {
        let config_dir = get_config_dir().unwrap();
        write_config_file(LockboxConfig::default(), &config_dir).unwrap();
        let config = get_config();

        assert!(config.is_ok())
    }
}