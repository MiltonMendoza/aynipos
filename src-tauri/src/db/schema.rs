use anyhow::Result;
use rusqlite::Connection;

const MIGRATION_V1: &str = include_str!("../../migrations/001_initial.sql");
const MIGRATION_V2: &str = include_str!("../../migrations/002_sale_notes.sql");
const MIGRATION_V3: &str = include_str!("../../migrations/003_lot_movements.sql");

pub fn run_migrations(conn: &Connection) -> Result<()> {
    // Create migrations tracking table
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS _migrations (
            version INTEGER PRIMARY KEY,
            applied_at TEXT DEFAULT (datetime('now'))
        );"
    )?;

    // Check current version
    let current_version: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM _migrations",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    // Apply migrations
    if current_version < 1 {
        log::info!("Applying migration v1: initial schema");
        conn.execute_batch(MIGRATION_V1)?;
        conn.execute("INSERT INTO _migrations (version) VALUES (1)", [])?;
    }

    if current_version < 2 {
        log::info!("Applying migration v2: sale notes");
        conn.execute_batch(MIGRATION_V2)?;
        conn.execute("INSERT INTO _migrations (version) VALUES (2)", [])?;
    }

    if current_version < 3 {
        log::info!("Applying migration v3: lot tracking in movements");
        conn.execute_batch(MIGRATION_V3)?;
        conn.execute("INSERT INTO _migrations (version) VALUES (3)", [])?;
    }

    log::info!("Database at version {}", std::cmp::max(current_version, 3));
    Ok(())
}

