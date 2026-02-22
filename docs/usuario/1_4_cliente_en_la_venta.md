# 👤 Cliente en la Venta

> Asocia un cliente (nombre, NIT/CI) a cada venta directamente desde la caja, con buscador rápido y creación de clientes nuevos sin salir de la pantalla de ventas.

---

## ¿Qué puedo hacer con esto?

Cada vez que realizas una venta, puedes **asociar un cliente** con su nombre y NIT. Esto sirve para:

- 📄 **Facturas** — El NIT del cliente es necesario para emitir facturas (preparación para facturación electrónica)
- 📊 **Historial** — Ver qué cliente compró qué, en el historial de ventas
- 🔁 **Clientes frecuentes** — Los clientes que agregues quedan guardados y puedes buscarlos rápidamente en futuras ventas

Si no seleccionas ningún cliente, la venta se registra como **"Sin Nombre"** con NIT **0** (esto es normal para ventas sin factura).

---

## ¿Cómo se usa?

### Seleccionar un cliente existente

#### Paso 1: Ubica la sección de cliente
En el panel derecho (el carrito), justo debajo del título **🛒 Carrito**, verás una barra que muestra:

```
👤 Sin Nombre
   NIT: 0                    [+ Cliente (F3)]
```

Esta es la sección de cliente. Por defecto dice "Sin Nombre".

#### Paso 2: Abrir el buscador
Haz clic en el botón **"+ Cliente (F3)"** o presiona la tecla **F3** en el teclado.

Se abrirá un campo de búsqueda que dice *"🔍 Buscar por nombre, NIT o teléfono..."*.

#### Paso 3: Buscar al cliente
Escribe parte del **nombre**, **NIT** o **teléfono** del cliente. Los resultados aparecen automáticamente mientras escribes.

Por ejemplo:
- Escribir `Juan` mostrará todos los clientes que se llamen Juan
- Escribir `123456` mostrará clientes con ese NIT
- Escribir `77712` mostrará clientes con ese número de teléfono

#### Paso 4: Seleccionar al cliente
Haz clic en el cliente correcto en la lista de resultados. La sección de cliente se actualizará mostrando el nombre y NIT del cliente seleccionado:

```
👤 Juan Pérez
   NIT: 12345678              [✕] [Cambiar (F3)]
```

¡Listo! Ahora cuando completes la venta, quedará asociada a este cliente.

---

### Crear un cliente nuevo (sin salir de la caja)

Si el cliente no existe en el sistema, puedes crearlo rápidamente sin ir a la página de Clientes.

#### Paso 1: Abrir el buscador
Presiona **F3** o haz clic en **"+ Cliente"**.

#### Paso 2: Hacer clic en "Crear nuevo cliente"
En la parte de abajo del buscador, haz clic en el botón **"➕ Crear nuevo cliente"** (texto azul).

> 💡 **Tip:** Si ya escribiste el nombre en el buscador y no encontró resultados, el nombre se copiará automáticamente al formulario de creación.

#### Paso 3: Llenar los datos
Se muestra un formulario rápido con dos campos:
- **Nombre del cliente** *(obligatorio)* — El nombre completo del cliente
- **NIT / CI** *(opcional)* — El número de NIT o cédula de identidad

#### Paso 4: Guardar
Haz clic en **"Guardar"** o presiona **Enter**. Verás un mensaje de confirmación:

```
👤 Cliente "Juan Pérez" creado
```

El cliente queda seleccionado automáticamente para la venta actual y guardado en el sistema para futuras ventas.

---

### Cambiar o quitar el cliente seleccionado

- **Para cambiar:** Haz clic en **"Cambiar (F3)"** o presiona **F3** y busca otro cliente
- **Para quitar:** Haz clic en el botón **✕** rojo que aparece al lado del nombre. La venta volverá a "Sin Nombre"

---

### ¿Dónde se ve el cliente al cobrar?

Cuando abres la pantalla de pago (con **F2** o el botón **💰 Cobrar**), verás el nombre y NIT del cliente seleccionado justo arriba del total a cobrar:

```
👤 Juan Pérez    NIT: 12345678
        Total a cobrar
         Bs 150.00
```

Esto te permite verificar que el cliente es correcto antes de confirmar la venta.

---

## ⌨️ Atajos de teclado

| Atajo | Acción |
|-------|--------|
| **F3** | Abrir o cerrar el buscador de clientes |
| **Esc** | Cerrar el buscador o el formulario de creación |
| **Enter** | Guardar el cliente nuevo (cuando estás en el formulario de creación) |

---

## 📋 ¿Dónde veo el cliente de ventas anteriores?

En la página **📋 Historial de Ventas**:

1. En la **tabla de ventas** verás una nueva columna **"Cliente"** que muestra el nombre del cliente de cada venta
2. Al hacer clic en una venta para ver su detalle, verás el nombre del cliente en la parte superior, justo debajo del número de venta

Las ventas sin cliente asociado muestran **"Sin Nombre"**.

---

## ❓ Preguntas frecuentes

### ¿Es obligatorio seleccionar un cliente?
No. Si no seleccionas ningún cliente, la venta se registra como "Sin Nombre" con NIT 0. Esto es completamente normal para ventas al público en general sin factura.

### ¿Qué pasa con el cliente si limpio el carrito?
Se quita también. Al limpiar el carrito (con **F4** o el botón "Limpiar"), el cliente vuelve a "Sin Nombre" y empiezas de cero.

### ¿Puedo cambiar el cliente después de completar la venta?
No. Una vez que confirmas la venta, el cliente queda registrado permanentemente. Asegúrate de verificar el cliente correcto antes de cobrar.

### ¿El cliente nuevo que creo se guarda para siempre?
Sí. Los clientes que creas desde la caja quedan guardados en el sistema. Puedes verlos y editarlos desde la página **👥 Clientes** del menú lateral. También aparecerán en futuras búsquedas desde la caja.

### ¿Puedo crear un cliente sin NIT?
Sí. Solo el nombre es obligatorio. El NIT/CI es opcional. Puedes agregarlo después desde la página de Clientes.

### ¿Puedo buscar por teléfono?
Sí. El buscador busca por nombre, NIT **y** teléfono al mismo tiempo. Si el cliente fue registrado con su número de teléfono, puedes encontrarlo así.

---

## 💡 Tips

- ⚡ Usa **F3** para abrir el buscador rápidamente sin usar el mouse — ideal para no perder velocidad en la caja
- 🔍 No necesitas escribir el nombre completo — con las primeras letras ya aparecen los resultados
- 📱 Si un cliente te da su NIT, puedes buscarlo directamente escribiendo el número de NIT
- ➕ Si es un cliente nuevo, busca primero su nombre — si no existe, haz clic en "Crear nuevo" y el nombre ya estará copiado en el formulario
- 🔁 Los clientes más frecuentes aparecen rápido — solo escribe las primeras letras de su nombre

---

## ⚠️ Notas importantes

- El cliente se asocia a la **venta completa**, no a productos individuales
- Si necesitas corregir los datos de un cliente (nombre, NIT, teléfono), ve a la página **👥 Clientes** desde el menú lateral
- Al anular una venta, la asociación con el cliente se mantiene en el registro (para trazabilidad)
- Tener el NIT correcto del cliente será importante cuando se active la **facturación electrónica** en una futura actualización
