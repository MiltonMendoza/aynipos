# 📋 Registro de Actividad (Audit Log)

> Consulta un historial detallado de todo lo que pasa en tu negocio: quién hizo qué y cuándo.

## ¿Qué puedo hacer con esto?

El Registro de Actividad es como una **bitácora automática** que guarda cada acción importante que se realiza en AyniPOS. Esto te permite:

- 🔍 **Saber quién hizo qué** — Ver qué usuario realizó cada venta, ajuste o cambio
- 📅 **Revisar por fecha** — Filtrar el historial por rango de fechas
- 🎯 **Filtrar por tipo de acción** — Ver solo ventas, solo anulaciones, solo ajustes de inventario, etc.
- 🛡️ **Mayor control** — Detectar operaciones sospechosas o errores

> ⚠️ **Solo los Administradores** pueden ver el Registro de Actividad. Los cajeros e inventaristas no tienen acceso a esta sección.

## ¿Qué acciones se registran?

| Icono | Acción | ¿Cuándo se registra? |
|-------|--------|----------------------|
| 🔑 | Login | Cada vez que un usuario ingresa al sistema |
| 💰 | Venta | Al completar una venta en el Punto de Venta |
| 🚫 | Anulación | Al anular una venta desde el Historial de Ventas |
| ➕ | Producto creado | Al agregar un nuevo producto al inventario |
| ✏️ | Producto editado | Al modificar los datos de un producto |
| 📦 | Ajuste de inventario | Al hacer compras, ajustes o devoluciones de stock |
| 🔓 | Caja abierta | Al abrir una caja registradora |
| 🔒 | Caja cerrada | Al cerrar una caja registradora |
| 👤 | Usuario creado | Al crear un nuevo usuario |
| 🔄 | Usuario editado | Al cambiar datos o rol de un usuario |
| ❌ | Usuario eliminado | Al eliminar un usuario |

## ¿Cómo se usa?

### Paso 1: Ir a Configuración

Haz clic en **⚙️ Configuración** en el menú lateral izquierdo.

### Paso 2: Buscar "Registro de Actividad"

Baja hasta la sección **📋 Registro de Actividad**, que aparece debajo de la lista de usuarios.

### Paso 3: Cargar el registro

Presiona el botón **🔄 Actualizar** para cargar las acciones más recientes.

### Paso 4: Filtrar (opcional)

Puedes usar los filtros para buscar acciones específicas:

- **Acción**: Selecciona un tipo de acción (Ventas, Anulaciones, Ajustes inventario, etc.)
- **Desde / Hasta**: Elige un rango de fechas para ver solo ese período

Los filtros se aplican inmediatamente al cambiarlos.

### Paso 5: Ver más resultados

Si necesitas ver más acciones, presiona el botón **📄 Cargar más** que aparece al final de la tabla.

## ¿Qué información muestra cada registro?

Cada fila del registro incluye:

| Columna | Descripción |
|---------|-------------|
| **Fecha** | Día y hora exacta de la acción |
| **Usuario** | Nombre del usuario que realizó la acción |
| **Acción** | Tipo de acción con un ícono y color distintivo |
| **Detalle** | Descripción breve (ej: "Venta #42 por Bs 150.00") |

Los colores de las acciones te ayudan a identificar rápidamente:
- 🟢 **Verde** — Acciones positivas (ventas, creaciones, apertura de caja)
- 🟡 **Amarillo** — Modificaciones (ediciones, ajustes)
- 🔴 **Rojo** — Acciones sensibles (anulaciones, eliminaciones)
- 🔵 **Azul** — Información (logins)

## Preguntas frecuentes

### ¿Los cajeros pueden ver el registro de actividad?
No. Solo los usuarios con rol de **Administrador** pueden ver esta sección. Los cajeros e inventaristas no verán el "Registro de Actividad" en su pantalla de Configuración.

### ¿Se puede borrar el registro?
No. El registro es permanente y no se puede modificar ni borrar. Esto garantiza la transparencia del historial.

### ¿Se registra automáticamente o tengo que hacer algo?
Es 100% automático. Cada vez que alguien realiza una acción en el sistema, se guarda automáticamente sin que el usuario tenga que hacer nada extra.

### ¿Cuántas acciones puedo ver?
Por defecto se muestran las últimas 50 acciones. Puedes cargar más presionando "📄 Cargar más" al final de la tabla.

### ¿Puedo buscar acciones de un cajero específico?
Actualmente puedes filtrar por **tipo de acción** y **rango de fechas**. El nombre del usuario aparece en cada registro, así que puedes identificar visualmente las acciones de cada persona.

## Notas importantes

- 📝 El registro se guarda **localmente** en tu computadora, junto con el resto de los datos de AyniPOS
- 🔒 Solo los **Administradores** tienen acceso al registro de actividad
- ⚡ El registro no afecta la velocidad del sistema — se guarda en segundo plano
- 📋 Cada detalle incluye información útil como números de venta, nombres de productos o montos
