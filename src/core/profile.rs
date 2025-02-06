//! Contains core logic for profile manipulation subcommands

use crate::core::data;
use crate::core::data::profile::Profile;
use crate::{log_info, new_err};

pub fn create(password: &str, profile_name: &str) -> crate::Result<()> {
    log_info!("Creating a new profile with name \"{}\"", profile_name);
    let mut profiles = data::get_profiles()?;
    profiles.new_profile(Profile::new(profile_name, password)?)?;
    Ok(())
}

pub fn delete(password: &str, profile_name: &str) -> crate::Result<()> {
    log_info!("Deleting profile \"{}\"", profile_name);
    let mut profiles = data::get_profiles()?;

    profiles.delete_profile(password, profile_name)?;
    Ok(())
}

pub fn select(password: &str, profile_name: &str) -> crate::Result<()> {
    log_info!("Switching profile to \"{}\"", profile_name);
    let mut profiles = data::get_profiles()?;

    if let Ok(profile) = profiles.get_current_profile() {
        if profile_name == profile.name {
            return Err(new_err!(ProfileError: AlreadySelected, profile_name))
        }
    }

    profiles.set_current(password, profile_name)?;
    Ok(())
}

pub fn get_current() -> crate::Result<String> {
    log_info!("Getting current profile");
    let mut profiles = data::get_profiles()?;
    let profile = profiles.get_current_profile()?;
    Ok(profile.name.to_string())
}

pub fn get_all() -> crate::Result<Vec<String>> {
    log_info!("Listing all available profiles");

    let profiles = data::get_profiles()?;
    let profile_list = profiles.get_profiles().into_iter()
        .map(|p| p.name.to_string())
        .collect::<Vec<String>>();
    Ok(profile_list)
}