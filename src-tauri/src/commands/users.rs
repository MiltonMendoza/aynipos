use crate::db::Database;
use crate::db::models::*;
use sha2::{Sha256, Digest};
use tauri::State;
use uuid::Uuid;

fn hash_pin(pin: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(pin.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[tauri::command]
pub fn get_users(db: State<'_, Database>) -> Result<Vec<User>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "SELECT id, name, role, is_active, created_at, updated_at FROM users WHERE is_active = 1 ORDER BY name"
    ).map_err(|e| e.to_string())?;

    let rows = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            role: row.get(2)?,
            is_active: row.get::<_, i32>(3)? == 1,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
        })
    }).map_err(|e| e.to_string())?;

    Ok(rows.filter_map(|r| r.ok()).collect())
}

#[tauri::command]
pub fn create_user(db: State<'_, Database>, user: CreateUser) -> Result<User, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Validate PIN: must be 4-6 digits
    if user.pin.len() < 4 || user.pin.len() > 6 || !user.pin.chars().all(|c| c.is_ascii_digit()) {
        return Err("El PIN debe ser numérico de 4 a 6 dígitos".to_string());
    }

    let pin_hash = hash_pin(&user.pin);

    // Check PIN uniqueness
    let existing: i32 = conn.query_row(
        "SELECT COUNT(*) FROM users WHERE pin_hash = ?1 AND is_active = 1",
        [&pin_hash],
        |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    if existing > 0 {
        return Err("Ya existe un usuario con ese PIN. Elija otro PIN.".to_string());
    }

    let id = Uuid::new_v4().to_string();
    let role = user.role.unwrap_or_else(|| "cashier".to_string());

    conn.execute(
        "INSERT INTO users (id, name, pin_hash, role) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![&id, &user.name, &pin_hash, &role],
    ).map_err(|e| e.to_string())?;

    Ok(User {
        id,
        name: user.name,
        role,
        is_active: true,
        created_at: None,
        updated_at: None,
    })
}

#[tauri::command]
pub fn update_user(db: State<'_, Database>, user: UpdateUser) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Build update query dynamically
    let mut updates = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(ref name) = user.name {
        updates.push("name = ?");
        params.push(Box::new(name.clone()));
    }

    if let Some(ref pin) = user.pin {
        // Validate PIN
        if pin.len() < 4 || pin.len() > 6 || !pin.chars().all(|c| c.is_ascii_digit()) {
            return Err("El PIN debe ser numérico de 4 a 6 dígitos".to_string());
        }

        let pin_hash = hash_pin(pin);

        // Check uniqueness (excluding current user)
        let existing: i32 = conn.query_row(
            "SELECT COUNT(*) FROM users WHERE pin_hash = ?1 AND is_active = 1 AND id != ?2",
            rusqlite::params![&pin_hash, &user.id],
            |row| row.get(0),
        ).map_err(|e| e.to_string())?;

        if existing > 0 {
            return Err("Ya existe un usuario con ese PIN. Elija otro PIN.".to_string());
        }

        updates.push("pin_hash = ?");
        params.push(Box::new(pin_hash));
    }

    if let Some(ref role) = user.role {
        updates.push("role = ?");
        params.push(Box::new(role.clone()));
    }

    if let Some(is_active) = user.is_active {
        updates.push("is_active = ?");
        params.push(Box::new(if is_active { 1i32 } else { 0i32 }));
    }

    if updates.is_empty() {
        return Ok(());
    }

    updates.push("updated_at = datetime('now', '-4 hours')");

    let sql = format!("UPDATE users SET {} WHERE id = ?", updates.join(", "));
    params.push(Box::new(user.id.clone()));

    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    conn.execute(&sql, param_refs.as_slice()).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn delete_user(db: State<'_, Database>, id: String) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Check if this is the last active admin
    let admin_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM users WHERE role = 'admin' AND is_active = 1 AND id != ?1",
        [&id],
        |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    let user_role: String = conn.query_row(
        "SELECT role FROM users WHERE id = ?1",
        [&id],
        |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    if user_role == "admin" && admin_count == 0 {
        return Err("No puede eliminar al último administrador del sistema".to_string());
    }

    conn.execute(
        "UPDATE users SET is_active = 0, updated_at = datetime('now', '-4 hours') WHERE id = ?1",
        [&id],
    ).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn login_with_pin(db: State<'_, Database>, pin: String) -> Result<User, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let pin_hash = hash_pin(&pin);

    let user = conn.query_row(
        "SELECT id, name, role, is_active, created_at, updated_at FROM users WHERE pin_hash = ?1 AND is_active = 1",
        [&pin_hash],
        |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                role: row.get(2)?,
                is_active: row.get::<_, i32>(3)? == 1,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        },
    ).map_err(|_| "PIN incorrecto".to_string())?;

    Ok(user)
}
