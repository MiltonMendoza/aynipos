use crate::db::Database;
use crate::db::models::*;
use rusqlite::OptionalExtension;
use tauri::State;
use uuid::Uuid;

// After migrations 007 (supplier_id) and 008 (dose), columns via ALTER TABLE are appended:
// p.*  → 0:id  1:sku  2:barcode  3:name  4:description  5:category_id
//         6:purchase_price  7:sale_price  8:tax_rate  9:unit  10:min_stock
//         11:is_active  12:metadata  13:created_at  14:updated_at
//         15:supplier_id (migration 007)  16:dose (migration 008)
// + computed: 17:current_stock  18:category_name  19:supplier_name

fn map_product_with_stock(row: &rusqlite::Row<'_>) -> rusqlite::Result<ProductWithStock> {
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
            supplier_id: row.get(15)?,
            dose: row.get(16)?,
        },
        current_stock: row.get(17)?,
        category_name: row.get(18)?,
        supplier_name: row.get(19)?,
        nearest_expiry_date: row.get(20)?,
        expiry_status: row.get(21)?,
    })
}

#[tauri::command]
pub fn get_products(db: State<'_, Database>, search: Option<String>, category_id: Option<String>, active_only: Option<bool>) -> Result<Vec<ProductWithStock>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let active_filter = active_only.unwrap_or(true);

    let mut query = String::from(
        "SELECT p.*,
                COALESCE(inv.total_stock, 0) as current_stock,
                c.name as category_name,
                s.name as supplier_name,
                inv.nearest_expiry,
                CASE
                    WHEN inv.nearest_expiry IS NULL THEN NULL
                    WHEN inv.nearest_expiry < DATE('now', '-4 hours') THEN 'expired'
                    WHEN inv.nearest_expiry <= DATE('now', '-4 hours', '+4 months') THEN 'expiring'
                    ELSE 'active'
                END as expiry_status
         FROM products p
         LEFT JOIN (
             SELECT product_id,
                    SUM(quantity) as total_stock,
                    MIN(CASE WHEN quantity > 0 AND expiry_date IS NOT NULL THEN expiry_date END) as nearest_expiry
             FROM inventory
             GROUP BY product_id
         ) inv ON inv.product_id = p.id
         LEFT JOIN categories c ON c.id = p.category_id
         LEFT JOIN suppliers s ON s.id = p.supplier_id
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

    query.push_str(" ORDER BY p.name ASC");

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();

    let rows = stmt.query_map(param_refs.as_slice(), map_product_with_stock).map_err(|e| e.to_string())?;

    let products: Vec<ProductWithStock> = rows.filter_map(|r| r.ok()).collect();
    Ok(products)
}

#[tauri::command]
pub fn get_product(db: State<'_, Database>, id: String) -> Result<ProductWithStock, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.query_row(
        "SELECT p.*,
                COALESCE(inv.total_stock, 0) as current_stock,
                c.name as category_name,
                s.name as supplier_name,
                inv.nearest_expiry,
                CASE
                    WHEN inv.nearest_expiry IS NULL THEN NULL
                    WHEN inv.nearest_expiry < DATE('now', '-4 hours') THEN 'expired'
                    WHEN inv.nearest_expiry <= DATE('now', '-4 hours', '+4 months') THEN 'expiring'
                    ELSE 'active'
                END as expiry_status
         FROM products p
         LEFT JOIN (
             SELECT product_id,
                    SUM(quantity) as total_stock,
                    MIN(CASE WHEN quantity > 0 AND expiry_date IS NOT NULL THEN expiry_date END) as nearest_expiry
             FROM inventory
             GROUP BY product_id
         ) inv ON inv.product_id = p.id
         LEFT JOIN categories c ON c.id = p.category_id
         LEFT JOIN suppliers s ON s.id = p.supplier_id
         WHERE p.id = ?1",
        [&id],
        map_product_with_stock,
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
        "INSERT INTO products (id, sku, barcode, name, description, category_id, purchase_price, sale_price, tax_rate, unit, min_stock, metadata, supplier_id, dose)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
        rusqlite::params![
            &id, &product.sku, &product.barcode, &product.name, &product.description,
            &product.category_id, product.purchase_price, product.sale_price,
            tax_rate, &unit, min_stock, &product.metadata, &product.supplier_id, &product.dose
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
        supplier_id: product.supplier_id,
        dose: product.dose,
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
    add_field!(product.supplier_id, "supplier_id");
    add_field!(product.dose, "dose");

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
        "SELECT p.*,
                COALESCE(inv.total_stock, 0) as current_stock,
                c.name as category_name,
                s.name as supplier_name,
                inv.nearest_expiry,
                CASE
                    WHEN inv.nearest_expiry IS NULL THEN NULL
                    WHEN inv.nearest_expiry < DATE('now', '-4 hours') THEN 'expired'
                    WHEN inv.nearest_expiry <= DATE('now', '-4 hours', '+4 months') THEN 'expiring'
                    ELSE 'active'
                END as expiry_status
         FROM products p
         LEFT JOIN (
             SELECT product_id,
                    SUM(quantity) as total_stock,
                    MIN(CASE WHEN quantity > 0 AND expiry_date IS NOT NULL THEN expiry_date END) as nearest_expiry
             FROM inventory
             GROUP BY product_id
         ) inv ON inv.product_id = p.id
         LEFT JOIN categories c ON c.id = p.category_id
         LEFT JOIN suppliers s ON s.id = p.supplier_id
         WHERE p.barcode = ?1 AND p.is_active = 1"
    ).map_err(|e| e.to_string())?;

    let result = stmt.query_row([&barcode], map_product_with_stock);

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
