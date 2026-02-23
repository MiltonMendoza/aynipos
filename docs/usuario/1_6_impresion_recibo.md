# 🖨️ Impresión de Recibo

> Genera e imprime un comprobante de venta para entregar al cliente.

## ¿Qué puedo hacer con esto?

Después de completar una venta puedes generar un **recibo impreso** con todos los detalles de la compra: productos, cantidades, precios, descuentos, total, método de pago, y los datos de tu negocio. También puedes **guardar el recibo como PDF** para archivarlo digitalmente.

Puedes imprimir recibos de dos formas:
- **Inmediatamente después de vender** — desde la pantalla de éxito
- **Después, desde el historial** — buscando cualquier venta anterior

---

## ¿Cómo se usa?

### Opción 1: Imprimir justo después de vender

Cuando completas una venta, aparece la pantalla de éxito con el monto total. En esa pantalla verás dos botones:

1. Haz clic en **🖨️ Imprimir Recibo**
2. Se abrirá tu **navegador de internet** (Safari, Chrome, etc.) con el recibo
3. En el navegador, haz clic en el botón azul **🖨️ Imprimir Recibo** o presiona **Cmd+P** (Mac) / **Ctrl+P** (Windows)
4. Aparecerá el diálogo de impresión del sistema donde puedes:
   - Elegir tu impresora
   - Guardar como PDF
   - Ajustar el número de copias
5. Haz clic en **Imprimir** o **Guardar**
6. Vuelve a AyniPOS y haz clic en **✕ Cerrar** para continuar vendiendo

> 💡 **Tip:** Si no necesitas imprimir, simplemente haz clic en **✕ Cerrar** o en cualquier parte oscura de la pantalla para cerrar la ventana de éxito.

### Opción 2: Imprimir desde el historial de ventas

Puedes reimprimir el recibo de cualquier venta anterior:

1. Ve a **📋 Ventas** en el menú lateral
2. Busca la venta que necesitas en la lista (puedes filtrar por fecha)
3. Haz clic sobre la venta para ver sus detalles
4. En el panel de detalles (lado derecho), haz clic en **🖨️ Imprimir Recibo**
5. Se abrirá el navegador con el recibo listo para imprimir

> 📌 **Nota:** Puedes reimprimir recibos tanto de ventas **completadas** como de ventas **anuladas** (por ejemplo, para documentar la anulación).

---

## ¿Qué información incluye el recibo?

El recibo incluye automáticamente:

| Sección | Detalle |
|---------|---------|
| 🏪 **Encabezado** | Nombre del negocio, NIT, dirección, teléfono, ciudad |
| 🧾 **Datos de la venta** | Número de venta, fecha y hora, nombre del cliente |
| 📦 **Productos** | Nombre, cantidad, precio unitario, total por producto |
| 💰 **Descuentos** | Descuentos por ítem y/o descuento global (si aplica) |
| 💵 **Totales** | Subtotal, Débito Fiscal, y **TOTAL** en grande |
| 💳 **Método de pago** | Efectivo, Tarjeta, o QR |
| 📝 **Notas** | Observaciones de la venta (si las hay) |
| 🙏 **Pie** | "¡Gracias por su compra!" |

> ⚠️ El recibo dice **"Este recibo no es una factura"** — es un comprobante de venta, no un documento fiscal.

---

## ¿Cómo configuro los datos de mi negocio?

Los datos que aparecen en el encabezado del recibo (nombre, NIT, dirección, etc.) se toman de la **Configuración** de la app:

1. Ve a **⚙️ Configuración** en el menú lateral
2. Busca la sección de **Datos del negocio**
3. Llena todos los campos:
   - Nombre del negocio
   - NIT
   - Dirección
   - Teléfono
   - Ciudad
4. Estos datos aparecerán automáticamente en todos los recibos

---

## Guardar como PDF

Si no tienes impresora o quieres guardar una copia digital:

1. Cuando se abra el diálogo de impresión, busca la opción **PDF** en la esquina inferior izquierda (en Mac) o el desplegable de **Destino** (en Windows)
2. Selecciona **"Guardar como PDF"**
3. Elige la carpeta donde quieres guardar el archivo
4. Haz clic en **Guardar**

> 💡 **Tip:** Puedes crear una carpeta "Recibos" en tu computadora para tener todo organizado por día o mes.

---

## Preguntas frecuentes

### ¿El recibo se imprime directamente en la impresora?
No. El recibo se abre primero en tu navegador de internet (Safari, Chrome) donde puedes **ver cómo queda antes de imprimir**. Desde ahí eliges imprimir o guardar como PDF.

### ¿Puedo reimprimir un recibo?
¡Sí! Ve a **📋 Ventas**, busca la venta, haz clic sobre ella y presiona **🖨️ Imprimir Recibo**. Puedes reimprimir cuantas veces necesites.

### ¿Qué tamaño tiene el recibo?
El recibo está diseñado para papel de **80mm de ancho**, que es el estándar de las impresoras térmicas de tickets. Si imprimes en hoja tamaño carta, el recibo se centrará en la página.

### ¿Por qué se abre el navegador en vez de imprimir directo?
AyniPOS usa esta técnica para garantizar que funcione con **cualquier impresora** del sistema, incluidas las impresoras térmicas, de escritorio, e incluso la opción de guardar como PDF.

### ¿Y si no veo los datos de mi negocio en el recibo?
Ve a **⚙️ Configuración** y verifica que hayas llenado los campos de datos del negocio. Si están vacíos, el recibo mostrará "Mi Negocio" como nombre y omitirá los demás campos.

### ¿Puedo personalizar el diseño del recibo?
Por el momento no. El diseño está optimizado para impresoras térmicas de 80mm con un formato estándar de farmacia/tienda.

---

## Notas importantes

- 🌐 **Se necesita un navegador de internet** instalado en la computadora (Safari, Chrome, Firefox, etc.)
- 🖨️ **Impresoras compatibles**: Cualquier impresora que aparezca en tu sistema — térmicas, inkjet, láser
- 📄 **PDF**: Siempre puedes guardar como PDF aunque no tengas impresora conectada
- 📊 **Datos del negocio**: Configúralos una sola vez en ⚙️ Configuración y aparecerán en todos los recibos
