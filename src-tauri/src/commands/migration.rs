use crate::db::Database;
use crate::db::models::*;
use rusqlite::OptionalExtension;
use tauri::State;
use uuid::Uuid;

// ─── Tipos para migración de datos legados ─────────────────────────────────

/// Un producto con su descripción parseada lista para previsualización
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LegacyProductRow {
    pub id: String,
    pub sku: String,
    pub name: String,
    pub raw_description: String,        // descripción original completa
    pub parsed_description: String,     // texto antes del primer "|"
    pub parsed_dose: String,            // texto después de "Dosis:"
    pub parsed_lab: String,             // texto después de "Laboratorio:"
    pub current_dose: Option<String>,   // valor actual en products.dose
    pub current_supplier_id: Option<String>,
    pub apply: bool,                    // si el usuario marcó esta fila para migrar
}

/// Una entrada de laboratorio único para el paso de mapeo
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LabEntry {
    pub name: String,
    pub count: u32,                         // cuántos productos tienen este lab
    pub action: String,                     // "create" | "ignore" | "existing"
    pub existing_supplier_id: Option<String>, // si action = "existing"
}

/// Payload que el frontend envía al aplicar la migración
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MigrationPayload {
    pub products: Vec<ProductMigrationItem>,
    pub lab_map: Vec<LabMapEntry>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProductMigrationItem {
    pub product_id: String,
    pub new_description: String,
    pub new_dose: String,
    pub lab_name: String,   // nombre del laboratorio (para buscar en lab_map)
    pub apply: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LabMapEntry {
    pub name: String,
    pub action: String,                       // "create" | "ignore" | "existing"
    pub existing_supplier_id: Option<String>, // si action = "existing"
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MigrationResult {
    pub products_updated: u32,
    pub suppliers_created: u32,
    pub suppliers_linked: u32,
    pub skipped: u32,
}

// ─── Funciones helper de parseo ────────────────────────────────────────────

fn parse_description_parts(raw: &str) -> (String, String, String) {
    // Formato: "[desc] | Dosis: [dose] | Laboratorio: [lab]"
    let pipe_pos = raw.find('|');
    let desc_clean = match pipe_pos {
        Some(p) => raw[..p].trim().to_string(),
        None => raw.trim().to_string(),
    };

    let dose = if let Some(d_start) = raw.find("Dosis:") {
        let after = &raw[d_start + 6..];
        let end = after.find('|').map(|p| p).unwrap_or(after.len());
        after[..end].trim().to_string()
    } else {
        String::new()
    };

    let lab = if let Some(l_start) = raw.find("Laboratorio:") {
        raw[l_start + 12..].trim().to_string()
    } else {
        String::new()
    };

    (desc_clean, dose, lab)
}

// ─── Comando 1: Previsualización ───────────────────────────────────────────

/// Devuelve todos los productos cuya descripción tiene el formato legado
/// "[desc] | Dosis: [x] | Laboratorio: [y]" junto con sus valores parseados.
#[tauri::command]
pub fn get_legacy_preview(db: State<'_, Database>) -> Result<Vec<LegacyProductRow>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "SELECT p.id, p.sku, p.name, p.description, p.dose, p.supplier_id
         FROM products p
         WHERE p.is_active = 1 AND p.description LIKE '%|%'
         ORDER BY p.name"
    ).map_err(|e| e.to_string())?;

    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,          // id
            row.get::<_, String>(1)?,          // sku
            row.get::<_, String>(2)?,          // name
            row.get::<_, Option<String>>(3)?,  // description
            row.get::<_, Option<String>>(4)?,  // dose
            row.get::<_, Option<String>>(5)?,  // supplier_id
        ))
    }).map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for row in rows {
        let (id, sku, name, description, dose, supplier_id) = row.map_err(|e| e.to_string())?;
        let raw = description.unwrap_or_default();
        let (parsed_description, parsed_dose, parsed_lab) = parse_description_parts(&raw);

        result.push(LegacyProductRow {
            id,
            sku,
            name,
            raw_description: raw,
            parsed_description,
            parsed_dose,
            parsed_lab,
            current_dose: dose,
            current_supplier_id: supplier_id,
            apply: true, // por defecto todos seleccionados
        });
    }

    Ok(result)
}

// ─── Comando 2: Lista de laboratorios únicos ──────────────────────────────

/// Devuelve laboratorios únicos con su frecuencia y proveedores existentes para mapeo.
#[tauri::command]
pub fn get_legacy_labs(db: State<'_, Database>) -> Result<Vec<LabEntry>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Obtener todos los productos con lab parseado
    let mut stmt = conn.prepare(
        "SELECT description FROM products WHERE is_active = 1 AND description LIKE '%|%'"
    ).map_err(|e| e.to_string())?;

    let descs: Vec<String> = stmt.query_map([], |row| row.get(0))
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    // Contar frecuencia de cada laboratorio
    let mut lab_counts: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
    for desc in &descs {
        let (_, _, lab) = parse_description_parts(desc);
        if !lab.is_empty() {
            *lab_counts.entry(lab).or_insert(0) += 1;
        }
    }

    // Obtener proveedores existentes para sugerencia automática
    let mut supplier_stmt = conn.prepare(
        "SELECT id, name FROM suppliers ORDER BY name"
    ).map_err(|e| e.to_string())?;

    let suppliers: Vec<(String, String)> = supplier_stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    }).map_err(|e| e.to_string())?
    .filter_map(|r| r.ok())
    .collect();

    // Construir lista ordenada por frecuencia desc
    let mut entries: Vec<LabEntry> = lab_counts.into_iter().map(|(name, count)| {
        // Buscar coincidencia exacta (case-insensitive) con proveedor existente
        let existing = suppliers.iter().find(|(_, sname)| {
            sname.to_lowercase() == name.to_lowercase()
        });

        let (action, existing_supplier_id) = match existing {
            Some((sid, _)) => ("existing".to_string(), Some(sid.clone())),
            None => ("create".to_string(), None),
        };

        LabEntry { name, count, action, existing_supplier_id }
    }).collect();

    entries.sort_by(|a, b| b.count.cmp(&a.count).then(a.name.cmp(&b.name)));
    Ok(entries)
}

// ─── Comando 3: Aplicar migración ─────────────────────────────────────────

/// Aplica la migración de datos legados:
/// - Actualiza description, dose de productos marcados
/// - Crea proveedores nuevos según lab_map (action = "create")
/// - Vincula products.supplier_id a proveedor existente o recién creado
#[tauri::command]
pub fn apply_legacy_migration(
    db: State<'_, Database>,
    payload: MigrationPayload,
) -> Result<MigrationResult, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut result = MigrationResult {
        products_updated: 0,
        suppliers_created: 0,
        suppliers_linked: 0,
        skipped: 0,
    };

    // Paso 1: Resolver lab_map → supplier_id
    // Crea o recupera el supplier_id para cada lab según su action
    let mut lab_to_supplier: std::collections::HashMap<String, Option<String>> =
        std::collections::HashMap::new();

    for entry in &payload.lab_map {
        let supplier_id = match entry.action.as_str() {
            "create" => {
                // Crear proveedor nuevo con ese nombre
                let new_id = Uuid::new_v4().to_string();
                conn.execute(
                    "INSERT INTO suppliers (id, name) VALUES (?1, ?2)",
                    rusqlite::params![&new_id, &entry.name],
                ).map_err(|e| format!("Error al crear proveedor '{}': {}", entry.name, e))?;
                result.suppliers_created += 1;
                Some(new_id)
            }
            "existing" => entry.existing_supplier_id.clone(),
            _ => None, // "ignore"
        };
        lab_to_supplier.insert(entry.name.clone(), supplier_id);
    }

    // Paso 2: Actualizar cada producto marcado
    for item in &payload.products {
        if !item.apply {
            result.skipped += 1;
            continue;
        }

        let supplier_id = lab_to_supplier.get(&item.lab_name).cloned().flatten();
        let had_supplier = supplier_id.is_some();

        conn.execute(
            "UPDATE products
             SET description = ?1,
                 dose = CASE WHEN ?2 = '' THEN dose ELSE ?2 END,
                 supplier_id = CASE WHEN ?3 IS NULL THEN supplier_id ELSE ?3 END,
                 updated_at = datetime('now', '-4 hours')
             WHERE id = ?4",
            rusqlite::params![
                &item.new_description,
                &item.new_dose,
                &supplier_id,
                &item.product_id
            ],
        ).map_err(|e| format!("Error al actualizar producto {}: {}", item.product_id, e))?;

        result.products_updated += 1;
        if had_supplier {
            result.suppliers_linked += 1;
        }
    }

    Ok(result)
}

// ─── Comando auxiliar: proveedores existentes (para el select del frontend) ─

/// Devuelve lista simple de proveedores (id + name) para el dropdown de mapeo
#[tauri::command]
pub fn get_suppliers_simple(db: State<'_, Database>) -> Result<Vec<(String, String)>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, name FROM suppliers WHERE is_active = 1 ORDER BY name"
    ).map_err(|e| e.to_string())?;

    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    }).map_err(|e| e.to_string())?;

    Ok(rows.filter_map(|r| r.ok()).collect())
}
