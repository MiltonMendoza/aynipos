# Reglas de Desarrollo del Proyecto AyniPOS

## 🧪 Pruebas Unitarias Obligatorias para Funciones Críticas

Para garantizar la estabilidad financiera, de inventario y contable del sistema, se establecen las siguientes reglas obligatorias de verificación para cualquier agente de desarrollo:

1. **Flujo de Ventas y Caja**:
   - Todo cambio en el cálculo de totales, descuentos, impuestos o saldos en `src-tauri/src/commands/sales.rs` o `src-tauri/src/commands/cash_register.rs` debe acompañarse de pruebas unitarias en `src-tauri/src/tests/sales.rs` y `src-tauri/src/tests/cash_register.rs`.
   - Se debe verificar explícitamente que los descuentos no generen totales negativos y que la aritmética decimal sea exacta.

2. **Inventario y Lotes**:
   - Cualquier modificación en la deducción de stock al vender, ajustes manuales, ingresos por compra, devoluciones o lógica de lotes en `src-tauri/src/commands/inventory.rs` o `src-tauri/src/commands/products.rs` debe tener cobertura en `src-tauri/src/tests/inventory.rs` y `src-tauri/src/tests/products.rs`.

3. **Reportes y Contabilidad**:
   - Toda consulta SQL o cálculo matemático que genere reportes financieros (como el Margen de Ganancia, Suma de Capital en inventario o totales del Dashboard) debe verificarse mediante aserciones estrictas de sumatoria matemática.

## 🚀 Validación Obligatoria
Antes de dar por completado cualquier requerimiento o feature, es obligatorio ejecutar localmente:
- `cargo test --manifest-path src-tauri/Cargo.toml` (para verificar que las funciones núcleo no se hayan roto)
- `npm run check` (para garantizar que los componentes frontend compilen sin errores de rutas o tipado)
