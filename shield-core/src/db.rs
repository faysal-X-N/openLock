use rusqlite::{Connection, params, OptionalExtension};
use crate::Result;
use std::path::Path;
use uuid::Uuid;

pub struct Db {
    conn: Connection,
}

impl Db {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        Self::init(&conn)?;
        Ok(Self { conn })
    }

    fn init(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS entries (
                uuid TEXT PRIMARY KEY,
                data BLOB NOT NULL,
                nonce BLOB NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS config (
                key TEXT PRIMARY KEY,
                value BLOB NOT NULL
            )",
            [],
        )?;
        
        Ok(())
    }

    pub fn save_config(&self, key: &str, value: &[u8]) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO config (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn get_config(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let mut stmt = self.conn.prepare("SELECT value FROM config WHERE key = ?1")?;
        let res = stmt.query_row(params![key], |row| row.get(0)).optional()?;
        Ok(res)
    }

    pub fn save_entry(&self, uuid: &Uuid, data: &[u8], nonce: &[u8], updated_at: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO entries (uuid, data, nonce, updated_at) VALUES (?1, ?2, ?3, ?4)",
            params![uuid.to_string(), data, nonce, updated_at],
        )?;
        Ok(())
    }

    pub fn get_entry(&self, uuid: &Uuid) -> Result<Option<(Vec<u8>, Vec<u8>)>> {
        let mut stmt = self.conn.prepare("SELECT data, nonce FROM entries WHERE uuid = ?1")?;
        let res = stmt.query_row(params![uuid.to_string()], |row| {
            Ok((row.get(0)?, row.get(1)?))
        }).optional()?;
        Ok(res)
    }

    pub fn list_entries(&self) -> Result<Vec<(String, Vec<u8>, Vec<u8>)>> {
        let mut stmt = self.conn.prepare("SELECT uuid, data, nonce FROM entries")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;

        let mut entries = Vec::new();
        for row in rows {
            entries.push(row?);
        }
        Ok(entries)
    }
    
    pub fn delete_entry(&self, uuid: &Uuid) -> Result<()> {
        self.conn.execute("DELETE FROM entries WHERE uuid = ?1", params![uuid.to_string()])?;
        Ok(())
    }
}
