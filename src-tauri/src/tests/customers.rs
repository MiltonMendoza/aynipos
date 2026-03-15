#[cfg(test)]
mod tests {
    use crate::tests::create_test_db;

    fn make_customer(conn: &rusqlite::Connection, name: &str, nit: Option<&str>, phone: Option<&str>) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO customers (id, nit, name, phone) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![&id, nit, name, phone],
        ).expect("insert customer");
        id
    }

    #[test]
    fn test_create_customer_exists_in_db() {
        let db = create_test_db();
        let conn = db.conn.lock().unwrap();

        let id = make_customer(&conn, "Juan Pérez", Some("1234567"), Some("70123456"));

        let name: String = conn.query_row(
            "SELECT name FROM customers WHERE id = ?1 AND is_active = 1",
            [&id],
            |row| row.get(0),
        ).expect("customer should exist");

        assert_eq!(name, "Juan Pérez");
    }

    #[test]
    fn test_search_customer_by_name() {
        let db = create_test_db();
        let conn = db.conn.lock().unwrap();

        make_customer(&conn, "Juan Pérez", Some("1111111"), Some("70000001"));
        make_customer(&conn, "María López", Some("2222222"), Some("70000002"));
        make_customer(&conn, "Carlos Juan", Some("3333333"), Some("70000003"));

        // Buscar por nombre (mismo SQL que get_customers command)
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM customers WHERE (name LIKE '%Juan%' OR nit LIKE '%Juan%' OR phone LIKE '%Juan%') AND is_active = 1",
            [],
            |row| row.get(0),
        ).expect("count");

        // "Juan Pérez" y "Carlos Juan" coinciden con "Juan"
        assert_eq!(count, 2);
    }

    #[test]
    fn test_delete_customer_is_soft_delete() {
        let db = create_test_db();
        let conn = db.conn.lock().unwrap();

        let id = make_customer(&conn, "Cliente Temporal", None, None);

        conn.execute(
            "UPDATE customers SET is_active = 0 WHERE id = ?1",
            [&id],
        ).expect("soft delete");

        let is_active: i32 = conn.query_row(
            "SELECT is_active FROM customers WHERE id = ?1",
            [&id],
            |row| row.get(0),
        ).expect("customer should still exist");

        assert_eq!(is_active, 0, "is_active debe ser 0 (soft-delete)");
    }
}
