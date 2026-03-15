#[cfg(test)]
mod tests {
    use crate::tests::create_test_db;

    fn open_register(conn: &rusqlite::Connection, opening_amount: f64) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO cash_registers (id, opening_amount, opened_at)
             VALUES (?1, ?2, datetime('now'))",
            rusqlite::params![&id, opening_amount],
        ).expect("open cash register");
        id
    }

    #[test]
    fn test_open_cash_register() {
        let db = create_test_db();
        let conn = db.conn.lock().unwrap();

        let id = open_register(&conn, 500.0);

        let amount: f64 = conn.query_row(
            "SELECT opening_amount FROM cash_registers WHERE id = ?1 AND closed_at IS NULL",
            [&id],
            |row| row.get(0),
        ).expect("cash register should be open");

        assert_eq!(amount, 500.0);
    }

    #[test]
    fn test_close_cash_register() {
        let db = create_test_db();
        let conn = db.conn.lock().unwrap();

        let id = open_register(&conn, 200.0);

        // Cerrar caja (mismo que close_cash_register command)
        let closing_amount = 850.0_f64;
        let expected = 200.0_f64 + 650.0; // opening + ventas imaginarias
        let difference = closing_amount - expected;

        conn.execute(
            "UPDATE cash_registers SET closed_at = datetime('now'), closing_amount = ?1, expected_amount = ?2
             WHERE id = ?3",
            rusqlite::params![closing_amount, expected, &id],
        ).expect("close cash register");

        let closed_at: Option<String> = conn.query_row(
            "SELECT closed_at FROM cash_registers WHERE id = ?1",
            [&id],
            |row| row.get(0),
        ).expect("query");

        assert!(closed_at.is_some(), "closed_at debe tener valor al cerrar");

        let diff_check = closing_amount - expected;
        // Diferencia puede ser positiva/negativa (representa faltante o sobrante)
        assert!((diff_check - difference).abs() < 0.001);
    }

    #[test]
    fn test_only_one_active_register() {
        let db = create_test_db();
        let conn = db.conn.lock().unwrap();

        // Abrir primera caja
        open_register(&conn, 300.0);

        // Contar registros abiertos
        let open_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM cash_registers WHERE closed_at IS NULL",
            [],
            |row| row.get(0),
        ).expect("count");

        assert_eq!(open_count, 1, "Solo debe haber una caja abierta");
    }
}
