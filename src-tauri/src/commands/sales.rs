use crate::db::Database;
use crate::db::models::*;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub fn create_sale(db: State<'_, Database>, sale: CreateSale) -> Result<Sale, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let sale_id = Uuid::new_v4().to_string();

    // Get next sale number
    let sale_number: i64 = conn.query_row(
        "SELECT CAST(value AS INTEGER) + 1 FROM settings WHERE key = 'sale_number_sequence'",
        [],
        |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    // Get active cash register
    let cash_register_id: Option<String> = conn.query_row(
        "SELECT id FROM cash_registers WHERE closed_at IS NULL ORDER BY opened_at DESC LIMIT 1",
        [],
        |row| row.get(0),
    ).ok();

    let mut subtotal = 0.0_f64;
    let mut tax_amount = 0.0_f64;
    let discount_amount = sale.discount_amount.unwrap_or(0.0);
    let mut sale_items_data: Vec<SaleItem> = Vec::new();

    // Calculate each line item
    for item in &sale.items {
        let product: Product = conn.query_row(
            "SELECT * FROM products WHERE id = ?1 AND is_active = 1",
            [&item.product_id],
            |row| {
                Ok(Product {
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
                })
            },
        ).map_err(|e| format!("Product {} not found: {}", item.product_id, e))?;

        let item_discount = item.discount.unwrap_or(0.0);
        let item_subtotal = (product.sale_price * item.quantity) - item_discount;
        let item_tax = item_subtotal * product.tax_rate;
        let item_total = item_subtotal;

        subtotal += item_subtotal;
        tax_amount += item_tax;

        sale_items_data.push(SaleItem {
            id: Uuid::new_v4().to_string(),
            sale_id: sale_id.clone(),
            product_id: item.product_id.clone(),
            product_name: product.name.clone(),
            quantity: item.quantity,
            unit_price: product.sale_price,
            discount: item_discount,
            tax_rate: product.tax_rate,
            subtotal: item_subtotal,
            total: item_total,
        });
    }

    let total = subtotal - discount_amount;

    // Insert sale
    conn.execute(
        "INSERT INTO sales (id, sale_number, customer_id, cash_register_id, subtotal, tax_amount, discount_amount, total, payment_method, payment_details, status, notes)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 'completed', ?11)",
        rusqlite::params![
            &sale_id, sale_number, &sale.customer_id, &cash_register_id,
            subtotal, tax_amount, discount_amount, total,
            &sale.payment_method, &sale.payment_details, &sale.notes
        ],
    ).map_err(|e| e.to_string())?;

    // Insert sale items and update inventory
    for si in &sale_items_data {
        conn.execute(
            "INSERT INTO sale_items (id, sale_id, product_id, product_name, quantity, unit_price, discount, tax_rate, subtotal, total)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![
                &si.id, &si.sale_id, &si.product_id, &si.product_name,
                si.quantity, si.unit_price, si.discount, si.tax_rate,
                si.subtotal, si.total
            ],
        ).map_err(|e| e.to_string())?;

        // Decrease stock
        conn.execute(
            "UPDATE inventory SET quantity = quantity - ?1, updated_at = datetime('now')
             WHERE product_id = ?2",
            rusqlite::params![si.quantity, &si.product_id],
        ).map_err(|e| e.to_string())?;

        // Record inventory movement
        conn.execute(
            "INSERT INTO inventory_movements (id, product_id, movement_type, quantity, reference_id, notes)
             VALUES (?1, ?2, 'sale', ?3, ?4, ?5)",
            rusqlite::params![
                Uuid::new_v4().to_string(), &si.product_id,
                -(si.quantity), &sale_id,
                format!("Venta #{}", sale_number)
            ],
        ).map_err(|e| e.to_string())?;
    }

    // Update sale number sequence
    conn.execute(
        "UPDATE settings SET value = ?1 WHERE key = 'sale_number_sequence'",
        [sale_number.to_string()],
    ).map_err(|e| e.to_string())?;

    // Look up customer name
    let customer_name: Option<String> = if let Some(ref cid) = sale.customer_id {
        conn.query_row(
            "SELECT name FROM customers WHERE id = ?1",
            [cid],
            |row| row.get(0),
        ).ok()
    } else {
        None
    };

    Ok(Sale {
        id: sale_id,
        sale_number,
        customer_id: sale.customer_id,
        customer_name,
        cash_register_id,
        subtotal,
        tax_amount,
        discount_amount,
        total,
        payment_method: sale.payment_method,
        payment_details: sale.payment_details,
        notes: sale.notes,
        status: "completed".to_string(),
        cufd: None,
        cuf: None,
        siat_status: Some("pending".to_string()),
        created_at: None,
    })
}

#[tauri::command]
pub fn get_sales(db: State<'_, Database>, date_from: Option<String>, date_to: Option<String>, status: Option<String>) -> Result<Vec<Sale>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut query = String::from(
        "SELECT s.*, c.name as customer_name FROM sales s LEFT JOIN customers c ON s.customer_id = c.id WHERE 1=1"
    );
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    let mut idx = 1;

    if let Some(ref from) = date_from {
        query.push_str(&format!(" AND s.created_at >= ?{}", idx));
        params.push(Box::new(from.clone()));
        idx += 1;
    }

    if let Some(ref to) = date_to {
        query.push_str(&format!(" AND s.created_at <= ?{}", idx));
        params.push(Box::new(to.clone()));
        idx += 1;
    }

    if let Some(ref s) = status {
        query.push_str(&format!(" AND s.status = ?{}", idx));
        params.push(Box::new(s.clone()));
        // idx += 1;
    }

    query.push_str(" ORDER BY s.created_at DESC");

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();

    let rows = stmt.query_map(param_refs.as_slice(), |row| {
        Ok(Sale {
            id: row.get(0)?,
            sale_number: row.get(1)?,
            customer_id: row.get(2)?,
            customer_name: row.get(17)?,
            cash_register_id: row.get(3)?,
            subtotal: row.get(4)?,
            tax_amount: row.get(5)?,
            discount_amount: row.get(6)?,
            total: row.get(7)?,
            payment_method: row.get(8)?,
            payment_details: row.get(9)?,
            notes: row.get(16)?,
            status: row.get(10)?,
            cufd: row.get(11)?,
            cuf: row.get(12)?,
            siat_status: row.get(13)?,
            created_at: row.get(15)?,
        })
    }).map_err(|e| e.to_string())?;

    Ok(rows.filter_map(|r| r.ok()).collect())
}

#[tauri::command]
pub fn get_sale_items(db: State<'_, Database>, sale_id: String) -> Result<Vec<SaleItem>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "SELECT * FROM sale_items WHERE sale_id = ?1 ORDER BY product_name"
    ).map_err(|e| e.to_string())?;

    let rows = stmt.query_map([&sale_id], |row| {
        Ok(SaleItem {
            id: row.get(0)?,
            sale_id: row.get(1)?,
            product_id: row.get(2)?,
            product_name: row.get(3)?,
            quantity: row.get(4)?,
            unit_price: row.get(5)?,
            discount: row.get(6)?,
            tax_rate: row.get(7)?,
            subtotal: row.get(8)?,
            total: row.get(9)?,
        })
    }).map_err(|e| e.to_string())?;

    Ok(rows.filter_map(|r| r.ok()).collect())
}

#[tauri::command]
pub fn cancel_sale(db: State<'_, Database>, sale_id: String) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Get sale items to restore inventory
    let mut stmt = conn.prepare(
        "SELECT product_id, quantity FROM sale_items WHERE sale_id = ?1"
    ).map_err(|e| e.to_string())?;

    let items: Vec<(String, f64)> = stmt.query_map([&sale_id], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
    }).map_err(|e| e.to_string())?
    .filter_map(|r| r.ok())
    .collect();

    // Restore inventory
    for (product_id, quantity) in &items {
        conn.execute(
            "UPDATE inventory SET quantity = quantity + ?1, updated_at = datetime('now') WHERE product_id = ?2",
            rusqlite::params![quantity, product_id],
        ).map_err(|e| e.to_string())?;

        conn.execute(
            "INSERT INTO inventory_movements (id, product_id, movement_type, quantity, reference_id, notes)
             VALUES (?1, ?2, 'return', ?3, ?4, 'Anulación de venta')",
            rusqlite::params![Uuid::new_v4().to_string(), product_id, quantity, &sale_id],
        ).map_err(|e| e.to_string())?;
    }

    // Mark sale as cancelled
    conn.execute(
        "UPDATE sales SET status = 'cancelled' WHERE id = ?1",
        [&sale_id],
    ).map_err(|e| e.to_string())?;

    Ok(())
}
