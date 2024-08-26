use chacha20poly1305::{
    aead::{Aead, KeyInit},
    AeadCore, ChaCha20Poly1305
};
use rand::rngs::OsRng;
use crate::{Error, Key, Nonce, Result};

pub fn generate_key() -> Key {
    ChaCha20Poly1305::generate_key(&mut OsRng).into()
}

pub fn generate_nonce() -> Nonce {
    ChaCha20Poly1305::generate_nonce(&mut OsRng).into()
}

pub fn encrypt(key: &Key, nonce: &Nonce, data: &[u8]) -> Result<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new(key.into());

    let ciphertext = cipher.encrypt(nonce.into(), data)
        .map_err(|err| Error::CipherError(format!("Encryption error: {}", err)))?;
    Ok(ciphertext)
}

pub fn decrypt(key: &Key, nonce: &Nonce, data: &[u8]) -> Result<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new(key.into());

    let plaintext = cipher.decrypt(nonce.into(), data)
        .map_err(|err| Error::CipherError(format!("Decryption error: {}", err)))?;
    Ok(plaintext)
}