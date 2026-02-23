use crate::db::Database;
use crate::db::models::*;
use rusqlite::OptionalExtension;
use tauri::State;
use uuid::Uuid;
use std::fs::File;
use std::path::Path;

/// Export all active products with stock to a CSV file.
#[tauri::command]
pub fn export_products_csv(db: State<'_, Database>, file_path: String) -> Result<u32, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "SELECT p.sku, p.barcode, p.name, p.description, c.name as category_name,
                p.purchase_price, p.sale_price, p.tax_rate, p.unit, p.min_stock,
                COALESCE((SELECT SUM(i.quantity) FROM inventory i WHERE i.product_id = p.id), 0) as stock
         FROM products p
         LEFT JOIN categories c ON c.id = p.category_id
         WHERE p.is_active = 1
         ORDER BY p.name"
    ).map_err(|e| e.to_string())?;

    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,        // sku
            row.get::<_, Option<String>>(1)?, // barcode
            row.get::<_, String>(2)?,         // name
            row.get::<_, Option<String>>(3)?, // description
            row.get::<_, Option<String>>(4)?, // category_name
            row.get::<_, f64>(5)?,            // purchase_price
            row.get::<_, f64>(6)?,            // sale_price
            row.get::<_, f64>(7)?,            // tax_rate
            row.get::<_, String>(8)?,         // unit
            row.get::<_, i32>(9)?,            // min_stock
            row.get::<_, f64>(10)?,           // stock
        ))
    }).map_err(|e| e.to_string())?;

    let file = File::create(&file_path)
        .map_err(|e| format!("Error al crear archivo: {}", e))?;
    let mut wtr = csv::Writer::from_writer(file);

    // Write header
    wtr.write_record(&[
        "sku", "codigo_barras", "nombre", "descripcion", "categoria",
        "precio_compra", "precio_venta", "tasa_impuesto", "unidad",
        "stock_minimo", "stock_actual"
    ]).map_err(|e| format!("Error al escribir encabezados: {}", e))?;

    let mut count: u32 = 0;
    for row in rows {
        let r = row.map_err(|e| e.to_string())?;
        wtr.write_record(&[
            r.0,                                    // sku
            r.1.unwrap_or_default(),                // barcode
            r.2,                                    // name
            r.3.unwrap_or_default(),                // description
            r.4.unwrap_or_default(),                // category
            format!("{:.2}", r.5),                  // purchase_price
            format!("{:.2}", r.6),                  // sale_price
            format!("{:.2}", r.7),                  // tax_rate
            r.8,                                    // unit
            r.9.to_string(),                        // min_stock
            format!("{:.0}", r.10),                 // stock
        ]).map_err(|e| format!("Error al escribir fila: {}", e))?;
        count += 1;
    }

    wtr.flush().map_err(|e| format!("Error al finalizar archivo: {}", e))?;
    Ok(count)
}

/// Import products from a CSV file. Upserts by SKU.
#[tauri::command]
pub fn import_products_csv(db: State<'_, Database>, file_path: String) -> Result<ImportResult, String> {
    let path = Path::new(&file_path);
    if !path.exists() {
        return Err("El archivo no existe".to_string());
    }

    let file = File::open(path)
        .map_err(|e| format!("Error al abrir archivo: {}", e))?;
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .trim(csv::Trim::All)
        .from_reader(file);

    let headers = rdr.headers()
        .map_err(|e| format!("Error al leer encabezados del CSV: {}", e))?
        .clone();

    // Map header names to indices
    let col = |name: &str| -> Option<usize> {
        headers.iter().position(|h| h.eq_ignore_ascii_case(name))
    };

    let idx_sku = col("sku").ok_or("Columna 'sku' no encontrada en el CSV")?;
    let idx_name = col("nombre").ok_or("Columna 'nombre' no encontrada en el CSV")?;
    let idx_category = col("categoria").ok_or("Columna 'categoria' no encontrada en el CSV")?;
    let idx_purchase = col("precio_compra").ok_or("Columna 'precio_compra' no encontrada en el CSV")?;
    let idx_sale = col("precio_venta").ok_or("Columna 'precio_venta' no encontrada en el CSV")?;

    let idx_barcode = col("codigo_barras");
    let idx_desc = col("descripcion");
    let idx_tax = col("tasa_impuesto");
    let idx_unit = col("unidad");
    let idx_min_stock = col("stock_minimo");
    let idx_initial_stock = col("stock_inicial");

    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut result = ImportResult {
        created: 0,
        updated: 0,
        errors: Vec::new(),
    };

    let get_field = |record: &csv::StringRecord, idx: usize| -> String {
        record.get(idx).unwrap_or("").trim().to_string()
    };

    let get_optional = |record: &csv::StringRecord, idx: Option<usize>| -> Option<String> {
        idx.and_then(|i| record.get(i))
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
    };

    for (i, record_result) in rdr.records().enumerate() {
        let row_num = (i + 2) as u32; // +2: 1 for 0-index, 1 for header row

        let record = match record_result {
            Ok(r) => r,
            Err(e) => {
                result.errors.push(ImportError {
                    row: row_num,
                    message: format!("Error al leer fila: {}", e),
                });
                continue;
            }
        };

        // Required fields
        let sku = get_field(&record, idx_sku);
        let name = get_field(&record, idx_name);
        let category_name = get_field(&record, idx_category);
        let purchase_str = get_field(&record, idx_purchase);
        let sale_str = get_field(&record, idx_sale);

        // Validate required fields
        if sku.is_empty() {
            result.errors.push(ImportError { row: row_num, message: "SKU está vacío".to_string() });
            continue;
        }
        if name.is_empty() {
            result.errors.push(ImportError { row: row_num, message: "Nombre está vacío".to_string() });
            continue;
        }
        if category_name.is_empty() {
            result.errors.push(ImportError { row: row_num, message: "Categoría está vacía".to_string() });
            continue;
        }

        let purchase_price: f64 = match purchase_str.parse() {
            Ok(v) if v > 0.0 => v,
            Ok(_) => {
                result.errors.push(ImportError { row: row_num, message: "Precio de compra debe ser mayor a 0".to_string() });
                continue;
            }
            Err(_) => {
                result.errors.push(ImportError { row: row_num, message: format!("Precio de compra inválido: '{}'", purchase_str) });
                continue;
            }
        };

        let sale_price: f64 = match sale_str.parse() {
            Ok(v) if v > 0.0 => v,
            Ok(_) => {
                result.errors.push(ImportError { row: row_num, message: "Precio de venta debe ser mayor a 0".to_string() });
                continue;
            }
            Err(_) => {
                result.errors.push(ImportError { row: row_num, message: format!("Precio de venta inválido: '{}'", sale_str) });
                continue;
            }
        };

        // Optional fields
        let barcode = get_optional(&record, idx_barcode);
        let description = get_optional(&record, idx_desc);
        let tax_rate: f64 = get_optional(&record, idx_tax)
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.13);
        let unit = get_optional(&record, idx_unit).unwrap_or_else(|| "unidad".to_string());
        let min_stock: i32 = get_optional(&record, idx_min_stock)
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let initial_stock: f64 = get_optional(&record, idx_initial_stock)
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.0);

        // Validate barcode uniqueness (if provided)
        if let Some(ref bc) = barcode {
            let existing: Option<String> = conn.query_row(
                "SELECT sku FROM products WHERE barcode = ?1 AND is_active = 1 AND sku != ?2",
                rusqlite::params![bc, &sku],
                |row| row.get(0),
            ).optional().map_err(|e| e.to_string())?;

            if let Some(existing_sku) = existing {
                result.errors.push(ImportError {
                    row: row_num,
                    message: format!("Código de barras '{}' ya pertenece al producto con SKU '{}'", bc, existing_sku),
                });
                continue;
            }
        }

        // Resolve category — find or create
        let category_id: String = {
            let existing_id: Option<String> = conn.query_row(
                "SELECT id FROM categories WHERE LOWER(name) = LOWER(?1) AND is_active = 1",
                [&category_name],
                |row| row.get(0),
            ).optional().map_err(|e| e.to_string())?;

            match existing_id {
                Some(id) => id,
                None => {
                    let new_id = Uuid::new_v4().to_string();
                    conn.execute(
                        "INSERT INTO categories (id, name) VALUES (?1, ?2)",
                        rusqlite::params![&new_id, &category_name],
                    ).map_err(|e| e.to_string())?;
                    new_id
                }
            }
        };

        // Check if product already exists by SKU
        let existing_product_id: Option<String> = conn.query_row(
            "SELECT id FROM products WHERE sku = ?1",
            [&sku],
            |row| row.get(0),
        ).optional().map_err(|e| e.to_string())?;

        match existing_product_id {
            Some(product_id) => {
                // UPDATE existing product
                conn.execute(
                    "UPDATE products SET name = ?1, barcode = ?2, description = ?3, category_id = ?4,
                     purchase_price = ?5, sale_price = ?6, tax_rate = ?7, unit = ?8, min_stock = ?9,
                     is_active = 1, updated_at = datetime('now', '-4 hours')
                     WHERE id = ?10",
                    rusqlite::params![
                        &name, &barcode, &description, &category_id,
                        purchase_price, sale_price, tax_rate, &unit, min_stock,
                        &product_id
                    ],
                ).map_err(|e| {
                    format!("Error al actualizar producto SKU '{}': {}", sku, e)
                })?;
                result.updated += 1;
            }
            None => {
                // CREATE new product
                let product_id = Uuid::new_v4().to_string();
                conn.execute(
                    "INSERT INTO products (id, sku, barcode, name, description, category_id, purchase_price, sale_price, tax_rate, unit, min_stock)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                    rusqlite::params![
                        &product_id, &sku, &barcode, &name, &description, &category_id,
                        purchase_price, sale_price, tax_rate, &unit, min_stock
                    ],
                ).map_err(|e| {
                    format!("Error al crear producto SKU '{}': {}", sku, e)
                })?;

                // Create inventory record
                let inv_id = Uuid::new_v4().to_string();
                conn.execute(
                    "INSERT INTO inventory (id, product_id, quantity) VALUES (?1, ?2, ?3)",
                    rusqlite::params![&inv_id, &product_id, initial_stock],
                ).map_err(|e| e.to_string())?;

                // If initial stock > 0, register inventory movement
                if initial_stock > 0.0 {
                    let mov_id = Uuid::new_v4().to_string();
                    conn.execute(
                        "INSERT INTO inventory_movements (id, product_id, movement_type, quantity, notes)
                         VALUES (?1, ?2, 'purchase', ?3, 'Stock inicial por importación CSV')",
                        rusqlite::params![&mov_id, &product_id, initial_stock],
                    ).map_err(|e| e.to_string())?;
                }

                result.created += 1;
            }
        }
    }

    Ok(result)
}
