use chacha20poly1305::{
    aead::{Aead, KeyInit},
    AeadCore, ChaCha20Poly1305
};
use rand::rngs::OsRng;
use crate::log_fatal;

pub type Key = [u8; 32];
pub type Nonce = [u8; 12];

pub fn generate_key() -> Key {
    ChaCha20Poly1305::generate_key(&mut OsRng).into()
}

pub fn generate_nonce() -> Nonce {
    ChaCha20Poly1305::generate_nonce(&mut OsRng).into()
}

pub fn encrypt(key: &Key, nonce: &Nonce, data: &[u8]) -> Vec<u8> {
    let cipher = ChaCha20Poly1305::new(key.into());

    let ciphertext = cipher.encrypt(nonce.into(), data);
    if let Err(err) = ciphertext {
        log_fatal!("Error encrypting data: {}", err);
    }

    ciphertext.unwrap()
}

pub fn decrypt(key: &Key, nonce: &Nonce, data: &[u8]) -> Vec<u8> {
    let cipher = ChaCha20Poly1305::new(key.into());

    let plaintext = cipher.decrypt(nonce.into(), data);
    if let Err(err) = plaintext {
        log_fatal!("Error decrypting data: {}", err);
    }

    plaintext.unwrap()
}