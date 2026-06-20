import type { User, AppRoute } from '$lib/types';

// ─── Permissions ───────────────────────────────────────

export type Permission =
  | 'pos'
  | 'view_sales'
  | 'cancel_sales'
  | 'view_inventory'
  | 'adjust_inventory'
  | 'manage_products'
  | 'import_export_products'
  | 'view_customers'
  | 'view_reports_sales'
  | 'view_reports_inventory'
  | 'manage_settings'
  | 'manage_users'
  | 'manage_cash_register'
  | 'view_audit_log'
  | 'manage_suppliers';

const ROLE_PERMISSIONS: Record<string, Permission[]> = {
  admin: [
    'pos',
    'view_sales',
    'cancel_sales',
    'view_inventory',
    'adjust_inventory',
    'manage_products',
    'import_export_products',
    'view_customers',
    'view_reports_sales',
    'view_reports_inventory',
    'manage_settings',
    'manage_users',
    'manage_cash_register',
    'view_audit_log',
    'manage_suppliers',
  ],
  cashier: [
    'pos',
    'view_sales',
    'view_customers',
    'view_reports_sales',
    'manage_cash_register',
  ],
  inventory: [
    'view_inventory',
    'adjust_inventory',
    'manage_products',
    'import_export_products',
    'view_reports_inventory',
    'manage_suppliers',
  ],
};

// ─── Route → Permission mapping ───────────────────────

const ROUTE_PERMISSIONS: Record<AppRoute, Permission[]> = {
  pos: ['pos'],
  sales: ['view_sales'],
  inventory: ['view_inventory'],
  customers: ['view_customers'],
  suppliers: ['manage_suppliers'],
  reports: ['view_reports_sales', 'view_reports_inventory'],
  settings: ['manage_settings', 'manage_cash_register'],
  migration: ['manage_settings'], // solo admin
};

// ─── Public API ────────────────────────────────────────

export function hasPermission(user: User | null, permission: Permission): boolean {
  if (!user) return false;
  const permissions = ROLE_PERMISSIONS[user.role];
  if (!permissions) return false;
  return permissions.includes(permission);
}

export function hasAnyPermission(user: User | null, permissions: Permission[]): boolean {
  return permissions.some((p) => hasPermission(user, p));
}

export function canAccessRoute(user: User | null, route: AppRoute): boolean {
  if (!user) return false;
  const requiredPermissions = ROUTE_PERMISSIONS[route];
  if (!requiredPermissions) return false;
  // User needs at least one of the required permissions
  return hasAnyPermission(user, requiredPermissions);
}

export function getAllowedRoutes(user: User | null): AppRoute[] {
  if (!user) return [];
  const allRoutes: AppRoute[] = ['pos', 'sales', 'inventory', 'customers', 'suppliers', 'reports', 'settings'];
  return allRoutes.filter((route) => canAccessRoute(user, route));
}

export function getDefaultRoute(user: User | null): AppRoute {
  const allowed = getAllowedRoutes(user);
  return allowed[0] || 'pos';
}

export function getRoleLabel(role: string): string {
  switch (role) {
    case 'admin': return 'Administrador';
    case 'cashier': return 'Cajero';
    case 'inventory': return 'Inventarista';
    default: return role;
  }
}

export function getRoleIcon(role: string): string {
  switch (role) {
    case 'admin': return '👑';
    case 'cashier': return '👤';
    case 'inventory': return '📦';
    default: return '👤';
  }
}
