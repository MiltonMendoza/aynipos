#[cfg(test)]
mod tests {
    use crate::tests::create_test_db;

    fn seed_product_with_stock(conn: &rusqlite::Connection, name: &str, stock: f64) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO products (id, sku, name, purchase_price, sale_price, tax_rate, unit)
             VALUES (?1, ?2, ?3, 5.0, 10.0, 0.13, 'unidad')",
            rusqlite::params![&id, format!("SKU-{}", &id[..8]), name],
        ).expect("insert product");
        let inv_id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO inventory (id, product_id, quantity) VALUES (?1, ?2, ?3)",
            rusqlite::params![inv_id, &id, stock],
        ).expect("insert inventory");
        id
    }

    #[test]
    fn test_adjust_inventory_increase() {
        let db = create_test_db();
        let conn = db.conn.lock().unwrap();
        let product_id = seed_product_with_stock(&conn, "Arroz 1kg", 50.0);

        // Ajuste de inventario positivo (+20 unidades)
        let adjustment = 20.0_f64;
        conn.execute(
            "UPDATE inventory SET quantity = quantity + ?1, updated_at = datetime('now') WHERE product_id = ?2",
            rusqlite::params![adjustment, &product_id],
        ).expect("adjust inventory");

        // Registrar movimiento
        conn.execute(
            "INSERT INTO inventory_movements (id, product_id, movement_type, quantity, notes)
             VALUES (?1, ?2, 'adjustment', ?3, 'Ajuste de prueba')",
            rusqlite::params![uuid::Uuid::new_v4().to_string(), &product_id, adjustment],
        ).expect("insert movement");

        let stock: f64 = conn.query_row(
            "SELECT quantity FROM inventory WHERE product_id = ?1",
            [&product_id],
            |row| row.get(0),
        ).expect("get stock");

        assert_eq!(stock, 70.0, "50 + 20 = 70");
    }

    #[test]
    fn test_adjust_inventory_decrease() {
        let db = create_test_db();
        let conn = db.conn.lock().unwrap();
        let product_id = seed_product_with_stock(&conn, "Fideo 500g", 30.0);

        // Ajuste negativo (-10 unidades = merma/pérdida)
        let adjustment = -10.0_f64;
        conn.execute(
            "UPDATE inventory SET quantity = quantity + ?1, updated_at = datetime('now') WHERE product_id = ?2",
            rusqlite::params![adjustment, &product_id],
        ).expect("adjust inventory");

        let stock: f64 = conn.query_row(
            "SELECT quantity FROM inventory WHERE product_id = ?1",
            [&product_id],
            |row| row.get(0),
        ).expect("get stock");

        assert_eq!(stock, 20.0, "30 - 10 = 20");
    }

    #[test]
    fn test_inventory_movement_is_recorded() {
        let db = create_test_db();
        let conn = db.conn.lock().unwrap();
        let product_id = seed_product_with_stock(&conn, "Agua 1L", 100.0);

        conn.execute(
            "INSERT INTO inventory_movements (id, product_id, movement_type, quantity, notes)
             VALUES (?1, ?2, 'purchase', 50.0, 'Compra inicial')",
            rusqlite::params![uuid::Uuid::new_v4().to_string(), &product_id],
        ).expect("insert movement");

        let movement_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM inventory_movements WHERE product_id = ?1 AND movement_type = 'purchase'",
            [&product_id],
            |row| row.get(0),
        ).expect("count movements");

        assert_eq!(movement_count, 1);
    }
}
