use aes_gcm::{
    aead::{Aead, KeyInit, AeadCore},
    Aes256Gcm, Key, Nonce
};
use argon2::{Argon2, Params, Algorithm, Version};
use rand::Rng;
use crate::{Result, ShieldError};

pub const SALT_LEN: usize = 32;
pub const KEY_LEN: usize = 32;
pub const NONCE_LEN: usize = 12;

pub fn generate_salt() -> [u8; SALT_LEN] {
    let mut salt = [0u8; SALT_LEN];
    rand::thread_rng().fill(&mut salt);
    salt
}

pub fn derive_key(password: &[u8], salt: &[u8]) -> Result<Key<Aes256Gcm>> {
    let mut output_key = [0u8; KEY_LEN];
    // Use default params or customize for stronger security
    // m_cost: memory size in KiB. Default is often 19456 (19 MiB).
    // t_cost: iterations. Default is 2.
    // p_cost: parallelism. Default is 1.
    let params = Params::default();
    
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    
    argon2.hash_password_into(password, salt, &mut output_key)
        .map_err(|_| ShieldError::CryptoError)?;
        
    Ok(*Key::<Aes256Gcm>::from_slice(&output_key))
}

pub fn encrypt(data: &[u8], key: &Key<Aes256Gcm>) -> Result<(Vec<u8>, Vec<u8>)> {
    let nonce = Aes256Gcm::generate_nonce(&mut rand::thread_rng()); // 96-bits
    let cipher = Aes256Gcm::new(key);
    let ciphertext = cipher.encrypt(&nonce, data)
        .map_err(|_| ShieldError::CryptoError)?;
    Ok((ciphertext, nonce.to_vec()))
}

pub fn decrypt(data: &[u8], key: &Key<Aes256Gcm>, nonce: &[u8]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce);
    let plaintext = cipher.decrypt(nonce, data)
        .map_err(|_| ShieldError::DecryptionError)?;
    Ok(plaintext)
}
