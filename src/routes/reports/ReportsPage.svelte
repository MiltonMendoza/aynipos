<script lang="ts">
  import { onMount } from 'svelte';
  import { getDashboardStats, getSales } from '$lib/services/api';
  import type { DashboardStats, Sale } from '$lib/types';

  let stats: DashboardStats = $state({
    total_sales_today: 0, total_transactions_today: 0, total_products: 0,
    low_stock_count: 0, expiring_soon_count: 0
  });
  let recentSales: Sale[] = $state([]);

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
