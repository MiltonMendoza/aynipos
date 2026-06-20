use crate::db::Database;
use crate::db::models::*;
use rusqlite::OptionalExtension;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub fn get_suppliers(db: State<'_, Database>, search: Option<String>) -> Result<Vec<Supplier>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut query = String::from(
        "SELECT id, name, contact_name, phone, email, address, notes, is_active, created_at, updated_at
         FROM suppliers WHERE is_active = 1"
    );

    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(ref s) = search {
        query.push_str(" AND (name LIKE ?1 OR contact_name LIKE ?1 OR phone LIKE ?1)");
        params.push(Box::new(format!("%{}%", s)));
    }

    query.push_str(" ORDER BY name ASC");

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();

    let rows = stmt.query_map(param_refs.as_slice(), |row| {
        Ok(Supplier {
            id: row.get(0)?,
            name: row.get(1)?,
            contact_name: row.get(2)?,
            phone: row.get(3)?,
            email: row.get(4)?,
            address: row.get(5)?,
            notes: row.get(6)?,
            is_active: row.get::<_, i32>(7)? == 1,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    }).map_err(|e| e.to_string())?;

    Ok(rows.filter_map(|r| r.ok()).collect())
}

#[tauri::command]
pub fn get_supplier(db: State<'_, Database>, id: String) -> Result<Supplier, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.query_row(
        "SELECT id, name, contact_name, phone, email, address, notes, is_active, created_at, updated_at
         FROM suppliers WHERE id = ?1",
        [&id],
        |row| {
            Ok(Supplier {
                id: row.get(0)?,
                name: row.get(1)?,
                contact_name: row.get(2)?,
                phone: row.get(3)?,
                email: row.get(4)?,
                address: row.get(5)?,
                notes: row.get(6)?,
                is_active: row.get::<_, i32>(7)? == 1,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        },
    ).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_supplier(db: State<'_, Database>, supplier: CreateSupplier) -> Result<Supplier, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let id = Uuid::new_v4().to_string();

    // Validate name uniqueness
    let existing: Option<String> = conn.query_row(
        "SELECT name FROM suppliers WHERE LOWER(name) = LOWER(?1) AND is_active = 1",
        [&supplier.name],
        |row| row.get(0),
    ).optional().map_err(|e| e.to_string())?;

    if existing.is_some() {
        return Err(format!("Ya existe un proveedor con el nombre: {}", supplier.name));
    }

    conn.execute(
        "INSERT INTO suppliers (id, name, contact_name, phone, email, address, notes)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        rusqlite::params![
            &id, &supplier.name, &supplier.contact_name,
            &supplier.phone, &supplier.email, &supplier.address, &supplier.notes
        ],
    ).map_err(|e| e.to_string())?;

    Ok(Supplier {
        id,
        name: supplier.name,
        contact_name: supplier.contact_name,
        phone: supplier.phone,
        email: supplier.email,
        address: supplier.address,
        notes: supplier.notes,
        is_active: true,
        created_at: None,
        updated_at: None,
    })
}

#[tauri::command]
pub fn update_supplier(db: State<'_, Database>, supplier: UpdateSupplier) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Validate name uniqueness (exclude self)
    if let Some(ref name) = supplier.name {
        let existing: Option<String> = conn.query_row(
            "SELECT name FROM suppliers WHERE LOWER(name) = LOWER(?1) AND is_active = 1 AND id != ?2",
            rusqlite::params![name, &supplier.id],
            |row| row.get(0),
        ).optional().map_err(|e| e.to_string())?;

        if existing.is_some() {
            return Err(format!("Ya existe un proveedor con el nombre: {}", name));
        }
    }

    let mut updates = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    let mut idx = 1;

    macro_rules! add_field {
        ($field:expr, $col:expr) => {
            if let Some(ref val) = $field {
                updates.push(format!("{} = ?{}", $col, idx));
                params.push(Box::new(val.clone()));
                idx += 1;
            }
        };
    }

    add_field!(supplier.name, "name");
    add_field!(supplier.contact_name, "contact_name");
    add_field!(supplier.phone, "phone");
    add_field!(supplier.email, "email");
    add_field!(supplier.address, "address");
    add_field!(supplier.notes, "notes");

    if let Some(active) = supplier.is_active {
        updates.push(format!("is_active = ?{}", idx));
        params.push(Box::new(active as i32));
        idx += 1;
    }

    if updates.is_empty() {
        return Ok(());
    }

    updates.push(format!("updated_at = datetime('now', '-4 hours')"));
    let query = format!(
        "UPDATE suppliers SET {} WHERE id = ?{}",
        updates.join(", "),
        idx
    );
    params.push(Box::new(supplier.id));

    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    conn.execute(&query, param_refs.as_slice()).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn delete_supplier(db: State<'_, Database>, id: String) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE suppliers SET is_active = 0, updated_at = datetime('now', '-4 hours') WHERE id = ?1",
        [&id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}
