# 📥 Exportar Reportes a Excel/PDF

> Descarga cualquier reporte de AyniPOS como archivo CSV (para Excel) o como PDF para imprimir.

## ¿Qué puedo hacer con esto?

- **Descargar en CSV** cualquier reporte para abrirlo en Excel, LibreOffice o Google Sheets
- **Generar un PDF** de cualquier reporte para imprimirlo o guardarlo
- Exportar los 4 reportes principales:
  - 📈 Gráfico de ventas
  - 🏆 Productos más vendidos
  - 💰 Margen de ganancia
  - 📦 Reporte de inventario

---

## ¿Cómo se usa?

### Paso 1: Ir a Reportes

Haz clic en **📊 Reportes** en el menú lateral.

### Paso 2: Cargar los datos

Navega a la sección del reporte que quieres exportar y asegúrate de que tenga datos. Puedes elegir el período (Hoy, Esta semana, Este mes, etc.) para obtener la información que necesitas.

### Paso 3: Buscar los botones de exportar

Cuando una sección tiene datos, aparecen **dos botones** en la esquina superior derecha del encabezado:

| Botón | Qué hace |
|-------|----------|
| **📥 CSV** | Descarga un archivo `.csv` que se abre en Excel |
| **📄 PDF** | Genera un documento para imprimir o guardar como PDF |

> 💡 Si no ves los botones, es porque la sección no tiene datos todavía. Prueba cambiando el período a **"Todo"** o **"Este mes"**.

---

## Exportar a CSV (Excel)

1. Haz clic en el botón **📥 CSV** de la sección que quieras
2. Se abrirá una ventana para **elegir dónde guardar** el archivo
3. Elige la carpeta y haz clic en **Guardar**
4. Abre el archivo `.csv` con Excel, LibreOffice Calc, o Google Sheets

### ¿Qué incluye el CSV?

| Reporte | Columnas del archivo |
|---------|---------------------|
| 📈 Gráfico de ventas | Período, Ventas (Bs), Transacciones |
| 🏆 Productos más vendidos | Ranking, Producto, Cantidad, Ingresos |
| 💰 Margen de ganancia | Producto, P. Compra, P. Venta Prom., Cantidad, Ingresos, Costo, Utilidad, Margen % |
| 📦 Inventario | Producto, SKU, Categoría, Stock, P. Compra, P. Venta, Valor Costo, Valor Venta, Días sin mov. |

Todos los CSV incluyen una fila de **TOTAL** al final con los resúmenes.

---

## Exportar a PDF

1. Haz clic en el botón **📄 PDF** de la sección que quieras
2. Se abrirá una **ventana del navegador** con el reporte formateado
3. Haz clic en el botón **🖨️ Imprimir / Guardar PDF** (o usa `Ctrl+P` / `Cmd+P`)
4. En la ventana de impresión, selecciona **"Guardar como PDF"** como destino
5. Haz clic en **Guardar**

### ¿Qué incluye el PDF?

- **Título** del reporte con fecha de generación
- **Tarjetas resumen** con los totales principales (en los reportes que las tienen)
- **Tabla completa** con todos los datos, incluyendo totales
- **Badges de color** en los reportes de margen e inventario

---

## Preguntas frecuentes

### ¿Puedo abrir el CSV en Excel?

Sí. Solo haz doble clic en el archivo `.csv` y Excel lo abrirá automáticamente. Si los datos aparecen en una sola columna, usa la opción de importar datos con separador de coma.

### ¿El PDF se genera automáticamente?

No como archivo PDF directamente. El sistema genera una página lista para imprimir en tu navegador. Desde ahí tú eliges si imprimir en papel o **guardar como PDF** usando la opción del navegador.

### ¿Los botones no aparecen?

Los botones de exportar solo aparecen cuando hay datos cargados en esa sección. Verifica que:
- Tengas ventas registradas en el período seleccionado (para Gráficos, Top Productos, y Margen)
- Tengas productos activos con stock (para Inventario)

### ¿Qué nombre tiene el archivo CSV?

El nombre incluye el tipo de reporte y la fecha. Por ejemplo:
- `ventas_day_2026-02-22.csv`
- `productos_top_2026-02-22.csv`
- `margen_ganancia_2026-02-22.csv`
- `inventario_2026-02-22.csv`

### ¿Puedo exportar con filtros aplicados?

Sí. El CSV y PDF exportan **exactamente lo que ves en pantalla**, con el período y filtros que hayas seleccionado.

### ¿Se incluyen ventas anuladas?

No. Los reportes solo incluyen ventas con estado **completado**.

---

## 💡 Tips útiles

- **Respaldo mensual**: Descarga el reporte de inventario en CSV cada fin de mes como respaldo de tu stock
- **Análisis en Excel**: Usa el CSV de "Margen de ganancia" para hacer tus propios gráficos y análisis en Excel
- **Imprimir para reuniones**: Genera el PDF de "Productos más vendidos" para mostrar a tu equipo qué se está vendiendo mejor
- **Archivar reportes**: Guarda los PDFs de cierre mensual en una carpeta organizada por mes para tener tu historial

---

## Notas importantes

- Los reportes se exportan con los **datos y filtros que tengas seleccionados** en ese momento
- El formato CSV usa comas como separador y es compatible con Excel, LibreOffice y Google Sheets
- Para el PDF, necesitas un navegador web instalado (Chrome, Edge, Firefox, etc.)
- Mientras se está exportando, los botones se deshabilitan brevemente para evitar clicks dobles
