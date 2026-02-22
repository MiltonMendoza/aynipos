-- Add lot tracking to inventory movements
ALTER TABLE inventory_movements ADD COLUMN lot_number TEXT;
ALTER TABLE inventory_movements ADD COLUMN expiry_date TEXT;
