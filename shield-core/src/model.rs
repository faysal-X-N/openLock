use serde::{Deserialize, Serialize, Serializer, Deserializer};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use secrecy::{SecretBox, ExposeSecret};

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub uuid: Uuid,
    pub name: String,
    pub url: Option<String>,
    pub username: Option<String>,
    #[serde(serialize_with = "serialize_secret", deserialize_with = "deserialize_secret")]
    pub password: SecretBox<String>,
    pub notes: Option<String>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Clone for Entry {
    fn clone(&self) -> Self {
        Self {
            uuid: self.uuid,
            name: self.name.clone(),
            url: self.url.clone(),
            username: self.username.clone(),
            password: SecretBox::new(Box::new(self.password.expose_secret().clone())),
            notes: self.notes.clone(),
            tags: self.tags.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

fn serialize_secret<S>(secret: &SecretBox<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(secret.expose_secret())
}

fn deserialize_secret<'de, D>(deserializer: D) -> Result<SecretBox<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(SecretBox::new(Box::new(s)))
}

impl Entry {
    pub fn new(name: String, username: Option<String>, password: String) -> Self {
        let now = Utc::now();
        Self {
            uuid: Uuid::new_v4(),
            name,
            url: None,
            username,
            password: SecretBox::new(Box::new(password)),
            notes: None,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_timestamp(&mut self) {
        self.updated_at = Utc::now();
    }
}
