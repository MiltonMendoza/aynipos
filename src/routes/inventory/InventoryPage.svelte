<script lang="ts">
  import { onMount } from 'svelte';
  import type { ProductWithStock, Category, CreateProduct, UpdateProduct, ImportResult, InventoryLot, InventoryMovement, User, Supplier } from '$lib/types';
  import { getInventory, adjustInventory, getCategories, createProduct, createCategory, updateProduct, exportProductsCsv, importProductsCsv, getProductLots, deleteLot, getInventoryMovements, logAction, getSuppliers, deleteProduct } from '$lib/services/api';
  import { DataTableState } from '$lib/utils/datatable.svelte';
  import TablePagination from '$lib/components/TablePagination.svelte';

  let { currentUser }: { currentUser: User | null } = $props();
  import { save, open } from '@tauri-apps/plugin-dialog';

  let inventory: ProductWithStock[] = $state([]);
  let categories: Category[] = $state([]);
  let suppliers: Supplier[] = $state([]);
  let filter = $state<'all' | 'low' | 'expiring'>('all');

  let table = new DataTableState<ProductWithStock>([], [
    'product.sku',
    'product.name',
    'category_name',
    'supplier_name'
  ]);

  $effect(() => {
    table.currentPage;
    openDropdownId = null;
  });

  let showAddProduct = $state(false);
  let showAddCategory = $state(false);
  let showAdjust = $state(false);
  let adjustProduct: ProductWithStock | null = $state(null);
  let adjustTargetStock = $state<number | null>(null);
  let adjustType = $state('adjustment');
  let adjustNotes = $state('');
  let adjustLotNumber = $state('');
  let adjustExpiryDate = $state('');

  // Delta reactivo: stock deseado - stock actual
  let adjustDelta = $derived(
    adjustTargetStock !== null && adjustProduct !== null
      ? adjustTargetStock - (adjustProduct as ProductWithStock).current_stock
      : 0
  );

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

  // Dropdown de acciones por fila
  let openDropdownId = $state<string | null>(null);

  // Archivar producto
  let archiveToast = $state<{ name: string } | null>(null);
  let archiveConfirm = $state<ProductWithStock | null>(null);
  let archiving = $state(false);

  async function handleArchiveProduct(ps: ProductWithStock) {
    if (ps.current_stock > 0) {
      archiveConfirm = ps;
      return;
    }
    await doArchive(ps);
  }

  async function doArchive(ps: ProductWithStock) {
    archiving = true;
    try {
      await deleteProduct(ps.product.id);
      archiveConfirm = null;
      archiveToast = { name: ps.product.name };
      setTimeout(() => { archiveToast = null; }, 4000);
      await loadInventory();
    } catch (e) {
      console.error('Error archivando producto:', e);
    }
    archiving = false;
  }

  // Posición y producto activo del dropdown (position:fixed, escapa del overflow de la tabla)
  let dropdownPos = $state({ top: 0, left: 0 });
  let dropdownActiveProduct = $state<ProductWithStock | null>(null);

  function toggleDropdown(e: MouseEvent, ps: ProductWithStock) {
    e.stopPropagation();
    if (openDropdownId === ps.product.id) {
      openDropdownId = null;
      dropdownActiveProduct = null;
      return;
    }
    // Captura la posición del botón ⋮ para anclar el menú con position:fixed
    const btn = e.currentTarget as HTMLElement;
    const rect = btn.getBoundingClientRect();
    dropdownPos = { top: rect.bottom + 4, left: rect.left };
    dropdownActiveProduct = ps; // guardamos referencia estable al producto
    openDropdownId = ps.product.id;
  }

  onMount(() => {
    loadInventory();
    // Cerrar dropdown al hacer click fuera
    const closeDropdown = () => { openDropdownId = null; };
    document.addEventListener('click', closeDropdown);
    return () => document.removeEventListener('click', closeDropdown);
  });

  async function loadInventory() {
    try {
      if (filter === 'low') {
        inventory = await getInventory(true);
      } else if (filter === 'expiring') {
        inventory = await getInventory(false, 30);
      } else {
        inventory = await getInventory();
      }
      table.data = inventory;
      table.currentPage = 1;
      categories = await getCategories();
      suppliers = await getSuppliers();
    } catch {
      inventory = [];
      table.data = [];
    }
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
    if (adjustTargetStock === null || adjustTargetStock === undefined) {
      e.qty = 'Ingresá el stock deseado';
    } else if (adjustTargetStock < 0) {
      e.qty = 'El stock no puede ser negativo';
    } else if (adjustDelta === 0) {
      e.qty = 'El stock deseado es igual al actual — sin cambios';
    }
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
    adjustTargetStock = ps.current_stock; // empieza con el stock actual para que el usuario lo edite
    adjustType = 'adjustment';
    adjustNotes = '';
    adjustLotNumber = '';
    adjustExpiryDate = '';
    adjustErrors = {};
    showAdjust = true;
  }

  async function handleAdjust() {
    if (!validateAdjust()) return;
    if (!adjustProduct) return;
    const qty = adjustDelta; // negativo si reduce, positivo si agrega
    const type = qty < 0 ? 'adjustment' : adjustType; // reducciones siempre son 'adjustment'
    const prevStock = adjustProduct.current_stock;
    try {
      await adjustInventory(
        adjustProduct.product.id, qty, type,
        adjustNotes || undefined,
        adjustLotNumber || undefined,
        adjustExpiryDate || undefined
      );
      if (currentUser) {
        const sign = qty >= 0 ? '+' : '';
        logAction(currentUser.id, currentUser.name, 'inventory_adjusted', 'product', adjustProduct.product.id,
          `Stock ajustado: ${prevStock} → ${adjustTargetStock} (${sign}${qty} u.) en "${adjustProduct.product.name}"`);
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
      supplier_id: ps.product.supplier_id ?? undefined,
      dose: ps.product.dose ?? undefined,
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
  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      if (showAddCategory) showAddCategory = false;
      else if (showInlineCategoryAdd) showInlineCategoryAdd = false;
      else if (showInlineCategoryEdit) showInlineCategoryEdit = false;
      else if (showAddProduct) showAddProduct = false;
      else if (showEditProduct) showEditProduct = false;
      else if (showAdjust) showAdjust = false;
      else if (showLots) showLots = false;
      else if (showMovements) showMovements = false;
      else if (showImportResult) showImportResult = false;
    }
  }
</script>

<svelte:window
  onkeydown={handleKeydown}
  onclick={() => { openDropdownId = null; }}
/>

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

  <!-- Filters and Search Bar -->
  <div style="display: flex; justify-content: space-between; align-items: center; gap: var(--space-md); margin-bottom: var(--space-md); flex-wrap: wrap;">
    <div class="flex gap-sm">
      {#each [
        { key: 'all' as const, label: 'Todos', icon: '📋' },
        { key: 'low' as const, label: 'Bajo Stock', icon: '⚠️' },
        { key: 'expiring' as const, label: 'Por Vencer', icon: '⏰' },
      ] as f}
        <button
          class="btn btn-sm"
          class:btn-primary={filter === f.key}
          class:btn-ghost={filter !== f.key}
          onclick={() => filter = f.key}
        >
          {f.icon} {f.label}
        </button>
      {/each}
    </div>

    <!-- Search box -->
    <div style="position: relative; width: 320px;">
      <input
        class="input input-compact"
        style="padding-right: 30px !important;"
        placeholder="🔍 Buscar por nombre, SKU, categoría o proveedor..."
        bind:value={table.search}
        oninput={() => { table.currentPage = 1; openDropdownId = null; }}
      />
      {#if table.search}
        <button
          onclick={() => { table.search = ''; table.currentPage = 1; openDropdownId = null; }}
          style="position: absolute; right: 8px; top: 50%; transform: translateY(-50%); background: none; border: none; color: var(--text-muted); cursor: pointer; font-size: 14px; padding: 4px;"
        >
          ✕
        </button>
      {/if}
    </div>
  </div>

  <!-- Inventory table -->
  <div class="table-container">
    <!-- Leyenda de vencimiento -->
    <div class="expiry-legend" style="margin-bottom: var(--space-md);">
      <span style="font-weight: 600;">Vencimiento:</span>
      <div class="expiry-legend-item">
        <span class="expiry-legend-dot expired"></span>
        <span>Expirado</span>
      </div>
      <div class="expiry-legend-item">
        <span class="expiry-legend-dot expiring"></span>
        <span>Por vencer (≤ 4 meses)</span>
      </div>
      <div class="expiry-legend-item">
        <span class="expiry-legend-dot active"></span>
        <span>Vigente</span>
      </div>
    </div>
    <table>
      <thead>
        <tr>
          <th style="width: 48px;"></th>
          <th onclick={() => table.sortBy('product.sku')} style="cursor: pointer; user-select: none;">
            SKU {table.sortColumn === 'product.sku' ? (table.sortDirection === 'asc' ? '↑' : '↓') : ''}
          </th>
          <th onclick={() => table.sortBy('product.name')} style="cursor: pointer; user-select: none;">
            Producto {table.sortColumn === 'product.name' ? (table.sortDirection === 'asc' ? '↑' : '↓') : ''}
          </th>
          <th onclick={() => table.sortBy('product.dose')} style="cursor: pointer; user-select: none;">
            Dosis {table.sortColumn === 'product.dose' ? (table.sortDirection === 'asc' ? '↑' : '↓') : ''}
          </th>
          <th onclick={() => table.sortBy('category_name')} style="cursor: pointer; user-select: none;">
            Categoría {table.sortColumn === 'category_name' ? (table.sortDirection === 'asc' ? '↑' : '↓') : ''}
          </th>
          <th onclick={() => table.sortBy('supplier_name')} style="cursor: pointer; user-select: none;">
            Proveedor {table.sortColumn === 'supplier_name' ? (table.sortDirection === 'asc' ? '↑' : '↓') : ''}
          </th>
          <th onclick={() => table.sortBy('product.purchase_price')} style="cursor: pointer; user-select: none;">
            P. Compra {table.sortColumn === 'product.purchase_price' ? (table.sortDirection === 'asc' ? '↑' : '↓') : ''}
          </th>
          <th onclick={() => table.sortBy('product.sale_price')} style="cursor: pointer; user-select: none;">
            P. Venta {table.sortColumn === 'product.sale_price' ? (table.sortDirection === 'asc' ? '↑' : '↓') : ''}
          </th>
          <th onclick={() => table.sortBy('current_stock')} style="cursor: pointer; user-select: none;">
            Stock {table.sortColumn === 'current_stock' ? (table.sortDirection === 'asc' ? '↑' : '↓') : ''}
          </th>
          <th onclick={() => table.sortBy('nearest_expiry_date')} style="cursor: pointer; user-select: none;">
            Vencimiento {table.sortColumn === 'nearest_expiry_date' ? (table.sortDirection === 'asc' ? '↑' : '↓') : ''}
          </th>
        </tr>
      </thead>
      <tbody>
        {#if table.paginated.length === 0}
          <tr><td colspan="10" class="text-center text-muted" style="padding: var(--space-3xl);">Sin productos</td></tr>
        {:else}
          {#each table.paginated as ps}
            <tr
              class:row-expired={ps.expiry_status === 'expired'}
              class:row-low-stock={ps.current_stock <= ps.product.min_stock && ps.product.min_stock > 0}
              class:row-expiring={ps.expiry_status === 'expiring' && !(ps.current_stock <= ps.product.min_stock && ps.product.min_stock > 0)}
            >
              <!-- Acciones: solo el trigger. El menú se renderiza como portal fijo al final del archivo -->
              <td>
                <div class="action-dropdown">
                  <button
                    class="btn btn-ghost btn-sm action-trigger"
                    style="padding: 4px 8px; font-size: var(--font-size-base);"
                    onclick={(e) => toggleDropdown(e, ps)}
                  >⋮</button>
                </div>
              </td>
              <td class="font-mono text-sm">{ps.product.sku}</td>
              <td style="font-weight: 600;">{ps.product.name}</td>
              <td class="text-muted">
                {#if ps.product.dose}
                  <span class="badge badge-info" style="font-size: var(--font-size-xs);">{ps.product.dose}</span>
                {:else}—{/if}
              </td>
              <td class="text-muted">{ps.category_name || '—'}</td>
              <td class="text-muted">{ps.supplier_name || '—'}</td>
              <td>{formatCurrency(ps.product.purchase_price)}</td>
              <td style="font-weight: 600; color: var(--accent-primary);">{formatCurrency(ps.product.sale_price)}</td>
              <!-- Stock horizontal: STOCK (Min: MIN) -->
              <td style="font-weight: 700;">
                {ps.current_stock}
                {#if ps.product.min_stock > 0}
                  <span style="font-weight: 400; font-size: var(--font-size-xs); color: var(--text-muted); margin-left: 2px;">
                    (Min: {ps.product.min_stock})
                  </span>
                {/if}
              </td>
              <!-- Vencimiento: solo fecha compacta -->
              <td style="font-weight: 500;">
                {#if ps.nearest_expiry_date}
                  {new Date(ps.nearest_expiry_date + 'T12:00:00').toLocaleDateString('es-BO', { day: '2-digit', month: 'short', year: 'numeric' })}
                {:else}
                  —
                {/if}
              </td>
            </tr>
          {/each}
        {/if}
      </tbody>
    </table>
  </div>
  <TablePagination {table} />
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
          <input class="input" class:input-error={productErrors.name} bind:value={newProduct.name} oninput={() => clearProductError('name')} placeholder="Ej: Zapatillas Deportivas XYZ" />
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
                    placeholder="Ej: Ropa, Electrónica"
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
        <!-- Dosis (farmacéutico) -->
        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-lg);">
          <div class="input-group">
            <label class="input-label">Dosis</label>
            <input class="input" bind:value={newProduct.dose} placeholder="ej: 500mg, 10ml, 250mg/5ml" />
          </div>
          <div class="input-group">
            <label class="input-label">Proveedor</label>
            <select class="select" bind:value={newProduct.supplier_id}>
              <option value={undefined}>Sin proveedor</option>
              {#each suppliers as s}
                <option value={s.id}>{s.name}</option>
              {/each}
            </select>
          </div>
        </div>
        <!-- Descripción -->
        <div class="input-group">
          <label class="input-label">Descripción</label>
          <textarea class="input" bind:value={newProduct.description} placeholder="Descripción del producto..." rows="2" style="resize: vertical;"></textarea>
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
          <input class="input" class:input-error={editErrors.name} bind:value={editProduct.name} oninput={() => clearEditError('name')} placeholder="Ej: Zapatillas Deportivas XYZ" />
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
                    placeholder="Ej: Ropa, Electrónica"
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
        <!-- Dosis (farmacéutico) -->
        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-lg);">
          <div class="input-group">
            <label class="input-label">Dosis</label>
            <input class="input" bind:value={editProduct.dose} placeholder="ej: 500mg, 10ml, 250mg/5ml" />
          </div>
          <div class="input-group">
            <label class="input-label">Proveedor</label>
            <select class="select" bind:value={editProduct.supplier_id}>
              <option value={undefined}>Sin proveedor</option>
              {#each suppliers as s}
                <option value={s.id}>{s.name}</option>
              {/each}
            </select>
          </div>
        </div>
        <div class="input-group">
          <label class="input-label">Descripción</label>
          <textarea class="input" bind:value={editProduct.description} placeholder="Descripción del producto..." rows="2" style="resize: vertical;"></textarea>
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
          <input class="input" class:input-error={categoryErrors.name} bind:value={newCategoryName} oninput={() => { if (categoryErrors.name) categoryErrors = {}; }} placeholder="Ej: Ropa, Electrónica" />
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

<!-- ─── Portal: dropdown de acciones (position:fixed, escapa del overflow de la tabla) ─── -->
{#if openDropdownId}
  {#if dropdownActiveProduct}
    <div
      class="action-menu"
      role="menu"
      style="position: fixed; top: {dropdownPos.top}px; left: {dropdownPos.left}px; z-index: 9000;"
      onclick={(e) => e.stopPropagation()}
    >
      <button class="action-item" onclick={() => { openDropdownId = null; const p = dropdownActiveProduct; dropdownActiveProduct = null; if(p) openEditProduct(p); }}>✏️ Editar</button>
      <button class="action-item" onclick={() => { openDropdownId = null; const p = dropdownActiveProduct; dropdownActiveProduct = null; if(p) openLots(p); }}>📦 Lotes</button>
      <button class="action-item" onclick={() => { openDropdownId = null; const p = dropdownActiveProduct; dropdownActiveProduct = null; if(p) openAdjust(p); }}>📊 Ajustar stock</button>
      <button class="action-item" onclick={() => { openDropdownId = null; const p = dropdownActiveProduct; dropdownActiveProduct = null; if(p) openMovements(p); }}>📜 Historial</button>
      <div style="height: 1px; background: var(--border-primary); margin: 4px 0;"></div>
      <button class="action-item" style="color: var(--accent-danger);" onclick={() => { openDropdownId = null; const p = dropdownActiveProduct; dropdownActiveProduct = null; if(p) handleArchiveProduct(p); }}>🗑️ Eliminar</button>
    </div>
  {/if}
{/if}

{#if showAdjust && adjustProduct}
  <div class="modal-overlay">
    <div class="modal" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h3 class="modal-title">📊 Ajustar Stock</h3>
        <button class="btn btn-ghost btn-sm" onclick={() => showAdjust = false}>✕</button>
      </div>
      <div class="modal-body">

        <!-- Info del producto -->
        <div style="background: var(--bg-tertiary); border-radius: var(--radius-md); padding: var(--space-md) var(--space-lg); margin-bottom: var(--space-lg);">
          <div style="font-weight: 700;">{adjustProduct.product.name}</div>
          <div class="text-sm text-muted">SKU: {adjustProduct.product.sku}</div>
        </div>

        <!-- Visualización: actual → deseado -->
        <div style="display: flex; align-items: center; justify-content: center; gap: var(--space-xl); margin-bottom: var(--space-lg); padding: var(--space-lg); background: var(--bg-tertiary); border-radius: var(--radius-md);">
          <div style="text-align: center;">
            <div class="text-muted" style="font-size: var(--font-size-xs); text-transform: uppercase; letter-spacing: 0.05em; margin-bottom: 4px;">Stock actual</div>
            <div style="font-size: 2.2rem; font-weight: 800;">{adjustProduct.current_stock}</div>
          </div>
          <div style="font-size: 1.8rem; color: var(--text-muted);">→</div>
          <div style="text-align: center;">
            <div class="text-muted" style="font-size: var(--font-size-xs); text-transform: uppercase; letter-spacing: 0.05em; margin-bottom: 4px;">Stock deseado</div>
            <div style="font-size: 2.2rem; font-weight: 800; color: {adjustDelta < 0 ? 'var(--accent-danger)' : adjustDelta > 0 ? 'var(--accent-success)' : 'var(--text-muted)'};">
              {adjustTargetStock ?? adjustProduct.current_stock}
            </div>
          </div>
        </div>

        <!-- Input principal -->
        <div class="input-group">
          <label class="input-label">¿Cuánto debería quedar en stock? *</label>
          <input
            class="input"
            class:input-error={adjustErrors.qty}
            type="number"
            min="0"
            step="1"
            bind:value={adjustTargetStock}
            oninput={() => { if (adjustErrors.qty) adjustErrors = {}; }}
            placeholder="Ej: 15"
            style="font-size: 1.2rem; font-weight: 700; text-align: center;"
          />
          {#if adjustErrors.qty}<span class="field-error">{adjustErrors.qty}</span>{/if}
        </div>

        <!-- Label informativo del delta -->
        {#if adjustTargetStock !== null && adjustDelta !== 0}
          <div style="
            display: flex; align-items: center; gap: var(--space-sm);
            padding: var(--space-sm) var(--space-md);
            border-radius: var(--radius-md);
            margin-top: calc(var(--space-sm) * -1);
            margin-bottom: var(--space-md);
            background: {adjustDelta < 0
              ? 'color-mix(in srgb, var(--accent-danger) 10%, transparent)'
              : 'color-mix(in srgb, var(--accent-success) 10%, transparent)'};
            border: 1px solid {adjustDelta < 0
              ? 'color-mix(in srgb, var(--accent-danger) 30%, transparent)'
              : 'color-mix(in srgb, var(--accent-success) 30%, transparent)'};
          ">
            <span style="font-size: 1.1rem;">{adjustDelta < 0 ? '📉' : '📈'}</span>
            <span style="font-size: var(--font-size-sm); font-weight: 600; color: {adjustDelta < 0 ? 'var(--accent-danger)' : 'var(--accent-success)'};">
              {adjustDelta < 0
                ? `Se reducirán ${Math.abs(adjustDelta)} unidades del stock actual`
                : `Se agregarán ${adjustDelta} unidades al stock actual`}
            </span>
          </div>
        {/if}

        <!-- Tipo de entrada — solo visible si se está agregando stock -->
        {#if adjustDelta > 0}
          <div class="input-group">
            <label class="input-label">Tipo de entrada</label>
            <select class="select" bind:value={adjustType}>
              <option value="purchase">🛒 Compra</option>
              <option value="return">↩️ Devolución de cliente</option>
              <option value="adjustment">🔧 Ajuste manual</option>
            </select>
          </div>
        {/if}

        <!-- Lote y vencimiento -->
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
          <label class="input-label">Motivo del ajuste</label>
          <input class="input" bind:value={adjustNotes} placeholder="Ej: merma, corrección de conteo, vencimiento..." />
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-ghost" onclick={() => showAdjust = false}>Cancelar</button>
        <button
          class="btn btn-primary"
          disabled={adjustDelta === 0 || adjustTargetStock === null}
          onclick={handleAdjust}
        >✅ Aplicar ajuste</button>
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
        <div style="display: grid; grid-template-columns: 1fr 1fr 1fr {importResult.lots_created > 0 ? '1fr' : ''}; gap: var(--space-lg); margin-bottom: var(--space-xl);">
          <div class="stat-card" style="text-align: center;">
            <div style="font-size: var(--font-size-2xl); font-weight: 700; color: var(--accent-success);">{importResult.created}</div>
            <div class="text-sm text-muted">✅ Creados</div>
          </div>
          <div class="stat-card" style="text-align: center;">
            <div style="font-size: var(--font-size-2xl); font-weight: 700; color: var(--accent-primary);">{importResult.updated}</div>
            <div class="text-sm text-muted">🔄 Actualizados</div>
          </div>
          {#if importResult.lots_created > 0}
          <div class="stat-card" style="text-align: center;">
            <div style="font-size: var(--font-size-2xl); font-weight: 700; color: var(--accent-warning, #f59e0b);">{importResult.lots_created}</div>
            <div class="text-sm text-muted">📅 Lotes de vencimiento creados</div>
          </div>
          {/if}
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

<!-- ─── Modal confirmación: archivar producto con stock ─── -->
{#if archiveConfirm}
  <div class="modal-overlay" onclick={() => archiveConfirm = null}>
    <div class="modal" onclick={(e) => e.stopPropagation()} style="max-width: 420px;">
      <div class="modal-header">
        <h3 class="modal-title">🗑️ Eliminar Producto</h3>
        <button class="btn btn-ghost btn-sm" onclick={() => archiveConfirm = null}>✕</button>
      </div>
      <div class="modal-body">
        <div style="display: flex; flex-direction: column; gap: var(--space-md);">
          <div style="
            background: color-mix(in srgb, var(--accent-warning) 12%, transparent);
            border: 1px solid color-mix(in srgb, var(--accent-warning) 40%, transparent);
            border-radius: var(--radius-md);
            padding: var(--space-md) var(--space-lg);
            display: flex; gap: var(--space-md); align-items: flex-start;
          ">
            <span style="font-size: 1.5rem; line-height: 1;">⚠️</span>
            <div>
              <div style="font-weight: 700; margin-bottom: 4px;">Este producto tiene stock activo</div>
              <div class="text-sm text-muted">
                <strong>{archiveConfirm.product.name}</strong> tiene
                <strong style="color: var(--accent-warning);">{archiveConfirm.current_stock} unidades</strong> en inventario.
                Al archivar, el producto desaparecerá del sistema pero el historial de ventas quedará intacto.
              </div>
            </div>
          </div>
          <p class="text-sm text-muted">
            Si querés registrar la salida del stock antes de archivar, cerrá este diálogo y usá <strong>Ajustar stock</strong>.
          </p>
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-ghost" onclick={() => archiveConfirm = null}>Cancelar</button>
        <button
          class="btn btn-danger"
          disabled={archiving}
          onclick={() => archiveConfirm && doArchive(archiveConfirm)}
        >
          {archiving ? 'Eliminando...' : '🗑️ Eliminar de todas formas'}
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- ─── Toast: producto archivado ─── -->
{#if archiveToast}
  <div style="
    position: fixed;
    bottom: var(--space-xl);
    left: 50%;
    transform: translateX(-50%);
    z-index: 9999;
    background: var(--bg-card);
    border: 1px solid var(--border-primary);
    border-left: 4px solid var(--accent-success);
    border-radius: var(--radius-md);
    padding: var(--space-md) var(--space-xl);
    display: flex;
    align-items: center;
    gap: var(--space-md);
    box-shadow: 0 8px 32px rgba(0,0,0,0.3);
    animation: slideUp 0.25s ease;
    min-width: 280px;
  ">
    <span style="font-size: 1.2rem;">✅</span>
    <div>
      <div style="font-weight: 600; font-size: var(--font-size-sm);">Producto eliminado</div>
      <div class="text-muted" style="font-size: var(--font-size-xs);">"{archiveToast.name}" ya no aparecerá en el sistema</div>
    </div>
    <button
      class="btn btn-ghost btn-sm"
      onclick={() => archiveToast = null}
      style="margin-left: auto; padding: 2px 6px;"
    >✕</button>
  </div>
{/if}

<style>
  @keyframes slideUp {
    from { opacity: 0; transform: translateX(-50%) translateY(16px); }
    to   { opacity: 1; transform: translateX(-50%) translateY(0); }
  }
</style>
