# 🔐 Login con PIN

> Ingresa al sistema con un PIN numérico rápido y seguro.

## ¿Qué puedo hacer con esto?

AyniPOS ahora pide un **PIN de 4 a 6 dígitos** para ingresar al sistema. Cada persona que use la caja tiene su propio PIN, así se sabe quién está trabajando en cada momento.

Al instalar la app, ya viene un usuario **Administrador** con el PIN **1234**. Ese PIN debe cambiarse después de la primera vez.

## ¿Cómo se usa?

### Paso 1: Ingresar el PIN

Al abrir AyniPOS, verás una pantalla con un **teclado numérico**:

- Puedes tocar los botones en pantalla **o** escribir los números con el teclado
- Los puntos (●) se van llenando conforme ingresas cada dígito
- Cuando completes tu PIN, presiona **Enter** o toca **🔓 Ingresar**

> 💡 **Tip:** También puedes escribir los números directamente con el teclado físico. No es necesario tocar los botones en pantalla.

### Paso 2: Acceder al sistema

Si el PIN es correcto, entras directamente al **Punto de Venta** y tu nombre aparece en la parte inferior de la barra lateral.

Si el PIN es incorrecto:
- 🔴 Verás un mensaje **"PIN incorrecto"**
- La pantalla se sacude brevemente para indicar el error
- Se limpia el PIN para que lo intentes de nuevo

### Paso 3: Cerrar sesión

Cuando termines tu turno, haz clic en **🚪 Cerrar Sesión** en la parte inferior de la barra lateral izquierda. Volverás a la pantalla de PIN.

---

## 👥 Administrar Usuarios

Solo desde la sección **Configuración** se pueden crear, editar o eliminar usuarios.

### Crear un nuevo usuario

1. Ve a **⚙️ Configuración**
2. Busca la sección **👥 Usuarios** (debajo de Caja Registradora)
3. Haz clic en **➕ Nuevo Usuario**
4. Completa los datos:
   - **Nombre**: el nombre del cajero o empleado
   - **PIN**: un número de 4 a 6 dígitos (debe ser único)
   - **Confirmar PIN**: repite el mismo número
   - **Rol**: "Cajero" o "Administrador"
5. Haz clic en **➕ Crear**

### Editar un usuario

1. En la tabla de usuarios, haz clic en **✏️** junto al usuario
2. Modifica el nombre, rol o PIN
3. Para cambiar el PIN, escribe uno nuevo. Si no quieres cambiarlo, deja el campo vacío
4. Haz clic en **💾 Guardar**

### Eliminar un usuario

1. En la tabla de usuarios, haz clic en **🗑️** junto al usuario
2. Confirma la eliminación

> ⚠️ **Importante:** No se puede eliminar al último Administrador del sistema. Siempre debe haber al menos uno.

---

## Preguntas frecuentes

### ¿Qué pasa si olvidé mi PIN?
Un **Administrador** puede cambiar tu PIN desde **Configuración > Usuarios**. Edita el usuario y escribe un nuevo PIN.

### ¿Puedo usar el mismo PIN para dos personas?
No. Cada usuario debe tener un PIN diferente. Si intentas usar uno que ya existe, el sistema mostrará un error.

### ¿Cuál es el PIN por defecto?
El sistema viene con un usuario **Administrador** y PIN **1234**. Te recomendamos cambiarlo la primera vez que ingreses.

### ¿Qué diferencia hay entre Administrador y Cajero?
Por ahora ambos roles tienen acceso completo al sistema. En futuras versiones, el rol de **Cajero** tendrá acceso limitado (solo Punto de Venta y Ventas).

---

## Notas importantes

- 🔒 Tu PIN se guarda de forma **segura** (encriptado) en el sistema
- 📱 Puedes ingresar el PIN con el teclado físico o tocando los botones en pantalla
- 🚪 Siempre cierra sesión al terminar tu turno
- 👑 No se puede eliminar al último administrador
