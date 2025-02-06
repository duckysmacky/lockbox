//! Contains handlers for the key subcommand

use clap::ArgMatches;
use crate::cli::prompts;
use crate::{exits_on, log_error, log_success, options};

pub fn handle_key_new(g_args: &ArgMatches, _args: &ArgMatches) {
    let password = match g_args.get_one::<String>("PASSWORD") {
        None => prompts::prompt_password("Please enter the password for the current profile:"),
        Some(password) => password.to_string()
    };

    match crate::new_key(&password) {
        Ok(_) => log_success!("Successfully generated new encryption key for the current profile"),
        Err(err) => {
            log_error!("Unable to generate a new encryption key");
            exits_on!(err; all);
        }
    }
}

pub fn handle_key_get(g_args: &ArgMatches, args: &ArgMatches) {
    let password = match g_args.get_one::<String>("PASSWORD") {
        None => prompts::prompt_password("Please enter the password for the current profile:"),
        Some(password) => password.to_string()
    };

    let options = options::KeyGetOptions {
        as_byte_array: args.get_flag("BYTE-FORMAT"),
    };

    match crate::get_key(&password, options) {
        Ok(key) => {
            // TODO: add current profile name
            log_success!("Encryption key for the current profile:\n    {}", key);
        }
        Err(err) => {
            log_error!("Unable to get an encryption key for the current profile");
            exits_on!(err; all);
        }
    }
}

pub fn handle_key_set(g_args: &ArgMatches, args: &ArgMatches) {
	let password = match g_args.get_one::<String>("PASSWORD") {
        None => prompts::prompt_password("Please enter the password for the current profile:"),
        Some(password) => password.to_string()
    };

	let new_key = args.get_one::<String>("KEY").expect("Key is required");

    match crate::set_key(&new_key, &password) {
        Ok(_) => log_success!("Successfully set a new encryption key for the current profile"),
        Err(err) => {
            log_error!("Unable to set an encryption key for the current profile");
            exits_on!(err; all);
        }
    }
}