# 👤 Cajero por Turno

> Cada vez que abres o cierras la caja, AyniPOS registra qué cajero hizo la operación. Así puedes ver el historial completo de turnos y saber exactamente quién fue responsable de cada sesión.

## ¿Qué puedo hacer con esto?

- **Ver quién abrió la caja**: cuando la caja está abierta, se muestra el nombre del cajero responsable
- **Reporte de cierre por cajero**: al cerrar caja, el reporte impreso incluye el nombre del cajero que trabajó ese turno
- **Historial de turnos**: una tabla con todas las sesiones de caja pasadas, mostrando quién atendió cada una
- **Filtrar por cajero**: en Reportes, puedes ver solo los turnos de un cajero específico para control individual

---

## ¿Cómo se usa?

### Paso 1: Abrir la caja

1. Ve a **⚙️ Configuración**
2. En la sección **💰 Caja Registradora**, presiona **🔓 Abrir Caja**
3. Ingresa el monto inicial y confirma
4. La caja queda abierta y muestra tu nombre como cajero responsable:
   > 👤 **Cajero: Juan Pérez**

> 💡 **Tip:** No necesitas hacer nada especial — AyniPOS identifica automáticamente quién está logueado y lo asocia a la caja.

### Paso 2: Cerrar la caja

1. Presiona **🔒 Cerrar Caja** al terminar tu turno
2. Ingresa el monto real que hay en caja
3. El reporte de cierre que se imprime ahora incluye **tu nombre como cajero**

### Paso 3: Ver el historial de turnos

1. Ve a **📊 Reportes**
2. Busca la sección **📋 Historial de Cajas**
3. Verás una tabla con todas las sesiones anteriores:

| Dato | Descripción |
|------|-------------|
| **Apertura** | Fecha y hora en que se abrió la caja |
| **Cierre** | Fecha y hora en que se cerró |
| **Cajero** | Nombre del cajero que atendió ese turno |
| **Monto Inicial** | Con cuánto se abrió la caja |
| **Total Ventas** | Cuánto se vendió durante el turno |
| **Transacciones** | Cantidad de ventas realizadas |
| **Diferencia** | Sobrante (+) o faltante (−) al cerrar |

### Paso 4: Filtrar por cajero

1. En la sección de historial, usa el filtro **Cajero** (arriba a la derecha)
2. Selecciona el nombre del cajero
3. La tabla mostrará solo los turnos de esa persona

### Paso 5: Ver detalle de un turno

1. Haz clic en el botón **📊** a la derecha de cualquier turno
2. Se abrirá el reporte completo de cierre de esa sesión, con desglose por método de pago

---

## Preguntas frecuentes

### ¿Qué pasa si no había usuarios cuando abrí la caja?
Las cajas abiertas antes de crear usuarios aparecerán con "—" en la columna de cajero. A partir de ahora, todas las cajas nuevas se asocian automáticamente al usuario que las abre.

### ¿Un cajero puede ver los turnos de otros cajeros?
Depende de los permisos. Los cajeros pueden ver el historial en la sección de Reportes, pero solo los administradores pueden acceder a la gestión completa de usuarios.

### ¿Puedo imprimir el reporte de un turno anterior?
Sí. En el historial, haz clic en el botón 📊 de cualquier turno para abrir e imprimir su reporte detallado.

### ¿Se puede abrir la caja con un usuario y cerrarla con otro?
La caja se cierra con el usuario que inicie la operación de cierre; sin embargo, el registro de apertura siempre muestra quién la abrió originalmente.

---

## Notas importantes

- 🔒 Cada apertura de caja queda **permanentemente asociada** al cajero que la realizó
- 📋 El historial se carga automáticamente al entrar a Reportes
- 🖨️ Los reportes impresos de cierre ahora incluyen el nombre del cajero
- 📊 Puedes cargar más turnos presionando **"📄 Cargar más"** al final de la tabla
