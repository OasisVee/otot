use anyhow::{Context, Result};
use rusqlite::{Connection, params};
use std::path::PathBuf;
use std::time::SystemTime;
use url::Url;

pub trait Database {
    fn add_visit(&mut self, url: &str, timestamp: SystemTime) -> Result<()>;
    fn fuzzy_match(&self, pattern: &[String]) -> Result<Vec<String>>;
    fn get_best_match(&self, pattern: &[String]) -> Result<Option<String>>;
}

pub struct SqliteDatabase {
    conn: Connection,
}

impl SqliteDatabase {
    pub fn open() -> Result<Self> {
        let db_path = Self::get_db_path()?;
        Self::open_at(&db_path)
    }

    pub fn open_at(path: &std::path::Path) -> Result<Self> {
        let conn = Connection::open(path).context("Failed to open database")?;

        let db = Self { conn };
        db.initialize_schema()?;
        Ok(db)
    }

    fn initialize_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS urls (
                id INTEGER PRIMARY KEY,
                full_url TEXT NOT NULL UNIQUE,
                segments TEXT NOT NULL,
                last_segment TEXT NOT NULL,
                score REAL NOT NULL DEFAULT 1.0,
                last_accessed INTEGER NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_urls_last_segment
                ON urls(last_segment COLLATE NOCASE);",
        )?;
        Ok(())
    }

    fn get_db_path() -> Result<PathBuf> {
        let data_dir = dirs::data_local_dir().context("Could not find local data directory")?;
        let app_dir = data_dir.join("zurl");
        std::fs::create_dir_all(&app_dir).context("Failed to create application directory")?;

        Ok(app_dir.join("history.db"))
    }
}
