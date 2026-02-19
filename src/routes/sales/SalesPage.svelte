<script lang="ts">
  import { onMount } from 'svelte';
  import type { Sale, SaleItem } from '$lib/types';
  import { getSales, getSaleItems, cancelSale } from '$lib/services/api';

  let sales: Sale[] = $state([]);
  let selectedSale: Sale | null = $state(null);
  let saleItems: SaleItem[] = $state([]);
  let loading = $state(true);

  onMount(async () => {
    await loadSales();
  });

  async function loadSales() {
    loading = true;
    try {
      sales = await getSales();
    } catch { sales = []; }
    loading = false;
  }

  async function viewSale(sale: Sale) {
    selectedSale = sale;
    saleItems = await getSaleItems(sale.id);
  }

  async function handleCancel(saleId: string) {
    if (!confirm('¿Estás seguro de anular esta venta?')) return;
    try {
      await cancelSale(saleId);
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

  <div style="display: flex; gap: var(--space-xl); height: calc(100vh - 140px);">
    <!-- Sales list -->
    <div style="flex: 1; overflow-y: auto;" class="table-container">
      <table>
        <thead>
          <tr>
            <th>#</th>
            <th>Fecha</th>
            <th>Total</th>
            <th>Pago</th>
            <th>Estado</th>
          </tr>
        </thead>
        <tbody>
          {#if sales.length === 0}
            <tr><td colspan="5" class="text-center text-muted" style="padding: var(--space-3xl);">No hay ventas registradas</td></tr>
          {:else}
            {#each sales as sale}
              <tr
                onclick={() => viewSale(sale)}
                style="cursor: pointer; {selectedSale?.id === sale.id ? 'background: var(--accent-primary-glow);' : ''}"
              >
                <td style="font-weight: 700;">#{sale.sale_number}</td>
                <td>{formatDate(sale.created_at)}</td>
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
    </div>

    <!-- Sale detail -->
    {#if selectedSale}
      <div class="card" style="width: 360px; min-width: 360px; overflow-y: auto;">
        <div class="flex items-center justify-between" style="margin-bottom: var(--space-xl);">
          <h3 style="font-weight: 700;">Venta #{selectedSale.sale_number}</h3>
          <span class="badge {statusBadge(selectedSale.status).class}">{statusBadge(selectedSale.status).label}</span>
        </div>

        <div style="display: flex; flex-direction: column; gap: var(--space-md); margin-bottom: var(--space-xl);">
          {#each saleItems as item}
            <div class="flex justify-between text-sm">
              <div>
                <div style="font-weight: 600;">{item.product_name}</div>
                <div class="text-xs text-muted">{item.quantity} × {formatCurrency(item.unit_price)}</div>
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
          <div style="margin-top: var(--space-xl);">
            <button class="btn btn-danger btn-block" onclick={() => handleCancel(selectedSale!.id)}>
              🚫 Anular Venta
            </button>
          </div>
        {/if}
      </div>
    {/if}
  </div>
</div>
