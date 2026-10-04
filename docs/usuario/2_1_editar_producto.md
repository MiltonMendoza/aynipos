# ✏️ Editar Producto

> Modifica la información de un producto existente: nombre, precios, categoría, código de barras y más.

> [!NOTE]
> **Nota de Configuración:** Los campos de **Dosis** y **Proveedor** son opcionales. Si tu rubro de negocio no los requiere, puedes desactivarlos en la pantalla de **Configuración ⚙️** para mantener este formulario limpio y simplificado.

---

## ¿Qué puedo hacer con esto?

Cuando necesites **corregir o actualizar** los datos de un producto que ya está registrado, puedes editarlo sin tener que borrarlo y crearlo de nuevo. Por ejemplo:

- Corregir el nombre o el SKU de un producto
- Cambiar el precio de compra o de venta
- Asignar o cambiar la categoría
- Agregar o corregir el código de barras
- Ajustar el stock mínimo para las alertas de "bajo stock"
- Agregar una descripción al producto

---

## ¿Cómo se usa?

### Paso 1: Ir a Inventario

En el menú lateral izquierdo, haz clic en **📦 Inventario** para abrir la tabla de productos.

### Paso 2: Buscar el producto

Ubica el producto que quieres editar en la tabla. Puedes usar los filtros de la parte superior (📋 Todos, ⚠️ Bajo Stock, ⏰ Por Vencer) para encontrarlo más fácil.

### Paso 3: Abrir el editor

En la fila del producto, haz clic en el botón **✏️ Editar** en la columna de Acciones.

Se abrirá una ventana (modal) con todos los datos actuales del producto ya cargados.

### Paso 4: Modificar los campos

Cambia los datos que necesites. Los campos disponibles son:

| Campo | ¿Obligatorio? | Descripción |
|-------|:---:|-------------|
| **SKU** | ✅ | Código interno del producto (ej: P001) |
| **Código de barras** | ❌ | El número del código de barras del empaque |
| **Nombre del producto** | ✅ | Nombre que aparece en la caja y el carrito |
| **Categoría** | ✅ | Grupo al que pertenece (ej: Medicamentos) |
| **Precio Compra (Bs)** | ✅ | Cuánto te cuesta el producto |
| **Precio Venta (Bs)** | ✅ | Cuánto le cobras al cliente |
| **Unidad** | ❌ | Tipo de unidad (ej: unidad, caja, blíster) |
| **Stock mínimo** | ❌ | Cuántas unidades mínimas quieres tener. Si baja de este número, aparece la alerta "Bajo" |
| **Dosis** | ❌ | Concentración o dosificación (ej: 500mg, 10ml). *Solo visible si está activo en Configuración.* |
| **Proveedor** | ❌ | El distribuidor asociado al producto. *Solo visible si está activo en Configuración.* |
| **Descripción** | ❌ | Texto libre para describir el producto |

### Paso 5: Guardar los cambios

Haz clic en el botón **💾 Guardar Cambios** en la parte inferior de la ventana.

Si todo está correcto, la ventana se cierra y la tabla se actualiza automáticamente con los nuevos datos.

---

## 🚫 Validaciones

Si falta algún dato obligatorio o hay un error, verás un mensaje en **rojo** debajo del campo que tiene el problema. Por ejemplo:

- _"El SKU es obligatorio"_ — si dejaste el SKU vacío
- _"El nombre es obligatorio"_ — si dejaste el nombre vacío
- _"La categoría es obligatoria"_ — si no seleccionaste una categoría
- _"El precio de compra debe ser mayor a 0"_ — si el precio es 0 o negativo
- _"El precio de venta debe ser mayor o igual al de compra"_ — si el precio de venta es menor que el de compra
- _"Ya existe un producto con ese código de barras: [nombre]"_ — si otro producto ya tiene ese código de barras

El error desaparece automáticamente cuando corriges el campo.

---

## ❓ Preguntas frecuentes

### ¿Puedo cambiar el stock desde aquí?
No. El stock se ajusta desde el botón **📊 Ajustar** en la misma fila del producto. El formulario de edición solo cambia los datos del producto, no su cantidad en inventario.

### ¿Puedo poner el mismo código de barras en dos productos?
No. El sistema no lo permite porque causaría problemas con el lector de código de barras en la pantalla de ventas. Si intentas guardar un código que ya tiene otro producto, verás un error indicando cuál producto ya lo usa.

### ¿Qué pasa si cambio el precio de venta?
El nuevo precio se aplica a las **ventas futuras**. Las ventas que ya se hicieron mantienen el precio que tenían en ese momento.

### ¿Puedo cerrar la ventana sin guardar?
Sí. Puedes cerrar haciendo clic en la **✕** de la esquina, en el botón **Cancelar**, o haciendo clic fuera de la ventana. Los cambios no se guardan hasta que presiones **💾 Guardar Cambios**.

### ¿Qué es el "Stock mínimo"?
Es la cantidad mínima que quieres tener de ese producto. Si el stock actual baja de ese número, en la tabla de inventario aparece un indicador rojo **"Bajo"** para avisarte que necesitas reabastecer.

---

## 💡 Tips

- 📝 Usa la **descripción** para anotar datos útiles como el laboratorio, presentación o ingredientes activos
- 🏷️ Mantén los **SKU** con un formato consistente (ej: MED-001, VIT-002) para encontrar productos más rápido
- 📊 Configura el **stock mínimo** en cada producto para recibir alertas antes de quedarte sin inventario
- 🔍 Si un producto no aparece con el lector de código de barras, revisa que tenga el código correcto en este formulario

---

## ⚠️ Notas importantes

- Los cambios son **inmediatos** — una vez que haces clic en Guardar, el producto se actualiza para todos
- Cambiar el nombre o precio **no afecta** las ventas ya realizadas
- Si necesitas quitar un producto del sistema, usa la opción de eliminar (soft delete), no borres los datos editando
