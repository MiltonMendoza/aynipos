use crate::db::Database;
use crate::db::models::*;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub fn open_cash_register(db: State<'_, Database>, opening_amount: f64) -> Result<CashRegister, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Check if there's already an open register
    let existing: Option<String> = conn.query_row(
        "SELECT id FROM cash_registers WHERE closed_at IS NULL",
        [],
        |row| row.get(0),
    ).ok();

    if existing.is_some() {
        return Err("Ya hay una caja abierta. Ciérrela antes de abrir una nueva.".to_string());
    }

    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    conn.execute(
        "INSERT INTO cash_registers (id, opened_at, opening_amount) VALUES (?1, ?2, ?3)",
        rusqlite::params![&id, &now, opening_amount],
    ).map_err(|e| e.to_string())?;

    Ok(CashRegister {
        id,
        opened_at: now,
        closed_at: None,
        opening_amount,
        closing_amount: None,
        expected_amount: None,
        notes: None,
        user_id: None,
    })
}

#[tauri::command]
pub fn close_cash_register(db: State<'_, Database>, closing_amount: f64, notes: Option<String>) -> Result<CashRegister, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Get open register
    let register_id: String = conn.query_row(
        "SELECT id FROM cash_registers WHERE closed_at IS NULL ORDER BY opened_at DESC LIMIT 1",
        [],
        |row| row.get(0),
    ).map_err(|_| "No hay caja abierta.".to_string())?;

    // Calculate expected amount
    let opening_amount: f64 = conn.query_row(
        "SELECT opening_amount FROM cash_registers WHERE id = ?1",
        [&register_id],
        |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    let sales_total: f64 = conn.query_row(
        "SELECT COALESCE(SUM(total), 0) FROM sales WHERE cash_register_id = ?1 AND status = 'completed' AND payment_method IN ('efectivo', 'mixto')",
        [&register_id],
        |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    let expected = opening_amount + sales_total;
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    conn.execute(
        "UPDATE cash_registers SET closed_at = ?1, closing_amount = ?2, expected_amount = ?3, notes = ?4 WHERE id = ?5",
        rusqlite::params![&now, closing_amount, expected, &notes, &register_id],
    ).map_err(|e| e.to_string())?;

    Ok(CashRegister {
        id: register_id,
        opened_at: String::new(),
        closed_at: Some(now),
        opening_amount,
        closing_amount: Some(closing_amount),
        expected_amount: Some(expected),
        notes,
        user_id: None,
    })
}

#[tauri::command]
pub fn get_current_cash_register(db: State<'_, Database>) -> Result<Option<CashRegister>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let result = conn.query_row(
        "SELECT * FROM cash_registers WHERE closed_at IS NULL ORDER BY opened_at DESC LIMIT 1",
        [],
        |row| {
            Ok(CashRegister {
                id: row.get(0)?,
                opened_at: row.get(1)?,
                closed_at: row.get(2)?,
                opening_amount: row.get(3)?,
                closing_amount: row.get(4)?,
                expected_amount: row.get(5)?,
                notes: row.get(6)?,
                user_id: row.get(7)?,
            })
        },
    );

    match result {
        Ok(cr) => Ok(Some(cr)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_cash_register_report(db: State<'_, Database>, register_id: String) -> Result<CashRegisterReport, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Get the cash register
    let register = conn.query_row(
        "SELECT * FROM cash_registers WHERE id = ?1",
        [&register_id],
        |row| {
            Ok(CashRegister {
                id: row.get(0)?,
                opened_at: row.get(1)?,
                closed_at: row.get(2)?,
                opening_amount: row.get(3)?,
                closing_amount: row.get(4)?,
                expected_amount: row.get(5)?,
                notes: row.get(6)?,
                user_id: row.get(7)?,
            })
        },
    ).map_err(|_| "Caja no encontrada.".to_string())?;

    // Completed sales by payment method
    let mut stmt = conn.prepare(
        "SELECT payment_method, COUNT(*), COALESCE(SUM(total), 0), COALESCE(SUM(discount_amount), 0), COALESCE(SUM(tax_amount), 0)
         FROM sales WHERE cash_register_id = ?1 AND status = 'completed'
         GROUP BY payment_method"
    ).map_err(|e| e.to_string())?;

    let mut total_sales = 0.0_f64;
    let mut total_transactions = 0_i64;
    let mut total_discount = 0.0_f64;
    let mut total_tax = 0.0_f64;
    let mut sales_cash = 0.0_f64;
    let mut sales_card = 0.0_f64;
    let mut sales_qr = 0.0_f64;
    let mut sales_mixed = 0.0_f64;
    let mut count_cash = 0_i64;
    let mut count_card = 0_i64;
    let mut count_qr = 0_i64;
    let mut count_mixed = 0_i64;

    let rows = stmt.query_map([&register_id], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, i64>(1)?,
            row.get::<_, f64>(2)?,
            row.get::<_, f64>(3)?,
            row.get::<_, f64>(4)?,
        ))
    }).map_err(|e| e.to_string())?;

    for row in rows {
        let (method, count, sum_total, sum_discount, sum_tax) = row.map_err(|e| e.to_string())?;
        total_sales += sum_total;
        total_transactions += count;
        total_discount += sum_discount;
        total_tax += sum_tax;

        match method.as_str() {
            "efectivo" => { sales_cash = sum_total; count_cash = count; },
            "tarjeta" => { sales_card = sum_total; count_card = count; },
            "qr" => { sales_qr = sum_total; count_qr = count; },
            "mixto" => { sales_mixed = sum_total; count_mixed = count; },
            _ => {}
        }
    }

    // Cancelled transactions
    let cancelled_transactions: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sales WHERE cash_register_id = ?1 AND status = 'cancelled'",
        [&register_id],
        |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    let difference = register.closing_amount.unwrap_or(0.0) - register.expected_amount.unwrap_or(0.0);

    Ok(CashRegisterReport {
        register,
        total_sales,
        total_transactions,
        cancelled_transactions,
        total_discount,
        total_tax,
        sales_cash,
        sales_card,
        sales_qr,
        sales_mixed,
        count_cash,
        count_card,
        count_qr,
        count_mixed,
        difference,
    })
}
