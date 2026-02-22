# ✅ Validaciones Robustas

> El sistema verifica automáticamente que todo esté correcto antes de procesar una venta, crear un producto o realizar cualquier operación.

## ¿Qué puedo hacer con esto?

Las validaciones son controles automáticos que AyniPOS realiza por vos para evitar errores. No necesitás hacer nada especial — el sistema te avisa cuando algo no está bien y te dice exactamente qué corregir. Así evitás problemas como vender sin stock, cobrar sin tener la caja abierta, o crear productos con datos incompletos.

## Validaciones en el Punto de Venta (POS)

### 🚫 No se puede vender si no hay stock

Si un producto tiene **stock = 0**, su tarjeta aparece atenuada (más transparente) con un borde rojo y la etiqueta **"Sin stock"**. No se puede hacer clic en ese producto.

Si intentás agregarlo de todas formas (por ejemplo, con el lector de código de barras), aparecerá el mensaje:

> ❌ Sin stock disponible para "[nombre del producto]"

### 📦 Alerta si supera el stock disponible

Si intentás poner más unidades de las que hay en inventario, el sistema te avisa:

> ⚠️ Solo hay [X] unidades disponibles de "[nombre del producto]"

Esto también funciona cuando ajustás la cantidad con los botones **+** y **−** o desde el campo de cantidad en el carrito.

### 🔢 Confirmación para cantidades grandes

Si intentás agregar más de **50 unidades** de un producto, el sistema te pregunta:

> ¿Seguro que desea agregar [X] unidades de "[nombre del producto]"?

Esto es para prevenir errores de tipeo. Podés confirmar o cancelar.

### 💰 Caja cerrada = no se puede cobrar

Si la caja registradora no está abierta, verás una etiqueta amarilla **"⚠️ Caja cerrada"** junto a la barra de búsqueda. Si intentás cobrar (presionando F2 o el botón Cobrar), aparecerá:

> ⚠️ Abre la caja registradora antes de cobrar (Configuración → Abrir Caja)

### 💵 Monto recibido insuficiente

Al cobrar en **efectivo**, si el monto recibido es menor al total de la venta, aparecerá un mensaje de error debajo del campo:

> El monto recibido es menor al total

No se podrá completar la venta hasta que corrijas el monto.

### 🗑️ Limpieza del carrito con confirmación

Para evitar borrar el carrito por accidente, presionar **F4** requiere **doble pulsación**:
1. La primera vez: aparece un aviso amarillo *"Presiona F4 de nuevo para limpiar el carrito"*
2. La segunda vez (dentro de 3 segundos): se limpia el carrito

## Validaciones al Crear/Editar Productos

Cuando creás o editás un producto, el sistema verifica que los datos estén completos y correctos:

| Campo | Validación | Mensaje de error |
|-------|-----------|-----------------|
| SKU | Obligatorio | "El SKU es obligatorio" |
| Nombre | Obligatorio | "El nombre es obligatorio" |
| Categoría | Obligatoria | "La categoría es obligatoria" |
| Precio de compra | Debe ser mayor a 0 | "El precio de compra debe ser mayor a 0" |
| Precio de venta | Debe ser mayor a 0 | "El precio de venta debe ser mayor a 0" |
| Precio de venta | Debe ser ≥ precio de compra | "El precio de venta debe ser mayor o igual al de compra" |
| Código de barras | No puede estar duplicado | Te avisa si otro producto ya tiene ese código |

> 💡 **Tip:** Los mensajes de error aparecen en **rojo debajo de cada campo** que tiene problemas. Cuando empezás a corregir el campo, el mensaje desaparece automáticamente.

## Validaciones al Crear Categorías

| Campo | Validación | Mensaje de error |
|-------|-----------|-----------------|
| Nombre | Obligatorio | "El nombre de la categoría es obligatorio" |

## Validaciones al Ajustar Inventario

| Campo | Validación | Mensaje de error |
|-------|-----------|-----------------|
| Cantidad | No puede ser 0 | "La cantidad no puede ser 0" |

## Validaciones de Clientes

| Campo | Validación | Mensaje de error |
|-------|-----------|-----------------|
| Nombre | Obligatorio | "El nombre es obligatorio" |
| Email | Formato válido (si se llena) | "El formato de email no es válido" |

> 💡 **Tip:** Al crear un cliente rápido desde el POS, también se valida que el nombre no esté vacío.

## Validaciones en la Caja Registradora

| Acción | Validación | Mensaje de error |
|--------|-----------|-----------------|
| Abrir caja | El monto inicial no puede ser negativo | "El monto debe ser mayor o igual a 0" |
| Cerrar caja | El monto final no puede ser negativo | "El monto debe ser mayor o igual a 0" |

## Validaciones de Eliminación

Antes de eliminar datos, el sistema siempre pide confirmación:
- **Eliminar cliente:** "¿Eliminar este cliente?"
- **Eliminar lote vacío:** "¿Eliminar este lote vacío?"
- **Anular venta:** "¿Estás seguro de anular esta venta?"

> 🔒 Solo podés eliminar lotes que tengan **0 unidades** — no se pueden borrar lotes con stock.

## ¿Cómo se ven los errores?

AyniPOS usa dos formas de mostrar errores:

### 1. Mensajes debajo del campo (errores inline)
Cuando un formulario tiene datos incorrectos o incompletos:
- El campo con error se resalta con un **borde rojo**
- Debajo aparece un **mensaje en rojo** explicando qué falta o qué está mal
- Al corregir el campo, el mensaje **desaparece solo**

### 2. Notificaciones emergentes (toasts)
Para errores durante la venta:
- 🟢 **Verde** = acción exitosa
- 🟡 **Amarillo** = advertencia (stock bajo, caja cerrada)
- 🔴 **Rojo** = error (sin stock, error del sistema)

Las notificaciones desaparecen solas después de 3 segundos.

## Preguntas frecuentes

### ¿Puedo ignorar las validaciones?

No, las validaciones obligatorias no se pueden saltar. Están ahí para proteger tu negocio de errores. Por ejemplo, no podrás vender un producto sin stock ni cobrar con la caja cerrada.

### ¿Puedo vender parcialmente si no tengo suficiente stock?

Sí, podés vender la cantidad que tengas disponible. Solo se bloquea cuando intentás vender **más** de lo que hay en inventario.

### ¿Qué pasa si pongo precio de venta menor al de compra?

El sistema te lo impide con el mensaje *"El precio de venta debe ser mayor o igual al de compra"*. Esto es para evitar que vendas a pérdida por error.

### ¿El sonido de error es diferente al de éxito?

Sí, cuando ocurre un error se reproduce un sonido distinto al de cuando agregás un producto o completás una venta. Esto te da una señal auditiva inmediata de que algo no está bien.

## Notas importantes

- 🎯 Las validaciones trabajan **automáticamente** — no necesitás activarlas
- 🔴 Los campos con error se resaltan en rojo para que los identifiques fácilmente
- ✏️ Al corregir un campo, el mensaje de error desaparece solo — no necesitás reiniciar nada
- 🔊 Los errores también tienen un sonido distinto para que los notes incluso sin mirar la pantalla
- ⏱️ Las notificaciones emergentes desaparecen solas después de 3 segundos
- 🔒 Las validaciones de eliminación siempre piden confirmación para evitar borrados accidentales
