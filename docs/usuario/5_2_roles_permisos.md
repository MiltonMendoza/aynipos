# 🛡️ Roles y Permisos

> Controla quién puede acceder a cada sección del sistema según su puesto de trabajo.

## ¿Qué puedo hacer con esto?

AyniPOS permite asignar un **rol** a cada usuario para limitar lo que puede ver y hacer dentro del sistema. De esta forma, cada empleado solo accede a las herramientas que necesita para su trabajo.

## 🎭 Los tres roles

### 👑 Administrador
Tiene acceso **completo** a todo el sistema:
- Punto de Venta, Ventas, Inventario, Clientes, Reportes, Configuración
- Puede crear y eliminar usuarios
- Puede anular ventas
- Puede modificar datos del negocio

### 👤 Cajero
Acceso enfocado en la **venta diaria**:
- ✅ Punto de Venta (vender productos)
- ✅ Historial de Ventas (consultar, imprimir recibos)
- ✅ Clientes (buscar y crear clientes)
- ✅ Reportes de ventas (gráficos, productos más vendidos)
- ✅ Abrir y cerrar caja (en Configuración)
- ❌ No puede anular ventas
- ❌ No puede ver Inventario
- ❌ No puede crear usuarios ni cambiar datos del negocio

### 📦 Inventarista
Acceso enfocado en el **control de productos y stock**:
- ✅ Inventario (ver stock, ajustar cantidades)
- ✅ Crear y editar productos
- ✅ Importar/exportar productos
- ✅ Reportes de inventario (valorización, margen de ganancia)
- ❌ No puede vender (no ve el Punto de Venta)
- ❌ No puede ver Ventas ni Clientes
- ❌ No puede abrir/cerrar caja

---

## ¿Cómo se usa?

### Paso 1: Crear un usuario con su rol

1. Inicia sesión como **Administrador**
2. Ve a **⚙️ Configuración**
3. En la sección **👥 Usuarios**, haz clic en **➕ Nuevo Usuario**
4. Completa los datos:
   - **Nombre**: el nombre del empleado
   - **PIN**: un número de 4 a 6 dígitos
   - **Confirmar PIN**: repite el mismo número
   - **Rol**: elige entre **Cajero**, **Inventarista** o **Administrador**
5. Haz clic en **➕ Crear**

> 💡 **Tip:** Si no estás seguro de qué rol asignar, usa **Cajero** para empleados que atienden clientes e **Inventarista** para quienes se encargan del stock.

### Paso 2: El usuario inicia sesión

Cuando el nuevo usuario ingrese su PIN:
- Solo verá las **secciones permitidas** en la barra lateral izquierda
- Automáticamente se abrirá la primera sección disponible para su rol
- Su nombre y rol aparecen en la parte inferior de la barra lateral

### Paso 3: Cambiar el rol de un usuario

1. Ve a **⚙️ Configuración → 👥 Usuarios**
2. Haz clic en **✏️** junto al usuario
3. Cambia el **Rol** en el selector
4. Haz clic en **💾 Guardar**
5. El usuario verá los cambios la próxima vez que inicie sesión

---

## ¿Qué ve cada rol?

| Sección | 👑 Admin | 👤 Cajero | 📦 Inventarista |
|---------|----------|-----------|-----------------|
| 🛒 Punto de Venta | ✅ | ✅ | ❌ |
| 📋 Ventas | ✅ | ✅ | ❌ |
| 📦 Inventario | ✅ | ❌ | ✅ |
| 👥 Clientes | ✅ | ✅ | ❌ |
| 📊 Reportes | ✅ Todos | ✅ Solo ventas | ✅ Solo inventario |
| ⚙️ Configuración | ✅ Todo | ✅ Solo caja | ❌ |

---

## Preguntas frecuentes

### ¿Qué pasa si un cajero necesita anular una venta?
Solo un **Administrador** puede anular ventas. El cajero debe pedir a un administrador que cierre sesión e ingrese con su PIN para realizar la anulación.

### ¿El cajero puede abrir y cerrar la caja?
Sí. El cajero ve la sección de **Caja Registradora** dentro de Configuración, pero no puede ver los datos del negocio ni los usuarios.

### ¿Puedo cambiar el rol de un usuario en cualquier momento?
Sí. Un Administrador puede cambiar el rol desde **Configuración → Usuarios**. Los cambios se aplican la próxima vez que el usuario inicie sesión.

### ¿Qué pasa si el administrador no está y necesito acceder a algo?
Solo un Administrador puede cambiar roles. Si necesitas acceso temporal a otra sección, el Administrador debe iniciar sesión y realizar la acción, o cambiar temporalmente tu rol.

---

## Notas importantes

- 👑 Siempre debe haber al menos un usuario **Administrador** en el sistema
- 🔄 Los cambios de rol se aplican al **cerrar e iniciar sesión** de nuevo
- 📊 En **Reportes**, cada rol ve solo los reportes relevantes a su trabajo
- 🔒 Los permisos protegen contra errores accidentales — cada persona ve solo lo que necesita
