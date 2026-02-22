# 💰 Margen de Ganancia

> Compara lo que te costó un producto contra lo que ganas al venderlo. Ve la utilidad bruta de cada producto y del negocio en general.

## ¿Qué puedo hacer con esto?

- Ver la **utilidad bruta** (ganancia) de cada producto vendido
- Comparar el **precio de compra** contra el **precio de venta promedio** real
- Ver el **porcentaje de margen** de cada producto con indicadores de color:
  - 🟢 Verde = margen alto (30% o más)
  - 🟡 Amarillo = margen medio (15% a 29%)
  - 🔴 Rojo = margen bajo (menos de 15%)
- Filtrar por **período**: hoy, esta semana, este mes, mes anterior, o personalizado
- Ver **tarjetas resumen** con los totales del negocio
- **Ordenar la tabla** por cualquier columna haciendo clic en el encabezado
- **Exportar** a CSV (para Excel) o PDF (para imprimir)

---

## ¿Cómo se usa?

### Paso 1: Ir a Reportes

Haz clic en **📊 Reportes** en el menú lateral. Baja hasta la sección **💰 Margen de Ganancia**.

### Paso 2: Elegir el período

Selecciona el rango de fechas que quieres analizar:

| Botón | Qué muestra |
|-------|-------------|
| **Hoy** | Solo las ventas de hoy |
| **Esta semana** | Desde el domingo hasta hoy |
| **Este mes** | Desde el primer día del mes hasta hoy |
| **Mes anterior** | Todo el mes pasado completo |
| **Todo** | Todas las ventas históricas |
| **Personalizado** | Tú eliges las fechas desde/hasta |

### Paso 3: Revisar las tarjetas resumen

En la parte superior de la sección verás 4 tarjetas:

- 💵 **Ingresos** — Total que recibiste por ventas (en Bs)
- 📦 **Costo** — Lo que te costaron esos productos (precio de compra × cantidad vendida)
- 📈 **Utilidad bruta** — La diferencia: Ingresos − Costo (lo que ganaste)
- **% Margen promedio** — El porcentaje general de ganancia del período

### Paso 4: Analizar la tabla de productos

La tabla muestra cada producto vendido con las siguientes columnas:

| Columna | Qué significa |
|---------|---------------|
| **Producto** | Nombre del producto |
| **P. Compra** | Lo que te costó comprarlo (precio unitario) |
| **P. Venta Prom.** | El precio promedio al que lo vendiste (puede variar si diste descuentos) |
| **Uds** | Cantidad de unidades vendidas en el período |
| **Ingresos** | Total que recibiste por ese producto |
| **Costo** | Lo que gastaste en ese producto (compra × cantidad) |
| **Utilidad** | Lo que ganaste (Ingresos − Costo). Coloreado según el margen |
| **Margen %** | Porcentaje de ganancia con badge de color |

### Paso 5: Ordenar por columna

Haz clic en el nombre de cualquier columna para ordenar la tabla:
- Primer clic → ordena de mayor a menor (↓)
- Segundo clic → ordena de menor a mayor (↑)

Por ejemplo, haz clic en **Margen %** para ver primero los productos más rentables, o haz clic de nuevo para ver los menos rentables.

---

## Exportar el reporte

Cuando hay datos, aparecen dos botones en la esquina superior derecha:

- **📥 CSV** — Descarga un archivo que puedes abrir en Excel con todos los productos y sus márgenes
- **📄 PDF** — Genera un documento para imprimir o guardar como PDF. Incluye las tarjetas resumen y la tabla completa con badges de color

---

## Preguntas frecuentes

### ¿Por qué no veo ningún producto?
No hay ventas completadas en el período seleccionado. Prueba seleccionando **"Todo"** para ver el historial completo.

### ¿De dónde sale el "Precio de Compra"?
Del campo **"Precio de compra"** que registraste al crear o editar el producto en Inventario. Si dejaste el precio de compra en 0, el margen será del 100%.

### ¿Qué significa que el margen esté en rojo?
Que la ganancia sobre el costo es menor al 15%. No quiere decir que pierdas dinero (a menos que sea negativo), solo que el margen es bajo comparado con los otros productos.

### ¿El "P. Venta Prom." puede ser diferente al precio que tengo en el producto?
Sí. El precio de venta promedio se calcula con los **precios reales de cada venta**. Si alguna vez vendiste con descuento, el promedio será menor que tu precio de lista.

### ¿Se incluyen las ventas anuladas?
No. Solo se cuentan las ventas con estado **completado**.

### ¿Puedo ver el margen de un solo producto?
No directamente, pero puedes ordenar la tabla por nombre de producto para encontrarlo rápidamente. También puedes usar fechas personalizadas para ver un período específico.

---

## 💡 Tips útiles

- **Fin de mes**: Revisa el margen promedio cada fin de mes para saber si tu negocio es rentable en general.
- **Productos en rojo**: Si un producto tiene margen bajo (🔴), considera ajustar su precio de venta o buscar un proveedor más barato.
- **Registra el precio de compra**: Para que este reporte sea útil, asegúrate de que todos los productos tengan su precio de compra actualizado en Inventario.
- **Compara períodos**: Alterna entre "Este mes" y "Mes anterior" para ver si tus márgenes están mejorando o empeorando.
- **Exporta a Excel**: Usa el botón CSV para abrir los datos en Excel y hacer tus propios análisis o gráficos.

---

## Notas importantes

- El reporte se actualiza automáticamente cada vez que cambias el período
- Los cálculos se basan en el **precio de compra actual** del producto y el **precio de venta real** de cada transacción
- La fila **"Total"** al final de la tabla muestra el resumen global
- Si modificas el precio de compra de un producto, el reporte reflejará el nuevo precio en todas las ventas (pasadas y futuras)
