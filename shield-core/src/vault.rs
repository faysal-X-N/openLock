use crate::db::Db;
use crate::crypto::{self, derive_key, encrypt, decrypt};
use crate::model::Entry;
use crate::{Result, ShieldError};
use aes_gcm::{Key, Aes256Gcm};
use std::path::Path;
use uuid::Uuid;
use secrecy::{ExposeSecret, SecretBox};

pub struct Vault {
    db: Db,
    key: Key<Aes256Gcm>,
}

impl Vault {
    pub fn open<P: AsRef<Path>>(path: P, password: &SecretBox<String>) -> Result<Self> {
        let db = Db::open(path)?;
        
        let salt = if let Some(s) = db.get_config("salt")? {
            s
        } else {
            // New DB initialization
            let s = crypto::generate_salt().to_vec();
            db.save_config("salt", &s)?;
            
            let key = derive_key(password.expose_secret().as_bytes(), &s)?;
            
            // Create verifier
            let verifier_str = b"SHIELD_VERIFIER";
            let (enc_verifier, nonce) = encrypt(verifier_str, &key)?;
            db.save_config("verifier", &enc_verifier)?;
            db.save_config("verifier_nonce", &nonce)?;
            
            return Ok(Self { db, key });
        };
        
        let key = derive_key(password.expose_secret().as_bytes(), &salt)?;
        
        // Verify
        if let Some(enc_verifier) = db.get_config("verifier")? {
            if let Some(nonce) = db.get_config("verifier_nonce")? {
                let decrypted = decrypt(&enc_verifier, &key, &nonce)?;
                if decrypted != b"SHIELD_VERIFIER" {
                    return Err(ShieldError::DecryptionError);
                }
            } else {
                 return Err(ShieldError::CryptoError);
            }
        }

        Ok(Self { db, key })
    }

    pub fn add_entry(&self, entry: &Entry) -> Result<()> {
        let json = serde_json::to_vec(entry)?;
        let (encrypted, nonce) = encrypt(&json, &self.key)?;
        self.db.save_entry(&entry.uuid, &encrypted, &nonce, &entry.updated_at.to_rfc3339())?;
        Ok(())
    }

    pub fn get_entry(&self, uuid: &Uuid) -> Result<Entry> {
        if let Some((data, nonce)) = self.db.get_entry(uuid)? {
            let json_bytes = decrypt(&data, &self.key, &nonce)?;
            let entry: Entry = serde_json::from_slice(&json_bytes)?;
            Ok(entry)
        } else {
            Err(ShieldError::EntryNotFound)
        }
    }
    
    pub fn list_entries(&self) -> Result<Vec<Entry>> {
        let rows = self.db.list_entries()?;
        let mut entries = Vec::new();
        for (_, data, nonce) in rows {
            if let Ok(json_bytes) = decrypt(&data, &self.key, &nonce) {
                if let Ok(entry) = serde_json::from_slice::<Entry>(&json_bytes) {
                    entries.push(entry);
                }
            }
        }
        Ok(entries)
    }

    pub fn delete_entry(&self, uuid: &Uuid) -> Result<()> {
        self.db.delete_entry(uuid)
    }

    pub fn update_entry(&self, entry: &Entry) -> Result<()> {
        let json = serde_json::to_vec(entry)?;
        let (encrypted, nonce) = encrypt(&json, &self.key)?;
        self.db.save_entry(&entry.uuid, &encrypted, &nonce, &entry.updated_at.to_rfc3339())?;
        Ok(())
    }
}
