import { invoke } from '@tauri-apps/api/core';
import type {
  ProductWithStock, CreateProduct, UpdateProduct,
  Customer, CreateCustomer,
  Sale, SaleItem, CreateSale,
  CashRegister, CashRegisterReport, ExpectedClosingInfo,
  InventoryMovement, InventoryLot,
  DashboardStats, TopSellingProduct, SalesChartDataPoint, ProfitMarginProduct, InventoryReportItem,
  Setting, Category, CreateCategory,
  ImportResult,
  User, CreateUser, UpdateUser,
  AuditLogEntry,
  BackupResult, BackupInfo,
  LicenseStatus
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

// ─── Import/Export ─────────────────────────────────────

export async function exportProductsCsv(filePath: string): Promise<number> {
  return invoke('export_products_csv', { filePath });
}

export async function importProductsCsv(filePath: string): Promise<ImportResult> {
  return invoke('import_products_csv', { filePath });
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

export async function getProductLots(productId: string): Promise<InventoryLot[]> {
  return invoke('get_product_lots', { productId });
}

export async function deleteLot(lotId: string): Promise<void> {
  return invoke('delete_lot', { lotId });
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

export async function openCashRegister(openingAmount: number, userId: string): Promise<CashRegister> {
  return invoke('open_cash_register', { openingAmount, userId });
}

export async function closeCashRegister(closingAmount: number, notes?: string): Promise<CashRegister> {
  return invoke('close_cash_register', { closingAmount, notes });
}

export async function getCurrentCashRegister(): Promise<CashRegister | null> {
  return invoke('get_current_cash_register');
}

export async function getCashRegisterReport(registerId: string): Promise<CashRegisterReport> {
  return invoke('get_cash_register_report', { registerId });
}

export async function getCashRegisterHistory(userId?: string, limit?: number): Promise<CashRegisterReport[]> {
  return invoke('get_cash_register_history', { userId, limit });
}

export async function getExpectedClosingAmount(): Promise<ExpectedClosingInfo> {
  return invoke('get_expected_closing_amount');
}

// ─── Dashboard ─────────────────────────────────────────

export async function getDashboardStats(): Promise<DashboardStats> {
  return invoke('get_dashboard_stats');
}

export async function getTopSellingProducts(
  dateFrom?: string, dateTo?: string, limit?: number
): Promise<TopSellingProduct[]> {
  return invoke('get_top_selling_products', { dateFrom, dateTo, limit });
}

export async function getSalesChartData(
  dateFrom?: string, dateTo?: string, groupBy?: string
): Promise<SalesChartDataPoint[]> {
  return invoke('get_sales_chart_data', { dateFrom, dateTo, groupBy });
}

export async function getProfitMarginReport(
  dateFrom?: string, dateTo?: string
): Promise<ProfitMarginProduct[]> {
  return invoke('get_profit_margin_report', { dateFrom, dateTo });
}

export async function getInventoryReport(inactiveDays?: number): Promise<InventoryReportItem[]> {
  return invoke('get_inventory_report', { inactiveDays });
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

// ─── Reports Export ────────────────────────────────────

export async function saveReportCsv(content: string, filePath: string): Promise<void> {
  return invoke('save_report_csv', { content, filePath });
}

export async function saveReportHtml(html: string): Promise<string> {
  return invoke('save_report_html', { html });
}

// ─── Users ─────────────────────────────────────────────

export async function getUsers(): Promise<User[]> {
  return invoke('get_users');
}

export async function createUser(user: CreateUser): Promise<User> {
  return invoke('create_user', { user });
}

export async function updateUser(user: UpdateUser): Promise<void> {
  return invoke('update_user', { user });
}

export async function deleteUser(id: string): Promise<void> {
  return invoke('delete_user', { id });
}

export async function loginWithPin(pin: string): Promise<User> {
  return invoke('login_with_pin', { pin });
}

// ─── Audit Log ─────────────────────────────────────────

export async function logAction(
  userId: string, userName: string, action: string,
  entityType?: string, entityId?: string, details?: string
): Promise<void> {
  return invoke('log_action', { userId, userName, action, entityType, entityId, details });
}

export async function getAuditLog(
  userId?: string, action?: string,
  dateFrom?: string, dateTo?: string, limit?: number
): Promise<AuditLogEntry[]> {
  return invoke('get_audit_log', { userId, action, dateFrom, dateTo, limit });
}

// ─── Backup ────────────────────────────────────────────

export async function createBackup(backupPath: string): Promise<BackupResult> {
  return invoke('create_backup', { backupPath });
}

export async function getBackupInfo(backupPath: string): Promise<BackupInfo> {
  return invoke('get_backup_info', { backupPath });
}

// ─── License ───────────────────────────────────────────

export async function getMachineId(): Promise<string> {
  return invoke('get_machine_id');
}

export async function getLicenseStatus(): Promise<LicenseStatus> {
  return invoke('get_license_status');
}

export async function activateLicense(key: string): Promise<LicenseStatus> {
  return invoke('activate_license', { key });
}

export async function deactivateLicense(): Promise<void> {
  return invoke('deactivate_license');
}
