use argon2::{self, Config};
use rand::random;
use super::Profile;
use crate::{Result, Error};

pub fn hash_password(password: &str) -> Result<(String, [u8; 16])> {
    let salt: [u8; 16] = random();
    let config = Config::default();
    let hashed_password = argon2::hash_encoded(password.as_bytes(), &salt, &config)
        .map_err(|err| Error::CipherError(format!("Hashing error - {}", err)))?;
    Ok((hashed_password, salt))
}

pub fn verify_password(input_password: &str, profile: Profile) -> bool {
    argon2::verify_encoded(&profile.password_hash, input_password.as_bytes())
        .unwrap_or(false)
}