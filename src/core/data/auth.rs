//! Contains functions related to user authentication and password

use argon2::{self, Config};
use rand::random;
use crate::{Result, Error};
use crate::core::data::profiles::Profile;

/// Hashes the given password. Returns hashed password and salt
pub fn hash_password(password: &str) -> Result<(String, [u8; 16])> {
    let salt: [u8; 16] = random();
    let config = Config::default();
    let hashed_password = argon2::hash_encoded(password.as_bytes(), &salt, &config)
        .map_err(|err| Error::CipherError(format!("Hashing error - {}", err)))?;
    Ok((hashed_password, salt))
}

/// Verifies password for the given profile
pub fn verify_password(input_password: &str, profile: Profile) -> bool {
    argon2::verify_encoded(&profile.password_hash, input_password.as_bytes())
        .unwrap_or(false)
}