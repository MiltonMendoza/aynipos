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

#[tauri::command]
pub fn get_top_selling_products(
    db: State<'_, Database>,
    date_from: Option<String>,
    date_to: Option<String>,
    limit: Option<i64>,
) -> Result<Vec<TopSellingProduct>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let actual_limit = limit.unwrap_or(10);

    let mut stmt = conn
        .prepare(
            "SELECT si.product_id, si.product_name,
                    SUM(si.quantity) as total_quantity,
                    SUM(si.total) as total_revenue
             FROM sale_items si
             JOIN sales s ON s.id = si.sale_id
             WHERE s.status = 'completed'
               AND (?1 IS NULL OR DATE(s.created_at) >= ?1)
               AND (?2 IS NULL OR DATE(s.created_at) <= ?2)
             GROUP BY si.product_id, si.product_name
             ORDER BY total_quantity DESC
             LIMIT ?3",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(
            rusqlite::params![date_from, date_to, actual_limit],
            |row| {
                Ok(TopSellingProduct {
                    product_id: row.get(0)?,
                    product_name: row.get(1)?,
                    total_quantity: row.get(2)?,
                    total_revenue: row.get(3)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;

    let mut products = Vec::new();
    for row in rows {
        products.push(row.map_err(|e| e.to_string())?);
    }

    Ok(products)
}

#[tauri::command]
pub fn get_sales_chart_data(
    db: State<'_, Database>,
    date_from: Option<String>,
    date_to: Option<String>,
    group_by: Option<String>,
) -> Result<Vec<SalesChartDataPoint>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let grouping = group_by.unwrap_or_else(|| "day".to_string());

    let group_expr = match grouping.as_str() {
        "week" => "strftime('%Y-W%W', created_at)",
        "month" => "strftime('%Y-%m', created_at)",
        _ => "DATE(created_at)", // day
    };

    let sql = format!(
        "SELECT {group_expr} as label,
                COALESCE(SUM(total), 0) as total_sales,
                COUNT(*) as transaction_count
         FROM sales
         WHERE status = 'completed'
           AND (?1 IS NULL OR DATE(created_at) >= ?1)
           AND (?2 IS NULL OR DATE(created_at) <= ?2)
         GROUP BY label
         ORDER BY label ASC"
    );

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(rusqlite::params![date_from, date_to], |row| {
            Ok(SalesChartDataPoint {
                label: row.get(0)?,
                total_sales: row.get(1)?,
                transaction_count: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut points = Vec::new();
    for row in rows {
        points.push(row.map_err(|e| e.to_string())?);
    }

    Ok(points)
}

#[tauri::command]
pub fn get_profit_margin_report(
    db: State<'_, Database>,
    date_from: Option<String>,
    date_to: Option<String>,
) -> Result<Vec<ProfitMarginProduct>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT si.product_id,
                    si.product_name,
                    p.purchase_price,
                    SUM(si.unit_price * si.quantity) / SUM(si.quantity) as avg_sale_price,
                    SUM(si.quantity) as total_quantity,
                    SUM(si.total) as total_revenue,
                    SUM(p.purchase_price * si.quantity) as total_cost,
                    SUM(si.total) - SUM(p.purchase_price * si.quantity) as gross_profit,
                    CASE WHEN SUM(p.purchase_price * si.quantity) > 0
                         THEN ((SUM(si.total) - SUM(p.purchase_price * si.quantity)) / SUM(p.purchase_price * si.quantity)) * 100
                         ELSE 0
                    END as margin_percent
             FROM sale_items si
             JOIN sales s ON s.id = si.sale_id
             JOIN products p ON p.id = si.product_id
             WHERE s.status = 'completed'
               AND (?1 IS NULL OR DATE(s.created_at) >= ?1)
               AND (?2 IS NULL OR DATE(s.created_at) <= ?2)
             GROUP BY si.product_id, si.product_name, p.purchase_price
             ORDER BY gross_profit DESC",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(rusqlite::params![date_from, date_to], |row| {
            Ok(ProfitMarginProduct {
                product_id: row.get(0)?,
                product_name: row.get(1)?,
                purchase_price: row.get(2)?,
                avg_sale_price: row.get(3)?,
                total_quantity: row.get(4)?,
                total_revenue: row.get(5)?,
                total_cost: row.get(6)?,
                gross_profit: row.get(7)?,
                margin_percent: row.get(8)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut products = Vec::new();
    for row in rows {
        products.push(row.map_err(|e| e.to_string())?);
    }

    Ok(products)
}
