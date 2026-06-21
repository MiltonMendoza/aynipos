-- Migration 009: Add user_id column to sales table
-- Links each sale to the user (cashier/admin) who processed it

ALTER TABLE sales ADD COLUMN user_id TEXT REFERENCES users(id);

-- Index for faster filtering by user
CREATE INDEX IF NOT EXISTS idx_sales_user_id ON sales(user_id);
