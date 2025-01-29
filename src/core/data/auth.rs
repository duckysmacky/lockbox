//! Contains functions related to user authentication and password

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use crate::{Result, new_err, Key};

/// Hashes the given password. Returns hashed password and key generated based on the password hash
/// used to encrypt the stored encryption key
pub fn hash_password(password: &str) -> Result<(String, Key)> {
    let salt = SaltString::generate(&mut OsRng);
    let mut password_key = Key::default();

    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)
        .map_err(|err| new_err!(EncryptionError: HashError, err))?.to_string();
    argon2.hash_password_into(password.as_bytes(), salt.as_str().as_bytes(), &mut password_key)
        .map_err(|err| new_err!(EncryptionError: HashError, err))?;
    Ok((password_hash, password_key))
}

/// Verifies password by comparing it to the password hash
pub fn verify_password(password_hash: &str, password: &str) -> Result<bool> {
    let hash = PasswordHash::new(password_hash)
        .map_err(|_| new_err!(InvalidData: BadHash, "Bad password hash"))?;

    let argon2 = Argon2::default();
    Ok(argon2.verify_password(password.as_bytes(), &hash).is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_password_hash() -> Result<()> {
        let password = "my_password123";
        let (hash, _) = hash_password(password)?;
        assert!(verify_password(&hash, password)?);
        Ok(())
    }
}