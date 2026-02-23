use crate::db::Database;
use crate::db::models::*;
use rusqlite::OptionalExtension;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub fn get_products(db: State<'_, Database>, search: Option<String>, category_id: Option<String>, active_only: Option<bool>) -> Result<Vec<ProductWithStock>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let active_filter = active_only.unwrap_or(true);

    let mut query = String::from(
        "SELECT p.*, COALESCE(SUM(i.quantity), 0) as current_stock, c.name as category_name
         FROM products p
         LEFT JOIN inventory i ON i.product_id = p.id
         LEFT JOIN categories c ON c.id = p.category_id
         WHERE 1=1"
    );
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if active_filter {
        query.push_str(" AND p.is_active = 1");
    }

    if let Some(ref s) = search {
        query.push_str(" AND (p.name LIKE ?1 OR p.sku LIKE ?1 OR p.barcode LIKE ?1)");
        params.push(Box::new(format!("%{}%", s)));
    }

    if let Some(ref cid) = category_id {
        let param_idx = params.len() + 1;
        query.push_str(&format!(" AND p.category_id = ?{}", param_idx));
        params.push(Box::new(cid.clone()));
    }

    query.push_str(" GROUP BY p.id ORDER BY p.name ASC");

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();

    let rows = stmt.query_map(param_refs.as_slice(), |row| {
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

    let products: Vec<ProductWithStock> = rows.filter_map(|r| r.ok()).collect();
    Ok(products)
}

#[tauri::command]
pub fn get_product(db: State<'_, Database>, id: String) -> Result<ProductWithStock, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.query_row(
        "SELECT p.*, COALESCE(SUM(i.quantity), 0) as current_stock, c.name as category_name
         FROM products p
         LEFT JOIN inventory i ON i.product_id = p.id
         LEFT JOIN categories c ON c.id = p.category_id
         WHERE p.id = ?1
         GROUP BY p.id",
        [&id],
        |row| {
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
        },
    ).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_product(db: State<'_, Database>, product: CreateProduct) -> Result<Product, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let id = Uuid::new_v4().to_string();
    let tax_rate = product.tax_rate.unwrap_or(0.13);
    let unit = product.unit.unwrap_or_else(|| "unidad".to_string());
    let min_stock = product.min_stock.unwrap_or(0);

    // Validate barcode uniqueness
    if let Some(ref barcode) = product.barcode {
        if !barcode.trim().is_empty() {
            let existing: Option<String> = conn.query_row(
                "SELECT name FROM products WHERE barcode = ?1 AND is_active = 1",
                [barcode],
                |row| row.get(0),
            ).optional().map_err(|e| e.to_string())?;
            if let Some(name) = existing {
                return Err(format!("Ya existe un producto con ese código de barras: {}", name));
            }
        }
    }

    conn.execute(
        "INSERT INTO products (id, sku, barcode, name, description, category_id, purchase_price, sale_price, tax_rate, unit, min_stock, metadata)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        rusqlite::params![
            &id, &product.sku, &product.barcode, &product.name, &product.description,
            &product.category_id, product.purchase_price, product.sale_price,
            tax_rate, &unit, min_stock, &product.metadata
        ],
    ).map_err(|e| e.to_string())?;

    // Create initial inventory record
    conn.execute(
        "INSERT INTO inventory (id, product_id, quantity) VALUES (?1, ?2, 0)",
        rusqlite::params![Uuid::new_v4().to_string(), &id],
    ).map_err(|e| e.to_string())?;

    Ok(Product {
        id,
        sku: product.sku,
        barcode: product.barcode,
        name: product.name,
        description: product.description,
        category_id: product.category_id,
        purchase_price: product.purchase_price,
        sale_price: product.sale_price,
        tax_rate,
        unit,
        min_stock,
        is_active: true,
        metadata: product.metadata,
        created_at: None,
        updated_at: None,
    })
}

#[tauri::command]
pub fn update_product(db: State<'_, Database>, product: UpdateProduct) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Validate barcode uniqueness (exclude current product)
    if let Some(ref barcode) = product.barcode {
        if !barcode.trim().is_empty() {
            let existing: Option<String> = conn.query_row(
                "SELECT name FROM products WHERE barcode = ?1 AND is_active = 1 AND id != ?2",
                rusqlite::params![barcode, &product.id],
                |row| row.get(0),
            ).optional().map_err(|e| e.to_string())?;
            if let Some(name) = existing {
                return Err(format!("Ya existe un producto con ese código de barras: {}", name));
            }
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

    add_field!(product.sku, "sku");
    add_field!(product.barcode, "barcode");
    add_field!(product.name, "name");
    add_field!(product.description, "description");
    add_field!(product.category_id, "category_id");
    add_field!(product.purchase_price, "purchase_price");
    add_field!(product.sale_price, "sale_price");
    add_field!(product.tax_rate, "tax_rate");
    add_field!(product.unit, "unit");
    add_field!(product.min_stock, "min_stock");
    add_field!(product.metadata, "metadata");

    if let Some(active) = product.is_active {
        updates.push(format!("is_active = ?{}", idx));
        params.push(Box::new(active as i32));
        idx += 1;
    }

    if updates.is_empty() {
        return Ok(());
    }

    updates.push(format!("updated_at = datetime('now', '-4 hours')"));
    let query = format!(
        "UPDATE products SET {} WHERE id = ?{}",
        updates.join(", "),
        idx
    );
    params.push(Box::new(product.id));

    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    conn.execute(&query, param_refs.as_slice()).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_product_by_barcode(db: State<'_, Database>, barcode: String) -> Result<Option<ProductWithStock>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "SELECT p.*, COALESCE(SUM(i.quantity), 0) as current_stock, c.name as category_name
         FROM products p
         LEFT JOIN inventory i ON i.product_id = p.id
         LEFT JOIN categories c ON c.id = p.category_id
         WHERE p.barcode = ?1 AND p.is_active = 1
         GROUP BY p.id"
    ).map_err(|e| e.to_string())?;

    let result = stmt.query_row([&barcode], |row| {
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
    });

    match result {
        Ok(product) => Ok(Some(product)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn delete_product(db: State<'_, Database>, id: String) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE products SET is_active = 0, updated_at = datetime('now', '-4 hours') WHERE id = ?1",
        [&id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}
