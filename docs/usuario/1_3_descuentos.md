# 💰 Descuentos por Ítem y Globales

> Aplica descuentos por porcentaje o monto fijo a productos individuales, o un descuento general al total de la venta.

---

## ¿Qué puedo hacer con esto?

Puedes aplicar **dos tipos de descuentos** durante una venta:

1. **Descuento por producto** — Rebaja el precio de un producto específico en el carrito (ej: "10% de descuento en este jarabe")
2. **Descuento global** — Rebaja el total de toda la venta (ej: "Bs 5 de descuento al total")

Cada tipo de descuento puede ser:
- **Porcentaje (%)** — Por ejemplo: 10%, 15%, 50%
- **Monto fijo (Bs)** — Por ejemplo: Bs 2.00, Bs 5.50, Bs 10.00

Los dos tipos de descuento se pueden combinar: puedes tener productos con descuento individual **y** un descuento global a la vez.

---

## ¿Cómo se usa?

### Descuento por producto

#### Paso 1: Agregar productos al carrito
Agrega los productos que el cliente quiere comprar, como lo haces normalmente.

#### Paso 2: Abrir el editor de descuento
En el carrito (panel derecho), busca el producto al que quieres aplicar descuento. Haz clic en el botón **"% Desc."** que aparece al lado del nombre del producto.

#### Paso 3: Elegir el tipo de descuento
Se abre un editor con dos botones:
- **%** — Para descuento por porcentaje
- **Bs** — Para descuento por monto fijo

Haz clic en el que necesites. El botón seleccionado se pone **azul**.

#### Paso 4: Ingresar el valor
Escribe el valor del descuento:
- Si elegiste **%**: escribe el porcentaje (ej: `10` para 10%)
- Si elegiste **Bs**: escribe el monto en bolivianos (ej: `5` para Bs 5.00)

El descuento se aplica **automáticamente** mientras escribes. Verás cómo cambia el subtotal del producto.

#### Paso 5: Verificar
En la línea del producto verás:
- El precio original tachado con el texto **"· Desc. −Bs X.XX"** en amarillo
- El nuevo subtotal ya con el descuento aplicado

Para **quitar** el descuento, haz clic en el botón **✕** rojo dentro del editor.

---

### Descuento global (al total de la venta)

#### Paso 1: Agregar productos al carrito
Agrega todos los productos de la venta.

#### Paso 2: Abrir el descuento global
En la zona de totales (abajo del carrito), haz clic en el botón **"+ Agregar descuento global"** (texto amarillo).

#### Paso 3: Elegir tipo y valor
Igual que el descuento por producto:
- Selecciona **%** o **Bs**
- Escribe el valor del descuento

#### Paso 4: Verificar
Verás una línea nueva en los totales:
- **"Descuento global"** con el monto en amarillo (ej: **−Bs 10.00**)
- El **Total** se actualiza automáticamente restando el descuento

Para **quitar** el descuento global, haz clic en el botón **✕** rojo al lado del campo.

---

## 📊 ¿Cómo se ven los totales con descuentos?

Cuando hay descuentos aplicados, la zona de totales del carrito muestra:

| Línea | Ejemplo |
|-------|---------|
| Subtotal | Bs 100.00 |
| Desc. por ítems | −Bs 8.00 |
| IVA (13%) | Bs 11.96 |
| Descuento global | −Bs 5.00 |
| **Total** | **Bs 87.00** |

Las líneas de descuento solo aparecen cuando hay descuentos activos. Sin descuentos, se ve igual que antes.

---

## 📋 ¿Dónde veo los descuentos de ventas anteriores?

En la página **📋 Historial de Ventas**:
1. Haz clic en una venta para ver su detalle
2. Si la venta tuvo descuentos por producto, verás **"· Desc. −Bs X.XX"** en cada producto afectado
3. Si hubo descuento global, verás una línea **"Descuento"** entre el Subtotal y el IVA

---

## ❓ Preguntas frecuentes

### ¿Puedo aplicar descuento por producto Y descuento global a la vez?
Sí. Primero se aplican los descuentos individuales a cada producto, y luego el descuento global se resta del subtotal resultante.

### ¿Qué pasa si pongo un porcentaje mayor a 100%?
AyniPOS lo limita automáticamente al 100%. Lo mismo con montos fijos: no puede ser mayor al precio total del producto (o al subtotal para descuentos globales).

### ¿Qué pasa si cambio la cantidad de un producto que ya tiene descuento?
El subtotal se recalcula manteniendo el monto de descuento fijo. Si necesitas ajustar el descuento, vuelve a abrir el editor con **"% Desc."** y cambia el valor.

### ¿Los descuentos se guardan en la venta?
Sí. Al completar la venta, tanto los descuentos por producto como el descuento global quedan registrados y son visibles en el Historial de Ventas.

### ¿Se puede poner descuento después de abrir la pantalla de pago?
No. Primero cierra la pantalla de pago con **Esc** o **Cancelar**, aplica el descuento, y luego vuelve a cobrar con **F2**.

### ¿Qué pasa con el descuento si limpio el carrito?
Se borra todo: los productos, los descuentos por ítem y el descuento global.

---

## 💡 Tips

- 🏷️ Usa **descuento por producto** cuando un artículo específico tiene promoción o está próximo a vencer
- 🧾 Usa **descuento global** para descuentos por volumen o cortesías al total de la compra
- ⚡ Para aplicar descuento rápido: haz clic en **"% Desc."** → escribe el porcentaje → listo, no necesitas confirmar nada
- 🔄 Si te equivocaste, solo presiona el botón **✕** rojo para quitar el descuento sin perder la venta

---

## ⚠️ Notas importantes

- Los descuentos **no se pueden editar** después de completar la venta — asegúrate de verificar los montos antes de cobrar
- El descuento global se muestra como un monto fijo en la venta, aunque lo hayas ingresado como porcentaje
- El IVA se calcula **después** de aplicar los descuentos por producto, pero **antes** del descuento global
