# AyniPOS — Arquitectura del Proyecto

> Documento de referencia para mantener contexto entre sesiones de desarrollo.
> Última actualización: 2026-02-18

## Stack Tecnológico

| Capa | Tecnología | Versión |
|------|-----------|---------|
| Backend | Rust + Tauri v2 | tauri 2.x |
| Frontend | Svelte 5 + SvelteKit | svelte 5.x |
| Base de datos | SQLite (rusqlite, bundled) | - |
| Build | Vite | 6.x |
| Adapter | @sveltejs/adapter-static (SPA) | - |

## Estructura del Proyecto

```
aynipos/
├── src-tauri/                    # Backend Rust
│   ├── Cargo.toml                # Dependencias Rust
│   ├── tauri.conf.json           # Config de Tauri (ventana 1280x800, min 1024x700)
│   ├── migrations/
│   │   └── 001_initial.sql       # Schema SQLite (10 tablas)
│   └── src/
│       ├── lib.rs                # Entry point: init DB + registra 25 comandos
│       ├── db/
│       │   ├── mod.rs            # Database struct con Mutex<Connection>, WAL mode
│       │   ├── schema.rs         # Migration runner (include_str!, versioned)
│       │   └── models.rs         # Data models: Product, Sale, Customer, etc.
│       └── commands/
│           ├── mod.rs            # Re-exports de módulos
│           ├── products.rs       # CRUD productos (search, create, update, soft-delete)
│           ├── sales.rs          # Crear venta (stock deduction), listar, cancelar (stock restore)
│           ├── inventory.rs      # Listar (filtros low-stock/expiry), ajustar, movimientos
│           ├── customers.rs      # CRUD clientes (search por nombre/NIT/teléfono)
│           ├── cash_register.rs  # Abrir/cerrar caja (reconciliación monto esperado vs real)
│           ├── dashboard.rs      # Stats: ventas hoy, transacciones, bajo stock, por vencer
│           └── settings.rs       # Get/update settings (key-value), CRUD categorías
│
├── src/                          # Frontend Svelte
│   ├── app.html                  # Template HTML (lang="es")
│   ├── app.css                   # Design system completo (dark theme, Inter font)
│   ├── lib/
│   │   ├── types/index.ts        # TypeScript types (mirror de Rust models)
│   │   └── services/api.ts       # Wrappers tipados para invoke() de Tauri
│   └── routes/
│       ├── +page.svelte          # Layout principal: sidebar + routing por componente
│       ├── pos/PosPage.svelte    # POS: grilla productos, carrito, modal de pago
│       ├── sales/SalesPage.svelte        # Historial de ventas + detalle
│       ├── inventory/InventoryPage.svelte # Tabla inventario + crear producto/categoría
│       ├── customers/CustomersPage.svelte # CRUD clientes
│       ├── reports/ReportsPage.svelte     # Dashboard stats + últimas ventas
│       └── settings/SettingsPage.svelte   # Datos negocio + caja registradora
│
├── ROADMAP.md                    # Plan de mejoras por fases (v0.2 → v1.0)
└── ARCHITECTURE.md               # Este archivo
```

## Patrones y Convenciones

### Backend (Rust)

1. **Comandos Tauri**: Cada función es `#[tauri::command]` y recibe `State<Database>` como primer param.
2. **Base de datos**: Se usa `Database` struct con `conn: Mutex<Connection>`. Cada comando hace `let conn = db.conn.lock().unwrap();`.
3. **Migrations**: Versionadas con tabla `_migrations`. SQL embebido con `include_str!`. Se ejecutan al inicio.
4. **IDs**: Se generan con `uuid::Uuid::new_v4().to_string()`.
5. **Soft delete**: Productos y clientes usan `is_active = 0` en lugar de DELETE.
6. **Errores**: Los comandos retornan `Result<T, String>` con `.map_err(|e| e.to_string())`.
7. **Fechas**: Se almacenan como TEXT en formato ISO 8601 (`datetime('now')`).
8. **Moneda**: `REAL` en SQLite, `f64` en Rust. Bolivia usa BOB con IVA 13%.

### Frontend (Svelte 5)

1. **State**: Se usa `$state()` (Svelte 5 runes), NO stores legacy.
2. **Props**: Se usa `$props()` para recibir props en componentes.
3. **Effects**: Se usa `$effect()` para reactividad.
4. **Routing**: NO usa SvelteKit routes. Es routing manual por componente en `+page.svelte` con lazy imports (`#await import(...)`).
5. **API calls**: Siempre a través de `$lib/services/api.ts`, nunca `invoke()` directo.
6. **CSS**: Sistema global en `app.css`. Componentes usan las clases del design system (`.btn`, `.card`, `.input`, `.modal-overlay`, etc.).
7. **Idioma UI**: Todo en **español** (labels, placeholders, mensajes).
8. **Moneda**: Se formatea como `Bs {monto.toFixed(2)}`.

### Design System (app.css)

- **Tema**: Dark mode con fondo `#0f1117`, textos claros
- **Font**: Inter (Google Fonts)
- **Colores accent**: Blue `#3b82f6`, Green `#10b981`, Yellow `#f59e0b`, Red `#ef4444`, Purple `#8b5cf6`
- **Componentes**: `.btn`, `.btn-primary/success/danger/ghost`, `.card`, `.stat-card`, `.input`, `.select`, `.table-container`, `.modal-overlay`, `.modal`, `.badge`, `.toast`
- **Variables CSS**: `--bg-*`, `--text-*`, `--accent-*`, `--border-*`, `--space-*`, `--radius-*`, `--font-size-*`

## Base de Datos — Tablas Principales

| Tabla | Propósito |
|-------|-----------|
| `settings` | Key-value para config del negocio (nombre, NIT, IVA, moneda) |
| `categories` | Categorías de productos |
| `products` | Catálogo de productos (SKU, precios, impuesto, soft-delete) |
| `inventory` | Stock por producto (cantidad, lote, vencimiento) |
| `customers` | Clientes (nombre, NIT, teléfono, email, soft-delete) |
| `cash_registers` | Sesiones de caja (apertura, cierre, montos) |
| `sales` | Ventas (total, impuesto, descuento, método de pago, estado) |
| `sale_items` | Líneas de venta (producto, cantidad, precio unitario) |
| `inventory_movements` | Auditoría de movimientos de stock (venta, compra, ajuste, devolución) |
| `siat_config` | Config para facturación electrónica SIAT (futuro) |

## Dependencias Rust Clave

```toml
tauri = { version = "2", features = ["devtools"] }
rusqlite = { version = "0.32", features = ["bundled"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
rust_decimal = { version = "1", features = ["serde-str"] }
tokio = { version = "1", features = ["full"] }
log = "0.4"
env_logger = "0.11"
```

## Cómo Ejecutar

```bash
# Desarrollo (frontend + backend hot reload)
cargo tauri dev

# Build producción
cargo tauri build

# Solo check del backend
cd src-tauri && cargo check

# Solo check del frontend
npx svelte-check
```

## Decisiones de Diseño Importantes

1. **SQLite bundled**: No requiere instalar SQLite en el sistema del usuario. Se compila dentro del binario.
2. **WAL mode**: Mejor performance para lecturas concurrentes.
3. **Routing manual**: No usamos rutas de SvelteKit porque Tauri sirve como SPA. La navigación es por estado en `+page.svelte`.
4. **Facturación Computarizada en Línea**: Modalidad elegida para SIAT (no requiere firma digital ADSIB, usa hash MD5).
5. **PDF para recibos**: En vez de ESC/POS directo, generamos PDF y usamos el diálogo de impresión del sistema.
6. **Preparado para multi-tenant**: Los campos `tenant_id` y `branch_id` existen en el schema pero no se usan aún.
