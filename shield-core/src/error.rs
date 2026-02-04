use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShieldError {
    #[error("Database error: {0}")]
    DbError(#[from] rusqlite::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Encryption error")]
    CryptoError,
    
    #[error("Decryption failed")]
    DecryptionError,

    #[error("Serialization error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    
    #[error("Entry not found")]
    EntryNotFound,
}
