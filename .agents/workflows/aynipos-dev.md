---
description: How to develop features for the AyniPOS project (add commands, pages, etc.)
---

# AyniPOS Development Workflow

## Before starting ANY work on AyniPOS:

1. **Read `ARCHITECTURE.md`** in the project root to understand the full project structure, conventions, and patterns.
2. **Read `ROADMAP.md`** to see what features are planned and their current status.
3. **Review `docs/usuario/`** — read the existing user documentation (especially `README.md` index) to understand documented flows, terminology, and UI patterns already communicated to the user.
4. **Check the current phase** being worked on and pick the next pending feature.

---

## Feature Implementation Process

Before writing ANY code, you MUST:

1. **Research** the current code related to the feature (read relevant files).
2. **Review related user docs** in `docs/usuario/` — check if the new feature affects or extends any already-documented flow. Note any terminology, UI patterns, or user expectations set by existing docs.
3. **Create an implementation plan** that includes:
   - What files will be modified, created, or deleted
   - Validation rules or business logic to add
   - Any new types, API functions, or commands needed
   - Impact on existing documented features (if any)
   - How the changes will be verified
   - In spanish 
3. **Request user review** of the plan via `notify_user` with `BlockedOnUser: true`.
4. **Wait for approval** before proceeding. If the user requests changes, update the plan and request review again.

> ⚠️ NEVER start writing code without an approved plan. Even for small features.

### Step 2: Execution

Once the plan is approved:

1. Implement changes following the approved plan.
2. Update the task checklist (`task.md`) as you complete items.

### Step 3: Verification

After implementation:

// turbo
1. Run `cd /Users/milmen/Projects/tableplus/aynipos/src-tauri && cargo check` to verify Rust compiles.
// turbo
2. Run `cd /Users/milmen/Projects/tableplus/aynipos && npx svelte-check` to verify Svelte/TypeScript.
3. Update `ROADMAP.md` — mark feature as ✅ Completado.
4. **Update user docs** — if the feature modifies behavior documented in `docs/usuario/`, update those docs. If the new feature introduces user-facing changes, prepare notes for `/user-docs` generation.
5. Create/update `walkthrough.md` artifact summarizing what was done.
6. Notify user with results.

---

## Adding a new Rust command:

// turbo
1. Check the existing commands in `src-tauri/src/commands/` for patterns.
2. Add the new function in the appropriate module (e.g., `products.rs` for product features).
3. Use `#[tauri::command]` attribute. Receive `db: State<Database>` as first parameter.
4. Return `Result<T, String>` where T is serializable.
5. Register the command in `src-tauri/src/lib.rs` inside the `.invoke_handler(tauri::generate_handler![...])` call.
6. Add the corresponding TypeScript wrapper in `src/lib/services/api.ts`.
7. Add any new types to `src/lib/types/index.ts`.

## Adding a new page/view:

1. Create a new folder under `src/routes/` with the page name (e.g., `src/routes/newpage/`).
2. Create `NewPage.svelte` inside that folder.
3. Add the route to the `AppRoute` type in `src/lib/types/index.ts`.
4. Add the lazy import in `src/routes/+page.svelte` inside the routing block.
5. Add the nav item in the `navItems` array in `+page.svelte`.

## Adding a new database migration:

1. Create `src-tauri/migrations/002_description.sql` (increment version number).
2. Add the migration constant in `src-tauri/src/db/schema.rs` using `include_str!`.
3. Add the version check block in `run_migrations()`.

## UI conventions:

- Use CSS classes from `app.css` design system (`.btn`, `.card`, `.input`, `.modal-overlay`, etc.).
- All UI text must be in **Spanish**.
- Currency format: `Bs {amount.toFixed(2)}`.
- Use Svelte 5 runes: `$state()`, `$props()`, `$effect()`.
- Never use `invoke()` directly — always go through `$lib/services/api.ts`.
- All forms must have inline validation with `.input-error` and `.field-error` classes (see feature 1.9).