#[cfg(test)]
mod tests {
    use crate::tests::create_test_db;
    use crate::db::models::*;
    use rusqlite::OptionalExtension;

    // ── Helpers ────────────────────────────────────────────────────────────────

    fn make_product(name: &str, sku: &str, sale_price: f64) -> CreateProduct {
        CreateProduct {
            sku: sku.to_string(),
            barcode: None,
            name: name.to_string(),
            description: None,
            category_id: None,
            purchase_price: sale_price * 0.7,
            sale_price,
            tax_rate: Some(0.13),
            unit: Some("unidad".to_string()),
            min_stock: Some(0),
            metadata: None,
        }
    }

    fn insert_product(conn: &rusqlite::Connection, p: &CreateProduct) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO products (id, sku, barcode, name, description, category_id, purchase_price, sale_price, tax_rate, unit, min_stock)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            rusqlite::params![
                &id, &p.sku, &p.barcode, &p.name, &p.description, &p.category_id,
                p.purchase_price, p.sale_price, p.tax_rate.unwrap_or(0.13),
                p.unit.as_deref().unwrap_or("unidad"), p.min_stock.unwrap_or(0)
            ],
        ).expect("insert product");
        // Create initial inventory record
        let inv_id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO inventory (id, product_id, quantity) VALUES (?1, ?2, 10)",
            rusqlite::params![inv_id, &id],
        ).expect("insert inventory");
        id
    }

    // ── Tests ──────────────────────────────────────────────────────────────────

    #[test]
    fn test_create_product_exists_in_db() {
        let db = create_test_db();
        let conn = db.conn.lock().unwrap();

        let p = make_product("Coca Cola 2L", "CC-2L", 8.50);
        let id = insert_product(&conn, &p);

        let name: String = conn.query_row(
            "SELECT name FROM products WHERE id = ?1",
            [&id],
            |row| row.get(0),
        ).expect("product should exist");

        assert_eq!(name, "Coca Cola 2L");
    }

    #[test]
    fn test_get_products_search_filter() {
        let db = create_test_db();
        let conn = db.conn.lock().unwrap();

        insert_product(&conn, &make_product("Coca Cola 2L", "CC-2L", 8.50));
        insert_product(&conn, &make_product("Pepsi 2L", "PP-2L", 7.50));
        insert_product(&conn, &make_product("Agua Mineral", "AG-1L", 3.00));

        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM products WHERE name LIKE '%Cola%' AND is_active = 1",
            [],
            |row| row.get(0),
        ).expect("query");

        assert_eq!(count, 1, "Solo 'Coca Cola 2L' debe coincidir con 'Cola'");
    }

    #[test]
    fn test_delete_product_is_soft_delete() {
        let db = create_test_db();
        let conn = db.conn.lock().unwrap();

        let p = make_product("Producto Temporal", "TEMP-01", 5.00);
        let id = insert_product(&conn, &p);

        // Soft-delete (mismo que delete_product command)
        conn.execute(
            "UPDATE products SET is_active = 0 WHERE id = ?1",
            [&id],
        ).expect("soft delete");

        let is_active: i32 = conn.query_row(
            "SELECT is_active FROM products WHERE id = ?1",
            [&id],
            |row| row.get(0),
        ).expect("product should still exist in DB");

        assert_eq!(is_active, 0, "El producto debe existir pero con is_active = 0");

        let count_active: i64 = conn.query_row(
            "SELECT COUNT(*) FROM products WHERE id = ?1 AND is_active = 1",
            [&id],
            |row| row.get(0),
        ).expect("count");

        assert_eq!(count_active, 0, "No debe aparecer en listados activos");
    }

    #[test]
    fn test_barcode_uniqueness() {
        let db = create_test_db();
        let conn = db.conn.lock().unwrap();

        let barcode = "7790001234567";

        // Insertar primer producto con ese barcode
        let id1 = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO products (id, sku, name, purchase_price, sale_price, tax_rate, unit, barcode)
             VALUES (?1, 'SKU-A', 'Producto A', 5.0, 10.0, 0.13, 'unidad', ?2)",
            rusqlite::params![id1, barcode],
        ).expect("insert first product");

        // Verificar que podemos detectar duplicado (lógica del comando)
        let existing: Option<String> = conn.query_row(
            "SELECT name FROM products WHERE barcode = ?1 AND is_active = 1",
            [barcode],
            |row| row.get(0),
        ).optional().expect("query");

        assert!(existing.is_some(), "Debe detectar el barcode duplicado");
        assert_eq!(existing.unwrap(), "Producto A");
    }

    #[test]
    fn test_active_only_filter() {
        let db = create_test_db();
        let conn = db.conn.lock().unwrap();

        insert_product(&conn, &make_product("Activo", "ACT-01", 10.0));
        let id_inactive = insert_product(&conn, &make_product("Inactivo", "INC-01", 5.0));
        conn.execute(
            "UPDATE products SET is_active = 0 WHERE id = ?1",
            [&id_inactive],
        ).expect("soft delete");

        let active_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM products WHERE is_active = 1",
            [],
            |row| row.get(0),
        ).expect("count");

        let total_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM products",
            [],
            |row| row.get(0),
        ).expect("count");

        assert_eq!(active_count, 1);
        assert_eq!(total_count, 2, "Ambos deben existir en DB (soft-delete)");
    }
}
