<script lang="ts">
  import { onMount } from 'svelte';
  import type { Customer, CreateCustomer } from '$lib/types';
  import { getCustomers, createCustomer, updateCustomer, deleteCustomer } from '$lib/services/api';
  import { DataTableState } from '$lib/utils/datatable.svelte';
  import TablePagination from '$lib/components/TablePagination.svelte';

  let customers: Customer[] = $state([]);
  let showModal = $state(false);
  let editing: Customer | null = $state(null);
  let form: CreateCustomer = $state({ name: '' });
  let errors: Record<string, string> = $state({});

  let table = new DataTableState<Customer>([], [
    'name',
    'nit',
    'phone',
    'email'
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

  onMount(loadCustomers);

  async function loadCustomers() {
    try {
      customers = await getCustomers();
      table.data = customers;
      table.currentPage = 1;
    } catch {
      customers = [];
      table.data = [];
    }
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

  function openEdit(c: Customer) {
    editing = c;
    form = { name: c.name, nit: c.nit || undefined, email: c.email || undefined, phone: c.phone || undefined, address: c.address || undefined };
    errors = {};
    showModal = true;
  }

  async function handleSave() {
    if (!validate()) return;
    try {
      if (editing) {
        await updateCustomer(editing.id, form);
      } else {
        await createCustomer(form);
      }
      showModal = false;
      errors = {};
      await loadCustomers();
    } catch (e) { alert('Error: ' + e); }
  }

  async function handleDelete(id: string) {
    if (!confirm('¿Eliminar este cliente?')) return;
    try {
      await deleteCustomer(id);
      await loadCustomers();
    } catch (e) { alert('Error: ' + e); }
  }
  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && showModal) {
      showModal = false;
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} onclick={() => { openDropdownId = null; }} />

<div class="page">
  <div class="page-header">
    <div>
      <h1 class="page-title">👥 Clientes</h1>
      <p class="page-subtitle">Gestiona tu cartera de clientes</p>
    </div>
    <button class="btn btn-primary" onclick={openNew}>➕ Nuevo Cliente</button>
  </div>

  <div style="margin-bottom: var(--space-md); position: relative; max-width: 320px;">
    <input
      class="input input-compact"
      style="padding-right: 30px !important;"
      placeholder="🔍 Buscar por nombre, NIT o teléfono..."
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

  <div class="table-container">
    <table>
      <thead>
        <tr>
          <th style="width: 48px;"></th>
          <th onclick={() => table.sortBy('name')} style="cursor: pointer; user-select: none;">
            Nombre {table.sortColumn === 'name' ? (table.sortDirection === 'asc' ? '↑' : '↓') : ''}
          </th>
          <th onclick={() => table.sortBy('nit')} style="cursor: pointer; user-select: none;">
            NIT {table.sortColumn === 'nit' ? (table.sortDirection === 'asc' ? '↑' : '↓') : ''}
          </th>
          <th onclick={() => table.sortBy('phone')} style="cursor: pointer; user-select: none;">
            Teléfono {table.sortColumn === 'phone' ? (table.sortDirection === 'asc' ? '↑' : '↓') : ''}
          </th>
          <th onclick={() => table.sortBy('email')} style="cursor: pointer; user-select: none;">
            Email {table.sortColumn === 'email' ? (table.sortDirection === 'asc' ? '↑' : '↓') : ''}
          </th>
        </tr>
      </thead>
      <tbody>
        {#if table.paginated.length === 0}
          <tr><td colspan="5" class="text-center text-muted" style="padding: var(--space-3xl);">No hay clientes registrados</td></tr>
        {:else}
          {#each table.paginated as c}
            <tr>
              <!-- Acciones como dropdown al inicio -->
              <td style="position: relative;">
                <div class="action-dropdown">
                  <button
                    class="btn btn-ghost btn-sm action-trigger"
                    style="padding: 4px 8px; font-size: var(--font-size-base);"
                    onclick={(e) => toggleDropdown(e, c.id)}
                  >⋮</button>
                  {#if openDropdownId === c.id}
                  <div class="action-menu" role="menu">
                    <button class="action-item" onclick={() => { openDropdownId = null; openEdit(c); }}>✏️ Editar</button>
                    <button class="action-item" onclick={() => { openDropdownId = null; handleDelete(c.id); }}>🗑️ Eliminar</button>
                  </div>
                  {/if}
                </div>
              </td>
              <td style="font-weight: 600;">{c.name}</td>
              <td class="font-mono">{c.nit || '—'}</td>
              <td>{c.phone || '—'}</td>
              <td class="text-muted">{c.email || '—'}</td>
            </tr>
          {/each}
        {/if}
      </tbody>
    </table>
  </div>
  <TablePagination {table} />
</div>

{#if showModal}
  <div class="modal-overlay">
    <div class="modal" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h3 class="modal-title">{editing ? '✏️ Editar' : '➕ Nuevo'} Cliente</h3>
        <button class="btn btn-ghost btn-sm" onclick={() => showModal = false}>✕</button>
      </div>
      <div class="modal-body">
        <div class="input-group">
          <label class="input-label">Nombre *</label>
          <input class="input" class:input-error={errors.name} bind:value={form.name} oninput={() => clearError('name')} placeholder="Juan Pérez" />
          {#if errors.name}<span class="field-error">{errors.name}</span>{/if}
        </div>
        <div class="input-group">
          <label class="input-label">NIT</label>
          <input class="input" bind:value={form.nit} placeholder="1234567013" />
        </div>
        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-lg);">
          <div class="input-group">
            <label class="input-label">Teléfono</label>
            <input class="input" bind:value={form.phone} placeholder="+591 7XXXXXXX" />
          </div>
          <div class="input-group">
            <label class="input-label">Email</label>
            <input class="input" class:input-error={errors.email} type="email" bind:value={form.email} oninput={() => clearError('email')} placeholder="email@ejemplo.com" />
            {#if errors.email}<span class="field-error">{errors.email}</span>{/if}
          </div>
        </div>
        <div class="input-group">
          <label class="input-label">Dirección</label>
          <input class="input" bind:value={form.address} placeholder="Av. Ejemplo #123" />
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-ghost" onclick={() => showModal = false}>Cancelar</button>
        <button class="btn btn-primary" onclick={handleSave}>💾 Guardar</button>
      </div>
    </div>
  </div>
{/if}
