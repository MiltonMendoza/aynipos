# 📊 Reporte de Cierre de Caja

> Genera un resumen imprimible con todas las ventas del turno al cerrar la caja registradora.

## ¿Qué puedo hacer con esto?

Cada vez que cierras la caja, el sistema genera automáticamente un **reporte completo del turno** que puedes imprimir o guardar como PDF. El reporte incluye:

- Cuántas ventas hiciste y por cuánto
- Desglose por método de pago (efectivo, tarjeta, QR)
- Ventas anuladas
- Impuestos y descuentos del día
- **Reconciliación**: si el dinero en caja coincide con lo esperado

Es ideal para el **cierre diario** y llevar el control del efectivo.

---

## ¿Cómo se usa?

### Paso 1: Cerrar la caja

1. Ve a **⚙️ Configuración** en el menú lateral
2. En la sección **💰 Caja Registradora**, haz clic en **🔒 Cerrar Caja**
3. Se abrirá una ventana pidiendo el **monto real** que hay en la caja (cuéntalo físicamente)
4. Ingresa el monto en bolivianos (Bs)
5. Opcionalmente agrega **notas** (por ejemplo: "Faltó cambio de Bs 10")
6. Haz clic en **🔒 Cerrar**

### Paso 2: Se abre el reporte automáticamente

Al cerrar la caja, se abrirá tu **navegador de internet** (Safari, Chrome, etc.) con el reporte completo listo para imprimir.

### Paso 3: Imprimir o guardar

1. Haz clic en el botón azul **🖨️ Imprimir Reporte**
2. O presiona **Ctrl+P** (Windows) / **Cmd+P** (Mac)
3. En el diálogo de impresión puedes:
   - Elegir tu impresora
   - Guardar como PDF
   - Ajustar opciones de impresión

> 💡 **Tip:** Si quieres archivar los cierres de caja digitalmente, guárdalos como PDF en una carpeta organizada por mes.

---

## ¿Qué información incluye el reporte?

| Sección | Detalle |
|---------|---------|
| 🏪 **Encabezado** | Nombre del negocio, NIT, dirección, teléfono, ciudad |
| 📅 **Período** | Fecha y hora de apertura y cierre de la caja |
| 💳 **Ventas por método de pago** | Tabla con: Efectivo, Tarjeta, QR — cada uno con número de ventas y monto total |
| ❌ **Ventas anuladas** | Cantidad de ventas que fueron canceladas en el turno |
| 📊 **Resumen** | Descuentos totales, IVA incluido, y **TOTAL VENTAS** en grande |
| 💰 **Reconciliación** | Monto de apertura, monto esperado, monto real, y **DIFERENCIA** |
| 📝 **Notas** | Observaciones que escribiste al cerrar (si las hay) |

---

## Reconciliación: ¿Qué significan los montos?

La sección de reconciliación es la más importante para el control del efectivo:

| Concepto | Significado |
|----------|-------------|
| **Monto apertura** | Lo que pusiste en caja al inicio del turno |
| **Monto esperado** | Apertura + ventas en efectivo = lo que **debería** haber en caja |
| **Monto real** | Lo que **realmente** contaste al cerrar |
| **Diferencia** | Real − Esperado. Si es positivo (verde) hay **sobrante**. Si es negativo (rojo) hay **faltante**. |

> 📌 **Nota:** El monto esperado solo considera ventas en **efectivo** y **mixto**, porque las ventas con tarjeta o QR no generan efectivo en caja.

---

## ¿Puedo reimprimir el reporte?

Sí. Después de cerrar la caja, aparece un botón **📊 Ver último cierre** en la sección de Caja Registradora:

1. Ve a **⚙️ Configuración**
2. En la sección **💰 Caja Registradora**, haz clic en **📊 Ver último cierre**
3. Se abrirá nuevamente el reporte en el navegador

> ⚠️ Este botón solo está disponible **mientras no abras una nueva caja**. Una vez que abres otra caja, el botón desaparece.

---

## Preguntas frecuentes

### ¿El reporte se genera solo al cerrar caja?
Sí. El reporte se genera **automáticamente** al momento de cerrar la caja. No necesitas hacer nada extra.

### ¿Y si no se abrió el reporte?
Haz clic en **📊 Ver último cierre** en la pantalla de Configuración para volver a generarlo.

### ¿Qué pasa si la diferencia es cero?
¡Perfecto! Significa que el efectivo en caja coincide exactamente con lo esperado. La diferencia aparecerá como **+Bs 0.00** en color verde.

### ¿Puedo cerrar caja sin haber vendido nada?
Sí. El reporte mostrará "Sin ventas en este turno" y los totales serán Bs 0.00.

### ¿El reporte incluye las ventas anuladas en los totales?
No. Las ventas anuladas se reportan como cantidad aparte, pero **no se suman** al total de ventas. Los totales solo incluyen ventas completadas.

### ¿Qué tamaño tiene el reporte?
Está diseñado para papel de **80mm de ancho** (impresoras térmicas de tickets). Si imprimes en papel carta, se centrará en la página.

---

## Notas importantes

- 📋 El reporte se abre **automáticamente** al cerrar caja — no necesitas buscarlo
- 🌐 Se necesita un **navegador de internet** instalado (Safari, Chrome, Firefox)
- 💰 Cuenta el dinero en caja **antes** de cerrar para ingresar el monto correcto
- 📄 Puedes guardar como **PDF** desde el diálogo de impresión para tener un archivo digital
- 🔄 El botón **📊 Ver último cierre** permite reimprimir mientras no abras una nueva caja
