# 📊 Ventas por Rol — ¿Qué ve cada usuario?

> El cajero ve solo sus propias ventas del día. El administrador ve el total de toda la tienda.

---

## ¿Qué puedo hacer con esto?

AyniPOS ajusta automáticamente la información que ve cada usuario según su rol. Esto significa que:

- 👤 **Cajero:** El Dashboard y los reportes muestran solo lo que **él mismo vendió hoy**
- 🛡️ **Administrador / Inventarista:** Ve el **total de toda la tienda**, sumando las ventas de todos los cajeros

No hay que configurar nada — el sistema lo hace automáticamente al iniciar sesión.

---

## ¿Cómo funciona en el Dashboard?

### Vista del Cajero

Cuando un cajero entra al **Dashboard**, las tarjetas de resumen muestran:

| Tarjeta | ¿Qué muestra? |
|---------|---------------|
| 💰 **Ventas hoy** | Solo el monto que ese cajero cobró hoy |
| 🔢 **Transacciones** | Solo la cantidad de ventas que ese cajero realizó hoy |

### Vista del Administrador

Cuando el administrador entra al **Dashboard**, las mismas tarjetas muestran:

| Tarjeta | ¿Qué muestra? |
|---------|---------------|
| 💰 **Ventas hoy** | El monto total vendido por **todos** los cajeros hoy |
| 🔢 **Transacciones** | La cantidad total de ventas realizadas por **todos** hoy |

---

## ¿Cómo funciona en los Reportes?

La misma lógica aplica en la sección de **Reportes**:

- El **cajero** solo puede ver y analizar sus propias ventas
- El **administrador** ve los datos consolidados de toda la tienda, con la opción de filtrar por cajero si lo necesita

---

## ❓ Preguntas frecuentes

### ¿Por qué el cajero no ve las ventas de sus compañeros?
Para proteger la privacidad de cada empleado y evitar comparaciones innecesarias. Cada cajero es responsable de su propio turno.

### ¿El cajero puede ver sus ventas de días anteriores?
Sí. En la sección de **Ventas**, el cajero puede ver su historial de ventas pasadas — solo las suyas.

### ¿Cómo sabe el sistema a quién pertenece cada venta?
Cada venta queda vinculada al usuario que estaba logueado al momento de realizarla. Por eso es importante que cada cajero use su propio PIN al entrar al sistema.

### ¿Qué pasa si dos cajeros comparten el mismo usuario?
No se recomienda hacerlo. Si comparten cuenta, el sistema no puede distinguir quién hizo cada venta y los reportes mezclarán los datos de ambos.

### ¿El administrador puede ver las ventas de un cajero específico?
Sí. En la vista de **Ventas**, el administrador puede ver la columna **Cajero** y filtrar u ordenar por nombre de cajero. También puede aplicar filtros en los reportes.

---

## 💡 Tips

- 🔑 Cada cajero debe entrar con su propio PIN para que sus ventas se registren correctamente
- 📊 El administrador puede revisar el rendimiento individual de cada cajero desde la vista de **Ventas** usando la columna Cajero
- 📅 Para ver un resumen del turno de un cajero, usá el reporte de **Cierre de Caja** al final del turno

---

## ⚠️ Notas importantes

- ⚙️ Esta separación es **automática** — no necesitás configurar nada
- 🔒 El cajero **nunca ve** las ventas de otros cajeros, ni en el Dashboard ni en los reportes
- 📋 Para que todo funcione correctamente, cada empleado debe usar su **propio usuario y PIN**
