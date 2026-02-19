use crate::db::Database;
use crate::db::models::*;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub fn get_customers(db: State<'_, Database>, search: Option<String>) -> Result<Vec<Customer>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut query = String::from("SELECT * FROM customers WHERE is_active = 1");
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(ref s) = search {
        query.push_str(" AND (name LIKE ?1 OR nit LIKE ?1 OR phone LIKE ?1)");
        params.push(Box::new(format!("%{}%", s)));
    }

    query.push_str(" ORDER BY name ASC");

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();

    let rows = stmt.query_map(param_refs.as_slice(), |row| {
        Ok(Customer {
            id: row.get(0)?,
            nit: row.get(1)?,
            name: row.get(2)?,
            email: row.get(3)?,
            phone: row.get(4)?,
            address: row.get(5)?,
            is_active: row.get::<_, i32>(6)? == 1,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
        })
    }).map_err(|e| e.to_string())?;

    Ok(rows.filter_map(|r| r.ok()).collect())
}

#[tauri::command]
pub fn create_customer(db: State<'_, Database>, customer: CreateCustomer) -> Result<Customer, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let id = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO customers (id, nit, name, email, phone, address) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params![&id, &customer.nit, &customer.name, &customer.email, &customer.phone, &customer.address],
    ).map_err(|e| e.to_string())?;

    Ok(Customer {
        id,
        nit: customer.nit,
        name: customer.name,
        email: customer.email,
        phone: customer.phone,
        address: customer.address,
        is_active: true,
        created_at: None,
        updated_at: None,
    })
}

#[tauri::command]
pub fn update_customer(db: State<'_, Database>, id: String, customer: CreateCustomer) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE customers SET nit = ?1, name = ?2, email = ?3, phone = ?4, address = ?5, updated_at = datetime('now') WHERE id = ?6",
        rusqlite::params![&customer.nit, &customer.name, &customer.email, &customer.phone, &customer.address, &id],
    ).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn delete_customer(db: State<'_, Database>, id: String) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("UPDATE customers SET is_active = 0, updated_at = datetime('now') WHERE id = ?1", [&id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
