use std::fs::File;
use std::io;
use std::io::{BufReader, Write};
use crate::data::{get_data_dir, Profile, ProfilesData};
use crate::{Error, log_debug, Result};
use crate::data::auth;
use crate::encryption::cipher;

const PROFILES_FILE_PATH: &str = "profiles.json";

pub fn get_current_profile() -> Result<Profile> {
    log_debug!("Getting current profile");

    let profiles = get_profiles_file();
    if let Err(err) = profiles {
        if err.kind() == io::ErrorKind::NotFound {
            return Err(Error::ProfileError("No profile data found".to_string()))
        }
        return Err(Error::IOError(format!("Error getting profiles data: {}", err)))
    }

    let current_profile = profiles.unwrap().current_profile;
    if current_profile.is_none() {
        return Err(Error::ProfileError("Current profile doesn\'t exist".to_string()))
    }

    let profile = get_profile(&current_profile.unwrap())?;
    Ok(profile)
}

pub fn get_profile(profile_name: &str) -> Result<Profile> {
    log_debug!("Getting profile with name \"{}\"", profile_name);

    let profiles = get_profiles_file();
    if let Err(err) = profiles {
        if err.kind() == io::ErrorKind::NotFound {
            return Err(Error::ProfileError("No profile data found".to_string()))
        }
        return Err(Error::IOError(format!("Error getting profiles data: {}", err)))
    }

    for profile in profiles.unwrap().profiles {
        if profile.name == profile_name {
            return Ok(profile)
        }
    }
    Err(Error::ProfileError(format!("Profile with name \"{}\" doesn\'t exist", profile_name)))
}

pub fn create_new_profile(name: &str, password: &str) -> Result<()> {
    log_debug!("Creating a new profile named \"{}\"", name);

    if let Ok(_) = get_profile(name) {
        return Err(Error::ProfileError(format!("Profile with name \"{}\" already exists", name)));
    }

    let (hash, _salt) = auth::hash_password(password);
    let profile = Profile {
        name: name.to_string(),
        key: cipher::generate_key(),
        password_hash: hash,
    };

    save_profile(profile)
        .map_err(|err| Error::IOError(format!("Unable to save profile data: {}", err)))
}

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
                    Some(profiles_data.profiles.get(profiles_data.profiles.len() - 1).unwrap().name.clone())
                }
            };
            save_profiles_file(profiles_data)?;
            return Ok(())
        }
    }
    Err(Error::ProfileError(format!("Profile with name \"{}\" doesn\'t exist", profile_name)))
}

pub fn save_profile(profile: Profile) -> io::Result<()> {
    log_debug!("Saving profile data: {:?}", profile);

    let mut profiles_data = get_profiles_file()?;
    profiles_data.current_profile = Some(profile.name.clone());
    profiles_data.profiles.push(profile);

    save_profiles_file(profiles_data)
}

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