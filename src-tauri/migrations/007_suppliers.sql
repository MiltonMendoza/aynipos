-- AyniPOS Migration V7: Suppliers module
-- Creates the suppliers table and links products to suppliers

-- Tabla de proveedores
CREATE TABLE IF NOT EXISTS suppliers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    contact_name TEXT,
    phone TEXT,
    email TEXT,
    address TEXT,
    notes TEXT,
    is_active INTEGER DEFAULT 1,
    created_at TEXT DEFAULT (datetime('now', '-4 hours')),
    updated_at TEXT DEFAULT (datetime('now', '-4 hours'))
);

-- FK en products (ALTER TABLE compatible con SQLite)
ALTER TABLE products ADD COLUMN supplier_id TEXT REFERENCES suppliers(id) ON DELETE SET NULL;

-- Índices de búsqueda
CREATE INDEX IF NOT EXISTS idx_suppliers_name ON suppliers(name);
CREATE INDEX IF NOT EXISTS idx_products_supplier ON products(supplier_id);
