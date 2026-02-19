pub mod models;
pub mod schema;

use anyhow::Result;
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new(app_data_dir: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(&app_data_dir)?;
        let db_path = app_data_dir.join("aynipos.db");
        let conn = Connection::open(&db_path)?;

        // Performance pragmas
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;
             PRAGMA cache_size = -8000;
             PRAGMA temp_store = MEMORY;"
        )?;

        // Run migrations
        schema::run_migrations(&conn)?;

        log::info!("Database initialized at {:?}", db_path);

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }
}
