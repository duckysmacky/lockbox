//! Contains function for user profile manipulation

use std::io::{self, BufReader, Write};
use std::fs::File;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::core::encryption::cipher;
use crate::{log_debug, Error, Key, Result};
use crate::core::data::os::get_data_dir;
use super::auth;

/// Name of the file which stores all the profile data
const PROFILES_FILE_NAME: &str = "profiles.json";

/// Struct representing a JSON profile data file
#[derive(Serialize, Deserialize, Debug)]
struct ProfilesData {
    pub current_profile: Option<String>,
    pub profiles: Vec<Profile>
}

/// Struct containing main information about a profile
#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub name: String,
    pub key: Key,
    pub password_hash: String,
}

impl ProfilesData {
    pub fn default() -> ProfilesData {
        ProfilesData {
            current_profile: None,
            profiles: vec![]
        }
    }
}

/// Sets the current profile to profile which name was supplied. Returns an error if given profile
/// doesn't exist
pub fn set_current_profile(profile_name: &str) -> Result<()> {
    log_debug!("Setting current profile to \"{}\"", profile_name);

    let data_dir = get_data_dir()?;
    let profiles_data = read_profiles_file(&data_dir);
    if let Err(err) = profiles_data {
        if err.kind() == io::ErrorKind::NotFound {
            return Err(Error::ProfileError("No profile data found".to_string()))
        }
        return Err(Error::IOError(format!("Error getting profiles data: {}", err)))
    }
    let mut profiles_data = profiles_data?;
    profiles_data.current_profile = Some(profile_name.to_string());

    write_profiles_file(profiles_data, &data_dir)?;
    Ok(())
}

/// Returns currently selected profile data
pub fn get_current_profile() -> Result<Profile> {
    log_debug!("Getting current profile");

    let data_dir = get_data_dir()?;
    let profiles_data = read_profiles_file(&data_dir);
    if let Err(err) = profiles_data {
        if err.kind() == io::ErrorKind::NotFound {
            return Err(Error::ProfileError("No profile data found".to_string()))
        }
        return Err(Error::IOError(format!("Error getting profiles data: {}", err)))
    }

    let current_profile = profiles_data?.current_profile;
    if current_profile.is_none() {
        return Err(Error::ProfileError("No profile is currently selected".to_string()))
    }

    let profile = get_profile(&current_profile.unwrap())?;
    Ok(profile)
}

/// Returns profile data for profile which name was supplied
pub fn get_profile(profile_name: &str) -> Result<Profile> {
    log_debug!("Getting profile with name \"{}\"", profile_name);

    for profile in get_profiles()? {
        if profile.name == profile_name {
            return Ok(profile)
        }
    }
    Err(Error::ProfileError(format!("Profile with name \"{}\" doesn\'t exist", profile_name)))
}

/// Returns a list of currently available profiles
pub fn get_profiles() -> Result<Vec<Profile>> {
    log_debug!("Getting all available profiles");

    let data_dir = get_data_dir()?;
    let profiles_data = read_profiles_file(&data_dir);
    if let Err(err) = profiles_data {
        if err.kind() == io::ErrorKind::NotFound {
            return Err(Error::ProfileError("No profile data found".to_string()))
        }
        return Err(Error::IOError(format!("Error getting profiles data: {}", err)))
    }

    Ok(profiles_data?.profiles)
}

/// Creates a new profile with provided name and password. Password is hashed automatically
pub fn create_new_profile(name: &str, password: &str) -> Result<()> {
    log_debug!("Creating a new profile named \"{}\"", name);

    if let Ok(_) = get_profile(name) {
        return Err(Error::ProfileError(format!("Profile with name \"{}\" already exists", name)));
    }

    let (hash, _salt) = auth::hash_password(password)?;
    let profile = Profile {
        name: name.to_string(),
        key: cipher::generate_key(),
        password_hash: hash,
    };

    save_profile(profile)
        .map_err(|err| Error::IOError(format!("Unable to save profile data: {}", err)))
}

/// Deletes a profile with provided name
pub fn delete_profile(profile_name: &str) -> Result<()> {
    log_debug!("Deleting profile with name \"{}\"", profile_name);

    let data_dir = get_data_dir()?;
    let mut profiles_data = read_profiles_file(&data_dir)?;
    for (i, profile) in profiles_data.profiles.iter().enumerate() {
        if profile.name == profile_name {
            profiles_data.profiles.remove(i);
            profiles_data.current_profile = {
                if profiles_data.profiles.is_empty() {
                    None
                } else {
                    Some(profiles_data.profiles.get(0).unwrap().name.clone())
                }
            };
            write_profiles_file(profiles_data, &data_dir)?;
            return Ok(())
        }
    }
    Err(Error::ProfileError(format!("Profile with name \"{}\" doesn\'t exist", profile_name)))
}

/// Saves provided profile data to profiles file. Updates existing profile or creates a new one, if
/// it doesn't already exist
pub fn save_profile(updated_profile: Profile) -> Result<()> {
    log_debug!("Saving profile data: {:?}", &updated_profile);

    let data_dir = get_data_dir()?;
    let mut profiles_data = read_profiles_file(&data_dir)?;
    let profile_name = updated_profile.name.clone();

    if profiles_data.profiles.is_empty() {
        if profiles_data.profiles.is_empty() {
            profiles_data.profiles.push(updated_profile);
            profiles_data.current_profile = Some(profile_name);
        }
    } else {
        for (i, profile) in profiles_data.profiles.iter().enumerate() {
            if i == profiles_data.profiles.len() - 1 {
                profiles_data.profiles.push(updated_profile);
                break;
            }
            if profile.name == profile_name {
                profiles_data.profiles.insert(i, updated_profile);
                break;
            }
        }
    }

    write_profiles_file(profiles_data, &data_dir)?;
    Ok(())
}

/// Writes to profiles profile provided profiles data. Overwrites old data
fn write_profiles_file(profiles_data: ProfilesData, profiles_directory: &PathBuf) -> io::Result<()> {
    log_debug!("Writing data to profiles file: {:?}", profiles_data);

    let profiles_file = profiles_directory.join(PROFILES_FILE_NAME);

    let mut file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(profiles_file)?;

    serde_json::to_writer(&file, &profiles_data)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Unable to serialize profiles file data"))?;

    file.flush()?;
    Ok(())
}

/// Reads and returns profiles data from file
fn read_profiles_file(profiles_directory: &PathBuf) -> io::Result<ProfilesData> {
    log_debug!("Getting profiles file data");

    let profiles_file = profiles_directory.join(PROFILES_FILE_NAME);

    let profiles_data: ProfilesData = match File::open(&profiles_file) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let profiles_data = serde_json::from_reader(reader);

                if let Err(_) = profiles_data {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Unable to deserialize profiles file data"
                    ))
                } else {
                    profiles_data?
                }
            },
            Err(err) => {
                if err.kind() == io::ErrorKind::NotFound {
                    ProfilesData::default()
                } else {
                    return Err(err);
                }
            }
        };

    log_debug!("Got keys file data: {:?}", profiles_data);
    Ok(profiles_data)
}