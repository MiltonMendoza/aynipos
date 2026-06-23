# 📦 Control de Lotes y Vencimiento

> Registra el número de lote y la fecha de vencimiento cada vez que ingresa mercadería, y recibe alertas visuales tipo semáforo cuando un producto está por vencer.

---

## ¿Qué puedo hacer con esto?

Cuando compras mercadería, cada caja o paquete viene con un **número de lote** y una **fecha de vencimiento** impresos en el empaque. Con esta función puedes:

- **Registrar** el lote y la fecha de vencimiento al ingresar stock
- **Ver todos los lotes** de un producto con un semáforo de colores
- **Identificar rápidamente** qué productos están por vencer
- **Eliminar** lotes vacíos que ya no necesitas

> 💡 **¿No vendes productos que caducan?** No hay problema. Los campos de lote y vencimiento son opcionales. Puedes usar AyniPOS normalmente sin llenarlos.

---

## ¿Cómo se usa?

### Registrar un lote al ingresar stock

#### Paso 1: Ir a Inventario

En el menú lateral izquierdo, haz clic en **📦 Inventario**.

#### Paso 2: Ajustar stock

Busca el producto en la tabla y haz clic en el menú **⋮** (tres puntos) en la fila del producto, luego seleccioná **📊 Ajustar Stock**.

#### Paso 3: Ingresar el stock final deseado

En la ventana de ajuste, el sistema ya muestra cuántas unidades tenés actualmente. Simplemente escribí **cuántas unidades querés tener al final**.

> **Ejemplo:** Si tenés 20 unidades y recibiste 30 más, escribí `50` — el sistema calcula automáticamente que se agregarán 30 unidades.

El sistema te confirma el cambio al instante:
- 📈 **"Se agregarán X unidades"** — si el número que pusiste es mayor al actual
- 📉 **"Se reducirán X unidades"** — si el número que pusiste es menor al actual
- El botón **Aplicar** se desactiva si el número es igual al actual (no hay cambio)

#### Paso 4: Elegir el tipo de movimiento (solo si se agrega stock)

Si el ajuste **suma** unidades, aparecerá un selector para indicar el motivo:

| Tipo | ¿Cuándo usarlo? |
|------|------------------|
| **Compra** | Llegó mercadería nueva del proveedor |
| **Devolución** | Un cliente devolvió un producto |
| **Ajuste manual** | Corrección de inventario (conteo físico, etc.) |

Si el ajuste **reduce** unidades, no se pide el tipo — se registra como ajuste.

#### Paso 5: Llenar los datos del lote (opcional)

Si querés registrar de qué lote viene la mercadería, completá estos campos:

| Campo | ¿Obligatorio? | Descripción |
|-------|:---:|-------------|
| **Número de lote** | ❌ | El código del lote que viene impreso en la caja (ej: LOTE-2026-A) |
| **Fecha de vencimiento** | ❌ | La fecha de caducidad del producto |

Puedes llenar uno, ambos, o ninguno según lo que necesites.

#### Paso 6: Aplicar

Haz clic en **✅ Aplicar**. El stock se actualiza y el lote queda registrado.

> 💡 **Tip:** Si recibís 50 unidades del mismo producto pero de **2 lotes distintos**, hacé dos ajustes separados — uno por cada lote con su fecha de vencimiento.

---

### Ver los lotes de un producto

#### Paso 1: Abrir la vista de lotes

En la tabla de inventario, haz clic en **📦 Lotes** en la fila del producto que quieres revisar.

#### Paso 2: Revisar el semáforo

Se abre una ventana con un resumen y una tabla de todos los lotes:

**Resumen en la parte superior:**
- **Total lotes** — Cuántos lotes diferentes tienes
- **Stock total** — La suma de unidades de todos los lotes
- **Lotes críticos** — Cuántos lotes están por vencer o ya vencieron

**Tabla de lotes:**

| Columna | Qué muestra |
|---------|-------------|
| **Lote** | Número de lote (o "Sin lote" si no se registró) |
| **Vencimiento** | Fecha de caducidad |
| **Cantidad** | Unidades disponibles en ese lote |
| **Estado** | Semáforo de vencimiento (ver abajo) |

---

## 🚦 Semáforo de Vencimiento

El semáforo te indica de un vistazo qué tan urgente es vender un lote:

| Color | Significado | Cuándo aparece |
|-------|-------------|----------------|
| 🟢 **OK** | Todo bien, queda tiempo de sobra | Más de 30 días para vencer, o sin fecha |
| 🟡 **Por vencer** | ¡Atención! Ponlo al frente para venderlo primero | Entre 7 y 30 días para vencer |
| 🔴 **Crítico** | ¡Urgente! Vence esta semana | Menos de 7 días para vencer |
| ❌ **Vencido** | Ya caducó, no se debe vender | La fecha de vencimiento ya pasó |

---

## 🗑️ Eliminar un lote vacío

Si un lote quedó con **cantidad 0** (ya se vendió todo), puedes eliminarlo para mantener la lista limpia:

1. Abre **📦 Lotes** del producto
2. Busca el lote con cantidad 0
3. Haz clic en **🗑️ Eliminar**
4. Confirma la eliminación

> ⚠️ Solo puedes eliminar lotes que tienen cantidad 0.

---

## 🔍 Filtrar productos por vencimiento

En la página de Inventario, usa el botón **⏰ Por Vencer** en los filtros de la parte superior. Esto te muestra solo los productos que tienen al menos un lote que vence dentro de los próximos 30 días.

---

## ❓ Preguntas frecuentes

### ¿Es obligatorio llenar el lote y la fecha de vencimiento?
No. Ambos campos son opcionales. Si vendes productos que no caducan (tornillos, ropa, etc.), simplemente no los llenes.

### ¿Puedo usar solo el número de lote sin la fecha de vencimiento?
Sí. Es útil cuando quieres rastrear de qué proveedor vino el producto pero no necesitas controlar la caducidad.

### ¿Puedo usar solo la fecha de vencimiento sin el número de lote?
También. Si el producto caduca pero no tiene un número de lote formal impreso.

### ¿Qué pasa si ingreso el mismo número de lote dos veces?
El sistema es inteligente: si ya existe un lote con ese número para el mismo producto, **suma la cantidad** al lote existente en vez de crear uno nuevo.

### ¿El semáforo se actualiza solo?
Sí. Cada vez que abres la vista de lotes, el sistema calcula el estado basándose en la fecha actual. Un lote que hoy está en 🟢 verde, pasará automáticamente a 🟡 amarillo cuando falten 30 días.

### ¿Puedo editar un lote que ya registré?
No directamente. Si necesitas corregir datos de un lote, puedes hacer un ajuste negativo para quitar la cantidad y luego un nuevo ajuste positivo con los datos correctos.

---

## 💡 Tips

- 🏷️ Usa un formato consistente para tus lotes (ej: `LOTE-2026-01`, `LAB-A123`) para encontrarlos fácil
- 📋 Revisa el filtro **⏰ Por Vencer** al menos una vez por semana para evitar que se te pase un producto
- 📊 Cuando recibas mercadería, registra cada lote por separado aunque sea del mismo producto — así el semáforo funciona correctamente
- 🔄 Vende primero los lotes más próximos a vencer (esto se llama FEFO: First Expired, First Out)

---

## ⚠️ Notas importantes

- Los lotes son **por producto** — cada producto tiene su propia lista de lotes
- El semáforo se calcula en el momento de abrir la ventana, con la fecha actual
- Si un lote aparece como ❌ Vencido, **no deberías venderlo** — revisa con tu proveedor si aplica devolución
- Los campos de lote y vencimiento también quedan registrados en el **📜 Historial** del producto
