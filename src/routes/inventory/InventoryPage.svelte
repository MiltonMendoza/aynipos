<script lang="ts">
  import { onMount } from 'svelte';
  import type { ProductWithStock, Category, CreateProduct } from '$lib/types';
  import { getInventory, adjustInventory, getCategories, createProduct, createCategory } from '$lib/services/api';

  let inventory: ProductWithStock[] = $state([]);
  let categories: Category[] = $state([]);
  let filter = $state<'all' | 'low' | 'expiring'>('all');
  let showAddProduct = $state(false);
  let showAddCategory = $state(false);
  let showAdjust = $state(false);
  let adjustProduct: ProductWithStock | null = $state(null);
  let adjustQty = $state(0);
  let adjustType = $state('purchase');
  let adjustNotes = $state('');

  // New product form
  let newProduct: CreateProduct = $state({
    sku: '', name: '', purchase_price: 0, sale_price: 0,
  });
  let newCategoryName = $state('');

  onMount(loadInventory);

  async function loadInventory() {
    try {
      if (filter === 'low') {
        inventory = await getInventory(true);
      } else if (filter === 'expiring') {
        inventory = await getInventory(false, 30);
      } else {
        inventory = await getInventory();
      }
      categories = await getCategories();
    } catch { inventory = []; }
  }

  async function handleAddProduct() {
    try {
      await createProduct(newProduct);
      showAddProduct = false;
      newProduct = { sku: '', name: '', purchase_price: 0, sale_price: 0 };
      await loadInventory();
    } catch (e) { alert('Error: ' + e); }
  }

  async function handleAddCategory() {
    if (!newCategoryName.trim()) return;
    try {
      await createCategory({ name: newCategoryName });
      newCategoryName = '';
      showAddCategory = false;
      await loadInventory();
    } catch (e) { alert('Error: ' + e); }
  }

  function openAdjust(ps: ProductWithStock) {
    adjustProduct = ps;
    adjustQty = 0;
    adjustType = 'purchase';
    adjustNotes = '';
    showAdjust = true;
  }

  async function handleAdjust() {
    if (!adjustProduct || adjustQty === 0) return;
    const qty = adjustType === 'adjustment' && adjustQty < 0 ? adjustQty : Math.abs(adjustQty);
    try {
      await adjustInventory(adjustProduct.product.id, qty, adjustType, adjustNotes || undefined);
      showAdjust = false;
      await loadInventory();
    } catch (e) { alert('Error: ' + e); }
  }

  function formatCurrency(n: number) { return `Bs ${n.toFixed(2)}`; }

  $effect(() => {
    loadInventory();
  });
</script>

<div class="page">
  <div class="page-header">
    <div>
      <h1 class="page-title">📦 Inventario</h1>
      <p class="page-subtitle">Gestiona productos, stock y categorías</p>
    </div>
    <div class="flex gap-md">
      <button class="btn btn-ghost" onclick={() => showAddCategory = true}>➕ Categoría</button>
      <button class="btn btn-primary" onclick={() => showAddProduct = true}>➕ Nuevo Producto</button>
    </div>
  </div>

  <!-- Filters -->
  <div class="flex gap-md" style="margin-bottom: var(--space-xl);">
    {#each [
      { key: 'all' as const, label: 'Todos', icon: '📋' },
      { key: 'low' as const, label: 'Bajo Stock', icon: '⚠️' },
      { key: 'expiring' as const, label: 'Por Vencer', icon: '⏰' },
    ] as f}
      <button
        class="btn"
        class:btn-primary={filter === f.key}
        class:btn-ghost={filter !== f.key}
        onclick={() => filter = f.key}
      >
        {f.icon} {f.label}
      </button>
    {/each}
  </div>

  <!-- Inventory table -->
  <div class="table-container">
    <table>
      <thead>
        <tr>
          <th>SKU</th>
          <th>Producto</th>
          <th>Categoría</th>
          <th>Precio Compra</th>
          <th>Precio Venta</th>
          <th>Stock</th>
          <th>Estado</th>
          <th>Acciones</th>
        </tr>
      </thead>
      <tbody>
        {#if inventory.length === 0}
          <tr><td colspan="8" class="text-center text-muted" style="padding: var(--space-3xl);">Sin productos</td></tr>
        {:else}
          {#each inventory as ps}
            <tr>
              <td class="font-mono text-sm">{ps.product.sku}</td>
              <td style="font-weight: 600;">{ps.product.name}</td>
              <td class="text-muted">{ps.category_name || '—'}</td>
              <td>{formatCurrency(ps.product.purchase_price)}</td>
              <td style="font-weight: 600; color: var(--accent-primary);">{formatCurrency(ps.product.sale_price)}</td>
              <td style="font-weight: 700;">{ps.current_stock}</td>
              <td>
                {#if ps.current_stock <= ps.product.min_stock && ps.product.min_stock > 0}
                  <span class="badge badge-danger">Bajo</span>
                {:else}
                  <span class="badge badge-success">OK</span>
                {/if}
              </td>
              <td>
                <button class="btn btn-ghost btn-sm" onclick={() => openAdjust(ps)}>📊 Ajustar</button>
              </td>
            </tr>
          {/each}
        {/if}
      </tbody>
    </table>
  </div>
</div>

<!-- Add Product Modal -->
{#if showAddProduct}
  <div class="modal-overlay" onclick={() => showAddProduct = false}>
    <div class="modal modal-lg" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h3 class="modal-title">➕ Nuevo Producto</h3>
        <button class="btn btn-ghost btn-sm" onclick={() => showAddProduct = false}>✕</button>
      </div>
      <div class="modal-body">
        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-lg);">
          <div class="input-group">
            <label class="input-label">SKU *</label>
            <input class="input" bind:value={newProduct.sku} placeholder="P001" />
          </div>
          <div class="input-group">
            <label class="input-label">Código de barras</label>
            <input class="input" bind:value={newProduct.barcode} placeholder="7890000..." />
          </div>
        </div>
        <div class="input-group">
          <label class="input-label">Nombre del producto *</label>
          <input class="input" bind:value={newProduct.name} placeholder="Paracetamol 500mg" />
        </div>
        <div class="input-group">
          <label class="input-label">Categoría</label>
          <select class="select" bind:value={newProduct.category_id}>
            <option value="">Sin categoría</option>
            {#each categories as cat}
              <option value={cat.id}>{cat.name}</option>
            {/each}
          </select>
        </div>
        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-lg);">
          <div class="input-group">
            <label class="input-label">Precio Compra (Bs) *</label>
            <input class="input" type="number" bind:value={newProduct.purchase_price} step="0.01" min="0" />
          </div>
          <div class="input-group">
            <label class="input-label">Precio Venta (Bs) *</label>
            <input class="input" type="number" bind:value={newProduct.sale_price} step="0.01" min="0" />
          </div>
        </div>
        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-lg);">
          <div class="input-group">
            <label class="input-label">Unidad</label>
            <input class="input" bind:value={newProduct.unit} placeholder="unidad" />
          </div>
          <div class="input-group">
            <label class="input-label">Stock mínimo</label>
            <input class="input" type="number" bind:value={newProduct.min_stock} min="0" />
          </div>
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-ghost" onclick={() => showAddProduct = false}>Cancelar</button>
        <button class="btn btn-primary" onclick={handleAddProduct} disabled={!newProduct.sku || !newProduct.name}>
          💾 Guardar Producto
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Add Category Modal -->
{#if showAddCategory}
  <div class="modal-overlay" onclick={() => showAddCategory = false}>
    <div class="modal" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h3 class="modal-title">➕ Nueva Categoría</h3>
        <button class="btn btn-ghost btn-sm" onclick={() => showAddCategory = false}>✕</button>
      </div>
      <div class="modal-body">
        <div class="input-group">
          <label class="input-label">Nombre</label>
          <input class="input" bind:value={newCategoryName} placeholder="Medicamentos" />
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-ghost" onclick={() => showAddCategory = false}>Cancelar</button>
        <button class="btn btn-primary" onclick={handleAddCategory}>💾 Guardar</button>
      </div>
    </div>
  </div>
{/if}

<!-- Adjust Inventory Modal -->
{#if showAdjust && adjustProduct}
  <div class="modal-overlay" onclick={() => showAdjust = false}>
    <div class="modal" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h3 class="modal-title">📊 Ajustar Inventario</h3>
        <button class="btn btn-ghost btn-sm" onclick={() => showAdjust = false}>✕</button>
      </div>
      <div class="modal-body">
        <div style="background: var(--bg-tertiary); border-radius: var(--radius-md); padding: var(--space-lg);">
          <div style="font-weight: 700;">{adjustProduct.product.name}</div>
          <div class="text-sm text-muted">Stock actual: <strong>{adjustProduct.current_stock}</strong></div>
        </div>
        <div class="input-group">
          <label class="input-label">Tipo de movimiento</label>
          <select class="select" bind:value={adjustType}>
            <option value="purchase">Compra (entrada)</option>
            <option value="adjustment">Ajuste</option>
            <option value="return">Devolución</option>
          </select>
        </div>
        <div class="input-group">
          <label class="input-label">Cantidad</label>
          <input class="input" type="number" bind:value={adjustQty} />
        </div>
        <div class="input-group">
          <label class="input-label">Notas</label>
          <input class="input" bind:value={adjustNotes} placeholder="Motivo del ajuste..." />
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-ghost" onclick={() => showAdjust = false}>Cancelar</button>
        <button class="btn btn-primary" onclick={handleAdjust}>✅ Aplicar</button>
      </div>
    </div>
  </div>
{/if}
