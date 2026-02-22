use crate::db::Database;
use crate::db::models::*;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub fn get_inventory(db: State<'_, Database>, low_stock_only: Option<bool>, expiring_days: Option<i32>) -> Result<Vec<ProductWithStock>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut query = String::from(
        "SELECT p.*, COALESCE(SUM(i.quantity), 0) as current_stock, c.name as category_name
         FROM products p
         LEFT JOIN inventory i ON i.product_id = p.id
         LEFT JOIN categories c ON c.id = p.category_id
         WHERE p.is_active = 1
         GROUP BY p.id"
    );

    if low_stock_only.unwrap_or(false) {
        query.push_str(" HAVING current_stock <= p.min_stock");
    }

    query.push_str(" ORDER BY p.name ASC");

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| {
        Ok(ProductWithStock {
            product: Product {
                id: row.get(0)?,
                sku: row.get(1)?,
                barcode: row.get(2)?,
                name: row.get(3)?,
                description: row.get(4)?,
                category_id: row.get(5)?,
                purchase_price: row.get(6)?,
                sale_price: row.get(7)?,
                tax_rate: row.get(8)?,
                unit: row.get(9)?,
                min_stock: row.get(10)?,
                is_active: row.get::<_, i32>(11)? == 1,
                metadata: row.get(12)?,
                created_at: row.get(13)?,
                updated_at: row.get(14)?,
            },
            current_stock: row.get(15)?,
            category_name: row.get(16)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut products: Vec<ProductWithStock> = rows.filter_map(|r| r.ok()).collect();

    // Filter expiring (if requested)
    if let Some(days) = expiring_days {
        let cutoff = chrono::Utc::now() + chrono::Duration::days(days as i64);
        let cutoff_str = cutoff.format("%Y-%m-%d").to_string();

        // Get products with expiring inventory
        let mut exp_stmt = conn.prepare(
            "SELECT DISTINCT product_id FROM inventory WHERE expiry_date IS NOT NULL AND expiry_date <= ?1"
        ).map_err(|e| e.to_string())?;

        let expiring_ids: Vec<String> = exp_stmt.query_map([&cutoff_str], |row| {
            row.get::<_, String>(0)
        }).map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

        products.retain(|p| expiring_ids.contains(&p.product.id));
    }

    Ok(products)
}

#[tauri::command]
pub fn get_product_lots(db: State<'_, Database>, product_id: String) -> Result<Vec<InventoryLot>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();

    let mut stmt = conn.prepare(
        "SELECT id, product_id, quantity, lot_number, expiry_date, cost_price, updated_at
         FROM inventory
         WHERE product_id = ?1
         ORDER BY expiry_date ASC NULLS LAST"
    ).map_err(|e| e.to_string())?;

    let rows = stmt.query_map([&product_id], |row| {
        let expiry_date: Option<String> = row.get(4)?;
        let expiry_status = match &expiry_date {
            None => "ok".to_string(),
            Some(date) => {
                if date <= &today {
                    "expired".to_string()
                } else if let Ok(exp) = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d") {
                    if let Ok(now) = chrono::NaiveDate::parse_from_str(&today, "%Y-%m-%d") {
                        let days_until = (exp - now).num_days();
                        if days_until <= 7 {
                            "danger".to_string()
                        } else if days_until <= 30 {
                            "warning".to_string()
                        } else {
                            "ok".to_string()
                        }
                    } else {
                        "ok".to_string()
                    }
                } else {
                    "ok".to_string()
                }
            }
        };

        Ok(InventoryLot {
            id: row.get(0)?,
            product_id: row.get(1)?,
            quantity: row.get(2)?,
            lot_number: row.get(3)?,
            expiry_date,
            cost_price: row.get(5)?,
            expiry_status,
            updated_at: row.get(6)?,
        })
    }).map_err(|e| e.to_string())?;

    Ok(rows.filter_map(|r| r.ok()).collect())
}

#[tauri::command]
pub fn adjust_inventory(
    db: State<'_, Database>,
    product_id: String,
    quantity: f64,
    movement_type: String,
    notes: Option<String>,
    lot_number: Option<String>,
    expiry_date: Option<String>,
) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Find or create the appropriate inventory record
    if let Some(ref lot) = lot_number {
        // With lot: find existing record with same product_id + lot_number
        let existing: Option<String> = conn.query_row(
            "SELECT id FROM inventory WHERE product_id = ?1 AND lot_number = ?2",
            rusqlite::params![&product_id, lot],
            |row| row.get(0),
        ).ok();

        if let Some(inv_id) = existing {
            conn.execute(
                "UPDATE inventory SET quantity = quantity + ?1, expiry_date = COALESCE(?2, expiry_date), updated_at = datetime('now') WHERE id = ?3",
                rusqlite::params![quantity, &expiry_date, &inv_id],
            ).map_err(|e| e.to_string())?;
        } else {
            conn.execute(
                "INSERT INTO inventory (id, product_id, quantity, lot_number, expiry_date) VALUES (?1, ?2, ?3, ?4, ?5)",
                rusqlite::params![
                    Uuid::new_v4().to_string(), &product_id, quantity, lot, &expiry_date
                ],
            ).map_err(|e| e.to_string())?;
        }
    } else {
        // Without lot: find existing record without lot_number
        let existing: Option<String> = conn.query_row(
            "SELECT id FROM inventory WHERE product_id = ?1 AND (lot_number IS NULL OR lot_number = '')",
            rusqlite::params![&product_id],
            |row| row.get(0),
        ).ok();

        if let Some(inv_id) = existing {
            conn.execute(
                "UPDATE inventory SET quantity = quantity + ?1, updated_at = datetime('now') WHERE id = ?2",
                rusqlite::params![quantity, &inv_id],
            ).map_err(|e| e.to_string())?;
        } else {
            conn.execute(
                "INSERT INTO inventory (id, product_id, quantity) VALUES (?1, ?2, ?3)",
                rusqlite::params![
                    Uuid::new_v4().to_string(), &product_id, quantity
                ],
            ).map_err(|e| e.to_string())?;
        }
    }

    // Record movement with lot info
    conn.execute(
        "INSERT INTO inventory_movements (id, product_id, movement_type, quantity, notes, lot_number, expiry_date)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        rusqlite::params![
            Uuid::new_v4().to_string(), &product_id,
            &movement_type, quantity, &notes, &lot_number, &expiry_date
        ],
    ).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn delete_lot(db: State<'_, Database>, lot_id: String) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Check quantity is 0
    let qty: f64 = conn.query_row(
        "SELECT quantity FROM inventory WHERE id = ?1",
        [&lot_id],
        |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    if qty != 0.0 {
        return Err("Solo se pueden eliminar lotes con cantidad 0".to_string());
    }

    conn.execute("DELETE FROM inventory WHERE id = ?1", [&lot_id])
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_inventory_movements(db: State<'_, Database>, product_id: Option<String>, limit: Option<i32>) -> Result<Vec<InventoryMovement>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let max = limit.unwrap_or(50);

    let mut query = String::from("SELECT * FROM inventory_movements WHERE 1=1");
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(ref pid) = product_id {
        query.push_str(" AND product_id = ?1");
        params.push(Box::new(pid.clone()));
    }

    query.push_str(&format!(" ORDER BY created_at DESC LIMIT {}", max));

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();

    let rows = stmt.query_map(param_refs.as_slice(), |row| {
        Ok(InventoryMovement {
            id: row.get(0)?,
            product_id: row.get(1)?,
            movement_type: row.get(2)?,
            quantity: row.get(3)?,
            reference_id: row.get(4)?,
            notes: row.get(5)?,
            created_at: row.get(6)?,
        })
    }).map_err(|e| e.to_string())?;

    Ok(rows.filter_map(|r| r.ok()).collect())
}
