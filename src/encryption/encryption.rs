use chacha20poly1305::{
    aead::{
        Aead,
        KeyInit
    },
    AeadCore,
    ChaCha20Poly1305,
    Error, Nonce
};
use rand::{
    rngs::OsRng,
    Rng
};

pub fn generate_key() -> [u8; 32]  {
    rand::thread_rng().gen::<[u8; 32]>()
}

pub fn generate_nonce() -> Nonce  {
    ChaCha20Poly1305::generate_nonce(&mut OsRng)
}

pub fn encrypt(key: &[u8], nonce: &Nonce, plaintext: &[u8]) -> Result<Vec<u8>, Error> {
    let cipher = ChaCha20Poly1305::new(key.into());
    
    let ciphertext = cipher.encrypt(nonce, plaintext.as_ref())?;

    Ok(ciphertext)
}

pub fn decrypt(key: &[u8], nonce: &Nonce, ciphertext: &[u8]) -> Result<Vec<u8>, Error> {
    let cipher = ChaCha20Poly1305::new(key.into());
    
    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())?;

    Ok(plaintext)
}