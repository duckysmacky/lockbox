//! Contains handlers for the profile subcommand

use clap::ArgMatches;
use crate::cli::prompts;
use crate::{exits_on, log_error, log_success, log_warn};

pub fn handle_profile_create(args: &ArgMatches) {
    let password = match args.get_one::<String>("PASSWORD") {
        None => prompts::prompt_password("Please enter a password for the new profile:"),
        Some(password) => password.to_string()
    };

    let name = args.get_one::<String>("NAME").expect("Profile name is required");

    match crate::create_profile(name, &password) {
        Ok(_) => log_success!("Successfully created new profile \"{}\"", name),
        Err(err) => {
            log_error!("Unable to create a new profile named \"{}\"", name);
            exits_on!(err; all);
        }
    }
}

pub fn handle_profile_delete(args: &ArgMatches) {
    let name = args.get_one::<String>("NAME").expect("Profile name is required");

    let password = match args.get_one::<String>("PASSWORD") {
        None => prompts::prompt_password(&format!("Please enter the password for {}", name)),
        Some(password) => password.to_string()
    };

    match crate::delete_profile(name, &password) {
        Ok(_) => log_success!("Successfully deleted profile \"{}\"", name),
        Err(err) => {
            log_error!("Unable to delete profile \"{}\"", name);
            exits_on!(err; all);
        }
    }
}

pub fn handle_profile_set(args: &ArgMatches) {
    let name = args.get_one::<String>("NAME").expect("Profile name is required");

    let password = match args.get_one::<String>("PASSWORD") {
        None => prompts::prompt_password(&format!("Please enter the password for {}", name)),
        Some(password) => password.to_string()
    };

    match crate::select_profile(name, &password) {
        Ok(_) => log_success!("Successfully set current profile to \"{}\"", name),
        Err(err) => {
            log_error!("Unable to switch to profile \"{}\"", name);
            exits_on!(err; all);
        }
    }
}

pub fn handle_profile_get(_args: &ArgMatches) {
    match crate::get_profile() {
        Ok(name) => log_success!("Currently selected profile: {}", name),
        Err(err) => {
            log_error!("Unable to get currently selected profile");
            exits_on!(err; all);
        }
    }
}

pub fn handle_profile_list(_args: &ArgMatches) {
    let profiles = crate::get_profiles();

    let profiles = profiles.unwrap_or_else(|err| {
        log_error!("Unable to get a list of all profiles");
        exits_on!(err; all);
    });
    let count = profiles.len();

    if count == 0 {
        log_warn!("No profiles found");
        log_warn!("New profile can be created with \"databoxer profile new\"");
    } else {
        if count > 1 {log_success!("There are {} profiles found:", count);}
        else {log_success!("There is {} profile found:", count);}
        for name in profiles {
            println!("\t- {}", name)
        }
    }
}