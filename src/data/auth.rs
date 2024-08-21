use argon2::{self, Config};
use rand::random;
use crate::log_fatal;
use super::Profile;

pub fn hash_password(password: &str) -> (String, [u8; 16]) {
    let salt: [u8; 16] = random();
    let config = Config::default();
    let hashed_password = argon2::hash_encoded(password.as_bytes(), &salt, &config)
        .unwrap_or_else(|err|log_fatal!("Error hashing password: {}", err));

    (hashed_password, salt)
}

pub fn verify_password(input_password: &str, profile: Profile) -> bool {
    argon2::verify_encoded(&profile.password_hash, input_password.as_bytes())
        .unwrap_or(false)
}