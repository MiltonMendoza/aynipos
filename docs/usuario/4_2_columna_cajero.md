# 👤 Columna "Cajero" en el Historial de Ventas

> El administrador puede ver quién realizó cada venta desde la lista de Ventas.

---

## ¿Qué puedo hacer con esto?

En la pantalla de **Ventas**, hay una columna especial llamada **"Cajero"** que muestra el nombre del usuario que realizó cada venta. Esto permite al administrador:

- 📋 Saber quién atendió a cada cliente
- 🔍 Filtrar y ordenar ventas por cajero
- 📊 Identificar el rendimiento de cada turno
- 🛡️ Revisar si una venta fue realizada por la persona correcta

> 🔒 Esta columna **solo la ve el Administrador** (y el Inventarista). Los cajeros no la ven porque únicamente acceden a sus propias ventas.

---

## ¿Cómo se usa?

### Paso 1: Ir a Ventas

En el menú lateral izquierdo, hacé clic en **🧾 Ventas**.

### Paso 2: Ver la columna Cajero

En la tabla de ventas, buscá la columna **"Cajero"**. Cada fila mostrará el nombre del usuario que realizó esa venta.

### Paso 3: Ordenar por Cajero

Hacé clic en el encabezado de la columna **"Cajero"** para ordenar la lista alfabéticamente por nombre de cajero. Hacé clic de nuevo para invertir el orden.

### Paso 4: Buscar ventas de un cajero específico

Usá el buscador o los filtros disponibles en la parte superior para encontrar ventas de una persona en particular.

---

## 🏷️ Resumen de quién ve qué

| Rol | ¿Ve la columna Cajero? | ¿Qué ventas ve? |
|-----|:---:|---|
| **Administrador** | ✅ Sí | Todas las ventas de todos los cajeros |
| **Inventarista** | ✅ Sí | Todas las ventas de todos los cajeros |
| **Cajero** | ❌ No | Solo sus propias ventas |

---

## ❓ Preguntas frecuentes

### ¿Puedo filtrar ventas por cajero?
Podés ordenar la columna haciendo clic en su encabezado. Para filtrar solo por un cajero específico, usá el buscador en la parte superior de la tabla.

### ¿Qué pasa si una venta no tiene cajero asignado?
Esto puede ocurrir en ventas muy antiguas (anteriores a la implementación de esta función). Aparecerá vacío o con un guión.

### ¿Puedo imprimir un reporte ordenado por cajero?
Sí. Ordená la tabla por cajero y luego usá la función de exportar a CSV/Excel desde el botón de exportar.

### ¿El cajero puede ver las ventas de sus compañeros?
No. El cajero solo ve sus propias ventas. No tiene acceso al registro completo de la tienda.

---

## ⚠️ Notas importantes

- 🔒 La columna **Cajero** es exclusiva del rol **Administrador** e **Inventarista**
- 📋 La columna es **ordenable** — hacé clic en el encabezado para ordenar
- 🕐 El cajero asignado es el que **estaba logueado** en el momento de la venta
- 🔄 Si un empleado cambia de rol, sus ventas pasadas siguen registradas con el rol que tenía al momento de realizarlas
