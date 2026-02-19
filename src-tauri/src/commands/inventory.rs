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

    // Update inventory
    conn.execute(
        "UPDATE inventory SET quantity = quantity + ?1, updated_at = datetime('now') WHERE product_id = ?2",
        rusqlite::params![quantity, &product_id],
    ).map_err(|e| e.to_string())?;

    // If lot/expiry info provided, update or insert new inventory record
    if lot_number.is_some() || expiry_date.is_some() {
        conn.execute(
            "INSERT INTO inventory (id, product_id, quantity, lot_number, expiry_date)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(id) DO UPDATE SET quantity = quantity + excluded.quantity",
            rusqlite::params![
                Uuid::new_v4().to_string(), &product_id, quantity,
                &lot_number, &expiry_date
            ],
        ).map_err(|e| e.to_string())?;
    }

    // Record movement
    conn.execute(
        "INSERT INTO inventory_movements (id, product_id, movement_type, quantity, notes)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![
            Uuid::new_v4().to_string(), &product_id,
            &movement_type, quantity, &notes
        ],
    ).map_err(|e| e.to_string())?;

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
