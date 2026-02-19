use crate::db::Database;
use crate::db::models::*;
use tauri::State;

#[tauri::command]
pub fn get_dashboard_stats(db: State<'_, Database>) -> Result<DashboardStats, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();

    let total_sales_today: f64 = conn.query_row(
        "SELECT COALESCE(SUM(total), 0) FROM sales WHERE DATE(created_at) = ?1 AND status = 'completed'",
        [&today],
        |row| row.get(0),
    ).unwrap_or(0.0);

    let total_transactions_today: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sales WHERE DATE(created_at) = ?1 AND status = 'completed'",
        [&today],
        |row| row.get(0),
    ).unwrap_or(0);

    let total_products: i64 = conn.query_row(
        "SELECT COUNT(*) FROM products WHERE is_active = 1",
        [],
        |row| row.get(0),
    ).unwrap_or(0);

    let low_stock_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM products p
         LEFT JOIN (SELECT product_id, SUM(quantity) as total_qty FROM inventory GROUP BY product_id) i
         ON i.product_id = p.id
         WHERE p.is_active = 1 AND COALESCE(i.total_qty, 0) <= p.min_stock AND p.min_stock > 0",
        [],
        |row| row.get(0),
    ).unwrap_or(0);

    let expiring_soon_count: i64 = conn.query_row(
        "SELECT COUNT(DISTINCT product_id) FROM inventory
         WHERE expiry_date IS NOT NULL AND expiry_date <= DATE('now', '+30 days')",
        [],
        |row| row.get(0),
    ).unwrap_or(0);

    Ok(DashboardStats {
        total_sales_today,
        total_transactions_today,
        total_products,
        low_stock_count,
        expiring_soon_count,
    })
}
