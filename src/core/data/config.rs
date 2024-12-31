//! Contains functions for program config

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read, Write};
use crate::log_debug;
use crate::core::error::{Result, Error};
use super::get_config_dir;

/// Name of the main configuration file
const CONFIG_FILE: &str = "lockbox.toml";

/// Struct representing a TOML Configuration file
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFileData {
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

impl ConfigFileData {
    pub fn default() -> ConfigFileData {
        ConfigFileData {
            general: GeneralConfig { },
            encryption: EncryptionConfig { },
            storage: StorageConfig { }
        }
    }
}

/// Fetches and returns config file data
#[allow(dead_code)]
pub fn get_config() -> Result<ConfigFileData> {
    log_debug!("Getting config data");

    match read_config_file() {
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
pub fn save_config(config: ConfigFileData) -> Result<()> {
    log_debug!("Saving config data");

    write_config_file(config).map_err(|err| Error::from(err))
}

/// Writes to config file. Overwrites old data
fn write_config_file(config_file: ConfigFileData) -> io::Result<()> {
    log_debug!("Writing data to config file: {:?}", config_file);

    let mut path = get_config_dir();
    path.push(CONFIG_FILE);

    let mut file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    file.write(toml::to_string(&config_file).unwrap().as_bytes())
        .map_err(|_| io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid data was provided for the configuration file"
        ))?;

    file.flush()?;
    Ok(())
}

/// Reads and returns config data from file
fn read_config_file() -> io::Result<ConfigFileData> {
    log_debug!("Getting config file data");

    let mut path = get_config_dir();
    path.push(CONFIG_FILE);

    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound {
                write_config_file(ConfigFileData::default())?;
                File::open(path)?
            } else {
                return Err(err)
            }
        }
    };

    let mut file_data = String::new();
    file.read_to_string(&mut file_data)?;

    let config_file: ConfigFileData = toml::from_str(&file_data)
        .map_err(|_| io::Error::new(
            io::ErrorKind::InvalidData,
            "Unable to deserialize config file data")
        )?;

    log_debug!("Got config file data: {:?}", config_file);
    file.flush()?;
    Ok(config_file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn write_default_config() {
        write_config_file(ConfigFileData::default()).unwrap();
        let config = get_config();

        assert!(config.is_ok())
    }
}