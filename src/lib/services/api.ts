import { invoke } from '@tauri-apps/api/core';
import type {
  ProductWithStock, CreateProduct, UpdateProduct,
  Customer, CreateCustomer,
  Sale, SaleItem, CreateSale,
  CashRegister,
  InventoryMovement,
  DashboardStats,
  Setting, Category, CreateCategory
} from '$lib/types';

// ─── Products ──────────────────────────────────────────

export async function getProducts(search?: string, categoryId?: string): Promise<ProductWithStock[]> {
  return invoke('get_products', { search, categoryId, activeOnly: true });
}

export async function getProduct(id: string): Promise<ProductWithStock> {
  return invoke('get_product', { id });
}

export async function getProductByBarcode(barcode: string): Promise<ProductWithStock | null> {
  return invoke('get_product_by_barcode', { barcode });
}

export async function createProduct(product: CreateProduct): Promise<ProductWithStock['product']> {
  return invoke('create_product', { product });
}

export async function updateProduct(product: UpdateProduct): Promise<void> {
  return invoke('update_product', { product });
}

export async function deleteProduct(id: string): Promise<void> {
  return invoke('delete_product', { id });
}

// ─── Sales ─────────────────────────────────────────────

export async function createSale(sale: CreateSale): Promise<Sale> {
  return invoke('create_sale', { sale });
}

export async function getSales(dateFrom?: string, dateTo?: string, status?: string): Promise<Sale[]> {
  return invoke('get_sales', { dateFrom, dateTo, status });
}

export async function getSaleItems(saleId: string): Promise<SaleItem[]> {
  return invoke('get_sale_items', { saleId });
}

export async function cancelSale(saleId: string): Promise<void> {
  return invoke('cancel_sale', { saleId });
}

// ─── Inventory ─────────────────────────────────────────

export async function getInventory(lowStockOnly?: boolean, expiringDays?: number): Promise<ProductWithStock[]> {
  return invoke('get_inventory', { lowStockOnly, expiringDays });
}

export async function adjustInventory(
  productId: string, quantity: number, movementType: string,
  notes?: string, lotNumber?: string, expiryDate?: string
): Promise<void> {
  return invoke('adjust_inventory', { productId, quantity, movementType, notes, lotNumber, expiryDate });
}

export async function getInventoryMovements(productId?: string, limit?: number): Promise<InventoryMovement[]> {
  return invoke('get_inventory_movements', { productId, limit });
}

// ─── Customers ─────────────────────────────────────────

export async function getCustomers(search?: string): Promise<Customer[]> {
  return invoke('get_customers', { search });
}

export async function createCustomer(customer: CreateCustomer): Promise<Customer> {
  return invoke('create_customer', { customer });
}

export async function updateCustomer(id: string, customer: CreateCustomer): Promise<void> {
  return invoke('update_customer', { id, customer });
}

export async function deleteCustomer(id: string): Promise<void> {
  return invoke('delete_customer', { id });
}

// ─── Cash Register ─────────────────────────────────────

export async function openCashRegister(openingAmount: number): Promise<CashRegister> {
  return invoke('open_cash_register', { openingAmount });
}

export async function closeCashRegister(closingAmount: number, notes?: string): Promise<CashRegister> {
  return invoke('close_cash_register', { closingAmount, notes });
}

export async function getCurrentCashRegister(): Promise<CashRegister | null> {
  return invoke('get_current_cash_register');
}

// ─── Dashboard ─────────────────────────────────────────

export async function getDashboardStats(): Promise<DashboardStats> {
  return invoke('get_dashboard_stats');
}

// ─── Settings ──────────────────────────────────────────

export async function getSettings(): Promise<Setting[]> {
  return invoke('get_settings');
}

export async function updateSetting(key: string, value: string): Promise<void> {
  return invoke('update_setting', { key, value });
}

export async function getCategories(): Promise<Category[]> {
  return invoke('get_categories');
}

export async function createCategory(category: CreateCategory): Promise<Category> {
  return invoke('create_category', { category });
}

export async function deleteCategory(id: string): Promise<void> {
  return invoke('delete_category', { id });
}

// ─── Receipts ──────────────────────────────────────────

export async function saveReceiptHtml(html: string): Promise<string> {
  return invoke('save_receipt_html', { html });
}
