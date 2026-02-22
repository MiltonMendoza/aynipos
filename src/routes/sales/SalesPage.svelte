<script lang="ts">
  import { onMount } from 'svelte';
  import type { Sale, SaleItem, User } from '$lib/types';
  import { getSales, getSaleItems, cancelSale, getSettings, logAction } from '$lib/services/api';
  import { printReceipt, extractBusinessInfo, type BusinessInfo } from '$lib/services/receipt';
  import { hasPermission } from '$lib/services/permissions';

  let { currentUser }: { currentUser: User | null } = $props();

  let sales: Sale[] = $state([]);
  let selectedSale: Sale | null = $state(null);
  let saleItems: SaleItem[] = $state([]);
  let loading = $state(true);
  let businessInfo: BusinessInfo = $state({ name: 'Mi Negocio', nit: '', address: '', phone: '', city: '' });

  // ─── Filtros ───────────────────────────────────────────
  let dateFrom = $state('');
  let dateTo = $state('');
  let statusFilter = $state('');
  let activePreset = $state('hoy');

  // ─── Resumen ───────────────────────────────────────────
  let summaryTotal = $derived(
    sales.filter(s => s.status === 'completed').reduce((sum, s) => sum + s.total, 0)
  );
  let summaryCount = $derived(sales.length);
  let summaryCompleted = $derived(sales.filter(s => s.status === 'completed').length);

  onMount(async () => {
    try {
      const allSettings = await getSettings();
      businessInfo = extractBusinessInfo(allSettings);
    } catch { /* ignore */ }
    applyPreset('hoy');
  });

  function todayStr(): string {
    const d = new Date();
    return d.getFullYear() + '-' + String(d.getMonth() + 1).padStart(2, '0') + '-' + String(d.getDate()).padStart(2, '0');
  }

  function applyPreset(preset: string) {
    activePreset = preset;
    const today = new Date();
    const yyyy = today.getFullYear();
    const mm = today.getMonth();
    const dd = today.getDate();

    switch (preset) {
      case 'hoy':
        dateFrom = todayStr();
        dateTo = todayStr();
        break;
      case 'semana': {
        const dayOfWeek = today.getDay(); // 0=Sun
        const monday = new Date(today);
        monday.setDate(dd - (dayOfWeek === 0 ? 6 : dayOfWeek - 1));
        dateFrom = monday.getFullYear() + '-' + String(monday.getMonth() + 1).padStart(2, '0') + '-' + String(monday.getDate()).padStart(2, '0');
        dateTo = todayStr();
        break;
      }
      case 'mes':
        dateFrom = yyyy + '-' + String(mm + 1).padStart(2, '0') + '-01';
        dateTo = todayStr();
        break;
      case '30dias': {
        const past = new Date(today);
        past.setDate(dd - 30);
        dateFrom = past.getFullYear() + '-' + String(past.getMonth() + 1).padStart(2, '0') + '-' + String(past.getDate()).padStart(2, '0');
        dateTo = todayStr();
        break;
      }
      case 'todo':
        dateFrom = '';
        dateTo = '';
        break;
    }
    loadSales();
  }

  async function loadSales() {
    loading = true;
    selectedSale = null;
    try {
      const from = dateFrom ? dateFrom + 'T00:00:00' : undefined;
      const to = dateTo ? dateTo + 'T23:59:59' : undefined;
      const st = statusFilter || undefined;
      sales = await getSales(from, to, st);
    } catch { sales = []; }
    loading = false;
  }

  function onDateChange() {
    activePreset = '';
    loadSales();
  }

  function onStatusChange() {
    loadSales();
  }

  async function viewSale(sale: Sale) {
    selectedSale = sale;
    saleItems = await getSaleItems(sale.id);
  }

  async function handleCancel(saleId: string) {
    if (!confirm('¿Estás seguro de anular esta venta?')) return;
    const sale = sales.find(s => s.id === saleId);
    try {
      await cancelSale(saleId);
      if (currentUser && sale) {
        logAction(currentUser.id, currentUser.name, 'sale_cancelled', 'sale', saleId, `Venta #${sale.sale_number} anulada`);
      }
      await loadSales();
      selectedSale = null;
    } catch (e) {
      alert('Error al anular: ' + e);
    }
  }

  function formatCurrency(n: number) { return `Bs ${n.toFixed(2)}`; }
  function formatDate(d: string | null) {
    if (!d) return '-';
    const date = new Date(d);
    return date.toLocaleDateString('es-BO', { day: '2-digit', month: '2-digit', year: 'numeric', hour: '2-digit', minute: '2-digit' });
  }

  function statusBadge(status: string) {
    switch (status) {
      case 'completed': return { class: 'badge-success', label: 'Completada' };
      case 'cancelled': return { class: 'badge-danger', label: 'Anulada' };
      default: return { class: 'badge-warning', label: status };
    }
  }
</script>

<div class="page">
  <div class="page-header">
    <div>
      <h1 class="page-title">📋 Historial de Ventas</h1>
      <p class="page-subtitle">Consulta y gestiona las ventas realizadas</p>
    </div>
    <button class="btn btn-ghost" onclick={loadSales}>🔄 Actualizar</button>
  </div>

  <!-- Barra de filtros -->
  <div class="filters-bar">
    <div class="filters-row">
      <div class="filter-group">
        <label class="filter-label">Desde</label>
        <input type="date" class="input" bind:value={dateFrom} onchange={onDateChange} />
      </div>
      <div class="filter-group">
        <label class="filter-label">Hasta</label>
        <input type="date" class="input" bind:value={dateTo} onchange={onDateChange} />
      </div>
      <div class="filter-group">
        <label class="filter-label">Estado</label>
        <select class="select" bind:value={statusFilter} onchange={onStatusChange}>
          <option value="">Todos</option>
          <option value="completed">Completadas</option>
          <option value="cancelled">Anuladas</option>
        </select>
      </div>
      <div class="presets-group">
        <button class="btn btn-sm {activePreset === 'hoy' ? 'btn-primary' : 'btn-ghost'}" onclick={() => applyPreset('hoy')}>Hoy</button>
        <button class="btn btn-sm {activePreset === 'semana' ? 'btn-primary' : 'btn-ghost'}" onclick={() => applyPreset('semana')}>Esta semana</button>
        <button class="btn btn-sm {activePreset === 'mes' ? 'btn-primary' : 'btn-ghost'}" onclick={() => applyPreset('mes')}>Este mes</button>
        <button class="btn btn-sm {activePreset === '30dias' ? 'btn-primary' : 'btn-ghost'}" onclick={() => applyPreset('30dias')}>Últimos 30 días</button>
        <button class="btn btn-sm {activePreset === 'todo' ? 'btn-primary' : 'btn-ghost'}" onclick={() => applyPreset('todo')}>Todo</button>
      </div>
    </div>

    <!-- Resumen de ventas filtradas -->
    <div class="summary-row">
      <div class="summary-item">
        <span class="summary-icon green">💰</span>
        <div>
          <div class="summary-value">{formatCurrency(summaryTotal)}</div>
          <div class="summary-label">Total vendido</div>
        </div>
      </div>
      <div class="summary-item">
        <span class="summary-icon blue">🧾</span>
        <div>
          <div class="summary-value">{summaryCount}</div>
          <div class="summary-label">Transacciones</div>
        </div>
      </div>
      <div class="summary-item">
        <span class="summary-icon purple">✅</span>
        <div>
          <div class="summary-value">{summaryCompleted}</div>
          <div class="summary-label">Completadas</div>
        </div>
      </div>
    </div>
  </div>

  <div style="display: flex; gap: var(--space-xl); height: calc(100vh - 320px);">
    <!-- Sales list -->
    <div style="flex: 1; overflow-y: auto;" class="table-container">
      {#if loading}
        <div class="text-center text-muted" style="padding: var(--space-3xl);">Cargando ventas...</div>
      {:else}
      <table>
        <thead>
          <tr>
            <th>#</th>
            <th>Fecha</th>
            <th>Cliente</th>
            <th>Total</th>
            <th>Pago</th>
            <th>Estado</th>
          </tr>
        </thead>
        <tbody>
          {#if sales.length === 0}
            <tr><td colspan="6" class="text-center text-muted" style="padding: var(--space-3xl);">No hay ventas en el período seleccionado</td></tr>
          {:else}
            {#each sales as sale}
              <tr
                onclick={() => viewSale(sale)}
                style="cursor: pointer; {selectedSale?.id === sale.id ? 'background: var(--accent-primary-glow);' : ''}"
              >
                <td style="font-weight: 700;">#{sale.sale_number}</td>
                <td>{formatDate(sale.created_at)}</td>
                <td class="text-sm">
                  <div class="truncate" style="max-width: 120px;">{sale.customer_name || 'Sin Nombre'}</div>
                </td>
                <td style="font-weight: 700; color: var(--accent-success);">{formatCurrency(sale.total)}</td>
                <td>
                  {#if sale.payment_method === 'efectivo'}💵
                  {:else if sale.payment_method === 'tarjeta'}💳
                  {:else if sale.payment_method === 'qr'}📱
                  {:else}💰{/if}
                  {sale.payment_method}
                </td>
                <td><span class="badge {statusBadge(sale.status).class}">{statusBadge(sale.status).label}</span></td>
              </tr>
            {/each}
          {/if}
        </tbody>
      </table>
      {/if}
    </div>

    <!-- Sale detail -->
    {#if selectedSale}
      <div class="card" style="width: 360px; min-width: 360px; overflow-y: auto;">
        <div class="flex items-center justify-between" style="margin-bottom: var(--space-xl);">
          <h3 style="font-weight: 700;">Venta #{selectedSale.sale_number}</h3>
          <span class="badge {statusBadge(selectedSale.status).class}">{statusBadge(selectedSale.status).label}</span>
        </div>

        <!-- Customer info -->
        <div class="flex items-center gap-sm" style="margin-bottom: var(--space-lg); padding: var(--space-sm) var(--space-md); background: var(--bg-tertiary); border-radius: var(--radius-md);">
          <span style="font-size: var(--font-size-sm);">👤</span>
          <div>
            <div style="font-weight: 600; font-size: var(--font-size-sm);">{selectedSale.customer_name || 'Sin Nombre'}</div>
          </div>
        </div>

        {#if selectedSale.notes}
          <div style="
            display: flex;
            align-items: flex-start;
            gap: var(--space-sm);
            margin-bottom: var(--space-lg);
            padding: var(--space-sm) var(--space-md);
            background: var(--bg-tertiary);
            border-radius: var(--radius-md);
          ">
            <span style="font-size: var(--font-size-sm);">📝</span>
            <div style="font-size: var(--font-size-sm); color: var(--text-muted); word-break: break-word;">{selectedSale.notes}</div>
          </div>
        {/if}

        <div style="display: flex; flex-direction: column; gap: var(--space-md); margin-bottom: var(--space-xl);">
          {#each saleItems as item}
            <div class="flex justify-between text-sm">
              <div>
                <div style="font-weight: 600;">{item.product_name}</div>
                <div class="text-xs text-muted">
                  {item.quantity} × {formatCurrency(item.unit_price)}
                  {#if item.discount > 0}
                    <span style="color: var(--accent-warning); margin-left: var(--space-sm);">· Desc. −{formatCurrency(item.discount)}</span>
                  {/if}
                </div>
              </div>
              <div style="font-weight: 600;">{formatCurrency(item.total)}</div>
            </div>
          {/each}
        </div>

        <div style="border-top: 1px solid var(--border-color); padding-top: var(--space-lg);">
          <div class="flex justify-between text-sm" style="margin-bottom: var(--space-xs);">
            <span class="text-muted">Subtotal</span>
            <span>{formatCurrency(selectedSale.subtotal)}</span>
          </div>
          {#if selectedSale.discount_amount > 0}
            <div class="flex justify-between text-sm" style="margin-bottom: var(--space-xs);">
              <span class="text-muted">Descuento</span>
              <span style="color: var(--accent-warning);">−{formatCurrency(selectedSale.discount_amount)}</span>
            </div>
          {/if}
          <div class="flex justify-between text-sm" style="margin-bottom: var(--space-xs);">
            <span class="text-muted">IVA</span>
            <span>{formatCurrency(selectedSale.tax_amount)}</span>
          </div>
          <div class="flex justify-between" style="font-size: var(--font-size-lg); font-weight: 800; margin-top: var(--space-md);">
            <span>Total</span>
            <span style="color: var(--accent-success);">{formatCurrency(selectedSale.total)}</span>
          </div>
        </div>

        {#if selectedSale.status === 'completed'}
          <div style="display: flex; gap: var(--space-md); margin-top: var(--space-xl);">
            <button
              class="btn btn-primary"
              style="flex: 1;"
              onclick={() => { if (selectedSale) printReceipt(selectedSale, saleItems, businessInfo); }}
            >
              🖨️ Imprimir Recibo
            </button>
            {#if hasPermission(currentUser, 'cancel_sales')}
              <button class="btn btn-danger" style="flex: 1;" onclick={() => handleCancel(selectedSale!.id)}>
                🚫 Anular Venta
              </button>
            {/if}
          </div>
        {:else}
          <div style="margin-top: var(--space-xl);">
            <button
              class="btn btn-ghost btn-block"
              onclick={() => { if (selectedSale) printReceipt(selectedSale, saleItems, businessInfo); }}
            >
              🖨️ Imprimir Recibo
            </button>
          </div>
        {/if}
      </div>
    {/if}
  </div>
</div>

<style>
  .filters-bar {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    padding: var(--space-lg);
    margin-bottom: var(--space-lg);
    display: flex;
    flex-direction: column;
    gap: var(--space-lg);
  }

  .filters-row {
    display: flex;
    align-items: flex-end;
    gap: var(--space-lg);
    flex-wrap: wrap;
  }

  .filter-group {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
  }

  .filter-label {
    font-size: var(--font-size-xs);
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .filter-group .input,
  .filter-group .select {
    width: 160px;
    height: 36px;
    font-size: var(--font-size-sm);
  }

  .presets-group {
    display: flex;
    gap: var(--space-xs);
    margin-left: auto;
  }

  .btn-sm {
    padding: var(--space-xs) var(--space-md);
    font-size: var(--font-size-xs);
    height: 36px;
  }

  .summary-row {
    display: flex;
    gap: var(--space-xl);
    padding-top: var(--space-md);
    border-top: 1px solid var(--border-color);
  }

  .summary-item {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
  }

  .summary-icon {
    width: 36px;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
  }

  .summary-icon.green { background: rgba(16, 185, 129, 0.15); }
  .summary-icon.blue { background: rgba(59, 130, 246, 0.15); }
  .summary-icon.purple { background: rgba(139, 92, 246, 0.15); }

  .summary-value {
    font-size: var(--font-size-base);
    font-weight: 700;
    color: var(--text-primary);
    line-height: 1.2;
  }

  .summary-label {
    font-size: var(--font-size-xs);
    color: var(--text-muted);
    line-height: 1.2;
  }
</style>
