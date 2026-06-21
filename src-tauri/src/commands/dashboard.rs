use crate::db::Database;
use crate::db::models::*;
use tauri::State;

#[tauri::command]
pub fn get_dashboard_stats(db: State<'_, Database>, user_id: Option<String>) -> Result<DashboardStats, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let today = (chrono::Utc::now() - chrono::Duration::hours(4)).format("%Y-%m-%d").to_string();

    // Ventas del día — filtradas por user_id si se proporciona (cajero solo ve las suyas)
    let total_sales_today: f64 = if let Some(ref uid) = user_id {
        conn.query_row(
            "SELECT COALESCE(SUM(total), 0) FROM sales WHERE DATE(created_at) = ?1 AND status = 'completed' AND user_id = ?2",
            rusqlite::params![&today, uid],
            |row| row.get(0),
        ).unwrap_or(0.0)
    } else {
        conn.query_row(
            "SELECT COALESCE(SUM(total), 0) FROM sales WHERE DATE(created_at) = ?1 AND status = 'completed'",
            [&today],
            |row| row.get(0),
        ).unwrap_or(0.0)
    };

    // Transacciones del día — igual filtrado
    let total_transactions_today: i64 = if let Some(ref uid) = user_id {
        conn.query_row(
            "SELECT COUNT(*) FROM sales WHERE DATE(created_at) = ?1 AND status = 'completed' AND user_id = ?2",
            rusqlite::params![&today, uid],
            |row| row.get(0),
        ).unwrap_or(0)
    } else {
        conn.query_row(
            "SELECT COUNT(*) FROM sales WHERE DATE(created_at) = ?1 AND status = 'completed'",
            [&today],
            |row| row.get(0),
        ).unwrap_or(0)
    };

    // Inventario — siempre global (el cajero necesita saber qué hay en stock)
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
         WHERE expiry_date IS NOT NULL AND expiry_date <= DATE('now', '-4 hours', '+30 days')",
        [],
        |row| row.get(0),
    ).unwrap_or(0);

    let total_capital: f64 = conn.query_row(
        "SELECT COALESCE(SUM(p.sale_price * COALESCE(i.total_qty, 0)), 0)
         FROM products p
         LEFT JOIN (SELECT product_id, SUM(quantity) as total_qty FROM inventory GROUP BY product_id) i
         ON i.product_id = p.id
         WHERE p.is_active = 1",
        [],
        |row| row.get(0),
    ).unwrap_or(0.0);

    Ok(DashboardStats {
        total_sales_today,
        total_transactions_today,
        total_products,
        low_stock_count,
        expiring_soon_count,
        total_capital,
    })
}

#[tauri::command]
pub fn get_top_selling_products(
    db: State<'_, Database>,
    date_from: Option<String>,
    date_to: Option<String>,
    limit: Option<i64>,
    user_id: Option<String>,
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
               AND (?4 IS NULL OR s.user_id = ?4)
             GROUP BY si.product_id, si.product_name
             ORDER BY total_quantity DESC
             LIMIT ?3",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(
            rusqlite::params![date_from, date_to, actual_limit, user_id],
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
    user_id: Option<String>,
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
           AND (?3 IS NULL OR user_id = ?3)
         GROUP BY label
         ORDER BY label ASC"
    );

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(rusqlite::params![date_from, date_to, user_id], |row| {
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

#[tauri::command]
pub fn get_inventory_report(
    db: State<'_, Database>,
    inactive_days: Option<i64>,
) -> Result<Vec<InventoryReportItem>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut sql = String::from(
        "SELECT p.id, p.name, p.sku,
                c.name as category_name,
                COALESCE(inv.total_stock, 0) as current_stock,
                p.purchase_price,
                p.sale_price,
                COALESCE(inv.total_stock, 0) * p.purchase_price as stock_cost_value,
                COALESCE(inv.total_stock, 0) * p.sale_price as stock_sale_value,
                lm.last_date as last_movement_date,
                CASE WHEN lm.last_date IS NOT NULL
                     THEN CAST(julianday('now', '-4 hours') - julianday(lm.last_date) AS INTEGER)
                     ELSE NULL
                END as days_without_movement
         FROM products p
         LEFT JOIN (
             SELECT product_id, SUM(quantity) as total_stock
             FROM inventory
             GROUP BY product_id
         ) inv ON inv.product_id = p.id
         LEFT JOIN categories c ON c.id = p.category_id
         LEFT JOIN (
             SELECT product_id, MAX(created_at) as last_date
             FROM inventory_movements
             GROUP BY product_id
         ) lm ON lm.product_id = p.id
         WHERE p.is_active = 1"
    );

    if let Some(days) = inactive_days {
        sql.push_str(&format!(
            " AND (lm.last_date IS NULL OR CAST(julianday('now', '-4 hours') - julianday(lm.last_date) AS INTEGER) >= {})",
            days
        ));
    }

    sql.push_str(" ORDER BY stock_cost_value DESC");

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(InventoryReportItem {
                product_id: row.get(0)?,
                product_name: row.get(1)?,
                sku: row.get(2)?,
                category_name: row.get(3)?,
                current_stock: row.get(4)?,
                purchase_price: row.get(5)?,
                sale_price: row.get(6)?,
                stock_cost_value: row.get(7)?,
                stock_sale_value: row.get(8)?,
                last_movement_date: row.get(9)?,
                days_without_movement: row.get(10)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut items = Vec::new();
    for row in rows {
        items.push(row.map_err(|e| e.to_string())?);
    }

    Ok(items)
}

#[tauri::command]
pub fn get_expiry_report(
    db: State<'_, Database>,
    status_filter: Option<String>, // "active" | "expiring" | "expired" | None = all with expiry
    search: Option<String>,
) -> Result<Vec<ExpiryReportItem>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut query = String::from(
        "SELECT
             p.id,
             p.name,
             p.sku,
             c.name as category_name,
             s.name as supplier_name,
             p.dose,
             COALESCE(inv.total_stock, 0) as current_stock,
             p.sale_price,
             p.purchase_price,
             COALESCE(inv.total_stock, 0) * p.sale_price as stock_sale_value,
             inv.nearest_expiry,
             CASE
                 WHEN inv.nearest_expiry IS NULL THEN 'active'
                 WHEN inv.nearest_expiry < DATE('now', '-4 hours') THEN 'expired'
                 WHEN inv.nearest_expiry <= DATE('now', '-4 hours', '+4 months') THEN 'expiring'
                 ELSE 'active'
             END as expiry_status
         FROM products p
         LEFT JOIN (
             SELECT
                 product_id,
                 SUM(quantity) as total_stock,
                 MIN(CASE WHEN quantity > 0 AND expiry_date IS NOT NULL THEN expiry_date END) as nearest_expiry
             FROM inventory
             GROUP BY product_id
         ) inv ON inv.product_id = p.id
         LEFT JOIN categories c ON c.id = p.category_id
         LEFT JOIN suppliers s ON s.id = p.supplier_id
         WHERE p.is_active = 1"
    );

    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(ref s) = search {
        query.push_str(" AND (p.name LIKE ?1 OR p.sku LIKE ?1)");
        params.push(Box::new(format!("%{}%", s)));
    }

    // Wrap with status filter using subquery
    let final_query = if let Some(ref status) = status_filter {
        format!(
            "SELECT * FROM ({}) sub WHERE expiry_status = '{}'",
            query,
            status.replace('\'', "''") // basic sanitize
        )
    } else {
        query + " ORDER BY inv.nearest_expiry ASC NULLS LAST, p.name ASC"
    };

    let ordered_query = if status_filter.is_some() {
        final_query + " ORDER BY nearest_expiry ASC NULLS LAST, product_name ASC"
    } else {
        final_query
    };

    let mut stmt = conn.prepare(&ordered_query).map_err(|e| e.to_string())?;
    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();

    let rows = stmt.query_map(param_refs.as_slice(), |row| {
        Ok(ExpiryReportItem {
            product_id: row.get(0)?,
            product_name: row.get(1)?,
            sku: row.get(2)?,
            category_name: row.get(3)?,
            supplier_name: row.get(4)?,
            dose: row.get(5)?,
            current_stock: row.get(6)?,
            sale_price: row.get(7)?,
            purchase_price: row.get(8)?,
            stock_sale_value: row.get(9)?,
            nearest_expiry_date: row.get(10)?,
            expiry_status: row.get(11)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut items = Vec::new();
    for row in rows {
        items.push(row.map_err(|e| e.to_string())?);
    }

    Ok(items)
}

// ─── Stock Report ─────────────────────────────────────────────────────────────

#[tauri::command]
pub fn get_stock_report(
    db: State<'_, Database>,
    exact_stock: Option<i64>,
) -> Result<Vec<StockReportItem>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let base = "
        SELECT
            p.id,
            p.name,
            p.sku,
            c.name as category_name,
            COALESCE(i.total_stock, 0) as current_stock,
            p.sale_price,
            COALESCE(i.total_stock, 0) * p.sale_price as stock_sale_value
        FROM products p
        LEFT JOIN (
            SELECT product_id, SUM(quantity) as total_stock
            FROM inventory
            GROUP BY product_id
        ) i ON i.product_id = p.id
        LEFT JOIN categories c ON c.id = p.category_id
        WHERE p.is_active = 1";

    let query = if let Some(exact) = exact_stock {
        format!("{} AND CAST(COALESCE(i.total_stock, 0) AS INTEGER) = {} ORDER BY p.name ASC", base, exact)
    } else {
        format!("{} ORDER BY current_stock ASC, p.name ASC", base)
    };

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| {
        Ok(StockReportItem {
            product_id: row.get(0)?,
            product_name: row.get(1)?,
            sku: row.get(2)?,
            category_name: row.get(3)?,
            current_stock: row.get(4)?,
            sale_price: row.get(5)?,
            stock_sale_value: row.get(6)?,
        })
    }).map_err(|e| e.to_string())?;

    Ok(rows.filter_map(|r| r.ok()).collect())
}

// ─── Expiry Range Report ──────────────────────────────────────────────────────

#[tauri::command]
pub fn get_expiry_range_report(
    db: State<'_, Database>,
    expiry_from: Option<String>,
    expiry_to: Option<String>,
    search: Option<String>,
) -> Result<Vec<ExpiryReportItem>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut query = String::from(
        "SELECT
             p.id,
             p.name,
             p.sku,
             c.name as category_name,
             s.name as supplier_name,
             p.dose,
             COALESCE(inv.total_stock, 0) as current_stock,
             p.sale_price,
             p.purchase_price,
             COALESCE(inv.total_stock, 0) * p.sale_price as stock_sale_value,
             inv.nearest_expiry,
             CASE
                 WHEN inv.nearest_expiry IS NULL THEN 'active'
                 WHEN inv.nearest_expiry < DATE('now', '-4 hours') THEN 'expired'
                 WHEN inv.nearest_expiry <= DATE('now', '-4 hours', '+4 months') THEN 'expiring'
                 ELSE 'active'
             END as expiry_status
         FROM products p
         LEFT JOIN (
             SELECT
                 product_id,
                 SUM(quantity) as total_stock,
                 MIN(CASE WHEN quantity > 0 AND expiry_date IS NOT NULL THEN expiry_date END) as nearest_expiry
             FROM inventory
             GROUP BY product_id
         ) inv ON inv.product_id = p.id
         LEFT JOIN categories c ON c.id = p.category_id
         LEFT JOIN suppliers s ON s.id = p.supplier_id
         WHERE p.is_active = 1
           AND COALESCE(inv.total_stock, 0) > 0
           AND inv.nearest_expiry IS NOT NULL"
    );

    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    let mut idx = 1usize;

    if let Some(ref from) = expiry_from {
        query.push_str(&format!(" AND inv.nearest_expiry >= ?{}", idx));
        params.push(Box::new(from.clone()));
        idx += 1;
    }

    if let Some(ref to) = expiry_to {
        query.push_str(&format!(" AND inv.nearest_expiry <= ?{}", idx));
        params.push(Box::new(to.clone()));
        idx += 1;
    }

    if let Some(ref s) = search {
        query.push_str(&format!(" AND (p.name LIKE ?{} OR p.sku LIKE ?{})", idx, idx));
        params.push(Box::new(format!("%{}%", s)));
        let _ = idx; // last param
    }

    query.push_str(" ORDER BY inv.nearest_expiry ASC, p.name ASC");

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();

    let rows = stmt.query_map(param_refs.as_slice(), |row| {
        Ok(ExpiryReportItem {
            product_id: row.get(0)?,
            product_name: row.get(1)?,
            sku: row.get(2)?,
            category_name: row.get(3)?,
            supplier_name: row.get(4)?,
            dose: row.get(5)?,
            current_stock: row.get(6)?,
            sale_price: row.get(7)?,
            purchase_price: row.get(8)?,
            stock_sale_value: row.get(9)?,
            nearest_expiry_date: row.get(10)?,
            expiry_status: row.get(11)?,
        })
    }).map_err(|e| e.to_string())?;

    Ok(rows.filter_map(|r| r.ok()).collect())
}

// ─── Inventory Chart Data ─────────────────────────────────────────────────────

#[tauri::command]
pub fn get_inventory_chart_data(
    db: State<'_, Database>,
) -> Result<InventoryChartData, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Estado de vencimiento por producto activo con stock
    let expired_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM (
             SELECT p.id FROM products p
             LEFT JOIN (
                 SELECT product_id,
                        SUM(quantity) as total_stock,
                        MIN(CASE WHEN quantity > 0 AND expiry_date IS NOT NULL THEN expiry_date END) as nearest_expiry
                 FROM inventory GROUP BY product_id
             ) inv ON inv.product_id = p.id
             WHERE p.is_active = 1
               AND COALESCE(inv.total_stock, 0) > 0
               AND inv.nearest_expiry IS NOT NULL
               AND inv.nearest_expiry < DATE('now', '-4 hours')
         )",
        [], |row| row.get(0),
    ).unwrap_or(0);

    let expiring_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM (
             SELECT p.id FROM products p
             LEFT JOIN (
                 SELECT product_id,
                        SUM(quantity) as total_stock,
                        MIN(CASE WHEN quantity > 0 AND expiry_date IS NOT NULL THEN expiry_date END) as nearest_expiry
                 FROM inventory GROUP BY product_id
             ) inv ON inv.product_id = p.id
             WHERE p.is_active = 1
               AND COALESCE(inv.total_stock, 0) > 0
               AND inv.nearest_expiry IS NOT NULL
               AND inv.nearest_expiry >= DATE('now', '-4 hours')
               AND inv.nearest_expiry <= DATE('now', '-4 hours', '+4 months')
         )",
        [], |row| row.get(0),
    ).unwrap_or(0);

    let active_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM products p
         LEFT JOIN (
             SELECT product_id, SUM(quantity) as total_stock
             FROM inventory GROUP BY product_id
         ) inv ON inv.product_id = p.id
         WHERE p.is_active = 1
           AND COALESCE(inv.total_stock, 0) > 0",
        [], |row| row.get(0),
    ).unwrap_or(0) - expired_count - expiring_count;

    // Distribución por stock
    let stock_zero: i64 = conn.query_row(
        "SELECT COUNT(*) FROM products p
         LEFT JOIN (SELECT product_id, SUM(quantity) as total_qty FROM inventory GROUP BY product_id) i
         ON i.product_id = p.id
         WHERE p.is_active = 1 AND CAST(COALESCE(i.total_qty, 0) AS INTEGER) = 0",
        [], |row| row.get(0),
    ).unwrap_or(0);

    let stock_1_5: i64 = conn.query_row(
        "SELECT COUNT(*) FROM products p
         LEFT JOIN (SELECT product_id, SUM(quantity) as total_qty FROM inventory GROUP BY product_id) i
         ON i.product_id = p.id
         WHERE p.is_active = 1
           AND CAST(COALESCE(i.total_qty, 0) AS INTEGER) BETWEEN 1 AND 5",
        [], |row| row.get(0),
    ).unwrap_or(0);

    let stock_6_10: i64 = conn.query_row(
        "SELECT COUNT(*) FROM products p
         LEFT JOIN (SELECT product_id, SUM(quantity) as total_qty FROM inventory GROUP BY product_id) i
         ON i.product_id = p.id
         WHERE p.is_active = 1
           AND CAST(COALESCE(i.total_qty, 0) AS INTEGER) BETWEEN 6 AND 10",
        [], |row| row.get(0),
    ).unwrap_or(0);

    let stock_gt_10: i64 = conn.query_row(
        "SELECT COUNT(*) FROM products p
         LEFT JOIN (SELECT product_id, SUM(quantity) as total_qty FROM inventory GROUP BY product_id) i
         ON i.product_id = p.id
         WHERE p.is_active = 1
           AND CAST(COALESCE(i.total_qty, 0) AS INTEGER) > 10",
        [], |row| row.get(0),
    ).unwrap_or(0);

    Ok(InventoryChartData {
        expired_count,
        expiring_count,
        active_count: active_count.max(0),
        stock_zero,
        stock_1_5,
        stock_6_10,
        stock_gt_10,
    })
}
