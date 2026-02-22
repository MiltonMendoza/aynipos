---
description: Generar documentación para el usuario final después de completar una feature del roadmap
---

# Generar Documentación de Usuario

> Genera documentación clara y amigable para el usuario final (cajero/dueño de negocio) después de completar una feature o mejora del ROADMAP.

## Cuándo usar

Ejecutar después de completar y verificar una feature del `ROADMAP.md`. Usar con:
```
/user-docs [número de feature]
```
Ejemplo: `/user-docs 1.2`

> 💡 **Recomendación:** Ejecutar este workflow en la **misma conversación** donde se implementó la feature. Así se tiene acceso al contexto del chat — detalles de implementación, problemas encontrados, y decisiones tomadas durante el desarrollo que son valiosos para la documentación.

---

## Pasos

### 1. Identificar la feature

1. Leer `ROADMAP.md` para encontrar la feature indicada por el usuario.
2. Confirmar que su estado es `✅ Completado`.
3. Si no está completada, notificar al usuario y no continuar.

### 2. Revisar contexto del desarrollo

1. Si estamos en la misma conversación donde se implementó la feature, revisar el **historial del chat** para extraer:
   - Problemas encontrados durante la implementación y cómo se resolvieron
   - Detalles de configuración descubiertos durante las pruebas
   - Limitaciones o requisitos que surgieron en la práctica
2. Buscar si existe un `walkthrough.md` en los artifacts de la conversación de implementación.

### 3. Investigar la implementación

1. Leer los archivos relevantes del frontend (páginas Svelte) para entender la UI del usuario.
2. Identificar:
   - **Qué puede hacer** el usuario con esta feature
   - **Dónde se accede** (qué página/sección de la app)
   - **Atajos de teclado** relacionados (si los hay)
   - **Validaciones** y mensajes de error que podría ver
   - **Flujo paso a paso** para usar la feature
   - **Casos especiales** o limitaciones (incluir lo descubierto en el paso 2)

### 4. Generar la documentación

Crear el archivo `docs/usuario/[fase]_[numero]_[nombre-corto].md` con este formato:

```markdown
# [Emoji] [Nombre de la Feature]

> [Descripción breve de una línea de lo que hace la feature]

## ¿Qué puedo hacer con esto?

[Explicación en lenguaje simple, sin términos técnicos]

## ¿Cómo se usa?

### Paso 1: [Acción]
[Instrucción clara con detalle visual]

### Paso 2: [Acción]
...

## Atajos de teclado
| Atajo | Acción |
|-------|--------|
| ... | ... |

## Preguntas frecuentes

### ¿Qué pasa si [situación]?
[Respuesta]

## Notas importantes
- [Nota 1]
- [Nota 2]
```

### Reglas de escritura:

- **Idioma**: Español (Bolivia)
- **Tono**: Amigable, directo, sin jerga técnica
- **Audiencia**: Cajero de farmacia o dueño de negocio pequeño con conocimientos básicos de computación
- **Usar emojis** para hacer el documento visualmente claro
- **Incluir tips** prácticos basados en el uso real
- **Mencionar errores comunes** y cómo resolverlos
- **NO incluir** detalles técnicos (código, base de datos, API, etc.)
- **NO asumir** que el usuario sabe inglés

### 5. Actualizar el índice

1. Si no existe `docs/usuario/README.md`, crearlo con un índice de toda la documentación.
2. Si ya existe, agregar la nueva entrada al índice.

### 6. Notificar al usuario

Mostrar al usuario:
- Un resumen de lo que se documentó
- La ruta del archivo generado
- Preguntar si necesita ajustes en tono, detalle o contenido
