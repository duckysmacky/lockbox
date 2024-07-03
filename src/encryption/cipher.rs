use chacha20poly1305::{
    aead::{Aead, KeyInit}, AeadCore, ChaCha20Poly1305, Error, Key, Nonce
};
use rand::rngs::OsRng;

pub fn generate_key() -> Key {
    ChaCha20Poly1305::generate_key(&mut OsRng)
}

pub fn generate_nonce() -> Nonce {
    ChaCha20Poly1305::generate_nonce(&mut OsRng)
}

pub fn encrypt(key: &[u8], nonce: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, Error> {
    let cipher = ChaCha20Poly1305::new(key.into());
    
    let ciphertext = cipher.encrypt(nonce.into(), plaintext)?;

    Ok(ciphertext)
}

pub fn decrypt(key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, Error> {
    let cipher = ChaCha20Poly1305::new(key.into());
    
    let plaintext = cipher.decrypt(nonce.into(), ciphertext)?;

    Ok(plaintext)
}