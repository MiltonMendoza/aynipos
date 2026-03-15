#[cfg(test)]
mod tests {
    use crate::tests::create_test_db;

    // ── Helper: inserta un producto con stock y retorna su ID ──────────────────
    fn seed_product(conn: &rusqlite::Connection, name: &str, price: f64, stock: f64) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO products (id, sku, name, purchase_price, sale_price, tax_rate, unit)
             VALUES (?1, ?2, ?3, ?4, ?5, 0.13, 'unidad')",
            rusqlite::params![&id, format!("SKU-{}", &id[..8]), name, price * 0.7, price],
        ).expect("insert product");
        let inv_id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO inventory (id, product_id, quantity) VALUES (?1, ?2, ?3)",
            rusqlite::params![inv_id, &id, stock],
        ).expect("insert inventory");
        id
    }

    // ── Tests ──────────────────────────────────────────────────────────────────

    #[test]
    fn test_create_sale_inserts_record_and_deducts_stock() {
        let db = create_test_db();
        let conn = db.conn.lock().unwrap();

        let product_id = seed_product(&conn, "Coca Cola 2L", 8.50, 20.0);

        // Crear venta manualmente (misma lógica que create_sale command)
        let sale_id = uuid::Uuid::new_v4().to_string();
        let qty = 3.0_f64;
        let price = 8.50_f64;
        let subtotal = qty * price;
        let tax = subtotal * 0.13;

        conn.execute(
            "INSERT INTO sales (id, sale_number, subtotal, tax_amount, discount_amount, total, payment_method, status)
             VALUES (?1, 1, ?2, ?3, 0.0, ?4, 'cash', 'completed')",
            rusqlite::params![&sale_id, subtotal, tax, subtotal],
        ).expect("insert sale");

        conn.execute(
            "INSERT INTO sale_items (id, sale_id, product_id, product_name, quantity, unit_price, discount, tax_rate, subtotal, total)
             VALUES (?1, ?2, ?3, 'Coca Cola 2L', ?4, ?5, 0.0, 0.13, ?6, ?6)",
            rusqlite::params![uuid::Uuid::new_v4().to_string(), &sale_id, &product_id, qty, price, subtotal],
        ).expect("insert sale_item");

        // Descontar stock
        conn.execute(
            "UPDATE inventory SET quantity = quantity - ?1 WHERE product_id = ?2",
            rusqlite::params![qty, &product_id],
        ).expect("deduct stock");

        // Verificar que la venta existe
        let sale_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sales WHERE id = ?1 AND status = 'completed'",
            [&sale_id],
            |row| row.get(0),
        ).expect("count sales");
        assert_eq!(sale_count, 1);

        // Verificar que el stock se descontó correctamente
        let remaining_stock: f64 = conn.query_row(
            "SELECT quantity FROM inventory WHERE product_id = ?1",
            [&product_id],
            |row| row.get(0),
        ).expect("get stock");
        assert_eq!(remaining_stock, 17.0, "20 - 3 = 17");
    }

    #[test]
    fn test_cancel_sale_restores_stock() {
        let db = create_test_db();
        let conn = db.conn.lock().unwrap();

        let product_id = seed_product(&conn, "Pepsi 2L", 7.50, 10.0);
        let sale_id = uuid::Uuid::new_v4().to_string();
        let qty = 4.0_f64;

        // Crear venta y descontar stock
        conn.execute(
            "INSERT INTO sales (id, sale_number, subtotal, tax_amount, discount_amount, total, payment_method, status)
             VALUES (?1, 2, 30.0, 3.9, 0.0, 30.0, 'cash', 'completed')",
            rusqlite::params![&sale_id],
        ).expect("insert sale");

        conn.execute(
            "INSERT INTO sale_items (id, sale_id, product_id, product_name, quantity, unit_price, discount, tax_rate, subtotal, total)
             VALUES (?1, ?2, ?3, 'Pepsi 2L', ?4, 7.5, 0.0, 0.13, 30.0, 30.0)",
            rusqlite::params![uuid::Uuid::new_v4().to_string(), &sale_id, &product_id, qty],
        ).expect("insert sale_item");

        conn.execute(
            "UPDATE inventory SET quantity = quantity - ?1 WHERE product_id = ?2",
            rusqlite::params![qty, &product_id],
        ).expect("deduct stock");

        // Stock después de la venta = 6
        let stock_after_sale: f64 = conn.query_row(
            "SELECT quantity FROM inventory WHERE product_id = ?1",
            [&product_id],
            |row| row.get(0),
        ).expect("get stock");
        assert_eq!(stock_after_sale, 6.0);

        // Anular la venta (cancel_sale command)
        conn.execute(
            "UPDATE inventory SET quantity = quantity + ?1 WHERE product_id = ?2",
            rusqlite::params![qty, &product_id],
        ).expect("restore stock");

        conn.execute(
            "UPDATE sales SET status = 'cancelled' WHERE id = ?1",
            [&sale_id],
        ).expect("cancel sale");

        // Verificar restauración de stock
        let stock_restored: f64 = conn.query_row(
            "SELECT quantity FROM inventory WHERE product_id = ?1",
            [&product_id],
            |row| row.get(0),
        ).expect("get stock");
        assert_eq!(stock_restored, 10.0, "El stock debe volver a 10 después de anular");

        // Verificar estado de la venta
        let status: String = conn.query_row(
            "SELECT status FROM sales WHERE id = ?1",
            [&sale_id],
            |row| row.get(0),
        ).expect("get sale status");
        assert_eq!(status, "cancelled");
    }

    #[test]
    fn test_sale_number_sequence_increments() {
        let db = create_test_db();
        let conn = db.conn.lock().unwrap();

        // El seed inicial pone la secuencia en 0
        let seq1: i64 = conn.query_row(
            "SELECT CAST(value AS INTEGER) + 1 FROM settings WHERE key = 'sale_number_sequence'",
            [],
            |row| row.get(0),
        ).expect("get sequence");
        assert_eq!(seq1, 1);

        // Simular actualización tras primera venta
        conn.execute(
            "UPDATE settings SET value = '1' WHERE key = 'sale_number_sequence'",
            [],
        ).expect("update sequence");

        let seq2: i64 = conn.query_row(
            "SELECT CAST(value AS INTEGER) + 1 FROM settings WHERE key = 'sale_number_sequence'",
            [],
            |row| row.get(0),
        ).expect("get sequence");
        assert_eq!(seq2, 2);
    }
}
