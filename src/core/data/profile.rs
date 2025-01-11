//! Contains function for user profile manipulation

use super::auth;
use super::io::{read_file, write_file};
use crate::core::data::os;
use crate::core::encryption::cipher;
use crate::{log_debug, log_error, log_info, Error, Key, Result};
use serde::{Deserialize, Serialize};
use std::io::{self};
use std::path::PathBuf;

/// Name of the file which stores all the profile data
const PROFILES_FILE_NAME: &str = "profiles.json";

/// Struct holding all the needed profile information for the program. Saved on the disk as a JSON
/// file
#[derive(Serialize, Deserialize, Debug)]
pub struct LockboxProfiles {
    current_profile: Option<String>,
    profiles: Vec<Profile>,
    #[serde(skip)]
    file_path: PathBuf
}

/// Object-driven approach
impl LockboxProfiles {
    /// Imports self from the stored "profiles.json" file. In case of the file missing, generates a
    /// new object with default empty values
    pub fn import() -> Result<Self> {
        log_debug!("Importing Lockbox profiles");
        let data_directory = os::get_data_dir()?;
        let profiles_file = data_directory.join(PROFILES_FILE_NAME);

        let profiles = match read_file(&profiles_file) {
            Ok(file_data) => {
                let mut profiles: LockboxProfiles = serde_json::from_str(&file_data)?;
                profiles.file_path = profiles_file;
                profiles
            },
            Err(err) => {
                if err.kind() == io::ErrorKind::NotFound {
                    log_info!("\"profiles.json\" file doesn't exist. Generating new profiles data");
                    Self::new(profiles_file)
                } else {
                    return Err(err.into());
                }
            }
        };

        Ok(profiles)
    }

    /// Bare-minimum constructor to use in case of file not being available to import from
    fn new(
        file_path: PathBuf
    ) -> Self {
        LockboxProfiles {
            current_profile: None,
            profiles: vec![],
            file_path
        }
    }

    /// Returns currently selected profile data
    pub fn get_current_profile(&mut self) -> Result<&mut Profile> {
        log_debug!("Getting current profile");
        let current_profile = self.current_profile.clone();

        let profile = match current_profile {
            None => return Err(Error::ProfileError("No profile is currently selected".to_string())),
            Some(profile_name) => {
                self.find_profile(&profile_name)?
            }
        };

        Ok(profile)
    }
    
    /// Returns a list of currently available profiles
    pub fn get_profiles(&self) -> &Vec<Profile> {
        log_debug!("Getting all available profiles");
        &self.profiles
    }

    /// Sets the current profile to profile which name was supplied. Returns an error if given
    /// profile doesn't exist
    pub fn set_current(&mut self, profile_name: &str) -> Result<()> {
        log_debug!("Setting current profile to \"{}\"", profile_name);

        self.current_profile = Some(profile_name.to_string());
        self.save()?;

        log_debug!("Set current profile to \"{}\"", profile_name);
        Ok(())
    }

    /// Deletes a profile with provided name
    pub fn delete_profile(&mut self, profile_name: &str) -> Result<()> {
        log_debug!("Deleting profile with name \"{}\"", profile_name);

        for (i, profile) in self.profiles.iter().enumerate() {
            if profile.name == profile_name {
                self.profiles.remove(i);
                self.current_profile = {
                    if self.profiles.is_empty() {
                        None
                    } else {
                        Some(self.profiles.get(0).unwrap().name.clone())
                    }
                };
                self.save()?;
                return Ok(())
            }
        }

        Err(Error::ProfileError(format!("Profile with name \"{}\" doesn\'t exist", profile_name)))
    }

    /// Saves provided profile data to profiles file. Updates existing profile or creates a new one,
    /// if it doesn't already exist
    #[allow(dead_code)]
    pub fn save_profile(&mut self, profile: Profile) -> Result<()> {
        log_debug!("Saving profile: {:?}", &profile);

        let profile_name = profile.name.clone();

        if self.profiles.is_empty() {
            self.profiles.push(profile);
            self.current_profile = Some(profile_name);
        } else {
            for i in 0..self.profiles.len() {
                if self.profiles[i].name == profile_name {
                    self.profiles.insert(i, profile);
                    break;
                }

                if i == self.profiles.len() - 1 {
                    self.profiles.push(profile);
                    break;
                }
            }
        }

        self.save()?;
        Ok(())
    }

    /// Adds a new profile to the profiles file. Errors if the profile already exists, as this
    /// functions only accepts new profiles
    pub fn new_profile(&mut self, profile: Profile) -> Result<()> {
        log_debug!("Adding a new profile: {:?}", &profile);

        let profile_name = profile.name.clone();
        if self.find_profile(&profile_name).is_ok() {
            return Err(Error::ProfileError(format!("Profile with name \"{}\" already exists", profile_name)));
        }
        self.profiles.push(profile);

        self.save()?;
        Ok(())
    }

    /// Returns profile for profile which name was supplied
    pub fn find_profile(&mut self, profile_name: &str) -> Result<&mut Profile> {
        log_debug!("Searching for profile with name \"{}\"", profile_name);

        for profile in &mut self.profiles {
            if profile.name == profile_name {
                return Ok(profile)
            }
        }

        Err(Error::ProfileError(format!("Profile with name \"{}\" doesn\'t exist", profile_name)))
    }

    /// Writes to the profile data file. Overwrites old data
    pub fn save(&self) -> Result<()> {
        log_debug!("Saving profiles data to \"profiles.json\"");
        let json_data = serde_json::to_string(&self)?;

        write_file(&self.file_path, &json_data, true)?;
        Ok(())
    }
}

/// Struct containing main information about a profile
#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub name: String,
    pub key: Key,
    password_hash: String
}

impl Profile {
    pub fn new(
        name: &str,
        password: &str
    ) -> Result<Self> {
        let (hash, _salt) = auth::hash_password(password)?;
        Ok(Profile {
            name: name.to_string(),
            key: cipher::generate_key(),
            password_hash: hash,
        })
    }

    /// Checks whether the provided password is valid for the profile
    pub fn verify_password(&self, password: &str) -> bool {
        auth::verify_password(&self.password_hash, password)
    }

    /// Sets a new key for the profile
    pub fn set_key(&mut self, key: Key) {
        self.key = key;
    }
}

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