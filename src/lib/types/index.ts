// ─── Product ───────────────────────────────────────────

export interface Product {
  id: string;
  sku: string;
  barcode: string | null;
  name: string;
  description: string | null;
  category_id: string | null;
  purchase_price: number;
  sale_price: number;
  tax_rate: number;
  unit: string;
  min_stock: number;
  is_active: boolean;
  metadata: string | null;
  created_at: string | null;
  updated_at: string | null;
}

export interface CreateProduct {
  sku: string;
  barcode?: string;
  name: string;
  description?: string;
  category_id?: string;
  purchase_price: number;
  sale_price: number;
  tax_rate?: number;
  unit?: string;
  min_stock?: number;
  metadata?: string;
}

export interface UpdateProduct {
  id: string;
  sku?: string;
  barcode?: string;
  name?: string;
  description?: string;
  category_id?: string;
  purchase_price?: number;
  sale_price?: number;
  tax_rate?: number;
  unit?: string;
  min_stock?: number;
  is_active?: boolean;
  metadata?: string;
}

export interface ProductWithStock {
  product: Product;
  current_stock: number;
  category_name: string | null;
}

// ─── Category ──────────────────────────────────────────

export interface Category {
  id: string;
  name: string;
  description: string | null;
  parent_id: string | null;
  sort_order: number;
  is_active: boolean;
  created_at: string | null;
  updated_at: string | null;
}

export interface CreateCategory {
  name: string;
  description?: string;
  parent_id?: string;
}

// ─── Customer ──────────────────────────────────────────

export interface Customer {
  id: string;
  nit: string | null;
  name: string;
  email: string | null;
  phone: string | null;
  address: string | null;
  is_active: boolean;
  created_at: string | null;
  updated_at: string | null;
}

export interface CreateCustomer {
  nit?: string;
  name: string;
  email?: string;
  phone?: string;
  address?: string;
}

// ─── Sales ─────────────────────────────────────────────

export interface Sale {
  id: string;
  sale_number: number;
  customer_id: string | null;
  customer_name: string | null;
  cash_register_id: string | null;
  subtotal: number;
  tax_amount: number;
  discount_amount: number;
  total: number;
  payment_method: string;
  payment_details: string | null;
  notes: string | null;
  status: string;
  cufd: string | null;
  cuf: string | null;
  siat_status: string | null;
  created_at: string | null;
}

export interface SaleItem {
  id: string;
  sale_id: string;
  product_id: string;
  product_name: string;
  quantity: number;
  unit_price: number;
  discount: number;
  tax_rate: number;
  subtotal: number;
  total: number;
}

export interface CreateSale {
  customer_id?: string;
  items: CreateSaleItem[];
  payment_method: string;
  payment_details?: string;
  discount_amount?: number;
  notes?: string;
}

export interface CreateSaleItem {
  product_id: string;
  quantity: number;
  discount?: number;
}

// ─── Cash Register ─────────────────────────────────────

export interface CashRegister {
  id: string;
  opened_at: string;
  closed_at: string | null;
  opening_amount: number;
  closing_amount: number | null;
  expected_amount: number | null;
  notes: string | null;
  user_id: string | null;
}

export interface CashRegisterReport {
  register: CashRegister;
  total_sales: number;
  total_transactions: number;
  cancelled_transactions: number;
  total_discount: number;
  total_tax: number;
  sales_cash: number;
  sales_card: number;
  sales_qr: number;
  sales_mixed: number;
  count_cash: number;
  count_card: number;
  count_qr: number;
  count_mixed: number;
  difference: number;
}

// ─── Inventory ─────────────────────────────────────────

export interface InventoryMovement {
  id: string;
  product_id: string;
  product_name: string;
  movement_type: string;
  quantity: number;
  reference_id: string | null;
  notes: string | null;
  lot_number: string | null;
  expiry_date: string | null;
  created_at: string | null;
}

export interface InventoryLot {
  id: string;
  product_id: string;
  quantity: number;
  lot_number: string | null;
  expiry_date: string | null;
  cost_price: number | null;
  expiry_status: string; // "ok" | "warning" | "danger" | "expired"
  updated_at: string | null;
}

// ─── Dashboard ─────────────────────────────────────────

export interface DashboardStats {
  total_sales_today: number;
  total_transactions_today: number;
  total_products: number;
  low_stock_count: number;
  expiring_soon_count: number;
}

export interface TopSellingProduct {
  product_id: string;
  product_name: string;
  total_quantity: number;
  total_revenue: number;
}

export interface SalesChartDataPoint {
  label: string;
  total_sales: number;
  transaction_count: number;
}

export interface ProfitMarginProduct {
  product_id: string;
  product_name: string;
  purchase_price: number;
  avg_sale_price: number;
  total_quantity: number;
  total_revenue: number;
  total_cost: number;
  gross_profit: number;
  margin_percent: number;
}

export interface InventoryReportItem {
  product_id: string;
  product_name: string;
  sku: string;
  category_name: string | null;
  current_stock: number;
  purchase_price: number;
  sale_price: number;
  stock_cost_value: number;
  stock_sale_value: number;
  last_movement_date: string | null;
  days_without_movement: number | null;
}

// ─── Settings ──────────────────────────────────────────

export interface Setting {
  key: string;
  value: string;
}

// ─── Import/Export ─────────────────────────────────────

export interface ImportResult {
  created: number;
  updated: number;
  errors: ImportError[];
}

export interface ImportError {
  row: number;
  message: string;
}

// ─── Cart (Frontend only) ──────────────────────────────

export interface CartItem {
  product: ProductWithStock;
  quantity: number;
  discount: number;
  subtotal: number;
}

// ─── Users ─────────────────────────────────────────────

export interface User {
  id: string;
  name: string;
  role: string;
  is_active: boolean;
  created_at: string | null;
  updated_at: string | null;
}

export interface CreateUser {
  name: string;
  pin: string;
  role?: string;
}

export interface UpdateUser {
  id: string;
  name?: string;
  pin?: string;
  role?: string;
  is_active?: boolean;
}

// ─── Navigation ────────────────────────────────────────

export type AppRoute = 'pos' | 'inventory' | 'customers' | 'sales' | 'reports' | 'settings';
