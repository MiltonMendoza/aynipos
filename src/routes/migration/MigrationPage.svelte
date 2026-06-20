<script lang="ts">
  import { onMount } from 'svelte';
  import { getLegacyPreview, getLegacyLabs, applyLegacyMigration, getSuppliersSimple } from '$lib/services/api';
  import type { LegacyProductRow, LabEntry, MigrationPayload, MigrationResult } from '$lib/types';

  // ── Estado general ────────────────────────────────────────────────────────
  let step = $state<1 | 2 | 3>(1);
  let loading = $state(false);
  let applying = $state(false);
  let error = $state('');
  let result = $state<MigrationResult | null>(null);

  // ── Paso 1: productos ─────────────────────────────────────────────────────
  let products = $state<LegacyProductRow[]>([]);
  let searchQuery = $state('');
  let showOnlySelected = $state(false);

  let filteredProducts = $derived(products.filter(p => {
    if (showOnlySelected && !p.apply) return false;
    if (!searchQuery) return true;
    const q = searchQuery.toLowerCase();
    return p.name.toLowerCase().includes(q) ||
           p.sku.toLowerCase().includes(q) ||
           p.parsed_lab.toLowerCase().includes(q);
  }));

  let selectedCount = $derived(products.filter(p => p.apply).length);

  // ── Paso 2: laboratorios ──────────────────────────────────────────────────
  let labs = $state<LabEntry[]>([]);
  let suppliers = $state<[string, string][]>([]); // [id, name][]
  let labSearch = $state('');

  let filteredLabs = $derived(labs.filter(l =>
    !labSearch || l.name.toLowerCase().includes(labSearch.toLowerCase())
  ));

  let labsToCreate = $derived(labs.filter(l => l.action === 'create').length);
  let labsToIgnore = $derived(labs.filter(l => l.action === 'ignore').length);
  let labsExisting = $derived(labs.filter(l => l.action === 'existing').length);

  // ── Carga inicial ─────────────────────────────────────────────────────────
  onMount(async () => {
    loading = true;
    try {
      products = await getLegacyPreview();
    } catch (e) {
      error = `Error al cargar productos: ${e}`;
    } finally {
      loading = false;
    }
  });

  async function loadStep2() {
    loading = true;
    error = '';
    try {
      [labs, suppliers] = await Promise.all([getLegacyLabs(), getSuppliersSimple()]);
      step = 2;
    } catch (e) {
      error = `Error al cargar laboratorios: ${e}`;
    } finally {
      loading = false;
    }
  }

  // ── Helpers ───────────────────────────────────────────────────────────────
  function toggleAll(val: boolean) {
    products = products.map(p => ({ ...p, apply: val }));
  }

  function toggleProduct(id: string) {
    products = products.map(p => p.id === id ? { ...p, apply: !p.apply } : p);
  }

  function updateDesc(id: string, val: string) {
    products = products.map(p => p.id === id ? { ...p, parsed_description: val } : p);
  }

  function updateDose(id: string, val: string) {
    products = products.map(p => p.id === id ? { ...p, parsed_dose: val } : p);
  }

  function setLabAction(name: string, action: LabEntry['action']) {
    labs = labs.map(l => l.name === name ? { ...l, action, existing_supplier_id: action !== 'existing' ? null : l.existing_supplier_id } : l);
  }

  function setLabSupplier(name: string, supplierId: string) {
    labs = labs.map(l => l.name === name ? { ...l, existing_supplier_id: supplierId || null } : l);
  }

  // ── Aplicar migración ─────────────────────────────────────────────────────
  async function handleApply() {
    applying = true;
    error = '';
    try {
      const payload: MigrationPayload = {
        products: products.map(p => ({
          product_id: p.id,
          new_description: p.parsed_description,
          new_dose: p.parsed_dose,
          lab_name: p.parsed_lab,
          apply: p.apply,
        })),
        lab_map: labs.map(l => ({
          name: l.name,
          action: l.action,
          existing_supplier_id: l.existing_supplier_id,
        })),
      };
      result = await applyLegacyMigration(payload);
      step = 3;
    } catch (e) {
      error = `Error al aplicar migración: ${e}`;
    } finally {
      applying = false;
    }
  }

  function actionLabel(a: string) {
    if (a === 'create') return 'Crear proveedor';
    if (a === 'existing') return 'Usar existente';
    return 'Ignorar';
  }

  function actionColor(a: string) {
    if (a === 'create') return 'var(--accent-success)';
    if (a === 'existing') return 'var(--accent-primary)';
    return 'var(--text-muted)';
  }
</script>

<!-- ────────────────────────── HEADER ────────────────────────────────────── -->
<div class="page-header" style="border-bottom: 1px solid var(--border-color); padding-bottom: var(--space-lg); margin-bottom: var(--space-xl);">
  <div>
    <h1 class="page-title">🔄 Migración de Datos Legados</h1>
    <p class="text-muted" style="margin-top: var(--space-xs);">
      Migra la descripción, dosis y laboratorio del sistema anterior al nuevo formato de AyniPOS.
    </p>
  </div>
</div>

<!-- ── Stepper ── -->
<div style="display: flex; align-items: center; gap: var(--space-sm); margin-bottom: var(--space-xl);">
  {#each [
    { n: 1, label: 'Revisar productos' },
    { n: 2, label: 'Mapear laboratorios' },
    { n: 3, label: 'Resultado' }
  ] as s}
    <div style="display: flex; align-items: center; gap: var(--space-sm);">
      <div style="
        width: 28px; height: 28px; border-radius: 50%;
        display: flex; align-items: center; justify-content: center;
        font-size: var(--font-size-sm); font-weight: 700;
        background: {step >= s.n ? 'var(--accent-primary)' : 'var(--bg-hover)'};
        color: {step >= s.n ? '#fff' : 'var(--text-muted)'};
        transition: background 0.2s;
      ">{s.n}</div>
      <span style="font-size: var(--font-size-sm); font-weight: {step === s.n ? '600' : '400'}; color: {step === s.n ? 'var(--text-primary)' : 'var(--text-muted)'};">{s.label}</span>
    </div>
    {#if s.n < 3}
      <div style="flex: 1; height: 1px; background: var(--border-color); max-width: 40px;"></div>
    {/if}
  {/each}
</div>

{#if error}
  <div class="alert alert-error" style="margin-bottom: var(--space-lg);">⚠️ {error}</div>
{/if}

<!-- ══════════════════ PASO 1: Revisar productos ══════════════════════════════ -->
{#if step === 1}
  {#if loading}
    <div style="text-align: center; padding: var(--space-2xl); color: var(--text-muted);">
      ⏳ Cargando productos...
    </div>
  {:else if products.length === 0}
    <div class="card" style="text-align: center; padding: var(--space-2xl);">
      <div style="font-size: 3rem; margin-bottom: var(--space-md);">✅</div>
      <h3>No hay datos para migrar</h3>
      <p class="text-muted">Todos los productos ya tienen el formato correcto.</p>
    </div>
  {:else}
    <!-- Barra de stats -->
    <div style="display: flex; gap: var(--space-md); margin-bottom: var(--space-lg); flex-wrap: wrap;">
      <div class="stat-card" style="flex: 1; min-width: 140px; text-align: center;">
        <div style="font-size: var(--font-size-2xl); font-weight: 700; color: var(--accent-primary);">{products.length}</div>
        <div class="text-sm text-muted">Productos detectados</div>
      </div>
      <div class="stat-card" style="flex: 1; min-width: 140px; text-align: center;">
        <div style="font-size: var(--font-size-2xl); font-weight: 700; color: var(--accent-success);">{selectedCount}</div>
        <div class="text-sm text-muted">Seleccionados</div>
      </div>
      <div class="stat-card" style="flex: 1; min-width: 140px; text-align: center;">
        <div style="font-size: var(--font-size-2xl); font-weight: 700; color: var(--text-muted);">{products.length - selectedCount}</div>
        <div class="text-sm text-muted">Omitidos</div>
      </div>
    </div>

    <!-- Filtros -->
    <div style="display: flex; gap: var(--space-md); margin-bottom: var(--space-md); flex-wrap: wrap; align-items: center;">
      <input
        class="input"
        style="flex: 1; min-width: 200px;"
        type="text"
        placeholder="🔍 Buscar por nombre, SKU o laboratorio..."
        bind:value={searchQuery}
      />
      <label style="display: flex; align-items: center; gap: var(--space-xs); font-size: var(--font-size-sm); cursor: pointer; white-space: nowrap;">
        <input type="checkbox" bind:checked={showOnlySelected} />
        Solo seleccionados
      </label>
      <button class="btn btn-ghost btn-sm" onclick={() => toggleAll(true)}>☑️ Todos</button>
      <button class="btn btn-ghost btn-sm" onclick={() => toggleAll(false)}>⬜ Ninguno</button>
    </div>

    <!-- Tabla -->
    <div style="overflow-x: auto; border-radius: var(--radius-lg); border: 1px solid var(--border-color);">
      <table style="min-width: 900px;">
        <thead>
          <tr>
            <th style="width: 40px; text-align: center;">✓</th>
            <th>Producto</th>
            <th style="width: 200px;">Descripción nueva <span class="text-muted">(editable)</span></th>
            <th style="width: 160px;">Dosis <span class="text-muted">(editable)</span></th>
            <th style="width: 140px;">Laboratorio detectado</th>
          </tr>
        </thead>
        <tbody>
          {#each filteredProducts as p (p.id)}
            <tr style="opacity: {p.apply ? 1 : 0.45}; transition: opacity 0.15s;">
              <td style="text-align: center;">
                <input type="checkbox" checked={p.apply} onchange={() => toggleProduct(p.id)} />
              </td>
              <td>
                <div style="font-weight: 600; font-size: var(--font-size-sm);">{p.name}</div>
                <div class="text-muted" style="font-size: var(--font-size-xs);">SKU: {p.sku}</div>
              </td>
              <td>
                <input
                  class="input"
                  style="font-size: var(--font-size-xs); padding: 4px 8px; width: 100%;"
                  type="text"
                  value={p.parsed_description}
                  oninput={(e) => updateDesc(p.id, (e.target as HTMLInputElement).value)}
                  disabled={!p.apply}
                />
              </td>
              <td>
                <input
                  class="input"
                  style="font-size: var(--font-size-xs); padding: 4px 8px; width: 100%;"
                  type="text"
                  value={p.parsed_dose}
                  oninput={(e) => updateDose(p.id, (e.target as HTMLInputElement).value)}
                  disabled={!p.apply}
                />
              </td>
              <td>
                {#if p.parsed_lab}
                  <span style="font-size: var(--font-size-xs); background: var(--bg-hover); padding: 2px 8px; border-radius: var(--radius-full);">
                    {p.parsed_lab}
                  </span>
                {:else}
                  <span class="text-muted" style="font-size: var(--font-size-xs);">—</span>
                {/if}
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>

    {#if filteredProducts.length === 0}
      <div style="text-align: center; padding: var(--space-xl); color: var(--text-muted);">
        Sin resultados para "{searchQuery}"
      </div>
    {/if}

    <!-- Acción -->
    <div style="display: flex; justify-content: flex-end; margin-top: var(--space-xl); gap: var(--space-md);">
      <span class="text-muted" style="align-self: center; font-size: var(--font-size-sm);">
        {selectedCount} de {products.length} productos serán migrados
      </span>
      <button
        class="btn btn-primary"
        onclick={loadStep2}
        disabled={selectedCount === 0 || loading}
      >
        {loading ? '⏳ Cargando...' : 'Siguiente: Mapear laboratorios →'}
      </button>
    </div>
  {/if}

<!-- ══════════════════ PASO 2: Mapear laboratorios ════════════════════════════ -->
{:else if step === 2}
  <div style="margin-bottom: var(--space-lg);">
    <p class="text-muted" style="font-size: var(--font-size-sm);">
      Se detectaron <strong>{labs.length} laboratorios únicos</strong> en los productos seleccionados.
      Decide qué hacer con cada uno: crear como proveedor nuevo, vincular a uno existente, o ignorar.
    </p>
  </div>

  <!-- Stats labs -->
  <div style="display: flex; gap: var(--space-md); margin-bottom: var(--space-lg); flex-wrap: wrap;">
    <div class="stat-card" style="flex: 1; min-width: 130px; text-align: center;">
      <div style="font-size: var(--font-size-xl); font-weight: 700; color: var(--accent-success);">{labsToCreate}</div>
      <div class="text-sm text-muted">Crear nuevos</div>
    </div>
    <div class="stat-card" style="flex: 1; min-width: 130px; text-align: center;">
      <div style="font-size: var(--font-size-xl); font-weight: 700; color: var(--accent-primary);">{labsExisting}</div>
      <div class="text-sm text-muted">Usar existentes</div>
    </div>
    <div class="stat-card" style="flex: 1; min-width: 130px; text-align: center;">
      <div style="font-size: var(--font-size-xl); font-weight: 700; color: var(--text-muted);">{labsToIgnore}</div>
      <div class="text-sm text-muted">Ignorar</div>
    </div>
  </div>

  <!-- Filtro labs -->
  <div style="margin-bottom: var(--space-md); display: flex; gap: var(--space-md); align-items: center; flex-wrap: wrap;">
    <input
      class="input"
      style="flex: 1; min-width: 200px;"
      type="text"
      placeholder="🔍 Filtrar laboratorios..."
      bind:value={labSearch}
    />
    <button class="btn btn-ghost btn-sm" onclick={() => labs = labs.map(l => ({ ...l, action: 'create', existing_supplier_id: null }))}>
      ☑️ Crear todos
    </button>
    <button class="btn btn-ghost btn-sm" onclick={() => labs = labs.map(l => ({ ...l, action: 'ignore', existing_supplier_id: null }))}>
      🚫 Ignorar todos
    </button>
  </div>

  <!-- Tabla de labs -->
  <div style="overflow-x: auto; border-radius: var(--radius-lg); border: 1px solid var(--border-color); margin-bottom: var(--space-xl);">
    <table>
      <thead>
        <tr>
          <th>Laboratorio detectado</th>
          <th style="width: 80px; text-align: center;">N° prod.</th>
          <th style="width: 160px;">Acción</th>
          <th>Proveedor existente</th>
        </tr>
      </thead>
      <tbody>
        {#each filteredLabs as lab (lab.name)}
          <tr>
            <td>
              <div style="font-weight: 600; font-size: var(--font-size-sm);">{lab.name}</div>
            </td>
            <td style="text-align: center;">
              <span style="background: var(--bg-hover); padding: 2px 8px; border-radius: var(--radius-full); font-size: var(--font-size-xs);">
                {lab.count}
              </span>
            </td>
            <td>
              <select
                class="select"
                style="font-size: var(--font-size-xs); padding: 4px 8px;"
                value={lab.action}
                onchange={(e) => setLabAction(lab.name, (e.target as HTMLSelectElement).value as LabEntry['action'])}
              >
                <option value="create">➕ Crear proveedor</option>
                <option value="existing">🔗 Usar existente</option>
                <option value="ignore">🚫 Ignorar</option>
              </select>
            </td>
            <td>
              {#if lab.action === 'existing'}
                <select
                  class="select"
                  style="font-size: var(--font-size-xs); padding: 4px 8px; width: 100%;"
                  value={lab.existing_supplier_id ?? ''}
                  onchange={(e) => setLabSupplier(lab.name, (e.target as HTMLSelectElement).value)}
                >
                  <option value="">— Seleccionar proveedor —</option>
                  {#each suppliers as [id, name]}
                    <option value={id}>{name}</option>
                  {/each}
                </select>
              {:else}
                <span class="text-muted" style="font-size: var(--font-size-xs);">
                  {lab.action === 'create' ? 'Se creará automáticamente' : 'No se vinculará'}
                </span>
              {/if}
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  <!-- Aviso advertencia -->
  <div style="
    background: color-mix(in srgb, var(--accent-warning, #f59e0b) 10%, transparent);
    border: 1px solid color-mix(in srgb, var(--accent-warning, #f59e0b) 30%, transparent);
    border-radius: var(--radius-md);
    padding: var(--space-md);
    margin-bottom: var(--space-xl);
    font-size: var(--font-size-sm);
  ">
    ⚠️ <strong>Esta acción es permanente.</strong> Se actualizarán {selectedCount} productos y se crearán {labsToCreate} proveedores nuevos.
    Los datos originales serán reemplazados. Asegúrate de haber revisado el mapeo antes de continuar.
  </div>

  <div style="display: flex; justify-content: space-between; gap: var(--space-md);">
    <button class="btn btn-ghost" onclick={() => step = 1}>← Volver a productos</button>
    <button
      class="btn btn-primary"
      onclick={handleApply}
      disabled={applying}
    >
      {applying ? '⏳ Aplicando migración...' : '✅ Aplicar migración'}
    </button>
  </div>

<!-- ══════════════════ PASO 3: Resultado ═════════════════════════════════════ -->
{:else if step === 3 && result}
  <div style="text-align: center; padding: var(--space-xl) 0; max-width: 500px; margin: 0 auto;">
    <div style="font-size: 4rem; margin-bottom: var(--space-lg);">🎉</div>
    <h2 style="margin-bottom: var(--space-sm);">¡Migración completada!</h2>
    <p class="text-muted" style="margin-bottom: var(--space-xl);">
      Los datos legados han sido migrados exitosamente al nuevo formato.
    </p>

    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-md); margin-bottom: var(--space-xl); text-align: center;">
      <div class="stat-card">
        <div style="font-size: var(--font-size-2xl); font-weight: 700; color: var(--accent-success);">{result.products_updated}</div>
        <div class="text-sm text-muted">Productos actualizados</div>
      </div>
      <div class="stat-card">
        <div style="font-size: var(--font-size-2xl); font-weight: 700; color: var(--accent-primary);">{result.suppliers_created}</div>
        <div class="text-sm text-muted">Proveedores creados</div>
      </div>
      <div class="stat-card">
        <div style="font-size: var(--font-size-2xl); font-weight: 700; color: var(--accent-primary);">{result.suppliers_linked}</div>
        <div class="text-sm text-muted">Vínculos a proveedor</div>
      </div>
      <div class="stat-card">
        <div style="font-size: var(--font-size-2xl); font-weight: 700; color: var(--text-muted);">{result.skipped}</div>
        <div class="text-sm text-muted">Omitidos</div>
      </div>
    </div>

    <p class="text-muted" style="font-size: var(--font-size-sm);">
      Puedes volver a ejecutar esta herramienta en cualquier momento — solo mostrará productos que aún no han sido migrados.
    </p>
  </div>
{/if}
