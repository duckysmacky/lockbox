use chacha20poly1305::{
    aead::{Aead, KeyInit}, AeadCore, ChaCha20Poly1305, Error
};
use rand::rngs::OsRng;

pub type Key = [u8; 32];
pub type Nonce = [u8; 12];

pub fn generate_key() -> Key {
    ChaCha20Poly1305::generate_key(&mut OsRng).into()
}

pub fn generate_nonce() -> Nonce {
    ChaCha20Poly1305::generate_nonce(&mut OsRng).into()
}

pub fn encrypt(key: &Key, nonce: &Nonce, plaintext: &[u8]) -> Result<Vec<u8>, Error> {
    let cipher = ChaCha20Poly1305::new(key.into());

    let ciphertext = cipher.encrypt(nonce.into(), plaintext)?;

    Ok(ciphertext)
}

pub fn decrypt(key: &Key, nonce: &Nonce, ciphertext: &[u8]) -> Result<Vec<u8>, Error> {
    let cipher = ChaCha20Poly1305::new(key.into());

    let plaintext = cipher.decrypt(nonce.into(), ciphertext)?;

    Ok(plaintext)
}