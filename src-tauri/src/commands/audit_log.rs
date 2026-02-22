use crate::db::Database;
use crate::db::models::*;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub fn log_action(
    db: State<'_, Database>,
    user_id: String,
    user_name: String,
    action: String,
    entity_type: Option<String>,
    entity_id: Option<String>,
    details: Option<String>,
) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let id = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO audit_log (id, user_id, user_name, action, entity_type, entity_id, details) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        rusqlite::params![&id, &user_id, &user_name, &action, &entity_type, &entity_id, &details],
    ).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_audit_log(
    db: State<'_, Database>,
    user_id: Option<String>,
    action: Option<String>,
    date_from: Option<String>,
    date_to: Option<String>,
    limit: Option<i32>,
) -> Result<Vec<AuditLogEntry>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut conditions = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(ref uid) = user_id {
        conditions.push("user_id = ?");
        params.push(Box::new(uid.clone()));
    }

    if let Some(ref act) = action {
        conditions.push("action = ?");
        params.push(Box::new(act.clone()));
    }

    if let Some(ref df) = date_from {
        conditions.push("created_at >= ?");
        params.push(Box::new(df.clone()));
    }

    if let Some(ref dt) = date_to {
        conditions.push("created_at <= ?");
        params.push(Box::new(format!("{} 23:59:59", dt)));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    let limit_val = limit.unwrap_or(100);

    let sql = format!(
        "SELECT id, user_id, user_name, action, entity_type, entity_id, details, created_at FROM audit_log {} ORDER BY created_at DESC LIMIT ?",
        where_clause
    );

    params.push(Box::new(limit_val));

    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;

    let rows = stmt.query_map(param_refs.as_slice(), |row| {
        Ok(AuditLogEntry {
            id: row.get(0)?,
            user_id: row.get(1)?,
            user_name: row.get(2)?,
            action: row.get(3)?,
            entity_type: row.get(4)?,
            entity_id: row.get(5)?,
            details: row.get(6)?,
            created_at: row.get(7)?,
        })
    }).map_err(|e| e.to_string())?;

    Ok(rows.filter_map(|r| r.ok()).collect())
}
