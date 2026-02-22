# 📜 Historial de Movimientos por Producto

> Consulta todas las entradas, salidas y ajustes de inventario de un producto con fechas, cantidades y detalles.

---

## ¿Qué puedo hacer con esto?

Con el historial de movimientos puedes ver **todo lo que pasó con el stock de un producto**: cuándo entró mercadería, cuándo se vendió, si hubo ajustes o devoluciones. Es como el "estado de cuenta" de cada producto.

Esto te sirve para:

- Verificar cuándo se recibió la última compra de un producto
- Ver cuántas unidades se vendieron en un período
- Revisar ajustes de inventario (por ejemplo, por productos dañados o faltantes)
- Controlar devoluciones registradas
- Identificar si un producto se mueve rápido o lento

---

## ¿Cómo se usa?

### Paso 1: Ir a Inventario

En el menú lateral izquierdo, haz clic en **📦 Inventario** para abrir la tabla de productos.

### Paso 2: Buscar el producto

Ubica el producto del que quieres ver el historial. Puedes usar los filtros de la parte superior (📋 Todos, ⚠️ Bajo Stock, ⏰ Por Vencer) para encontrarlo más fácil.

### Paso 3: Abrir el historial

En la fila del producto, haz clic en el botón **📜 Historial** en la columna de Acciones.

Se abrirá una ventana con toda la información de movimientos de ese producto.

### Paso 4: Revisar el resumen

En la parte superior del historial verás **tres tarjetas de resumen**:

| Tarjeta | ¿Qué muestra? |
|---------|----------------|
| **Entradas** (verde) | Total de unidades que ingresaron al stock (compras, devoluciones positivas) |
| **Salidas** (rojo) | Total de unidades que salieron del stock (ventas) |
| **Total movimientos** (azul) | Cantidad total de operaciones registradas |

### Paso 5: Revisar los movimientos detallados

Debajo del resumen hay una **tabla con todos los movimientos** ordenados del más reciente al más antiguo. Cada fila muestra:

| Columna | Descripción |
|---------|-------------|
| **Fecha** | Día y hora exacta del movimiento |
| **Tipo** | Tipo de movimiento con color (ver tabla abajo) |
| **Cantidad** | Cuántas unidades entraron (+) o salieron (-) |
| **Lote** | Número de lote asociado (si se registró) |
| **Notas** | Observación o referencia (ej: "Venta #45", motivo de ajuste) |

---

## 🏷️ Tipos de movimiento

Cada movimiento tiene una etiqueta de color para identificarlo rápidamente:

| Etiqueta | Significado |
|----------|-------------|
| 🟢 **Compra** | Ingreso de mercadería al stock |
| 🔴 **Venta** | Salida de producto por una venta |
| 🟡 **Ajuste** | Corrección manual del stock (puede ser positiva o negativa) |
| 🔵 **Devolución** | Producto devuelto que regresa al stock |

---

## ❓ Preguntas frecuentes

### ¿Cuántos movimientos puedo ver?
El historial muestra los **últimos 100 movimientos** del producto, ordenados del más reciente al más antiguo.

### ¿De dónde vienen los movimientos?
Se registran automáticamente cuando:
- **Vendes** un producto desde la pantalla de ventas → aparece como 🔴 Venta
- **Compras** mercadería usando el botón "📊 Ajustar" con tipo "Compra" → aparece como 🟢 Compra
- **Ajustas** el stock manualmente (productos dañados, faltantes, etc.) → aparece como 🟡 Ajuste
- **Anulas** una venta → aparece como 🔵 Devolución

### ¿Puedo borrar un movimiento del historial?
No. El historial es un registro permanente para mantener la trazabilidad del inventario. Los movimientos no se pueden editar ni eliminar.

### ¿Qué es la columna "Lote"?
Si al momento de ajustar o ingresar inventario registraste un **número de lote**, esa información aparece aquí. Esto te ayuda a rastrear de qué lote salió cada producto. Si no usas lotes, la columna mostrará un guión (—).

### ¿Los números negativos en "Cantidad" son normales?
Sí. Los números **negativos** (con signo -) indican que el producto **salió** del inventario (por ejemplo, una venta de -3 unidades). Los números **positivos** (con signo +) indican que **entró** mercadería.

---

## 💡 Tips

- 📊 Usa el resumen de **Entradas vs Salidas** para identificar rápidamente si un producto se vende más de lo que compras
- 🔍 Revisa las **notas** de los movimientos — las ventas automaticamente incluyen el número de venta (ej: "Venta #45") para que puedas cruzar información
- 📦 Si usas lotes, el historial te permite rastrear de qué lote salió cada unidad vendida
- 🕐 La fecha y hora exacta te sirve si necesitas verificar una venta o un ajuste específico

---

## ⚠️ Notas importantes

- El historial se actualiza **en tiempo real** — si acabas de hacer una venta o un ajuste, aparecerá inmediatamente al abrir el historial
- Los movimientos de **todas las ventas** se registran automáticamente, no necesitas hacer nada manual
- Para que los ajustes aparezcan con información de lote, debes ingresarlo al momento de usar el botón **📊 Ajustar**
