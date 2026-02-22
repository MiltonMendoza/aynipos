# 📥📤 Importar y Exportar Productos

> Carga cientos de productos desde un archivo CSV o descarga tu catálogo completo para respaldo o edición masiva.

---

## ¿Qué puedo hacer con esto?

- **Importar productos** desde un archivo CSV para cargar tu catálogo completo de una sola vez (ideal al empezar a usar AyniPOS o cuando recibes un lote grande de productos nuevos)
- **Exportar productos** a un archivo CSV que puedes abrir en Excel, Google Sheets o cualquier programa de hojas de cálculo
- Usar el archivo exportado como **respaldo** de tu catálogo
- Editar precios o datos de muchos productos a la vez en Excel y volver a importarlos

---

## ¿Dónde encuentro esta función?

En el menú lateral izquierdo haz clic en **📦 Inventario**. En la parte superior derecha verás los botones:

- **📥 Importar CSV** — para cargar productos desde un archivo
- **📤 Exportar CSV** — para descargar tu catálogo actual

---

## 📤 Exportar productos

### Paso 1: Haz clic en "📤 Exportar CSV"

El botón está en la parte superior derecha de la página de Inventario.

### Paso 2: Elige dónde guardar el archivo

Se abrirá una ventana del sistema para elegir la carpeta y el nombre del archivo. Por defecto se sugiere el nombre `productos.csv`.

### Paso 3: ¡Listo!

Verás un mensaje confirmando cuántos productos se exportaron. El archivo se puede abrir con Excel, Google Sheets, LibreOffice Calc o cualquier editor de texto.

### ¿Qué contiene el archivo exportado?

| Columna | Descripción |
|---------|-------------|
| `sku` | Código interno del producto |
| `codigo_barras` | Código de barras (si tiene) |
| `nombre` | Nombre del producto |
| `descripcion` | Descripción (si tiene) |
| `categoria` | Nombre de la categoría |
| `precio_compra` | Precio de compra en Bs |
| `precio_venta` | Precio de venta en Bs |
| `tasa_impuesto` | Tasa de impuesto (ej: 0.13 = 13%) |
| `unidad` | Unidad de medida |
| `stock_minimo` | Stock mínimo configurado |
| `stock_actual` | Cantidad actual en inventario |

---

## 📥 Importar productos

### Paso 1: Prepara tu archivo CSV

Crea un archivo CSV (por ejemplo en Excel, después guárdalo como CSV) con las siguientes columnas:

**Columnas obligatorias:**

| Columna | Ejemplo |
|---------|---------|
| `sku` | MED-001 |
| `nombre` | Paracetamol 500mg |
| `categoria` | Medicamentos |
| `precio_compra` | 3.50 |
| `precio_venta` | 5.00 |

**Columnas opcionales:**

| Columna | Ejemplo | Si no la incluyes... |
|---------|---------|---------------------|
| `codigo_barras` | 7891234567890 | Se deja sin código de barras |
| `descripcion` | Analgésico para dolor | Se deja sin descripción |
| `tasa_impuesto` | 0.13 | Se usa 13% (0.13) por defecto |
| `unidad` | unidad | Se usa "unidad" por defecto |
| `stock_minimo` | 10 | Se usa 0 por defecto |
| `stock_inicial` | 100 | Se crea con stock 0 |

> 💡 **Tip:** La forma más fácil de crear el CSV es **exportar primero** tus productos actuales, y usar ese archivo como plantilla. Así ya tienes el formato correcto.

### Paso 2: Haz clic en "📥 Importar CSV"

Se abrirá una ventana del sistema para seleccionar tu archivo CSV.

### Paso 3: Selecciona el archivo

Busca y selecciona el archivo `.csv` que preparaste.

### Paso 4: Revisa los resultados

Después de importar, se muestra una ventana con el resumen:

- ✅ **Creados** — productos nuevos que se agregaron al sistema
- 🔄 **Actualizados** — productos que ya existían (mismo SKU) y se actualizaron con los nuevos datos
- ❌ **Errores** — filas que no se pudieron importar, con el motivo del error

Si hay errores, se muestra una tabla con el número de fila y el problema encontrado. **Los errores no detienen la importación** — los productos válidos se importan normalmente.

---

## ❓ Preguntas frecuentes

### ¿Qué pasa si un producto ya existe en el sistema?

Si el **SKU** del CSV coincide con un producto que ya tienes, el sistema **actualiza** los datos del producto existente (nombre, precios, categoría, etc.) en vez de crear uno duplicado.

### ¿Puedo actualizar precios de muchos productos a la vez?

¡Sí! Esa es una de las mejores funciones de importar/exportar:

1. **Exporta** tus productos actuales a CSV
2. **Abre** el CSV en Excel
3. **Cambia** los precios que necesites
4. **Importa** el archivo de vuelta

Como los SKU son iguales, se actualizan los productos existentes.

### ¿Qué pasa con las categorías?

Si el CSV menciona una categoría que no existe en el sistema (por ejemplo "Vitaminas"), se **crea automáticamente**. No necesitas crear las categorías antes de importar.

### ¿Puedo usar el archivo de Excel directamente?

No. Debes guardar el archivo como **CSV** (valores separados por comas). En Excel: Archivo → Guardar como → selecciona "CSV (delimitado por comas)".

### ¿Qué pasa si hay un error en una fila del CSV?

Solo esa fila se salta. El resto de productos se importan normalmente. En el resumen de resultados puedes ver exactamente qué filas fallaron y por qué.

### ¿Puedo importar el stock inicial de cada producto?

Sí. Agrega la columna `stock_inicial` en tu CSV con la cantidad que quieras. Solo aplica para **productos nuevos** — si el producto ya existe, no se modifica el stock.

### ¿Puedo exportar solo los productos de una categoría?

No, actualmente se exportan todos los productos activos. Si necesitas filtrar, puedes abrir el CSV en Excel y filtrar las filas que necesites.

---

## 🚫 Errores comunes

| Error | ¿Qué significa? | ¿Qué hacer? |
|-------|-----------------|--------------|
| _"Columna 'sku' no encontrada"_ | El CSV no tiene una columna llamada `sku` | Revisa que la primera fila tenga los encabezados correctos |
| _"SKU está vacío"_ | Una fila tiene el campo SKU vacío | Asegúrate de que todas las filas tengan un SKU |
| _"Categoría está vacía"_ | Una fila no tiene categoría | Agrega el nombre de la categoría en esa fila |
| _"Precio de compra debe ser mayor a 0"_ | El precio es 0 o negativo | Corrige el precio en la fila indicada |
| _"Código de barras ya pertenece a..."_ | Otro producto ya tiene ese código de barras | Cambia o elimina el código de barras duplicado |

---

## 💡 Tips

- 🔄 Usa la exportación como **respaldo periódico** de tu catálogo de productos
- 📊 Exporta el CSV para analizar tu inventario completo en Excel con filtros y gráficos
- ✏️ Cuando necesites cambiar muchos precios a la vez (por ejemplo por inflación), exporta → edita en Excel → importa de vuelta
- 🏷️ Al importar por primera vez, mantén los SKU con un formato consistente (ej: MED-001, VIT-002) para identificar productos fácilmente
- 📋 Usa la columna `stock_inicial` solo en la primera importación para no tener que ajustar el stock uno por uno

---

## ⚠️ Notas importantes

- La importación procesa **fila por fila** — si una fila tiene error, las demás se importan normalmente
- Al actualizar un producto existente, se actualizan **todos los datos** (nombre, precios, categoría, etc.), no solo los que cambies
- El `stock_inicial` solo se aplica a productos **nuevos**, no a productos que ya existen
- La exportación incluye el **stock actual** como referencia, pero al re-importar no se modifica el stock de productos existentes
- Los archivos CSVdeben usar **coma** como separador (que es el formato estándar de CSV)
