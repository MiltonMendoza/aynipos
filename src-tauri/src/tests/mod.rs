// ─── Test Helper ──────────────────────────────────────────────────────────────
// Crea una base de datos SQLite en memoria con el schema completo.
// Se usa en todos los tests del proyecto para evitar dependencias de Tauri.

pub mod products;
pub mod sales;
pub mod inventory;
pub mod customers;
pub mod cash_register;

use crate::db::{Database, schema};
use rusqlite::Connection;
use std::sync::Mutex;

/// Crea una `Database` completa en memoria con todas las migraciones aplicadas.
/// Cada test que llame a esta función obtiene una DB limpia y aislada.
pub fn create_test_db() -> Database {
    let conn = Connection::open_in_memory().expect("Failed to open in-memory DB");

    // Aplicar los mismos PRAGMAs que en producción
    conn.execute_batch(
        "PRAGMA journal_mode = WAL;
         PRAGMA synchronous = NORMAL;
         PRAGMA foreign_keys = ON;",
    ).expect("Failed to set pragmas");

    // Correr todas las migraciones (usa los mismos SQL que producción)
    schema::run_migrations(&conn).expect("Failed to run migrations");

    // Seed mínimo: la secuencia de ventas necesaria para create_sale
    conn.execute(
        "INSERT OR IGNORE INTO settings (key, value) VALUES ('sale_number_sequence', '0')",
        [],
    ).expect("Failed to seed sale_number_sequence");

    Database { conn: Mutex::new(conn) }
}
