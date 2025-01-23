//! Contains methods related to encryption and decryption, key and nonce generation

use chacha20poly1305::{aead::{Aead, KeyInit}, AeadCore, ChaCha20Poly1305};
use rand::rngs::OsRng;
use crate::{new_err, Key, Nonce, Result};

/// Generates a new random 32-byte encryption key
pub fn generate_key() -> Key {
    ChaCha20Poly1305::generate_key(&mut OsRng).into()
}

/// Generates a new random 12-byte encryption nonce
pub fn generate_nonce() -> Nonce {
    ChaCha20Poly1305::generate_nonce(&mut OsRng).into()
}

/// Encrypts and returns encrypted bytes with ChaCha20Ply1305 algorithm using provided `Key` and
/// `Nonce`
pub fn encrypt(key: &Key, nonce: &Nonce, data: &[u8]) -> Result<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new(key.into());

    let ciphertext = cipher.encrypt(nonce.into(), data)
        .map_err(|err| new_err!(EncryptionError: CipherError, err))?;
    Ok(ciphertext)
}

/// Decrypts and returns decrypted bytes with ChaCha20Ply1305 algorithm using provided `Key` and
/// `Nonce`. Provided `Key` and `Nonce` should match the ones which were used to encrypt file for
/// successful decryption
pub fn decrypt(key: &Key, nonce: &Nonce, data: &[u8]) -> Result<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new(key.into());

    let plaintext = cipher.decrypt(nonce.into(), data)
        .map_err(|err| new_err!(EncryptionError: CipherError, err))?;
    Ok(plaintext)
}
