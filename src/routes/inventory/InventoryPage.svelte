<script lang="ts">
  import { onMount } from 'svelte';
  import type { ProductWithStock, Category, CreateProduct, UpdateProduct, ImportResult, InventoryLot, InventoryMovement, User } from '$lib/types';
  import { getInventory, adjustInventory, getCategories, createProduct, createCategory, updateProduct, exportProductsCsv, importProductsCsv, getProductLots, deleteLot, getInventoryMovements, logAction } from '$lib/services/api';

  let { currentUser }: { currentUser: User | null } = $props();
  import { save, open } from '@tauri-apps/plugin-dialog';

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
  let adjustLotNumber = $state('');
  let adjustExpiryDate = $state('');

  // Edit product
  let showEditProduct = $state(false);
  let editProduct: UpdateProduct = $state({ id: '' });
  let editErrors: Record<string, string> = $state({});

  // New product form
  let newProduct: CreateProduct = $state({
    sku: '', name: '', purchase_price: 0, sale_price: 0,
  });
  let newCategoryName = $state('');
  let showInlineCategoryAdd = $state(false); // for Add Product modal
  let showInlineCategoryEdit = $state(false); // for Edit Product modal
  let inlineCategoryName = $state('');
  let inlineCategoryError = $state('');
  let inlineCategoryTarget: 'add' | 'edit' = $state('add');

  // Validation errors
  let productErrors: Record<string, string> = $state({});
  let categoryErrors: Record<string, string> = $state({});
  let adjustErrors: Record<string, string> = $state({});

  // Import/Export
  let showImportResult = $state(false);
  let importResult: ImportResult | null = $state(null);
  let isExporting = $state(false);
  let isImporting = $state(false);

  // Lots
  let showLots = $state(false);
  let lotsProduct: ProductWithStock | null = $state(null);
  let lots: InventoryLot[] = $state([]);
  let lotsLoading = $state(false);

  // Movement history
  let showMovements = $state(false);
  let movementsProduct: ProductWithStock | null = $state(null);
  let movements: InventoryMovement[] = $state([]);
  let movementsLoading = $state(false);

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

  function validateProduct(): boolean {
    const e: Record<string, string> = {};
    if (!newProduct.sku.trim()) e.sku = 'El SKU es obligatorio';
    if (!newProduct.name.trim()) e.name = 'El nombre es obligatorio';
    if (!newProduct.category_id) e.category_id = 'La categoría es obligatoria';
    if (newProduct.purchase_price <= 0) e.purchase_price = 'El precio de compra debe ser mayor a 0';
    if (newProduct.sale_price <= 0) e.sale_price = 'El precio de venta debe ser mayor a 0';
    if (newProduct.sale_price > 0 && newProduct.purchase_price > 0 && newProduct.sale_price < newProduct.purchase_price) {
      e.sale_price = 'El precio de venta debe ser mayor o igual al de compra';
    }
    productErrors = e;
    return Object.keys(e).length === 0;
  }

  function validateCategory(): boolean {
    const e: Record<string, string> = {};
    if (!newCategoryName.trim()) e.name = 'El nombre de la categoría es obligatorio';
    categoryErrors = e;
    return Object.keys(e).length === 0;
  }

  function validateAdjust(): boolean {
    const e: Record<string, string> = {};
    if (adjustQty === 0) e.qty = 'La cantidad no puede ser 0';
    adjustErrors = e;
    return Object.keys(e).length === 0;
  }

  function clearProductError(field: string) {
    if (productErrors[field]) {
      const copy = { ...productErrors };
      delete copy[field];
      productErrors = copy;
    }
  }

  async function handleAddProduct() {
    if (!validateProduct()) return;
    try {
      const created = await createProduct(newProduct);
      if (currentUser) {
        logAction(currentUser.id, currentUser.name, 'product_created', 'product', created.id, `Producto "${newProduct.name}" creado`);
      }
      showAddProduct = false;
      newProduct = { sku: '', name: '', purchase_price: 0, sale_price: 0 };
      productErrors = {};
      await loadInventory();
    } catch (e) {
      const msg = String(e);
      if (msg.includes('código de barras')) {
        productErrors = { ...productErrors, barcode: msg };
      } else {
        alert('Error: ' + msg);
      }
    }
  }

  async function handleAddCategory() {
    if (!validateCategory()) return;
    try {
      await createCategory({ name: newCategoryName });
      newCategoryName = '';
      categoryErrors = {};
      showAddCategory = false;
      await loadInventory();
    } catch (e) { alert('Error: ' + e); }
  }

  async function handleInlineCategory() {
    if (!inlineCategoryName.trim()) {
      inlineCategoryError = 'El nombre es obligatorio';
      return;
    }
    try {
      const created = await createCategory({ name: inlineCategoryName.trim() });
      await loadInventory();
      // Auto-select the newly created category
      if (inlineCategoryTarget === 'add') {
        newProduct.category_id = created.id;
        clearProductError('category_id');
        showInlineCategoryAdd = false;
      } else {
        editProduct.category_id = created.id;
        clearEditError('category_id');
        showInlineCategoryEdit = false;
      }
      inlineCategoryName = '';
      inlineCategoryError = '';
    } catch (e) { alert('Error: ' + e); }
  }

  function openInlineCategory(target: 'add' | 'edit') {
    inlineCategoryTarget = target;
    inlineCategoryName = '';
    inlineCategoryError = '';
    if (target === 'add') {
      showInlineCategoryAdd = true;
      showInlineCategoryEdit = false;
    } else {
      showInlineCategoryEdit = true;
      showInlineCategoryAdd = false;
    }
  }

  function openAdjust(ps: ProductWithStock) {
    adjustProduct = ps;
    adjustQty = 0;
    adjustType = 'purchase';
    adjustNotes = '';
    adjustLotNumber = '';
    adjustExpiryDate = '';
    adjustErrors = {};
    showAdjust = true;
  }

  async function handleAdjust() {
    if (!validateAdjust()) return;
    if (!adjustProduct) return;
    const qty = adjustType === 'adjustment' && adjustQty < 0 ? adjustQty : Math.abs(adjustQty);
    try {
      await adjustInventory(
        adjustProduct.product.id, qty, adjustType,
        adjustNotes || undefined,
        adjustLotNumber || undefined,
        adjustExpiryDate || undefined
      );
      if (currentUser) {
        const sign = qty >= 0 ? '+' : '';
        logAction(currentUser.id, currentUser.name, 'inventory_adjusted', 'product', adjustProduct.product.id, `${sign}${qty} unidades de "${adjustProduct.product.name}" (${adjustType})`);
      }
      showAdjust = false;
      await loadInventory();
    } catch (e) { alert('Error: ' + e); }
  }

  function openAddProduct() {
    newProduct = { sku: '', name: '', purchase_price: 0, sale_price: 0 };
    productErrors = {};
    showAddProduct = true;
  }

  function openAddCategory() {
    newCategoryName = '';
    categoryErrors = {};
    showAddCategory = true;
  }

  function openEditProduct(ps: ProductWithStock) {
    editProduct = {
      id: ps.product.id,
      sku: ps.product.sku,
      barcode: ps.product.barcode ?? undefined,
      name: ps.product.name,
      description: ps.product.description ?? undefined,
      category_id: ps.product.category_id ?? undefined,
      purchase_price: ps.product.purchase_price,
      sale_price: ps.product.sale_price,
      tax_rate: ps.product.tax_rate,
      unit: ps.product.unit,
      min_stock: ps.product.min_stock,
    };
    editErrors = {};
    showEditProduct = true;
  }

  function validateEditProduct(): boolean {
    const e: Record<string, string> = {};
    if (!editProduct.sku?.trim()) e.sku = 'El SKU es obligatorio';
    if (!editProduct.name?.trim()) e.name = 'El nombre es obligatorio';
    if (!editProduct.category_id) e.category_id = 'La categoría es obligatoria';
    if ((editProduct.purchase_price ?? 0) <= 0) e.purchase_price = 'El precio de compra debe ser mayor a 0';
    if ((editProduct.sale_price ?? 0) <= 0) e.sale_price = 'El precio de venta debe ser mayor a 0';
    if ((editProduct.sale_price ?? 0) > 0 && (editProduct.purchase_price ?? 0) > 0 && (editProduct.sale_price ?? 0) < (editProduct.purchase_price ?? 0)) {
      e.sale_price = 'El precio de venta debe ser mayor o igual al de compra';
    }
    editErrors = e;
    return Object.keys(e).length === 0;
  }

  function clearEditError(field: string) {
    if (editErrors[field]) {
      const copy = { ...editErrors };
      delete copy[field];
      editErrors = copy;
    }
  }

  async function handleEditProduct() {
    if (!validateEditProduct()) return;
    try {
      await updateProduct(editProduct);
      if (currentUser) {
        logAction(currentUser.id, currentUser.name, 'product_updated', 'product', editProduct.id, `Producto "${editProduct.name}" actualizado`);
      }
      showEditProduct = false;
      editErrors = {};
      await loadInventory();
    } catch (e) {
      const msg = String(e);
      if (msg.includes('código de barras')) {
        editErrors = { ...editErrors, barcode: msg };
      } else {
        alert('Error: ' + msg);
      }
    }
  }

  function formatCurrency(n: number) { return `Bs ${n.toFixed(2)}`; }

  async function handleExportCsv() {
    isExporting = true;
    try {
      const filePath = await save({
        title: 'Exportar productos a CSV',
        defaultPath: 'productos.csv',
        filters: [{ name: 'CSV', extensions: ['csv'] }],
      });
      if (!filePath) { isExporting = false; return; }
      const count = await exportProductsCsv(filePath);
      alert(`✅ Se exportaron ${count} productos a CSV`);
    } catch (e) {
      alert('Error al exportar: ' + e);
    } finally {
      isExporting = false;
    }
  }

  async function handleImportCsv() {
    isImporting = true;
    try {
      const selected = await open({
        title: 'Importar productos desde CSV',
        multiple: false,
        filters: [{ name: 'CSV', extensions: ['csv'] }],
      });
      if (!selected) { isImporting = false; return; }
      const result = await importProductsCsv(selected);
      importResult = result;
      showImportResult = true;
      await loadInventory();
    } catch (e) {
      alert('Error al importar: ' + e);
    } finally {
      isImporting = false;
    }
  }

  async function openLots(ps: ProductWithStock) {
    lotsProduct = ps;
    lotsLoading = true;
    showLots = true;
    try {
      lots = await getProductLots(ps.product.id);
    } catch (e) {
      alert('Error al cargar lotes: ' + e);
      lots = [];
    } finally {
      lotsLoading = false;
    }
  }

  async function handleDeleteLot(lotId: string) {
    if (!confirm('¿Eliminar este lote vacío?')) return;
    try {
      await deleteLot(lotId);
      if (lotsProduct) {
        lots = await getProductLots(lotsProduct.product.id);
      }
      await loadInventory();
    } catch (e) {
      alert('Error: ' + e);
    }
  }

  function formatDate(date: string | null): string {
    if (!date) return '—';
    try {
      const d = new Date(date + 'T00:00:00');
      return d.toLocaleDateString('es-BO', { year: 'numeric', month: 'short', day: 'numeric' });
    } catch {
      return date;
    }
  }

  function formatDateTime(date: string | null): string {
    if (!date) return '—';
    try {
      const d = new Date(date);
      return d.toLocaleDateString('es-BO', { year: 'numeric', month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' });
    } catch {
      return date;
    }
  }

  function expiryBadge(status: string): { label: string; class: string } {
    switch (status) {
      case 'expired': return { label: '❌ Vencido', class: 'badge-danger' };
      case 'danger': return { label: '🔴 Crítico', class: 'badge-danger' };
      case 'warning': return { label: '🟡 Por vencer', class: 'badge-warning' };
      default: return { label: '🟢 OK', class: 'badge-success' };
    }
  }

  function movementBadge(type: string): { label: string; class: string } {
    switch (type) {
      case 'sale': return { label: '🔴 Venta', class: 'badge-danger' };
      case 'purchase': return { label: '🟢 Compra', class: 'badge-success' };
      case 'return': return { label: '🔵 Devolución', class: 'badge-info' };
      case 'adjustment': return { label: '🟡 Ajuste', class: 'badge-warning' };
      default: return { label: type, class: 'badge' };
    }
  }

  async function openMovements(ps: ProductWithStock) {
    movementsProduct = ps;
    movementsLoading = true;
    showMovements = true;
    try {
      movements = await getInventoryMovements(ps.product.id, 100);
    } catch (e) {
      alert('Error al cargar historial: ' + e);
      movements = [];
    } finally {
      movementsLoading = false;
    }
  }

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
      <button class="btn btn-ghost" onclick={handleImportCsv} disabled={isImporting}>
        {isImporting ? '⏳ Importando...' : '📥 Importar CSV'}
      </button>
      <button class="btn btn-ghost" onclick={handleExportCsv} disabled={isExporting}>
        {isExporting ? '⏳ Exportando...' : '📤 Exportar CSV'}
      </button>
      <button class="btn btn-ghost" onclick={openAddCategory}>➕ Categoría</button>
      <button class="btn btn-primary" onclick={openAddProduct}>➕ Nuevo Producto</button>
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
                <button class="btn btn-ghost btn-sm" onclick={() => openEditProduct(ps)}>✏️ Editar</button>
                <button class="btn btn-ghost btn-sm" onclick={() => openLots(ps)}>📦 Lotes</button>
                <button class="btn btn-ghost btn-sm" onclick={() => openAdjust(ps)}>📊 Ajustar</button>
                <button class="btn btn-ghost btn-sm" onclick={() => openMovements(ps)}>📜 Historial</button>
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
  <div class="modal-overlay">
    <div class="modal modal-lg" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h3 class="modal-title">➕ Nuevo Producto</h3>
        <button class="btn btn-ghost btn-sm" onclick={() => showAddProduct = false}>✕</button>
      </div>
      <div class="modal-body">
        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-lg);">
          <div class="input-group">
            <label class="input-label">SKU *</label>
            <input class="input" class:input-error={productErrors.sku} bind:value={newProduct.sku} oninput={() => clearProductError('sku')} placeholder="P001" />
            {#if productErrors.sku}<span class="field-error">{productErrors.sku}</span>{/if}
          </div>
          <div class="input-group">
            <label class="input-label">Código de barras</label>
            <input class="input" class:input-error={productErrors.barcode} bind:value={newProduct.barcode} oninput={() => clearProductError('barcode')} placeholder="7890000..." />
            {#if productErrors.barcode}<span class="field-error">{productErrors.barcode}</span>{/if}
          </div>
        </div>
        <div class="input-group">
          <label class="input-label">Nombre del producto *</label>
          <input class="input" class:input-error={productErrors.name} bind:value={newProduct.name} oninput={() => clearProductError('name')} placeholder="Paracetamol 500mg" />
          {#if productErrors.name}<span class="field-error">{productErrors.name}</span>{/if}
        </div>
        <div class="input-group">
          <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: var(--space-xs);">
            <label class="input-label" style="margin-bottom: 0;">Categoría *</label>
            <button
              type="button"
              class="btn btn-ghost btn-sm"
              style="font-size: var(--font-size-xs); padding: 2px 8px; color: var(--accent-primary);"
              onclick={() => openInlineCategory('add')}
            >
              {showInlineCategoryAdd ? '✕ Cancelar' : '＋ Nueva categoría'}
            </button>
          </div>
          {#if categories.length === 0 && !showInlineCategoryAdd}
            <div style="display: flex; align-items: center; gap: var(--space-sm); padding: var(--space-md); background: color-mix(in srgb, var(--accent-warning) 10%, transparent); border: 1px solid color-mix(in srgb, var(--accent-warning) 30%, transparent); border-radius: var(--radius-md); margin-bottom: var(--space-sm);">
              <span style="font-size: 1.1rem;">💡</span>
              <span class="text-sm" style="color: var(--text-secondary);">No hay categorías aún. Crea una con el botón <strong>＋ Nueva categoría</strong>.</span>
            </div>
          {/if}
          <select class="select" class:input-error={productErrors.category_id} bind:value={newProduct.category_id} onchange={() => clearProductError('category_id')}>
            <option value="">Seleccionar categoría</option>
            {#each categories as cat}
              <option value={cat.id}>{cat.name}</option>
            {/each}
          </select>
          {#if productErrors.category_id}<span class="field-error">{productErrors.category_id}</span>{/if}
          {#if showInlineCategoryAdd}
            <div style="display: flex; gap: var(--space-sm); margin-top: var(--space-sm); padding: var(--space-md); background: color-mix(in srgb, var(--accent-primary) 8%, transparent); border: 1px solid color-mix(in srgb, var(--accent-primary) 25%, transparent); border-radius: var(--radius-md); flex-direction: column;">
              <div class="text-sm" style="font-weight: 600; color: var(--accent-primary);">Nueva categoría</div>
              <div style="display: flex; gap: var(--space-sm); align-items: flex-start;">
                <div style="flex: 1;">
                  <input
                    class="input"
                    class:input-error={!!inlineCategoryError}
                    bind:value={inlineCategoryName}
                    placeholder="Ej: Medicamentos"
                    oninput={() => inlineCategoryError = ''}
                    onkeydown={(e) => { if (e.key === 'Enter') { e.preventDefault(); handleInlineCategory(); } }}
                    style="margin: 0;"
                  />
                  {#if inlineCategoryError}<span class="field-error">{inlineCategoryError}</span>{/if}
                </div>
                <button type="button" class="btn btn-primary btn-sm" onclick={handleInlineCategory} style="white-space: nowrap;">✓ Crear</button>
              </div>
            </div>
          {/if}
        </div>
        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-lg);">
          <div class="input-group">
            <label class="input-label">Precio Compra (Bs) *</label>
            <input class="input" class:input-error={productErrors.purchase_price} type="number" bind:value={newProduct.purchase_price} oninput={() => clearProductError('purchase_price')} step="0.01" min="0" />
            {#if productErrors.purchase_price}<span class="field-error">{productErrors.purchase_price}</span>{/if}
          </div>
          <div class="input-group">
            <label class="input-label">Precio Venta (Bs) *</label>
            <input class="input" class:input-error={productErrors.sale_price} type="number" bind:value={newProduct.sale_price} oninput={() => clearProductError('sale_price')} step="0.01" min="0" />
            {#if productErrors.sale_price}<span class="field-error">{productErrors.sale_price}</span>{/if}
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
        <button class="btn btn-primary" onclick={handleAddProduct}>
          💾 Guardar Producto
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Edit Product Modal -->
{#if showEditProduct}
  <div class="modal-overlay">
    <div class="modal modal-lg" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h3 class="modal-title">✏️ Editar Producto</h3>
        <button class="btn btn-ghost btn-sm" onclick={() => showEditProduct = false}>✕</button>
      </div>
      <div class="modal-body">
        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-lg);">
          <div class="input-group">
            <label class="input-label">SKU *</label>
            <input class="input" class:input-error={editErrors.sku} bind:value={editProduct.sku} oninput={() => clearEditError('sku')} placeholder="P001" />
            {#if editErrors.sku}<span class="field-error">{editErrors.sku}</span>{/if}
          </div>
          <div class="input-group">
            <label class="input-label">Código de barras</label>
            <input class="input" class:input-error={editErrors.barcode} bind:value={editProduct.barcode} oninput={() => clearEditError('barcode')} placeholder="7890000..." />
            {#if editErrors.barcode}<span class="field-error">{editErrors.barcode}</span>{/if}
          </div>
        </div>
        <div class="input-group">
          <label class="input-label">Nombre del producto *</label>
          <input class="input" class:input-error={editErrors.name} bind:value={editProduct.name} oninput={() => clearEditError('name')} placeholder="Paracetamol 500mg" />
          {#if editErrors.name}<span class="field-error">{editErrors.name}</span>{/if}
        </div>
        <div class="input-group">
          <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: var(--space-xs);">
            <label class="input-label" style="margin-bottom: 0;">Categoría *</label>
            <button
              type="button"
              class="btn btn-ghost btn-sm"
              style="font-size: var(--font-size-xs); padding: 2px 8px; color: var(--accent-primary);"
              onclick={() => openInlineCategory('edit')}
            >
              {showInlineCategoryEdit ? '✕ Cancelar' : '＋ Nueva categoría'}
            </button>
          </div>
          {#if categories.length === 0 && !showInlineCategoryEdit}
            <div style="display: flex; align-items: center; gap: var(--space-sm); padding: var(--space-md); background: color-mix(in srgb, var(--accent-warning) 10%, transparent); border: 1px solid color-mix(in srgb, var(--accent-warning) 30%, transparent); border-radius: var(--radius-md); margin-bottom: var(--space-sm);">
              <span style="font-size: 1.1rem;">💡</span>
              <span class="text-sm" style="color: var(--text-secondary);">No hay categorías aún. Crea una con el botón <strong>＋ Nueva categoría</strong>.</span>
            </div>
          {/if}
          <select class="select" class:input-error={editErrors.category_id} bind:value={editProduct.category_id} onchange={() => clearEditError('category_id')}
          >
            <option value="">Seleccionar categoría</option>
            {#each categories as cat}
              <option value={cat.id}>{cat.name}</option>
            {/each}
          </select>
          {#if editErrors.category_id}<span class="field-error">{editErrors.category_id}</span>{/if}
          {#if showInlineCategoryEdit}
            <div style="display: flex; gap: var(--space-sm); margin-top: var(--space-sm); padding: var(--space-md); background: color-mix(in srgb, var(--accent-primary) 8%, transparent); border: 1px solid color-mix(in srgb, var(--accent-primary) 25%, transparent); border-radius: var(--radius-md); flex-direction: column;">
              <div class="text-sm" style="font-weight: 600; color: var(--accent-primary);">Nueva categoría</div>
              <div style="display: flex; gap: var(--space-sm); align-items: flex-start;">
                <div style="flex: 1;">
                  <input
                    class="input"
                    class:input-error={!!inlineCategoryError}
                    bind:value={inlineCategoryName}
                    placeholder="Ej: Medicamentos"
                    oninput={() => inlineCategoryError = ''}
                    onkeydown={(e) => { if (e.key === 'Enter') { e.preventDefault(); handleInlineCategory(); } }}
                    style="margin: 0;"
                  />
                  {#if inlineCategoryError}<span class="field-error">{inlineCategoryError}</span>{/if}
                </div>
                <button type="button" class="btn btn-primary btn-sm" onclick={handleInlineCategory} style="white-space: nowrap;">✓ Crear</button>
              </div>
            </div>
          {/if}
        </div>
        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-lg);">
          <div class="input-group">
            <label class="input-label">Precio Compra (Bs) *</label>
            <input class="input" class:input-error={editErrors.purchase_price} type="number" bind:value={editProduct.purchase_price} oninput={() => clearEditError('purchase_price')} step="0.01" min="0" />
            {#if editErrors.purchase_price}<span class="field-error">{editErrors.purchase_price}</span>{/if}
          </div>
          <div class="input-group">
            <label class="input-label">Precio Venta (Bs) *</label>
            <input class="input" class:input-error={editErrors.sale_price} type="number" bind:value={editProduct.sale_price} oninput={() => clearEditError('sale_price')} step="0.01" min="0" />
            {#if editErrors.sale_price}<span class="field-error">{editErrors.sale_price}</span>{/if}
          </div>
        </div>
        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-lg);">
          <div class="input-group">
            <label class="input-label">Unidad</label>
            <input class="input" bind:value={editProduct.unit} placeholder="unidad" />
          </div>
          <div class="input-group">
            <label class="input-label">Stock mínimo</label>
            <input class="input" type="number" bind:value={editProduct.min_stock} min="0" />
          </div>
        </div>
        <div class="input-group">
          <label class="input-label">Descripción</label>
          <textarea class="input" bind:value={editProduct.description} placeholder="Descripción del producto..." rows="3" style="resize: vertical;"></textarea>
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-ghost" onclick={() => showEditProduct = false}>Cancelar</button>
        <button class="btn btn-primary" onclick={handleEditProduct}>
          💾 Guardar Cambios
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Add Category Modal -->
{#if showAddCategory}
  <div class="modal-overlay">
    <div class="modal" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h3 class="modal-title">➕ Nueva Categoría</h3>
        <button class="btn btn-ghost btn-sm" onclick={() => showAddCategory = false}>✕</button>
      </div>
      <div class="modal-body">
        <div class="input-group">
          <label class="input-label">Nombre *</label>
          <input class="input" class:input-error={categoryErrors.name} bind:value={newCategoryName} oninput={() => { if (categoryErrors.name) categoryErrors = {}; }} placeholder="Medicamentos" />
          {#if categoryErrors.name}<span class="field-error">{categoryErrors.name}</span>{/if}
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
  <div class="modal-overlay">
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
          <label class="input-label">Cantidad *</label>
          <input class="input" class:input-error={adjustErrors.qty} type="number" bind:value={adjustQty} oninput={() => { if (adjustErrors.qty) adjustErrors = {}; }} />
          {#if adjustErrors.qty}<span class="field-error">{adjustErrors.qty}</span>{/if}
        </div>
        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-lg);">
          <div class="input-group">
            <label class="input-label">Número de lote</label>
            <input class="input" bind:value={adjustLotNumber} placeholder="LOTE-2026-A" />
          </div>
          <div class="input-group">
            <label class="input-label">Fecha de vencimiento</label>
            <input class="input" type="date" bind:value={adjustExpiryDate} />
          </div>
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

<!-- Lots Modal -->
{#if showLots && lotsProduct}
  <div class="modal-overlay">
    <div class="modal modal-lg" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h3 class="modal-title">📦 Lotes — {lotsProduct.product.name}</h3>
        <button class="btn btn-ghost btn-sm" onclick={() => showLots = false}>✕</button>
      </div>
      <div class="modal-body">
        {#if lotsLoading}
          <div class="text-center text-muted" style="padding: var(--space-2xl);">Cargando lotes...</div>
        {:else if lots.length === 0}
          <div class="text-center text-muted" style="padding: var(--space-2xl);">
            <div style="font-size: var(--font-size-2xl); margin-bottom: var(--space-md);">📭</div>
            <div>No hay lotes registrados para este producto.</div>
            <div class="text-sm" style="margin-top: var(--space-sm);">Usa "📊 Ajustar" para agregar stock con número de lote.</div>
          </div>
        {:else}
          <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: var(--space-lg); margin-bottom: var(--space-xl);">
            <div class="stat-card" style="text-align: center;">
              <div style="font-size: var(--font-size-2xl); font-weight: 700; color: var(--accent-primary);">{lots.length}</div>
              <div class="text-sm text-muted">Total lotes</div>
            </div>
            <div class="stat-card" style="text-align: center;">
              <div style="font-size: var(--font-size-2xl); font-weight: 700; color: var(--accent-success);">{lots.reduce((s, l) => s + l.quantity, 0)}</div>
              <div class="text-sm text-muted">Stock total</div>
            </div>
            <div class="stat-card" style="text-align: center;">
              <div style="font-size: var(--font-size-2xl); font-weight: 700; color: var(--accent-danger);">{lots.filter(l => l.expiry_status === 'danger' || l.expiry_status === 'expired').length}</div>
              <div class="text-sm text-muted">Lotes críticos</div>
            </div>
          </div>
          <div class="table-container">
            <table>
              <thead>
                <tr>
                  <th>Lote</th>
                  <th>Vencimiento</th>
                  <th>Cantidad</th>
                  <th>Estado</th>
                  <th>Acciones</th>
                </tr>
              </thead>
              <tbody>
                {#each lots as lot}
                  {@const badge = expiryBadge(lot.expiry_status)}
                  <tr>
                    <td style="font-weight: 600;">{lot.lot_number || 'Sin lote'}</td>
                    <td>{formatDate(lot.expiry_date)}</td>
                    <td style="font-weight: 700;">{lot.quantity}</td>
                    <td><span class="badge {badge.class}">{badge.label}</span></td>
                    <td>
                      {#if lot.quantity === 0}
                        <button class="btn btn-ghost btn-sm" onclick={() => handleDeleteLot(lot.id)} style="color: var(--accent-danger);">🗑️ Eliminar</button>
                      {/if}
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {/if}
      </div>
      <div class="modal-footer">
        <button class="btn btn-primary" onclick={() => showLots = false}>Cerrar</button>
      </div>
    </div>
  </div>
{/if}

<!-- Movement History Modal -->
{#if showMovements && movementsProduct}
  <div class="modal-overlay">
    <div class="modal modal-lg" onclick={(e) => e.stopPropagation()} style="max-width: 900px;">
      <div class="modal-header">
        <h3 class="modal-title">📜 Historial — {movementsProduct.product.name}</h3>
        <button class="btn btn-ghost btn-sm" onclick={() => showMovements = false}>✕</button>
      </div>
      <div class="modal-body">
        {#if movementsLoading}
          <div class="text-center text-muted" style="padding: var(--space-2xl);">Cargando historial...</div>
        {:else if movements.length === 0}
          <div class="text-center text-muted" style="padding: var(--space-2xl);">
            <div style="font-size: var(--font-size-2xl); margin-bottom: var(--space-md);">📭</div>
            <div>No hay movimientos registrados para este producto.</div>
          </div>
        {:else}
          {@const totalIn = movements.filter(m => m.quantity > 0).reduce((s, m) => s + m.quantity, 0)}
          {@const totalOut = movements.filter(m => m.quantity < 0).reduce((s, m) => s + Math.abs(m.quantity), 0)}
          <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: var(--space-lg); margin-bottom: var(--space-xl);">
            <div class="stat-card" style="text-align: center;">
              <div style="font-size: var(--font-size-2xl); font-weight: 700; color: var(--accent-success);">+{totalIn}</div>
              <div class="text-sm text-muted">Entradas</div>
            </div>
            <div class="stat-card" style="text-align: center;">
              <div style="font-size: var(--font-size-2xl); font-weight: 700; color: var(--accent-danger);">-{totalOut}</div>
              <div class="text-sm text-muted">Salidas</div>
            </div>
            <div class="stat-card" style="text-align: center;">
              <div style="font-size: var(--font-size-2xl); font-weight: 700; color: var(--accent-primary);">{movements.length}</div>
              <div class="text-sm text-muted">Total movimientos</div>
            </div>
          </div>
          <div class="table-container" style="max-height: 400px; overflow-y: auto;">
            <table>
              <thead>
                <tr>
                  <th>Fecha</th>
                  <th>Tipo</th>
                  <th>Cantidad</th>
                  <th>Lote</th>
                  <th>Notas</th>
                </tr>
              </thead>
              <tbody>
                {#each movements as mov}
                  {@const badge = movementBadge(mov.movement_type)}
                  <tr>
                    <td class="text-sm">{formatDateTime(mov.created_at)}</td>
                    <td><span class="badge {badge.class}">{badge.label}</span></td>
                    <td style="font-weight: 700; color: {mov.quantity >= 0 ? 'var(--accent-success)' : 'var(--accent-danger)'};">
                      {mov.quantity >= 0 ? '+' : ''}{mov.quantity}
                    </td>
                    <td class="text-sm text-muted">{mov.lot_number || '—'}</td>
                    <td class="text-sm text-muted">{mov.notes || '—'}</td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {/if}
      </div>
      <div class="modal-footer">
        <button class="btn btn-primary" onclick={() => showMovements = false}>Cerrar</button>
      </div>
    </div>
  </div>
{/if}

<!-- Import Result Modal -->
{#if showImportResult && importResult}
  <div class="modal-overlay">
    <div class="modal" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h3 class="modal-title">📥 Resultado de Importación</h3>
        <button class="btn btn-ghost btn-sm" onclick={() => showImportResult = false}>✕</button>
      </div>
      <div class="modal-body">
        <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: var(--space-lg); margin-bottom: var(--space-xl);">
          <div class="stat-card" style="text-align: center;">
            <div style="font-size: var(--font-size-2xl); font-weight: 700; color: var(--accent-success);">{importResult.created}</div>
            <div class="text-sm text-muted">✅ Creados</div>
          </div>
          <div class="stat-card" style="text-align: center;">
            <div style="font-size: var(--font-size-2xl); font-weight: 700; color: var(--accent-primary);">{importResult.updated}</div>
            <div class="text-sm text-muted">🔄 Actualizados</div>
          </div>
          <div class="stat-card" style="text-align: center;">
            <div style="font-size: var(--font-size-2xl); font-weight: 700; color: var(--accent-danger);">{importResult.errors.length}</div>
            <div class="text-sm text-muted">❌ Errores</div>
          </div>
        </div>

        {#if importResult.errors.length > 0}
          <div style="max-height: 200px; overflow-y: auto;">
            <table>
              <thead>
                <tr>
                  <th>Fila</th>
                  <th>Error</th>
                </tr>
              </thead>
              <tbody>
                {#each importResult.errors as err}
                  <tr>
                    <td style="font-weight: 600;">{err.row}</td>
                    <td class="text-muted">{err.message}</td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {/if}
      </div>
      <div class="modal-footer">
        <button class="btn btn-primary" onclick={() => showImportResult = false}>Cerrar</button>
      </div>
    </div>
  </div>
{/if}
