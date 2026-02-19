use serde::{Deserialize, Serialize};

// ─── Product ───────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub sku: String,
    pub barcode: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub category_id: Option<String>,
    pub purchase_price: f64,
    pub sale_price: f64,
    pub tax_rate: f64,
    pub unit: String,
    pub min_stock: i32,
    pub is_active: bool,
    pub metadata: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProduct {
    pub sku: String,
    pub barcode: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub category_id: Option<String>,
    pub purchase_price: f64,
    pub sale_price: f64,
    pub tax_rate: Option<f64>,
    pub unit: Option<String>,
    pub min_stock: Option<i32>,
    pub metadata: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProduct {
    pub id: String,
    pub sku: Option<String>,
    pub barcode: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub category_id: Option<String>,
    pub purchase_price: Option<f64>,
    pub sale_price: Option<f64>,
    pub tax_rate: Option<f64>,
    pub unit: Option<String>,
    pub min_stock: Option<i32>,
    pub is_active: Option<bool>,
    pub metadata: Option<String>,
}

// ─── Category ──────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<String>,
    pub sort_order: i32,
    pub is_active: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCategory {
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<String>,
}

// ─── Customer ──────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: String,
    pub nit: Option<String>,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub is_active: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomer {
    pub nit: Option<String>,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

// ─── Inventory ─────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub id: String,
    pub product_id: String,
    pub quantity: f64,
    pub lot_number: Option<String>,
    pub expiry_date: Option<String>,
    pub cost_price: Option<f64>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryMovement {
    pub id: String,
    pub product_id: String,
    pub movement_type: String,
    pub quantity: f64,
    pub reference_id: Option<String>,
    pub notes: Option<String>,
    pub created_at: Option<String>,
}

// ─── Sales ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sale {
    pub id: String,
    pub sale_number: i64,
    pub customer_id: Option<String>,
    pub cash_register_id: Option<String>,
    pub subtotal: f64,
    pub tax_amount: f64,
    pub discount_amount: f64,
    pub total: f64,
    pub payment_method: String,
    pub payment_details: Option<String>,
    pub status: String,
    pub cufd: Option<String>,
    pub cuf: Option<String>,
    pub siat_status: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleItem {
    pub id: String,
    pub sale_id: String,
    pub product_id: String,
    pub product_name: String,
    pub quantity: f64,
    pub unit_price: f64,
    pub discount: f64,
    pub tax_rate: f64,
    pub subtotal: f64,
    pub total: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSale {
    pub customer_id: Option<String>,
    pub items: Vec<CreateSaleItem>,
    pub payment_method: String,
    pub payment_details: Option<String>,
    pub discount_amount: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSaleItem {
    pub product_id: String,
    pub quantity: f64,
    pub discount: Option<f64>,
}

// ─── Cash Register ─────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashRegister {
    pub id: String,
    pub opened_at: String,
    pub closed_at: Option<String>,
    pub opening_amount: f64,
    pub closing_amount: Option<f64>,
    pub expected_amount: Option<f64>,
    pub notes: Option<String>,
    pub user_id: Option<String>,
}

// ─── Settings ──────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    pub key: String,
    pub value: String,
}

// ─── Dashboard Stats ───────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_sales_today: f64,
    pub total_transactions_today: i64,
    pub total_products: i64,
    pub low_stock_count: i64,
    pub expiring_soon_count: i64,
}

// ─── Product with stock info ───────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductWithStock {
    pub product: Product,
    pub current_stock: f64,
    pub category_name: Option<String>,
}
