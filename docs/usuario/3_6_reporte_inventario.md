# 📦 Reporte de Inventario

> Conocé cuánto vale tu stock actual y detectá los productos que no se mueven.

## ¿Qué puedo hacer con esto?

Este reporte te permite saber en todo momento:

- **Cuánto vale tu inventario** — tanto a precio de costo (lo que pagaste) como a precio de venta (lo que cobrarías)
- **Qué productos están "dormidos"** — los que llevan muchos días sin ningún movimiento (sin vender, sin recibir, sin ajustar)
- **Exportar los datos** a CSV o PDF para compartir o archivar

Es ideal para tomar decisiones como: hacer promociones en productos parados, identificar capital estancado, o preparar un informe para tu contador.

---

## ¿Dónde lo encuentro?

1. Hacé clic en **📊 Reportes** en el menú lateral izquierdo
2. Buscá la sección **📦 Reporte de Inventario** (está debajo de "Margen de Ganancia")

---

## ¿Cómo se usa?

### Paso 1: Revisá las tarjetas resumen

Al entrar verás 4 tarjetas con información clave:

| Tarjeta | ¿Qué significa? |
|---------|------------------|
| 📋 **Productos** | Cantidad total de productos activos en tu catálogo |
| 💰 **Valor a costo** | Cuánto pagaste por todo el stock que tenés ahora |
| 🏷️ **Valor a venta** | Cuánto ganarías si vendieras todo tu stock actual |
| ⏳ **Sin movimiento 30+ d** | Productos que llevan más de 30 días sin ningún movimiento |

### Paso 2: Filtrá productos inactivos (opcional)

Usá el selector en la esquina superior derecha para filtrar:

- **Todos los productos** — muestra todo tu catálogo
- **Sin movimiento 30+ días** — productos con más de un mes sin actividad
- **Sin movimiento 60+ días** — productos con más de dos meses sin actividad
- **Sin movimiento 90+ días** — productos con más de tres meses sin actividad

> 💡 **Tip:** Filtrá por 90+ días para encontrar rápidamente productos candidatos a promoción o liquidación.

### Paso 3: Analizá la tabla

La tabla muestra una fila por cada producto con estas columnas:

| Columna | ¿Qué muestra? |
|---------|----------------|
| **Producto** | Nombre del producto |
| **SKU** | Código interno del producto |
| **Categoría** | A qué categoría pertenece |
| **Stock** | Unidades disponibles actualmente |
| **P. Compra** | Precio de compra unitario |
| **P. Venta** | Precio de venta unitario |
| **Valor Costo** | Stock × Precio de compra |
| **Valor Venta** | Stock × Precio de venta |
| **Inactividad** | Días desde el último movimiento |

### Paso 4: Ordená por cualquier columna

Hacé clic en el título de cualquier columna para ordenar de mayor a menor. Hacé clic de nuevo para invertir el orden.

> 💡 **Tip:** Ordená por **Valor Costo ↓** para ver dónde tenés más capital invertido, o por **Inactividad ↓** para ver qué productos llevan más tiempo sin moverse.

### Paso 5: Exportá el reporte

Cuando haya datos, aparecen dos botones en la esquina superior:

- **📥 CSV** — Descarga los datos en formato CSV (para abrir en Excel o Google Sheets)
- **📄 PDF** — Genera un PDF listo para imprimir o guardar

---

## ¿Qué significan los colores de inactividad?

Los badges de la columna "Inactividad" usan un semáforo:

| Color | Significado |
|-------|-------------|
| 🟢 **Verde** | Menos de 30 días — producto activo, todo bien |
| 🟡 **Amarillo** | 30 a 89 días — atención, lleva tiempo sin moverse |
| 🔴 **Rojo** | 90+ días — producto muy inactivo, considerar acción |
| ⚪ **Gris** (Sin mov.) | Nunca tuvo un movimiento registrado |

---

## Preguntas frecuentes

### ¿Qué cuenta como "movimiento"?
Cualquier entrada, salida o ajuste del producto: una venta, una compra, un ajuste de inventario, una devolución, etc.

### ¿"Valor a costo" y "Valor a venta" incluyen impuestos?
No. Son cálculos simples: stock actual × precio de compra, y stock actual × precio de venta. No incluyen impuestos ni descuentos.

### ¿Aparecen productos con stock 0?
Sí. Todos los productos activos aparecen, incluso los que tienen stock cero. Esto te ayuda a identificar productos que necesitan reposición.

### ¿Puedo ver la fila de totales?
Sí. Al final de la tabla hay una fila **Total** que suma el stock, el valor a costo y el valor a venta de todos los productos mostrados.

---

## Notas importantes

- 📊 Los datos se actualizan cada vez que abrís la sección o cambiás el filtro de inactividad
- 📥 El CSV se puede abrir directamente en Excel para hacer análisis más detallados
- 📄 El PDF incluye las tarjetas resumen y la tabla completa, listo para imprimir
- ⚠️ Solo se muestran productos **activos** (los eliminados no aparecen)
