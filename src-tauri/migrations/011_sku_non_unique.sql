-- Migration 011: Remove UNIQUE constraint on sku column.
-- Allows the same SKU to exist for different suppliers.
-- Uniqueness rule (sku + supplier) is enforced at application level (Rust).
--
-- Everything runs in a single implicit transaction (called from Rust via execute_batch).
-- If anything fails, SQLite rolls back automatically — data is safe.

-- 0. Limpiar tabla huérfana de intentos anteriores fallidos (si existe)
DROP TABLE IF EXISTS products_new;

-- 1. Tabla de backup (red de seguridad, queda persistida en la DB)
CREATE TABLE IF NOT EXISTS products_v9_backup AS SELECT * FROM products;

-- 2. Nueva tabla SIN UNIQUE en sku (misma estructura que la actual post-migraciones 007 y 008)
CREATE TABLE products_new (
    id             TEXT PRIMARY KEY,
    sku            TEXT NOT NULL,          -- ya no es UNIQUE (la constraint global se eliminó)
    barcode        TEXT,
    name           TEXT NOT NULL,
    description    TEXT,
    category_id    TEXT REFERENCES categories(id) ON DELETE SET NULL,
    purchase_price REAL NOT NULL DEFAULT 0,
    sale_price     REAL NOT NULL,
    tax_rate       REAL DEFAULT 0.13,
    unit           TEXT DEFAULT 'unidad',
    min_stock      INTEGER DEFAULT 0,
    is_active      INTEGER DEFAULT 1,
    metadata       TEXT,
    created_at     TEXT DEFAULT (datetime('now', '-4 hours')),
    updated_at     TEXT DEFAULT (datetime('now', '-4 hours')),
    supplier_id    TEXT REFERENCES suppliers(id) ON DELETE SET NULL,
    dose           TEXT
);

-- 3. Copiar todos los datos (activos e inactivos)
INSERT INTO products_new SELECT * FROM products;

-- 4. Eliminar tabla original
DROP TABLE products;

-- 5. Renombrar nueva tabla
ALTER TABLE products_new RENAME TO products;

-- 6. Re-crear índices (sku ya no tiene UNIQUE propio; los demás se mantienen iguales)
CREATE INDEX IF NOT EXISTS idx_products_barcode  ON products(barcode);
CREATE INDEX IF NOT EXISTS idx_products_category ON products(category_id);
CREATE INDEX IF NOT EXISTS idx_products_sku      ON products(sku);
CREATE INDEX IF NOT EXISTS idx_products_name     ON products(name);
