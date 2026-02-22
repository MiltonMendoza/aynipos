# 📡 Lector de Código de Barras

> Escanea productos con un lector de código de barras y se agregan automáticamente al carrito.

---

## ¿Qué puedo hacer con esto?

Puedes usar un **lector de código de barras** (USB o Bluetooth) para agregar productos al carrito de forma instantánea. Solo necesitas escanear el código del producto y AyniPOS lo busca automáticamente y lo agrega al carrito.

También funciona con tu **celular Android** usando una app de escaneo por Bluetooth.

---

## ¿Qué necesito?

- Un lector de código de barras USB **o** un celular Android con la app **"BT Scanner"**
- Que los productos tengan su **código de barras registrado** en el sistema (se configura al crear o editar el producto en Inventario)

---

## ¿Cómo se usa?

### Con lector USB (enchufar y listo)

#### Paso 1: Conectar el lector
Conecta el lector de código de barras al puerto USB de tu computadora. No necesitas instalar nada, funciona automáticamente.

#### Paso 2: Ir a la página de Ventas (POS)
Abre AyniPOS y ve a la página **POS** (Punto de Venta). El buscador🔍 se enfoca automáticamente para recibir el escaneo.

#### Paso 3: Escanear el producto
Apunta el lector al código de barras del producto y presiona el botón de escaneo. Verás:

1. El código aparece brevemente en el buscador
2. El indicador **"📡 Escaneando..."** se muestra por un momento
3. Si el producto se encuentra, se agrega al carrito con un sonido de confirmación ✅
4. El buscador se limpia automáticamente, listo para el siguiente escaneo

#### Paso 4: Repetir
Escanea todos los productos que el cliente quiera comprar. Si escaneas el mismo producto dos veces, la cantidad se incrementa automáticamente.

---

### Con celular Android (Bluetooth)

#### Paso 1: Instalar la app
En tu celular Android, descarga la app **"Bluetooth Barcode Scanner"** desde Google Play Store.

#### Paso 2: Conectar por Bluetooth
1. En tu celular: abre la app y ve a **Configuración** → selecciona modo **HID (Keyboard)**
2. En tu computadora: ve a **Configuración → Bluetooth** y activa el Bluetooth
3. Desde la app, conecta con tu computadora

#### Paso 3: Configurar la app
Asegúrate de tener estas opciones activadas:

| Opción | Valor |
|--------|-------|
| Modo de salida | HID Keyboard |
| Enviar Enter después del escaneo | ✅ Activado |

#### Paso 4: Escanear
1. Abre la app en tu celular
2. En AyniPOS, ve a la página **POS** y asegúrate de que el buscador🔍 esté seleccionado
3. Escanea el código de barras con la cámara de tu celular
4. El producto se agrega automáticamente al carrito

---

## ❓ Preguntas frecuentes

### ¿Qué pasa si escaneo un código que no está registrado?
AyniPOS buscará el código como texto normal en el buscador. Si no encuentra ningún producto con ese código, verás el mensaje **"No se encontraron productos"** en la zona de productos.

**Solución:** Ve a **Inventario**, edita el producto y agrega el código de barras correcto en el campo "Código de barras".

### ¿Qué pasa si el producto no tiene stock?
Verás el mensaje **"❌ Sin stock disponible"** y el producto no se agregará al carrito.

### ¿Puedo escanear el mismo producto varias veces?
Sí. Cada vez que escaneas, se incrementa la cantidad en 1. Si ya tienes 3 unidades en el carrito y escaneas otra vez, pasará a 4.

### ¿Puedo escanear sin hacer clic en el buscador primero?
Al abrir la página POS, el buscador se enfoca automáticamente. Si hiciste clic en otra parte, puedes:
- Hacer clic en la zona de productos para que el buscador recupere el enfoque
- Presionar **F1** para enfocar el buscador
- Hacer clic directamente en el buscador

### ¿Funciona con cualquier lector de código de barras?
Sí, funciona con cualquier lector que se conecte como "teclado" (HID), que es la gran mayoría de lectores del mercado.

---

## ⌨️ Atajos de teclado relacionados

| Atajo | Acción |
|-------|--------|
| **F1** | Enfocar el buscador (útil si se perdió el enfoque) |
| **+** / **-** | Ajustar cantidad del último producto en el carrito |
| **F2** | Abrir pantalla de cobro |
| **F4** (doble) | Limpiar el carrito |

---

## 💡 Tips

- 📌 **Registra los códigos de barras** de tus productos al ingresarlos al inventario. Así podrás usar el escáner desde el primer día.
- ⚡ Si usas lector USB, es **más rápido y confiable** que el celular Bluetooth.
- 🔄 Si el buscador pierde el enfoque, simplemente presiona **F1** y vuelve a escanear.
- 📱 Si usas celular como escáner, mantenlo cerca de la computadora para buena conexión Bluetooth.

---

## ⚠️ Notas importantes

- El buscador **debe estar enfocado** (cursor dentro del campo de búsqueda) para que el escaneo funcione
- Los códigos de barras deben tener **al menos 4 caracteres** para ser detectados
- Esta función **no funciona** mientras la ventana de pago está abierta — ciérrala primero con **Esc**
