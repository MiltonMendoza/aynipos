-- AyniPOS Initial Schema
-- SQLite with multi-tenant preparation

PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = ON;

-- Global settings (key-value store)
CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT DEFAULT (datetime('now'))
);

-- Product categories (hierarchical)
CREATE TABLE IF NOT EXISTS categories (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    parent_id TEXT REFERENCES categories(id) ON DELETE SET NULL,
    sort_order INTEGER DEFAULT 0,
    is_active INTEGER DEFAULT 1,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

-- Products
CREATE TABLE IF NOT EXISTS products (
    id TEXT PRIMARY KEY,
    sku TEXT UNIQUE NOT NULL,
    barcode TEXT,
    name TEXT NOT NULL,
    description TEXT,
    category_id TEXT REFERENCES categories(id) ON DELETE SET NULL,
    purchase_price REAL NOT NULL DEFAULT 0,
    sale_price REAL NOT NULL,
    tax_rate REAL DEFAULT 0.13,
    unit TEXT DEFAULT 'unidad',
    min_stock INTEGER DEFAULT 0,
    is_active INTEGER DEFAULT 1,
    metadata TEXT,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

-- Inventory / Stock
CREATE TABLE IF NOT EXISTS inventory (
    id TEXT PRIMARY KEY,
    product_id TEXT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    quantity REAL NOT NULL DEFAULT 0,
    lot_number TEXT,
    expiry_date TEXT,
    cost_price REAL,
    updated_at TEXT DEFAULT (datetime('now'))
);

-- Customers
CREATE TABLE IF NOT EXISTS customers (
    id TEXT PRIMARY KEY,
    nit TEXT,
    name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    address TEXT,
    is_active INTEGER DEFAULT 1,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

-- Cash registers (sessions)
CREATE TABLE IF NOT EXISTS cash_registers (
    id TEXT PRIMARY KEY,
    opened_at TEXT NOT NULL,
    closed_at TEXT,
    opening_amount REAL NOT NULL,
    closing_amount REAL,
    expected_amount REAL,
    notes TEXT,
    user_id TEXT
);

-- Sales
CREATE TABLE IF NOT EXISTS sales (
    id TEXT PRIMARY KEY,
    sale_number INTEGER NOT NULL,
    customer_id TEXT REFERENCES customers(id) ON DELETE SET NULL,
    cash_register_id TEXT REFERENCES cash_registers(id),
    subtotal REAL NOT NULL,
    tax_amount REAL NOT NULL,
    discount_amount REAL DEFAULT 0,
    total REAL NOT NULL,
    payment_method TEXT NOT NULL,
    payment_details TEXT,
    status TEXT DEFAULT 'completed',
    -- SIAT fields (Phase 3)
    cufd TEXT,
    cuf TEXT,
    siat_status TEXT DEFAULT 'pending',
    invoice_xml TEXT,
    created_at TEXT DEFAULT (datetime('now'))
);

-- Sale line items
CREATE TABLE IF NOT EXISTS sale_items (
    id TEXT PRIMARY KEY,
    sale_id TEXT NOT NULL REFERENCES sales(id) ON DELETE CASCADE,
    product_id TEXT NOT NULL REFERENCES products(id),
    product_name TEXT NOT NULL,
    quantity REAL NOT NULL,
    unit_price REAL NOT NULL,
    discount REAL DEFAULT 0,
    tax_rate REAL NOT NULL,
    subtotal REAL NOT NULL,
    total REAL NOT NULL
);

-- Inventory movements (audit trail)
CREATE TABLE IF NOT EXISTS inventory_movements (
    id TEXT PRIMARY KEY,
    product_id TEXT NOT NULL REFERENCES products(id),
    movement_type TEXT NOT NULL,
    quantity REAL NOT NULL,
    reference_id TEXT,
    notes TEXT,
    created_at TEXT DEFAULT (datetime('now'))
);

-- SIAT configuration (Phase 3)
CREATE TABLE IF NOT EXISTS siat_config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    expires_at TEXT,
    updated_at TEXT DEFAULT (datetime('now'))
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_products_barcode ON products(barcode);
CREATE INDEX IF NOT EXISTS idx_products_category ON products(category_id);
CREATE INDEX IF NOT EXISTS idx_products_sku ON products(sku);
CREATE INDEX IF NOT EXISTS idx_products_name ON products(name);
CREATE INDEX IF NOT EXISTS idx_sales_date ON sales(created_at);
CREATE INDEX IF NOT EXISTS idx_sales_customer ON sales(customer_id);
CREATE INDEX IF NOT EXISTS idx_sales_status ON sales(status);
CREATE INDEX IF NOT EXISTS idx_sale_items_sale ON sale_items(sale_id);
CREATE INDEX IF NOT EXISTS idx_inventory_product ON inventory(product_id);
CREATE INDEX IF NOT EXISTS idx_inventory_expiry ON inventory(expiry_date);
CREATE INDEX IF NOT EXISTS idx_movements_product ON inventory_movements(product_id);
CREATE INDEX IF NOT EXISTS idx_customers_nit ON customers(nit);

-- Seed default settings
INSERT OR IGNORE INTO settings (key, value) VALUES
    ('business_name', 'Mi Farmacia'),
    ('business_nit', ''),
    ('business_address', ''),
    ('business_phone', ''),
    ('business_city', ''),
    ('tax_rate', '0.13'),
    ('currency', 'BOB'),
    ('currency_symbol', 'Bs'),
    ('sale_number_sequence', '0'),
    ('siat_environment', 'pilot');

-- Seed default consumer (sin nombre)
INSERT OR IGNORE INTO customers (id, nit, name) VALUES
    ('default-consumer', '0', 'Sin Nombre');
