<script lang="ts">
  import { onMount } from 'svelte';
  import type { Supplier, CreateSupplier, User } from '$lib/types';
  import { getSuppliers, createSupplier, updateSupplier, deleteSupplier } from '$lib/services/api';
  import { hasPermission } from '$lib/services/permissions';
  import { DataTableState } from '$lib/utils/datatable.svelte';
  import TablePagination from '$lib/components/TablePagination.svelte';

  let { currentUser }: { currentUser: User | null } = $props();

  let suppliers: Supplier[] = $state([]);
  let showModal = $state(false);
  let editing: Supplier | null = $state(null);
  let form: CreateSupplier = $state({ name: '' });
  let errors: Record<string, string> = $state({});
  let toast = $state('');
  let toastTimeout: ReturnType<typeof setTimeout> | null = null;

  let table = new DataTableState<Supplier>([], [
    'name',
    'contact_name',
    'phone',
    'email',
    'address',
    'notes'
  ]);

  let openDropdownId = $state<string | null>(null);

  function toggleDropdown(e: MouseEvent, id: string) {
    e.stopPropagation();
    openDropdownId = openDropdownId === id ? null : id;
  }

  $effect(() => {
    table.currentPage;
    table.search;
    openDropdownId = null;
  });

  onMount(loadSuppliers);

  async function loadSuppliers() {
    try {
      suppliers = await getSuppliers();
      table.data = suppliers;
      table.currentPage = 1;
    } catch {
      suppliers = [];
      table.data = [];
    }
  }

  function showToast(msg: string) {
    toast = msg;
    if (toastTimeout) clearTimeout(toastTimeout);
    toastTimeout = setTimeout(() => (toast = ''), 3000);
  }

  function validate(): boolean {
    const e: Record<string, string> = {};
    if (!form.name.trim()) e.name = 'El nombre es obligatorio';
    if (form.email && !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(form.email)) {
      e.email = 'El formato de email no es válido';
    }
    errors = e;
    return Object.keys(e).length === 0;
  }

  function clearError(field: string) {
    if (errors[field]) {
      const copy = { ...errors };
      delete copy[field];
      errors = copy;
    }
  }

  function openNew() {
    editing = null;
    form = { name: '' };
    errors = {};
    showModal = true;
  }

  function openEdit(s: Supplier) {
    editing = s;
    form = {
      name: s.name,
      contact_name: s.contact_name ?? undefined,
      phone: s.phone ?? undefined,
      email: s.email ?? undefined,
      address: s.address ?? undefined,
      notes: s.notes ?? undefined,
    };
    errors = {};
    showModal = true;
  }

  async function handleSave() {
    if (!validate()) return;
    try {
      if (editing) {
        await updateSupplier({ id: editing.id, ...form });
        showToast('✅ Proveedor actualizado correctamente');
      } else {
        await createSupplier(form);
        showToast('✅ Proveedor creado correctamente');
      }
      showModal = false;
      errors = {};
      await loadSuppliers();
    } catch (e) {
      const msg = String(e);
      if (msg.includes('nombre')) {
        errors = { ...errors, name: msg };
      } else {
        alert('Error: ' + msg);
      }
    }
  }

  async function handleDelete(id: string, name: string) {
    if (!confirm(`¿Eliminar el proveedor "${name}"?\nLos productos asociados no serán eliminados.`)) return;
    try {
      await deleteSupplier(id);
      showToast('🗑️ Proveedor eliminado');
      await loadSuppliers();
    } catch (e) { alert('Error: ' + e); }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && showModal) showModal = false;
  }
</script>

<svelte:window onkeydown={handleKeydown} onclick={() => { openDropdownId = null; }} />

<div class="page">
  <div class="page-header">
    <div>
      <h1 class="page-title">🏭 Proveedores</h1>
      <p class="page-subtitle">Gestiona tus proveedores y distribuidores</p>
    </div>
    {#if hasPermission(currentUser, 'manage_suppliers')}
      <button id="btn-new-supplier" class="btn btn-primary" onclick={openNew}>➕ Nuevo Proveedor</button>
    {/if}
  </div>

  <!-- Search bar -->
  <div style="margin-bottom: var(--space-md); position: relative; max-width: 320px;">
    <input
      id="supplier-search"
      class="input input-compact"
      style="padding-right: 30px !important;"
      placeholder="🔍 Buscar por nombre, contacto o teléfono..."
      bind:value={table.search}
      oninput={() => table.currentPage = 1}
    />
    {#if table.search}
      <button
        onclick={() => { table.search = ''; table.currentPage = 1; }}
        style="position: absolute; right: 8px; top: 50%; transform: translateY(-50%); background: none; border: none; color: var(--text-muted); cursor: pointer; font-size: 14px; padding: 4px;"
      >
        ✕
      </button>
    {/if}
  </div>

  <!-- Table -->
  <div class="table-container">
    <table>
      <thead>
        <tr>
          <th style="width: 48px;"></th>
          <th onclick={() => table.sortBy('name')} style="cursor: pointer; user-select: none;">
            Nombre {table.sortColumn === 'name' ? (table.sortDirection === 'asc' ? '↑' : '↓') : ''}
          </th>
          <th onclick={() => table.sortBy('contact_name')} style="cursor: pointer; user-select: none;">
            Contacto {table.sortColumn === 'contact_name' ? (table.sortDirection === 'asc' ? '↑' : '↓') : ''}
          </th>
          <th onclick={() => table.sortBy('phone')} style="cursor: pointer; user-select: none;">
            Teléfono {table.sortColumn === 'phone' ? (table.sortDirection === 'asc' ? '↑' : '↓') : ''}
          </th>
          <th onclick={() => table.sortBy('email')} style="cursor: pointer; user-select: none;">
            Email {table.sortColumn === 'email' ? (table.sortDirection === 'asc' ? '↑' : '↓') : ''}
          </th>
          <th onclick={() => table.sortBy('address')} style="cursor: pointer; user-select: none;">
            Dirección {table.sortColumn === 'address' ? (table.sortDirection === 'asc' ? '↑' : '↓') : ''}
          </th>
          <th onclick={() => table.sortBy('notes')} style="cursor: pointer; user-select: none;">
            Notas {table.sortColumn === 'notes' ? (table.sortDirection === 'asc' ? '↑' : '↓') : ''}
          </th>
        </tr>
      </thead>
      <tbody>
        {#if table.paginated.length === 0}
          <tr>
            <td colspan="7" class="text-center text-muted" style="padding: var(--space-3xl);">
              {table.search ? 'No se encontraron proveedores con ese criterio' : 'No hay proveedores registrados'}
            </td>
          </tr>
        {:else}
          {#each table.paginated as s}
            <tr>
              <!-- Acciones como dropdown al inicio -->
              <td style="position: relative;">
                {#if hasPermission(currentUser, 'manage_suppliers')}
                <div class="action-dropdown">
                  <button
                    class="btn btn-ghost btn-sm action-trigger"
                    style="padding: 4px 8px; font-size: var(--font-size-base);"
                    onclick={(e) => toggleDropdown(e, s.id)}
                  >⋮</button>
                  {#if openDropdownId === s.id}
                  <div class="action-menu" role="menu">
                    <button class="action-item" onclick={() => { openDropdownId = null; openEdit(s); }}>✏️ Editar</button>
                    <button class="action-item" onclick={() => { openDropdownId = null; handleDelete(s.id, s.name); }}>🗑️ Eliminar</button>
                  </div>
                  {/if}
                </div>
                {:else}
                  —
                {/if}
              </td>
              <td style="font-weight: 600;">{s.name}</td>
              <td>{s.contact_name || '—'}</td>
              <td>{s.phone || '—'}</td>
              <td class="text-muted">{s.email || '—'}</td>
              <td class="text-muted" style="max-width: 180px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">{s.address || '—'}</td>
              <td class="text-muted" style="max-width: 160px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">{s.notes || '—'}</td>
            </tr>
          {/each}
        {/if}
      </tbody>
    </table>
  </div>
  <TablePagination {table} />
</div>

<!-- Modal Crear / Editar -->
{#if showModal}
  <div class="modal-overlay">
    <div class="modal" onclick={(e) => e.stopPropagation()} style="max-width: 520px;">
      <div class="modal-header">
        <h3 class="modal-title">{editing ? '✏️ Editar' : '➕ Nuevo'} Proveedor</h3>
        <button class="btn btn-ghost btn-sm" onclick={() => showModal = false}>✕</button>
      </div>
      <div class="modal-body">
        <!-- Nombre -->
        <div class="input-group">
          <label class="input-label">Nombre *</label>
          <input
            id="supplier-name"
            class="input"
            class:input-error={errors.name}
            bind:value={form.name}
            oninput={() => clearError('name')}
            placeholder="Distribuidora Farmacéutica S.A."
          />
          {#if errors.name}<span class="field-error">{errors.name}</span>{/if}
        </div>

        <!-- Contacto y Teléfono -->
        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-lg);">
          <div class="input-group">
            <label class="input-label">Persona de Contacto</label>
            <input class="input" bind:value={form.contact_name} placeholder="Juan Pérez" />
          </div>
          <div class="input-group">
            <label class="input-label">Teléfono</label>
            <input class="input" bind:value={form.phone} placeholder="+591 7XXXXXXX" />
          </div>
        </div>

        <!-- Email -->
        <div class="input-group">
          <label class="input-label">Email</label>
          <input
            class="input"
            class:input-error={errors.email}
            type="email"
            bind:value={form.email}
            oninput={() => clearError('email')}
            placeholder="ventas@proveedor.com"
          />
          {#if errors.email}<span class="field-error">{errors.email}</span>{/if}
        </div>

        <!-- Dirección -->
        <div class="input-group">
          <label class="input-label">Dirección</label>
          <input class="input" bind:value={form.address} placeholder="Av. Principal #123, Ciudad" />
        </div>

        <!-- Notas -->
        <div class="input-group">
          <label class="input-label">Notas</label>
          <textarea
            class="input"
            style="min-height: 72px; resize: vertical;"
            bind:value={form.notes}
            placeholder="Condiciones de pago, días de entrega, etc."
          ></textarea>
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-ghost" onclick={() => showModal = false}>Cancelar</button>
        <button id="btn-save-supplier" class="btn btn-primary" onclick={handleSave}>💾 Guardar</button>
      </div>
    </div>
  </div>
{/if}

<!-- Toast notification -->
{#if toast}
  <div style="
    position: fixed; bottom: 24px; right: 24px; z-index: 9999;
    background: var(--bg-card); border: 1px solid var(--border-color);
    padding: var(--space-md) var(--space-xl);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-lg);
    font-size: var(--font-size-sm);
    font-weight: 500;
    animation: slideIn 0.2s ease;
  ">
    {toast}
  </div>
{/if}

<style>
  @keyframes slideIn {
    from { transform: translateY(8px); opacity: 0; }
    to   { transform: translateY(0);   opacity: 1; }
  }
</style>
