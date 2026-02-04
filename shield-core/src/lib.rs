pub mod model;
pub mod db;
pub mod crypto;
pub mod error;
pub mod vault;

pub use error::ShieldError;
pub type Result<T> = std::result::Result<T, ShieldError>;
pub use vault::Vault;
