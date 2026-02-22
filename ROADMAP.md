# 🗺️ AyniPOS — Roadmap de Mejoras por Fases

> Plan de desarrollo incremental para completar el sistema de punto de venta.
> Cada fase es una versión funcional que agrega valor sin romper lo anterior.

---

## 📌 Resumen de Fases

| Fase | Versión | Nombre | Prioridad |
|------|---------|--------|-----------|
| 1 | v0.2 | Pulido del Core POS | 🔴 Alta — impacto diario |
| 2 | v0.3 | Gestión Completa de Inventario | 🔴 Alta |
| 3 | v0.4 | Reportes y Análisis | 🟡 Media |
| 4 | v0.5 | Facturación Electrónica SIAT | 🔴 Alta — cumplimiento legal |
| 5 | v0.6 | Multi-usuario y Seguridad | 🟡 Media |
| 6 | v0.7 | UI/UX Premium | 🟢 Baja |
| 7 | v1.0 | Escalabilidad Multi-sucursal | 🟢 Futuro |

---

## Fase 1: Pulido del Core POS (v0.2)

> _Hacer que la experiencia de venta diaria sea fluida y confiable._

| # | Feature | Descripción | Estado |
|---|---------|-------------|--------|
| 1.1 | **Atajos de teclado** | `F1` = buscar producto, `F2` = cobrar, `Esc` = cerrar modal, `+/-` = cantidad. Esencial para velocidad en caja. | ✅ Completado |
| 1.2 | **Lector de código de barras** | Input auto-focus que detecte escaneo rápido (caracteres en < 50ms) y agregue al carrito automáticamente. | ✅ Completado |
| 1.3 | **Descuentos por ítem y globales** | Descuento porcentual o fijo por producto individual, y descuento general aplicado al total de la venta. | ✅ Completado |
| 1.4 | **Cliente en la venta** | Asociar un cliente (NIT/CI) a la venta desde el POS, con buscador rápido inline. Necesario para facturación. | ✅ Completado |
| 1.5 | **Notas en la venta** | Campo de observaciones/notas en cada venta para referencia del cajero. | ✅ Completado |
| 1.6 | **Impresión de recibo** | Generar PDF del recibo/comprobante con datos del negocio y abrir diálogo de impresión del sistema. | ✅ Completado |
| 1.7 | **Sonidos y feedback visual** | Sonido sutil al agregar producto, vibración visual al error, animación de confeti/check en venta completada. | ✅ Completado |
| 1.8 | **Validaciones robustas** | No vender si stock = 0, alertar si caja cerrada, confirmar cantidades inusualmente grandes, validar precios > 0. | ✅ Completado |
| 1.9 | **Errores inline en formularios** | Mostrar mensajes de error debajo de cada campo inválido (ej: "SKU es requerido", "El precio debe ser mayor a 0"). No solo deshabilitar el botón, sino indicar claramente qué campo falta o es incorrecto. Aplica a: crear producto, crear cliente, ajustar inventario, abrir/cerrar caja. | ✅ Completado |

### Criterios de completitud Fase 1:
- [ ] Un cajero puede completar una venta completa usando solo el teclado
- [ ] El escáner de código de barras funciona sin configuración adicional
- [ ] Se puede imprimir un recibo después de cada venta
- [ ] El cliente queda asociado a la venta para futuras facturas

---

## Fase 2: Gestión Completa de Inventario (v0.3)

> _Control total del stock y productos._

| # | Feature | Descripción | Estado |
|---|---------|-------------|--------|
| 2.1 | **Editar producto** | Modal completo para editar nombre, precios, categoría, stock mínimo, código de barras. | ✅ Completado |
| 2.2 | **Importar/exportar productos** | Import desde CSV/Excel para carga masiva de catálogo. Export del inventario actual. | ✅ Completado |
| 2.3 | **Control de lotes y vencimiento** | Asignar número de lote y fecha de vencimiento por cada entrada de inventario. Alertas visuales por colores. | ✅ Completado |
| 2.4 | **Alertas automáticas** | Notificaciones en la app cuando un producto está bajo stock mínimo o próximo a vencer (7, 15, 30 días). | ⬜ Pendiente |
| 2.5 | **Historial de movimientos por producto** | Vista detallada de todas las entradas/salidas/ajustes por producto con fechas y responsable. | ✅ Completado |
| 2.6 | **Múltiples unidades de medida** | Venta por caja, blíster, unidad con conversión automática (ej: 1 caja = 10 blísteres = 100 unidades). | ⬜ Pendiente |
| 2.7 | **Fotos de productos** | Capturar con cámara o subir imagen del producto para identificación visual rápida en el POS. | ⬜ Pendiente |

### Criterios de completitud Fase 2:
- [ ] Se puede cargar un catálogo de 500+ productos desde CSV
- [ ] Los lotes con fecha de vencimiento aparecen con semáforo (verde/amarillo/rojo)
- [ ] El historial de movimientos muestra trazabilidad completa

---

## Fase 3: Reportes y Análisis (v0.4)

> _Tomar decisiones basadas en datos._

| # | Feature | Descripción | Estado |
|---|---------|-------------|--------|
| 3.1 | **Ventas por rango de fechas** | Filtro de fecha desde/hasta en historial de ventas. Selector de rango con presets (hoy, esta semana, este mes). | ✅ Completado |
| 3.2 | **Reporte de cierre de caja** | PDF con resumen: ventas por método de pago (efectivo/tarjeta/QR), totales, cantidad de transacciones, diferencias. | ✅ Completado |
| 3.3 | **Productos más vendidos** | Top 10/20 productos por cantidad vendida y por monto generado, con filtro de período. | ✅ Completado |
| 3.4 | **Gráficos de ventas** | Charts de ventas diarias/semanales/mensuales con barras y líneas de tendencia. | ✅ Completado |
| 3.5 | **Margen de ganancia** | Reporte comparando precio de compra vs precio de venta. Utilidad bruta por producto y global. | ✅ Completado |
| 3.6 | **Reporte de inventario** | Valorización del stock actual (costo y venta), productos sin movimiento en X días. | ⬜ Pendiente |
| 3.7 | **Exportar a Excel/PDF** | Botón para descargar cualquier reporte en formato Excel (.xlsx) o PDF. | ⬜ Pendiente |

### Criterios de completitud Fase 3:
- [ ] El dueño puede ver cuánto vendió en cualquier período
- [ ] El reporte de cierre de caja se puede imprimir al final del día
- [ ] Se identifican fácilmente los productos más y menos rentables

---

## Fase 4: Facturación Electrónica SIAT (v0.5)

> _Cumplimiento fiscal con el Servicio de Impuestos Nacionales (SIN) de Bolivia._
> _Modalidad: Facturación Computarizada en Línea._

| # | Feature | Descripción | Estado |
|---|---------|-------------|--------|
| 4.1 | **Configuración SIAT** | UI para ingresar credenciales del SIN: NIT, token API, código de sistema, sucursal, punto de venta. | ⬜ Pendiente |
| 4.2 | **Sincronización de catálogos** | Descargar catálogos del SIAT: actividades económicas, productos/servicios, tipos de documento de identidad, métodos de pago, monedas. | ⬜ Pendiente |
| 4.3 | **Obtención de CUFD** | Solicitar Código Único de Facturación Diaria al inicio de cada jornada. Renovación automática. | ⬜ Pendiente |
| 4.4 | **Generación de CUF** | Crear el Código Único de Factura por cada venta según algoritmo del SIN (hash MD5). | ⬜ Pendiente |
| 4.5 | **Emisión en línea** | Generar XML de factura, comprimir con Gzip, enviar al SIAT vía SOAP/REST y obtener confirmación. | ⬜ Pendiente |
| 4.6 | **Modo contingencia** | Detectar cuando no hay conexión, facturar offline con CUIS de contingencia. Enviar paquetes al volver la conexión. | ⬜ Pendiente |
| 4.7 | **Anulación de facturas** | Anular facturas emitidas según protocolo del SIN, registrar motivo. | ⬜ Pendiente |
| 4.8 | **Impresión de factura** | Formato de factura según normativa boliviana vigente. Soporte carta y media carta. | ⬜ Pendiente |

### Criterios de completitud Fase 4:
- [ ] Se puede emitir una factura válida que pase verificación del SIN
- [ ] El sistema maneja caídas de internet sin perder facturas
- [ ] Las facturas impresas cumplen con el formato normativo

---

## Fase 5: Multi-usuario y Seguridad (v0.6)

> _Preparar para equipos de trabajo._

| # | Feature | Descripción | Estado |
|---|---------|-------------|--------|
| 5.1 | **Login con PIN** | Cada cajero tiene un PIN numérico de 4-6 dígitos para acceder al sistema rápidamente. | ⬜ Pendiente |
| 5.2 | **Roles y permisos** | Administrador (acceso total), Cajero (solo POS y ventas), Inventarista (solo stock). Permisos granulares. | ⬜ Pendiente |
| 5.3 | **Registro de acciones (audit log)** | Log detallado: quién hizo qué y cuándo. Ventas, anulaciones, ajustes de inventario, cambios de precio. | ⬜ Pendiente |
| 5.4 | **Cajero por turno** | Cada apertura/cierre de caja asociada a un cajero específico. Reportes individuales por cajero. | ⬜ Pendiente |
| 5.5 | **Backup automático** | Respaldo automático del archivo SQLite a carpeta configurable (local o USB). Frecuencia configurable. | ⬜ Pendiente |

### Criterios de completitud Fase 5:
- [ ] Dos cajeros pueden tener sesiones independientes
- [ ] El administrador puede ver quién anuló una venta y cuándo
- [ ] Los backups se generan sin intervención del usuario

---

## Fase 6: UI/UX Premium (v0.7)

> _Experiencia visual de nivel profesional._

| # | Feature | Descripción | Estado |
|---|---------|-------------|--------|
| 6.1 | **Dashboard mejorado** | Página de inicio con gráficos interactivos, alertas priorizadas, accesos rápidos, resumen del día. | ⬜ Pendiente |
| 6.2 | **Tema claro/oscuro** | Toggle de temas con transición suave. Guardar preferencia por usuario. | ⬜ Pendiente |
| 6.3 | **Animaciones y transiciones** | Page transitions fluidas, cart animations (agregar/quitar), loading skeletons, micro-interacciones. | ⬜ Pendiente |
| 6.4 | **Modo pantalla completa** | `F11` para modo kiosko/caja dedicada. Ocultar barra de título y sidebar. | ⬜ Pendiente |
| 6.5 | **Búsqueda global** | `Cmd+K` / `Ctrl+K` para buscar productos, clientes, ventas, configuraciones desde cualquier pantalla. | ⬜ Pendiente |
| 6.6 | **Personalización de layout** | Elegir entre vista grilla (actual) o vista lista en el POS. Tamaño de tarjetas configurable. | ⬜ Pendiente |

### Criterios de completitud Fase 6:
- [ ] La app se siente fluida y profesional en todas las interacciones
- [ ] Un usuario nuevo puede operar intuitivamente sin manual

---

## Fase 7: Escalabilidad Multi-sucursal (v1.0)

> _Preparar para crecimiento del negocio._

| # | Feature | Descripción | Estado |
|---|---------|-------------|--------|
| 7.1 | **Multi-sucursal** | Migración a PostgreSQL como servidor central. SQLite como cache local para operación offline. | ⬜ Pendiente |
| 7.2 | **Sincronización bidireccional** | Sync entre sucursales y servidor central. Resolución de conflictos. Cola de sincronización. | ⬜ Pendiente |
| 7.3 | **Reportes consolidados** | Dashboard con ventas y stock de todas las sucursales en una sola vista. | ⬜ Pendiente |
| 7.4 | **Transferencias entre sucursales** | Crear órdenes de transferencia de stock de una sucursal a otra con trazabilidad. | ⬜ Pendiente |

### Criterios de completitud Fase 7:
- [ ] Dos sucursales pueden operar independientemente y sincronizar datos
- [ ] El dueño ve reportes consolidados de todo el negocio
- [ ] Las transferencias de stock son trazables de punta a punta

---

## 📊 Orden Recomendado de Implementación

```
v0.2 (Fase 1) ──► Más impacto inmediato para el uso diario
     │
v0.3 (Fase 2) ──► Solidificar el inventario
     │
v0.4 (Fase 3) ──► Reportes para tomar decisiones
     │
v0.5 (Fase 4) ──► Cumplimiento legal SIAT (obligatorio en Bolivia)
     │
v0.6 (Fase 5) ──► Multi-usuario para equipos
     │
v0.7 (Fase 6) ──► Pulir la experiencia visual
     │
v1.0 (Fase 7) ──► Escalar a múltiples sucursales
```

---

## 📝 Notas

- Cada fase produce una versión funcional y usable del sistema.
- Las fases pueden solaparse si hay dependencias (ej: impresión de recibo en Fase 1 es base para factura SIAT en Fase 4).
- El stack tecnológico se mantiene: **Rust/Tauri v2** + **Svelte 5** + **SQLite** hasta Fase 7.
- Actualizar este documento conforme se completen las features (cambiar ⬜ por ✅).

---

_Última actualización: 2026-02-18_
