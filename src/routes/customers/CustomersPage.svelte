<script lang="ts">
  import { onMount } from 'svelte';
  import type { Customer, CreateCustomer } from '$lib/types';
  import { getCustomers, createCustomer, updateCustomer, deleteCustomer } from '$lib/services/api';

  let customers: Customer[] = $state([]);
  let search = $state('');
  let showModal = $state(false);
  let editing: Customer | null = $state(null);
  let form: CreateCustomer = $state({ name: '' });

  onMount(loadCustomers);

  $effect(() => { loadCustomers(); });

  async function loadCustomers() {
    try {
      customers = await getCustomers(search || undefined);
    } catch { customers = []; }
  }

  function openNew() {
    editing = null;
    form = { name: '' };
    showModal = true;
  }

  function openEdit(c: Customer) {
    editing = c;
    form = { name: c.name, nit: c.nit || undefined, email: c.email || undefined, phone: c.phone || undefined, address: c.address || undefined };
    showModal = true;
  }

  async function handleSave() {
    try {
      if (editing) {
        await updateCustomer(editing.id, form);
      } else {
        await createCustomer(form);
      }
      showModal = false;
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
</script>

<div class="page">
  <div class="page-header">
    <div>
      <h1 class="page-title">👥 Clientes</h1>
      <p class="page-subtitle">Gestiona tu cartera de clientes</p>
    </div>
    <button class="btn btn-primary" onclick={openNew}>➕ Nuevo Cliente</button>
  </div>

  <div style="margin-bottom: var(--space-xl);">
    <input class="input" placeholder="🔍 Buscar por nombre, NIT o teléfono..." bind:value={search} />
  </div>

  <div class="table-container">
    <table>
      <thead>
        <tr>
          <th>Nombre</th>
          <th>NIT</th>
          <th>Teléfono</th>
          <th>Email</th>
          <th>Acciones</th>
        </tr>
      </thead>
      <tbody>
        {#if customers.length === 0}
          <tr><td colspan="5" class="text-center text-muted" style="padding: var(--space-3xl);">No hay clientes registrados</td></tr>
        {:else}
          {#each customers as c}
            <tr>
              <td style="font-weight: 600;">{c.name}</td>
              <td class="font-mono">{c.nit || '—'}</td>
              <td>{c.phone || '—'}</td>
              <td class="text-muted">{c.email || '—'}</td>
              <td>
                <div class="flex gap-sm">
                  <button class="btn btn-ghost btn-sm" onclick={() => openEdit(c)}>✏️</button>
                  <button class="btn btn-ghost btn-sm" onclick={() => handleDelete(c.id)}>🗑️</button>
                </div>
              </td>
            </tr>
          {/each}
        {/if}
      </tbody>
    </table>
  </div>
</div>

{#if showModal}
  <div class="modal-overlay" onclick={() => showModal = false}>
    <div class="modal" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h3 class="modal-title">{editing ? '✏️ Editar' : '➕ Nuevo'} Cliente</h3>
        <button class="btn btn-ghost btn-sm" onclick={() => showModal = false}>✕</button>
      </div>
      <div class="modal-body">
        <div class="input-group">
          <label class="input-label">Nombre *</label>
          <input class="input" bind:value={form.name} placeholder="Juan Pérez" />
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
            <input class="input" type="email" bind:value={form.email} placeholder="email@ejemplo.com" />
          </div>
        </div>
        <div class="input-group">
          <label class="input-label">Dirección</label>
          <input class="input" bind:value={form.address} placeholder="Av. Ejemplo #123" />
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-ghost" onclick={() => showModal = false}>Cancelar</button>
        <button class="btn btn-primary" onclick={handleSave} disabled={!form.name}>💾 Guardar</button>
      </div>
    </div>
  </div>
{/if}
