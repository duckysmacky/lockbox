//! Contains function for user profile manipulation

use std::io::{self, BufReader, Write};
use std::fs::File;
use crate::core::encryption::cipher;
use crate::{Error, Result, log_debug};
use super::{auth, get_data_dir, Profile, ProfilesData};

/// Name of the profile data file
const PROFILES_FILE_PATH: &str = "profiles.json";

/// Sets the current profile to profile which name was supplied. Returns an error if given profile
/// doesn't exist
pub fn set_current_profile(profile_name: &str) -> Result<()> {
    log_debug!("Setting current profile to \"{}\"", profile_name);

    let profiles_data = get_profiles_file();
    if let Err(err) = profiles_data {
        if err.kind() == io::ErrorKind::NotFound {
            return Err(Error::ProfileError("No profile data found".to_string()))
        }
        return Err(Error::IOError(format!("Error getting profiles data: {}", err)))
    }
    let mut profiles_data = profiles_data.unwrap();
    profiles_data.current_profile = Some(profile_name.to_string());

    save_profiles_file(profiles_data)
        .map_err(|err| Error::from(err))
}

/// Returns currently selected profile data
pub fn get_current_profile() -> Result<Profile> {
    log_debug!("Getting current profile");

    let profiles_data = get_profiles_file();
    if let Err(err) = profiles_data {
        if err.kind() == io::ErrorKind::NotFound {
            return Err(Error::ProfileError("No profile data found".to_string()))
        }
        return Err(Error::IOError(format!("Error getting profiles data: {}", err)))
    }

    let current_profile = profiles_data.unwrap().current_profile;
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

/// Returns a list of currently avaliable profiles
pub fn get_profiles() -> Result<Vec<Profile>> {
    log_debug!("Getting all available profiles");

    let profiles_data = get_profiles_file();
    if let Err(err) = profiles_data {
        if err.kind() == io::ErrorKind::NotFound {
            return Err(Error::ProfileError("No profile data found".to_string()))
        }
        return Err(Error::IOError(format!("Error getting profiles data: {}", err)))
    }

    Ok(profiles_data.unwrap().profiles)
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

    let mut profiles_data = get_profiles_file()?;
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
            save_profiles_file(profiles_data)?;
            return Ok(())
        }
    }
    Err(Error::ProfileError(format!("Profile with name \"{}\" doesn\'t exist", profile_name)))
}

/// Saves provided profile data to profiles file. Updates existing profile or creates a new one, if
/// doesn't already exist
pub fn save_profile(updated_profile: Profile) -> io::Result<()> {
    log_debug!("Saving profile data: {:?}", &updated_profile);

    let mut profiles_data = get_profiles_file()?;
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

    save_profiles_file(profiles_data)
}

/// Writes to profiles profile provided profiles data. Overwrites old data
fn save_profiles_file(profiles_data: ProfilesData) -> io::Result<()> {
    log_debug!("Writing data to profiles file: {:?}", profiles_data);

    let mut path = get_data_dir();
    path.push(PROFILES_FILE_PATH);

    let mut file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    serde_json::to_writer(&file, &profiles_data)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Unable to serialize profiles file data"))?;

    file.flush()?;
    Ok(())
}

/// Reads and returns profiles data from file
fn get_profiles_file() -> io::Result<ProfilesData> {
    log_debug!("Getting profiles file data");

    let mut path = get_data_dir();
    path.push(PROFILES_FILE_PATH);

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound {
                save_profiles_file(ProfilesData {
                    current_profile: None,
                    profiles: vec![],
                })?;
                File::open(path).unwrap()
            } else {
                return Err(err);
            }
        }
    };
    let reader = BufReader::new(file);

    let profiles: ProfilesData = serde_json::from_reader(reader)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Unable to deserialize profiles file data"))?;

    log_debug!("Got keys file data: {:?}", profiles);
    Ok(profiles)
}