<script lang="ts">
  import { onMount } from 'svelte';
  import { getDashboardStats, getSales, getTopSellingProducts } from '$lib/services/api';
  import type { DashboardStats, Sale, TopSellingProduct } from '$lib/types';

  let stats: DashboardStats = $state({
    total_sales_today: 0, total_transactions_today: 0, total_products: 0,
    low_stock_count: 0, expiring_soon_count: 0
  });
  let recentSales: Sale[] = $state([]);

  // ─── Top Selling Products ──────────────────────────────
  type PeriodPreset = 'today' | 'week' | 'month' | 'last_month' | 'all' | 'custom';

  let topProducts: TopSellingProduct[] = $state([]);
  let topLoading = $state(false);
  let selectedPeriod: PeriodPreset = $state('month');
  let customFrom = $state('');
  let customTo = $state('');
  let topLimit: number = $state(10);
  let sortBy: 'quantity' | 'revenue' = $state('quantity');

  function getDateRange(preset: PeriodPreset): { from?: string; to?: string } {
    const now = new Date();
    const fmt = (d: Date) => d.toISOString().split('T')[0];
    switch (preset) {
      case 'today':
        return { from: fmt(now), to: fmt(now) };
      case 'week': {
        const start = new Date(now);
        start.setDate(now.getDate() - now.getDay());
        return { from: fmt(start), to: fmt(now) };
      }
      case 'month': {
        const start = new Date(now.getFullYear(), now.getMonth(), 1);
        return { from: fmt(start), to: fmt(now) };
      }
      case 'last_month': {
        const start = new Date(now.getFullYear(), now.getMonth() - 1, 1);
        const end = new Date(now.getFullYear(), now.getMonth(), 0);
        return { from: fmt(start), to: fmt(end) };
      }
      case 'custom':
        return { from: customFrom || undefined, to: customTo || undefined };
      default:
        return {};
    }
  }

  async function loadTopProducts() {
    topLoading = true;
    try {
      const { from, to } = getDateRange(selectedPeriod);
      topProducts = await getTopSellingProducts(from, to, topLimit);
    } catch { topProducts = []; }
    topLoading = false;
  }

  $effect(() => {
    // Re-fetch when period, limit, or custom dates change
    selectedPeriod; topLimit; customFrom; customTo;
    loadTopProducts();
  });

  function sortedProducts(): TopSellingProduct[] {
    return [...topProducts].sort((a, b) =>
      sortBy === 'quantity'
        ? b.total_quantity - a.total_quantity
        : b.total_revenue - a.total_revenue
    );
  }

  function maxValue(): number {
    const sorted = sortedProducts();
    if (sorted.length === 0) return 1;
    return sortBy === 'quantity'
      ? Math.max(...sorted.map(p => p.total_quantity))
      : Math.max(...sorted.map(p => p.total_revenue));
  }

  function barPercent(p: TopSellingProduct): number {
    const max = maxValue();
    if (max === 0) return 0;
    const val = sortBy === 'quantity' ? p.total_quantity : p.total_revenue;
    return (val / max) * 100;
  }

  onMount(async () => {
    try {
      stats = await getDashboardStats();
      recentSales = (await getSales()).slice(0, 10);
    } catch {}
  });

  function fmt(n: number) { return `Bs ${n.toFixed(2)}`; }
</script>

<div class="page">
  <div class="page-header">
    <div>
      <h1 class="page-title">📊 Reportes</h1>
      <p class="page-subtitle">Resumen de actividad</p>
    </div>
  </div>

  <div class="card-grid card-grid-4" style="margin-bottom: var(--space-2xl);">
    <div class="stat-card"><div class="stat-icon green">💰</div><div class="stat-content"><div class="stat-value">{fmt(stats.total_sales_today)}</div><div class="stat-label">Ventas hoy</div></div></div>
    <div class="stat-card"><div class="stat-icon blue">🧾</div><div class="stat-content"><div class="stat-value">{stats.total_transactions_today}</div><div class="stat-label">Transacciones</div></div></div>
    <div class="stat-card"><div class="stat-icon yellow">⚠️</div><div class="stat-content"><div class="stat-value">{stats.low_stock_count}</div><div class="stat-label">Bajo stock</div></div></div>
    <div class="stat-card"><div class="stat-icon red">⏰</div><div class="stat-content"><div class="stat-value">{stats.expiring_soon_count}</div><div class="stat-label">Por vencer</div></div></div>
  </div>

  <!-- ─── Top Selling Products ──────────────────────────── -->
  <div class="card" style="margin-bottom: var(--space-2xl);">
    <div class="top-header">
      <h3 style="font-weight: 700;">🏆 Productos más vendidos</h3>
      <div class="top-controls">
        <div class="btn-group">
          <button class="btn btn-sm {sortBy === 'quantity' ? 'btn-primary' : 'btn-ghost'}" onclick={() => sortBy = 'quantity'}>Por cantidad</button>
          <button class="btn btn-sm {sortBy === 'revenue' ? 'btn-primary' : 'btn-ghost'}" onclick={() => sortBy = 'revenue'}>Por monto</button>
        </div>
        <select class="select select-sm" bind:value={topLimit}>
          <option value={10}>Top 10</option>
          <option value={20}>Top 20</option>
        </select>
      </div>
    </div>

    <div class="period-bar">
      <div class="period-presets">
        {#each [
          { key: 'today', label: 'Hoy' },
          { key: 'week', label: 'Esta semana' },
          { key: 'month', label: 'Este mes' },
          { key: 'last_month', label: 'Mes anterior' },
          { key: 'all', label: 'Todo' },
          { key: 'custom', label: 'Personalizado' },
        ] as preset}
          <button
            class="btn btn-sm {selectedPeriod === preset.key ? 'btn-primary' : 'btn-ghost'}"
            onclick={() => selectedPeriod = preset.key as PeriodPreset}
          >{preset.label}</button>
        {/each}
      </div>
      {#if selectedPeriod === 'custom'}
        <div class="custom-dates">
          <input type="date" class="input input-sm" bind:value={customFrom} />
          <span style="color: var(--text-muted);">—</span>
          <input type="date" class="input input-sm" bind:value={customTo} />
        </div>
      {/if}
    </div>

    {#if topLoading}
      <div class="top-empty">
        <span class="text-muted">Cargando...</span>
      </div>
    {:else if sortedProducts().length === 0}
      <div class="top-empty">
        <span style="font-size: 2rem;">📦</span>
        <span class="text-muted">No hay datos de ventas para este período</span>
      </div>
    {:else}
      <div class="top-list">
        {#each sortedProducts() as product, i}
          <div class="top-item">
            <div class="top-rank" class:gold={i === 0} class:silver={i === 1} class:bronze={i === 2}>
              {#if i === 0}🥇{:else if i === 1}🥈{:else if i === 2}🥉{:else}{i + 1}{/if}
            </div>
            <div class="top-info">
              <div class="top-name">{product.product_name}</div>
              <div class="top-bar-track">
                <div class="top-bar-fill" style="width: {barPercent(product)}%;"></div>
              </div>
            </div>
            <div class="top-values">
              <span class="top-primary">
                {sortBy === 'quantity' ? product.total_quantity.toFixed(0) + ' uds' : fmt(product.total_revenue)}
              </span>
              <span class="top-secondary">
                {sortBy === 'quantity' ? fmt(product.total_revenue) : product.total_quantity.toFixed(0) + ' uds'}
              </span>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>

  <div class="card">
    <h3 style="font-weight: 700; margin-bottom: var(--space-lg);">Últimas Ventas</h3>
    <div class="table-container" style="border: none;">
      <table>
        <thead><tr><th>#</th><th>Total</th><th>Método</th><th>Estado</th></tr></thead>
        <tbody>
          {#each recentSales as s}
            <tr>
              <td style="font-weight: 700;">#{s.sale_number}</td>
              <td style="font-weight: 700; color: var(--accent-success);">{fmt(s.total)}</td>
              <td>{s.payment_method}</td>
              <td><span class="badge {s.status === 'completed' ? 'badge-success' : 'badge-danger'}">{s.status === 'completed' ? 'OK' : 'Anulada'}</span></td>
            </tr>
          {:else}
            <tr><td colspan="4" class="text-center text-muted">Sin ventas</td></tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>
</div>

<style>
  .top-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: var(--space-lg);
    flex-wrap: wrap;
    gap: var(--space-sm);
  }
  .top-controls {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
  }
  .btn-group {
    display: flex;
    gap: 2px;
    background: var(--bg-tertiary);
    border-radius: var(--radius-md);
    padding: 2px;
  }
  .btn-sm {
    padding: 0.3rem 0.75rem;
    font-size: var(--font-size-sm);
  }
  .select-sm {
    padding: 0.3rem 0.5rem;
    font-size: var(--font-size-sm);
    min-width: auto;
  }
  .period-bar {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--space-md);
    margin-bottom: var(--space-lg);
    padding-bottom: var(--space-md);
    border-bottom: 1px solid var(--border-primary);
  }
  .period-presets {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
  }
  .custom-dates {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
  }
  .custom-dates .input {
    width: 140px;
    padding: 0.3rem 0.5rem;
    font-size: var(--font-size-sm);
  }
  .top-empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: var(--space-sm);
    padding: var(--space-2xl) 0;
  }

  .top-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
  }
  .top-item {
    display: flex;
    align-items: center;
    gap: var(--space-md);
    padding: var(--space-sm) var(--space-md);
    border-radius: var(--radius-md);
    transition: background 0.15s;
  }
  .top-item:hover {
    background: var(--bg-secondary);
  }
  .top-rank {
    width: 2rem;
    text-align: center;
    font-weight: 700;
    font-size: var(--font-size-sm);
    color: var(--text-muted);
    flex-shrink: 0;
  }
  .top-rank.gold { font-size: 1.25rem; }
  .top-rank.silver { font-size: 1.15rem; }
  .top-rank.bronze { font-size: 1.1rem; }

  .top-info {
    flex: 1;
    min-width: 0;
  }
  .top-name {
    font-weight: 600;
    font-size: var(--font-size-sm);
    margin-bottom: 4px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .top-bar-track {
    height: 6px;
    background: var(--bg-tertiary);
    border-radius: 3px;
    overflow: hidden;
  }
  .top-bar-fill {
    height: 100%;
    background: linear-gradient(90deg, #3b82f6, #8b5cf6);
    border-radius: 3px;
    transition: width 0.4s ease;
  }
  .top-values {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    flex-shrink: 0;
    min-width: 90px;
  }
  .top-primary {
    font-weight: 700;
    font-size: var(--font-size-sm);
    color: var(--text-primary);
  }
  .top-secondary {
    font-size: var(--font-size-xs);
    color: var(--text-muted);
  }
</style>
