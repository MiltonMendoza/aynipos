-- AyniPOS Migration V8: Product dose field
-- Adds pharmaceutical dose field to products table
-- Examples: "500mg", "10ml", "250mg/5ml", "1g"

ALTER TABLE products ADD COLUMN dose TEXT;
