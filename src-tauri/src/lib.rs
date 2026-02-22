mod db;
mod commands;

use db::Database;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // Initialize database in app data directory
            let app_data_dir = app.path().app_data_dir()
                .expect("Failed to get app data directory");

            let database = Database::new(app_data_dir)
                .expect("Failed to initialize database");

            app.manage(database);

            log::info!("AyniPOS initialized successfully");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Products
            commands::products::get_products,
            commands::products::get_product,
            commands::products::create_product,
            commands::products::update_product,
            commands::products::delete_product,
            commands::products::get_product_by_barcode,
            // Sales
            commands::sales::create_sale,
            commands::sales::get_sales,
            commands::sales::get_sale_items,
            commands::sales::cancel_sale,
            // Inventory
            commands::inventory::get_inventory,
            commands::inventory::adjust_inventory,
            commands::inventory::get_inventory_movements,
            commands::inventory::get_product_lots,
            commands::inventory::delete_lot,
            // Customers
            commands::customers::get_customers,
            commands::customers::create_customer,
            commands::customers::update_customer,
            commands::customers::delete_customer,
            // Cash Register
            commands::cash_register::open_cash_register,
            commands::cash_register::close_cash_register,
            commands::cash_register::get_current_cash_register,
            commands::cash_register::get_cash_register_report,
            // Dashboard
            commands::dashboard::get_dashboard_stats,
            // Settings
            commands::settings::get_settings,
            commands::settings::update_setting,
            commands::settings::get_categories,
            commands::settings::create_category,
            commands::settings::delete_category,
            // Receipts
            commands::receipts::save_receipt_html,
            // Import/Export
            commands::import_export::export_products_csv,
            commands::import_export::import_products_csv,
        ])
        .run(tauri::generate_context!())
        .expect("error while running AyniPOS");
}
