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

/// Parsea una fecha en múltiples formatos al formato ISO YYYY-MM-DD.
/// Acepta: YYYY-MM-DD, DD/MM/YYYY, DD-MM-YYYY, MM/YYYY, MM-YYYY
fn parse_date(s: &str) -> Option<String> {
    let s = s.trim();
    if s.is_empty() { return None; }

    // YYYY-MM-DD
    if s.len() == 10 && s.as_bytes()[4] == b'-' {
        return Some(s.to_string());
    }
    // DD/MM/YYYY o DD-MM-YYYY
    if s.len() == 10 {
        let sep = if s.contains('/') { '/' } else { '-' };
        let parts: Vec<&str> = s.splitn(3, sep).collect();
        if parts.len() == 3 {
            if let (Ok(d), Ok(m), Ok(y)) = (
                parts[0].parse::<u32>(),
                parts[1].parse::<u32>(),
                parts[2].parse::<u32>(),
            ) {
                if d >= 1 && d <= 31 && m >= 1 && m <= 12 && y >= 2000 {
                    return Some(format!("{:04}-{:02}-{:02}", y, m, d));
                }
            }
        }
    }
    // MM/YYYY o MM-YYYY → último día del mes
    if s.len() == 7 {
        let sep = if s.contains('/') { '/' } else { '-' };
        let parts: Vec<&str> = s.splitn(2, sep).collect();
        if parts.len() == 2 {
            if let (Ok(m), Ok(y)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                if m >= 1 && m <= 12 && y >= 2000 {
                    // Último día del mes: avanzar al mes siguiente y restar 1 día
                    let (last_day_m, last_day_y) = if m == 12 { (1u32, y + 1) } else { (m + 1, y) };
                    // Usar el día 1 del mes siguiente como referencia
                    let _ = (last_day_m, last_day_y); // calculado pero usamos el día 28 como seguro
                    let last_day = match m {
                        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                        4 | 6 | 9 | 11 => 30,
                        2 => if y % 4 == 0 && (y % 100 != 0 || y % 400 == 0) { 29 } else { 28 },
                        _ => 28,
                    };
                    return Some(format!("{:04}-{:02}-{:02}", y, m, last_day));
                }
            }
        }
    }
    None
}

/// Import products from a CSV file. Upserts by SKU.
/// Columnas opcionales de migración de vencimiento:
///   fecha_vencimiento  — fecha del lote (DD/MM/YYYY, MM/YYYY, YYYY-MM-DD)
///   lote               — número/nombre del lote (default: "MIGRACIÓN")
///   cantidad_lote      — stock del lote (default: 0)
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

    // Map header names to indices (case-insensitive)
    let col = |name: &str| -> Option<usize> {
        headers.iter().position(|h| h.eq_ignore_ascii_case(name))
    };

    let idx_sku      = col("sku").ok_or("Columna 'sku' no encontrada en el CSV")?;
    let idx_name     = col("nombre").ok_or("Columna 'nombre' no encontrada en el CSV")?;
    let idx_category = col("categoria").ok_or("Columna 'categoria' no encontrada en el CSV")?;
    let idx_purchase = col("precio_compra").ok_or("Columna 'precio_compra' no encontrada en el CSV")?;
    let idx_sale     = col("precio_venta").ok_or("Columna 'precio_venta' no encontrada en el CSV")?;

    let idx_barcode       = col("codigo_barras");
    let idx_desc          = col("descripcion");
    let idx_tax           = col("tasa_impuesto");
    let idx_unit          = col("unidad");
    let idx_min_stock     = col("stock_minimo");
    let idx_initial_stock = col("stock_inicial");

    // ── Columnas opcionales de migración de vencimiento ──────────────────
    let idx_expiry       = col("fecha_vencimiento");
    let idx_lot_num      = col("lote");

    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut result = ImportResult {
        created: 0,
        updated: 0,
        lots_created: 0,
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
        let row_num = (i + 2) as u32;

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

        // ── Campos requeridos ─────────────────────────────────────────────
        let sku           = get_field(&record, idx_sku);
        let name          = get_field(&record, idx_name);
        let category_name = get_field(&record, idx_category);
        let purchase_str  = get_field(&record, idx_purchase);
        let sale_str      = get_field(&record, idx_sale);

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

        // ── Campos opcionales del producto ────────────────────────────────
        let barcode       = get_optional(&record, idx_barcode);
        let description   = get_optional(&record, idx_desc);
        let tax_rate: f64 = get_optional(&record, idx_tax)
            .and_then(|s| s.parse().ok()).unwrap_or(0.13);
        let unit          = get_optional(&record, idx_unit).unwrap_or_else(|| "unidad".to_string());
        let min_stock: i32 = get_optional(&record, idx_min_stock)
            .and_then(|s| s.parse().ok()).unwrap_or(0);
        let initial_stock: f64 = get_optional(&record, idx_initial_stock)
            .and_then(|s| s.parse().ok()).unwrap_or(0.0);

        // ── Columnas de migración de vencimiento (opcionales) ─────────────
        let expiry_raw  = get_optional(&record, idx_expiry);
        let lot_number  = get_optional(&record, idx_lot_num)
            .unwrap_or_else(|| "MIGRACIÓN".to_string());
        let expiry_date = expiry_raw.as_deref().and_then(parse_date);

        // Advertir si la fecha vino pero no pudo parsearse
        if expiry_raw.is_some() && expiry_date.is_none() {
            result.errors.push(ImportError {
                row: row_num,
                message: format!(
                    "Fecha de vencimiento inválida '{}' para SKU '{}' — use DD/MM/YYYY, MM/YYYY o YYYY-MM-DD",
                    expiry_raw.unwrap_or_default(), sku
                ),
            });
            // No interrumpimos: se importa el producto igual, solo se omite el lote
        }

        // Validar unicidad de barcode
        if let Some(ref bc) = barcode {
            let existing: Option<String> = conn.query_row(
                "SELECT sku FROM products WHERE barcode = ?1 AND is_active = 1 AND sku != ?2",
                rusqlite::params![bc, &sku],
                |row| row.get(0),
            ).optional().map_err(|e| e.to_string())?;

            if let Some(existing_sku) = existing {
                result.errors.push(ImportError {
                    row: row_num,
                    message: format!("Código de barras '{}' ya pertenece al SKU '{}'", bc, existing_sku),
                });
                continue;
            }
        }

        // ── Resolver categoría (find or create) ───────────────────────────
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

        // ── Upsert producto por SKU ────────────────────────────────────────
        let existing_product_id: Option<String> = conn.query_row(
            "SELECT id FROM products WHERE sku = ?1",
            [&sku],
            |row| row.get(0),
        ).optional().map_err(|e| e.to_string())?;

        let product_id = match existing_product_id {
            Some(pid) => {
                conn.execute(
                    "UPDATE products SET name = ?1, barcode = ?2, description = ?3, category_id = ?4,
                     purchase_price = ?5, sale_price = ?6, tax_rate = ?7, unit = ?8, min_stock = ?9,
                     is_active = 1, updated_at = datetime('now', '-4 hours')
                     WHERE id = ?10",
                    rusqlite::params![
                        &name, &barcode, &description, &category_id,
                        purchase_price, sale_price, tax_rate, &unit, min_stock,
                        &pid
                    ],
                ).map_err(|e| format!("Error al actualizar SKU '{}': {}", sku, e))?;
                result.updated += 1;
                pid
            }
            None => {
                let new_pid = Uuid::new_v4().to_string();
                conn.execute(
                    "INSERT INTO products (id, sku, barcode, name, description, category_id,
                     purchase_price, sale_price, tax_rate, unit, min_stock)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                    rusqlite::params![
                        &new_pid, &sku, &barcode, &name, &description, &category_id,
                        purchase_price, sale_price, tax_rate, &unit, min_stock
                    ],
                ).map_err(|e| format!("Error al crear SKU '{}': {}", sku, e))?;

                // Lote inicial
                let inv_id = Uuid::new_v4().to_string();
                conn.execute(
                    "INSERT INTO inventory (id, product_id, quantity) VALUES (?1, ?2, ?3)",
                    rusqlite::params![&inv_id, &new_pid, initial_stock],
                ).map_err(|e| e.to_string())?;

                if initial_stock > 0.0 {
                    let mov_id = Uuid::new_v4().to_string();
                    conn.execute(
                        "INSERT INTO inventory_movements (id, product_id, movement_type, quantity, notes)
                         VALUES (?1, ?2, 'purchase', ?3, 'Stock inicial por importación CSV')",
                        rusqlite::params![&mov_id, &new_pid, initial_stock],
                    ).map_err(|e| e.to_string())?;
                }

                result.created += 1;
                new_pid
            }
        };

        // ── Migración de fecha de vencimiento ─────────────────────────────
        // Estrategia: actualizar el lote existente con expiry_date IS NULL
        // (el lote que tiene el stock real). Si ya existe un lote con esa fecha → skip.
        if let Some(ref expiry) = expiry_date {
            // ¿Ya existe un lote con exactamente esta fecha? (idempotente)
            let already_exists: Option<String> = conn.query_row(
                "SELECT id FROM inventory WHERE product_id = ?1 AND expiry_date = ?2 LIMIT 1",
                rusqlite::params![&product_id, expiry],
                |row| row.get(0),
            ).optional().map_err(|e| e.to_string())?;

            if already_exists.is_none() {
                // Buscar el lote principal: sin fecha, con mayor stock
                let main_lot_id: Option<String> = conn.query_row(
                    "SELECT id FROM inventory
                     WHERE product_id = ?1 AND expiry_date IS NULL
                     ORDER BY quantity DESC
                     LIMIT 1",
                    rusqlite::params![&product_id],
                    |row| row.get(0),
                ).optional().map_err(|e| e.to_string())?;

                if let Some(lot_id) = main_lot_id {
                    // Actualizar el lote principal con la fecha y número de lote del CSV
                    conn.execute(
                        "UPDATE inventory
                         SET expiry_date = ?1,
                             lot_number = CASE WHEN lot_number IS NULL OR lot_number = '' THEN ?2 ELSE lot_number END,
                             updated_at = datetime('now', '-4 hours')
                         WHERE id = ?3",
                        rusqlite::params![expiry, &lot_number, &lot_id],
                    ).map_err(|e| format!("Error al actualizar lote de SKU '{}': {}", sku, e))?;
                    result.lots_created += 1;
                }
                // Si no hay lote sin fecha → el producto no tiene lote principal, skip silencioso
            }
            // Si already_exists → skip (ya estaba migrado)
        }

    }

    Ok(result)
}
