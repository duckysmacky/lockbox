use std::io;
use argon2::{self, Config};
use rand::random;
use crate::{cli, log_error, log_fatal, log_success};
use super::keys;

pub fn hash_password(password: &str) -> (String, [u8; 16]) {
    let salt: [u8; 16] = random();
    let config = Config::default();
    let hashed_password = argon2::hash_encoded(password.as_bytes(), &salt, &config)
        .unwrap_or_else(|err|log_fatal!("Error hashing password: {}", err));

    (hashed_password, salt)
}

pub fn verify_password(input_password: &str) -> bool {
    let key_data = match keys::get_keys_file() {
        Ok(keys_file) => {
            keys_file.key_data.unwrap_or_else(|| {
                log_error!("Key doesn't exist");
                log_success!("Creating a new key");
                keys::create_new_key();
                keys::get_key()
            })
        },
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound {
                log_error!("Key doesn't exist");
                log_success!("Creating a new key");
                keys::create_new_key();
                keys::get_key()
            } else {
                log_fatal!("An error occurred while trying to get key file data: {}", err);
            }
        }
    };

    argon2::verify_encoded(&key_data.password_hash, input_password.as_bytes()).unwrap_or(false)
}

pub fn prompt_password() -> String {
    println!("Please enter your password: ");
    cli::util::prompt_input().unwrap_or_else(|err|log_fatal!("Error prompting password: {}", err))
}