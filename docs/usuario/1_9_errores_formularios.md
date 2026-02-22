# 🚫 Errores en Formularios

> Mensajes claros debajo de cada campo que te indican exactamente qué falta o qué está incorrecto.

## ¿Qué puedo hacer con esto?

Cada vez que llenás un formulario en AyniPOS (crear un producto, agregar un cliente, abrir caja, etc.), el sistema revisa que los datos estén completos. Si algo falta o está mal, aparece un **mensaje en rojo debajo del campo** explicándote exactamente qué necesitás corregir. Ya no tenés que adivinar qué está mal — el sistema te lo dice.

## ¿Dónde aparecen estos errores?

Los errores inline aparecen en **todos los formularios** de la app:

- ➕ Crear producto (Inventario)
- ✏️ Editar producto (Inventario)
- 📊 Ajustar inventario
- ➕ Crear categoría (Inventario)
- 👤 Crear/editar cliente (Clientes)
- 👤 Crear cliente rápido (desde el POS)
- 💰 Abrir caja (Configuración)
- 🔒 Cerrar caja (Configuración)
- 💵 Cobrar venta (POS)

## ¿Cómo se usa?

### Paso 1: Completá el formulario

Abrí cualquier formulario (por ejemplo, **➕ Nuevo Producto** desde la página de Inventario) y llenalo con los datos.

### Paso 2: Presioná "Guardar"

Si faltan datos obligatorios o hay algo incorrecto, el botón **Guardar** no hará nada excepto mostrar los errores. No se pierde información — todo lo que ya escribiste se mantiene.

### Paso 3: Mirá los mensajes en rojo

Cada campo con problemas mostrará:
1. Un **borde rojo** alrededor del campo
2. Un **mensaje explicativo** en rojo debajo del campo

Por ejemplo, al crear un producto sin completar los datos obligatorios podrías ver:

| Campo | Mensaje de error |
|-------|-----------------|
| SKU | "El SKU es obligatorio" |
| Nombre | "El nombre es obligatorio" |
| Categoría | "La categoría es obligatoria" |
| Precio de compra | "El precio de compra debe ser mayor a 0" |
| Precio de venta | "El precio de venta debe ser mayor a 0" |

### Paso 4: Corregí y el error desaparece solo

En cuanto empezás a escribir en el campo corregido, el mensaje de error **desaparece automáticamente**. No necesitás hacer nada más — solo corregí el dato y el campo vuelve a su estado normal.

### Paso 5: Intentá guardar de nuevo

Una vez que todos los campos estén correctos, presioná **Guardar** nuevamente y la acción se completará exitosamente.

## Ejemplos de errores por formulario

### ➕ Crear/Editar Producto

| Campo | Error | ¿Cuándo aparece? |
|-------|-------|-------------------|
| SKU | "El SKU es obligatorio" | Si dejás el campo vacío |
| Nombre | "El nombre es obligatorio" | Si dejás el campo vacío |
| Categoría | "La categoría es obligatoria" | Si no seleccionás ninguna |
| Precio compra | "El precio de compra debe ser mayor a 0" | Si ponés 0 o un valor negativo |
| Precio venta | "El precio de venta debe ser mayor a 0" | Si ponés 0 o un valor negativo |
| Precio venta | "El precio de venta debe ser mayor o igual al de compra" | Si el precio de venta es menor que el de compra |
| Código de barras | Aviso de duplicado | Si otro producto ya tiene ese mismo código |

### 👤 Crear/Editar Cliente

| Campo | Error | ¿Cuándo aparece? |
|-------|-------|-------------------|
| Nombre | "El nombre es obligatorio" | Si dejás el campo vacío |
| Email | "El formato de email no es válido" | Si el email está mal escrito (ej: falta el @) |

### ➕ Crear Categoría

| Campo | Error | ¿Cuándo aparece? |
|-------|-------|-------------------|
| Nombre | "El nombre de la categoría es obligatorio" | Si dejás el campo vacío |

### 📊 Ajustar Inventario

| Campo | Error | ¿Cuándo aparece? |
|-------|-------|-------------------|
| Cantidad | "La cantidad no puede ser 0" | Si dejás la cantidad en 0 |

### 💰 Abrir/Cerrar Caja

| Campo | Error | ¿Cuándo aparece? |
|-------|-------|-------------------|
| Monto | "El monto debe ser mayor o igual a 0" | Si ponés un número negativo |

### 💵 Cobrar Venta (Efectivo)

| Campo | Error | ¿Cuándo aparece? |
|-------|-------|-------------------|
| Monto recibido | "El monto recibido es menor al total" | Si el dinero recibido no alcanza para cubrir el total |

## Preguntas frecuentes

### ¿Puedo guardar sin completar los campos obligatorios?

No. Los campos marcados con asterisco (*) son obligatorios. El sistema no te deja guardar hasta que estén correctos. Esto es para evitar que se guarden datos incompletos que podrían causar problemas después.

### ¿Pierdo los datos que ya escribí cuando aparece un error?

No. Todo lo que ya escribiste se mantiene. Solo necesitás corregir el campo que tiene error y volver a presionar Guardar.

### ¿Cómo sé cuáles campos son obligatorios?

Los campos obligatorios tienen un asterisco (*) al lado del nombre del campo. Por ejemplo: **"Nombre *"**, **"SKU *"**, **"Categoría *"**.

### ¿El mensaje de error desaparece solo?

Sí. En cuanto empezás a corregir el campo (escribir, seleccionar una opción, etc.), el mensaje de error desaparece automáticamente. No necesitás cerrar nada ni recargar la página.

### ¿Puedo ver varios errores al mismo tiempo?

Sí. Si hay varios campos con problemas, **todos los errores aparecen al mismo tiempo**, cada uno debajo de su campo correspondiente. Así podés corregir todo de una vez sin tener que ir uno por uno.

### ¿Los errores aparecen con algún sonido?

No, los errores inline en formularios son silenciosos. Solo se resaltan visualmente con el borde rojo y el mensaje. Sin embargo, los errores durante la venta (como intentar vender sin stock) sí tienen un sonido de alerta.

## Notas importantes

- 🔴 Los campos con error se resaltan con un **borde rojo** para que sean fáciles de identificar
- ✏️ Los mensajes de error **desaparecen solos** al corregir el campo — no necesitás hacer nada extra
- 📋 Todos los errores aparecen **al mismo tiempo**, así podés corregir todo de una sola vez
- 💾 Los datos que ya escribiste **no se pierden** al mostrar errores
- ⭐ Los campos con asterisco (*) son los **obligatorios** — no podés guardar sin completarlos
- 🎯 Los mensajes están escritos en español claro para que sepas exactamente qué corregir
