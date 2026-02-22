<script lang="ts">
  import { onMount } from 'svelte';
  import { getDashboardStats, getSales, getTopSellingProducts, getSalesChartData, getProfitMarginReport, getInventoryReport, saveReportCsv, saveReportHtml } from '$lib/services/api';
  import type { DashboardStats, Sale, TopSellingProduct, SalesChartDataPoint, ProfitMarginProduct, InventoryReportItem } from '$lib/types';
  import { save } from '@tauri-apps/plugin-dialog';
  import { openPath } from '@tauri-apps/plugin-opener';

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

  // ─── Sales Chart ───────────────────────────────────────
  let chartData: SalesChartDataPoint[] = $state([]);
  let chartLoading = $state(false);
  let chartGroupBy: 'day' | 'week' | 'month' = $state('day');
  let chartPeriod: PeriodPreset = $state('month');
  let chartCustomFrom = $state('');
  let chartCustomTo = $state('');
  let hoveredBar: number | null = $state(null);

  // ─── Profit Margin ──────────────────────────────────────
  let marginProducts: ProfitMarginProduct[] = $state([]);
  let marginLoading = $state(false);
  let marginPeriod: PeriodPreset = $state('month');
  let marginCustomFrom = $state('');
  let marginCustomTo = $state('');
  let marginSortCol: 'product_name' | 'purchase_price' | 'avg_sale_price' | 'total_quantity' | 'total_revenue' | 'total_cost' | 'gross_profit' | 'margin_percent' = $state('gross_profit');
  let marginSortAsc = $state(false);

  // ─── Inventory Report ──────────────────────────────────
  let invItems: InventoryReportItem[] = $state([]);
  let invLoading = $state(false);
  let invInactiveDays: number | undefined = $state(undefined);
  type InvSortCol = 'product_name' | 'sku' | 'category_name' | 'current_stock' | 'purchase_price' | 'sale_price' | 'stock_cost_value' | 'stock_sale_value' | 'days_without_movement';
  let invSortCol: InvSortCol = $state('stock_cost_value');
  let invSortAsc = $state(false);

  // ─── Chart constants ───────────────────────────────────
  const CHART_W = 800;
  const CHART_H = 320;
  const PAD = { top: 20, right: 20, bottom: 60, left: 70 };
  const INNER_W = CHART_W - PAD.left - PAD.right;
  const INNER_H = CHART_H - PAD.top - PAD.bottom;

  function getDateRange(preset: PeriodPreset, cf: string, ct: string): { from?: string; to?: string } {
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
        return { from: cf || undefined, to: ct || undefined };
      default:
        return {};
    }
  }

  async function loadTopProducts() {
    topLoading = true;
    try {
      const { from, to } = getDateRange(selectedPeriod, customFrom, customTo);
      topProducts = await getTopSellingProducts(from, to, topLimit);
    } catch { topProducts = []; }
    topLoading = false;
  }

  async function loadChartData() {
    chartLoading = true;
    try {
      const { from, to } = getDateRange(chartPeriod, chartCustomFrom, chartCustomTo);
      chartData = await getSalesChartData(from, to, chartGroupBy);
    } catch { chartData = []; }
    chartLoading = false;
  }

  $effect(() => {
    selectedPeriod; topLimit; customFrom; customTo;
    loadTopProducts();
  });

  $effect(() => {
    chartPeriod; chartGroupBy; chartCustomFrom; chartCustomTo;
    loadChartData();
  });

  async function loadMarginData() {
    marginLoading = true;
    try {
      const { from, to } = getDateRange(marginPeriod, marginCustomFrom, marginCustomTo);
      marginProducts = await getProfitMarginReport(from, to);
    } catch { marginProducts = []; }
    marginLoading = false;
  }

  $effect(() => {
    marginPeriod; marginCustomFrom; marginCustomTo;
    loadMarginData();
  });

  async function loadInventoryReport() {
    invLoading = true;
    try {
      invItems = await getInventoryReport(invInactiveDays);
    } catch { invItems = []; }
    invLoading = false;
  }

  $effect(() => {
    invInactiveDays;
    loadInventoryReport();
  });

  function sortedInvItems(): InventoryReportItem[] {
    return [...invItems].sort((a, b) => {
      const va = a[invSortCol];
      const vb = b[invSortCol];
      if (va === null && vb === null) return 0;
      if (va === null) return 1;
      if (vb === null) return -1;
      if (typeof va === 'string') return invSortAsc ? (va as string).localeCompare(vb as string) : (vb as string).localeCompare(va as string);
      return invSortAsc ? (va as number) - (vb as number) : (vb as number) - (va as number);
    });
  }

  function toggleInvSort(col: InvSortCol) {
    if (invSortCol === col) invSortAsc = !invSortAsc;
    else { invSortCol = col; invSortAsc = false; }
  }

  function invTotalCostValue() { return invItems.reduce((s, p) => s + p.stock_cost_value, 0); }
  function invTotalSaleValue() { return invItems.reduce((s, p) => s + p.stock_sale_value, 0); }
  function invTotalStock() { return invItems.reduce((s, p) => s + p.current_stock, 0); }
  function invInactiveCount() { return invItems.filter(p => p.days_without_movement !== null && p.days_without_movement >= 30).length; }

  function inactiveBadgeClass(days: number | null): string {
    if (days === null) return 'badge-muted';
    if (days >= 90) return 'badge-danger';
    if (days >= 30) return 'badge-warning';
    return 'badge-success';
  }

  function sortedMarginProducts(): ProfitMarginProduct[] {
    return [...marginProducts].sort((a, b) => {
      const va = a[marginSortCol];
      const vb = b[marginSortCol];
      if (typeof va === 'string') return marginSortAsc ? (va as string).localeCompare(vb as string) : (vb as string).localeCompare(va as string);
      return marginSortAsc ? (va as number) - (vb as number) : (vb as number) - (va as number);
    });
  }

  function toggleMarginSort(col: typeof marginSortCol) {
    if (marginSortCol === col) marginSortAsc = !marginSortAsc;
    else { marginSortCol = col; marginSortAsc = false; }
  }

  function marginTotalRevenue() { return marginProducts.reduce((s, p) => s + p.total_revenue, 0); }
  function marginTotalCost() { return marginProducts.reduce((s, p) => s + p.total_cost, 0); }
  function marginTotalProfit() { return marginProducts.reduce((s, p) => s + p.gross_profit, 0); }
  function marginAvgPercent() {
    const cost = marginTotalCost();
    if (cost === 0) return 0;
    return (marginTotalProfit() / cost) * 100;
  }

  function marginColor(pct: number): string {
    if (pct >= 30) return 'var(--accent-success)';
    if (pct >= 15) return 'var(--accent-warning)';
    return 'var(--accent-danger)';
  }

  function marginBadgeClass(pct: number): string {
    if (pct >= 30) return 'badge-success';
    if (pct >= 15) return 'badge-warning';
    return 'badge-danger';
  }

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

  // ─── Chart computed helpers ────────────────────────────
  function chartMaxSales(): number {
    if (chartData.length === 0) return 1;
    const m = Math.max(...chartData.map(d => d.total_sales));
    if (m === 0) return 1;
    const mag = Math.pow(10, Math.floor(Math.log10(m)));
    return Math.ceil(m / mag) * mag;
  }

  function gridLines(): number[] {
    const max = chartMaxSales();
    const step = max / 4;
    return [0, step, step * 2, step * 3, max];
  }

  function barX(i: number): number {
    if (chartData.length === 0) return 0;
    const gap = Math.min(8, INNER_W / chartData.length * 0.2);
    const bw = (INNER_W - gap * (chartData.length + 1)) / chartData.length;
    return PAD.left + gap + i * (bw + gap);
  }

  function chartBarWidth(): number {
    if (chartData.length === 0) return 0;
    const gap = Math.min(8, INNER_W / chartData.length * 0.2);
    return Math.max(2, (INNER_W - gap * (chartData.length + 1)) / chartData.length);
  }

  function barHeight(d: SalesChartDataPoint): number {
    const max = chartMaxSales();
    return (d.total_sales / max) * INNER_H;
  }

  function barY(d: SalesChartDataPoint): number {
    return PAD.top + INNER_H - barHeight(d);
  }

  function trendPoints(): string {
    if (chartData.length < 2) return '';
    const bw = chartBarWidth();
    return chartData.map((d, i) => {
      const x = barX(i) + bw / 2;
      const y = barY(d);
      return `${x},${y}`;
    }).join(' ');
  }

  function chartTotal(): number {
    return chartData.reduce((s, d) => s + d.total_sales, 0);
  }

  function chartAvg(): number {
    if (chartData.length === 0) return 0;
    return chartTotal() / chartData.length;
  }

  function chartBestLabel(): string {
    if (chartData.length === 0) return '-';
    const best = chartData.reduce((a, b) => a.total_sales > b.total_sales ? a : b);
    return formatChartLabel(best.label);
  }

  function chartTotalTransactions(): number {
    return chartData.reduce((s, d) => s + d.transaction_count, 0);
  }

  function formatChartLabel(label: string): string {
    if (label.includes('-W')) return `Sem ${label.split('-W')[1]}`;
    if (/^\d{4}-\d{2}$/.test(label)) {
      const months = ['Ene','Feb','Mar','Abr','May','Jun','Jul','Ago','Sep','Oct','Nov','Dic'];
      const [, m] = label.split('-');
      return `${months[parseInt(m) - 1]}`;
    }
    if (/^\d{4}-\d{2}-\d{2}$/.test(label)) {
      const parts = label.split('-');
      return `${parts[2]}/${parts[1]}`;
    }
    return label;
  }

  onMount(async () => {
    try {
      stats = await getDashboardStats();
      recentSales = (await getSales()).slice(0, 10);
    } catch {}
  });

  function fmt(n: number) { return `Bs ${n.toFixed(2)}`; }
  function fmtShort(n: number): string {
    if (n >= 1000000) return `${(n / 1000000).toFixed(1)}M`;
    if (n >= 1000) return `${(n / 1000).toFixed(1)}K`;
    return n.toFixed(0);
  }

  // ─── Export helpers ─────────────────────────────────────
  let exporting = $state(false);

  function todayStr(): string {
    return new Date().toISOString().split('T')[0];
  }

  async function doExportCsv(filename: string, content: string) {
    const filePath = await save({
      defaultPath: filename,
      filters: [{ name: 'CSV', extensions: ['csv'] }],
    });
    if (!filePath) return;
    await saveReportCsv(content, filePath);
  }

  async function doExportPdf(title: string, bodyHtml: string) {
    const html = `<!DOCTYPE html>
<html lang="es">
<head>
  <meta charset="UTF-8">
  <title>${title} — AyniPOS</title>
  <style>
    *, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }
    body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; font-size: 12px; color: #000; background: #f5f5f5; padding: 20px; }
    .report { max-width: 900px; margin: 0 auto; background: #fff; padding: 24px; border-radius: 8px; box-shadow: 0 2px 12px rgba(0,0,0,0.08); }
    h1 { font-size: 20px; margin-bottom: 4px; }
    .subtitle { font-size: 12px; color: #666; margin-bottom: 16px; }
    table { width: 100%; border-collapse: collapse; margin: 12px 0; }
    th { background: #f8f9fa; font-size: 11px; text-transform: uppercase; letter-spacing: 0.5px; text-align: left; padding: 8px 6px; border-bottom: 2px solid #dee2e6; }
    th.num { text-align: right; }
    td { padding: 6px; border-bottom: 1px solid #eee; font-size: 11px; }
    td.num { text-align: right; font-variant-numeric: tabular-nums; }
    td.bold { font-weight: 700; }
    tfoot td { border-top: 2px solid #000; font-weight: 700; }
    .summary-grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 12px; margin-bottom: 16px; }
    .summary-card { background: #f8f9fa; border-radius: 6px; padding: 12px; text-align: center; }
    .summary-card .value { font-size: 18px; font-weight: 700; }
    .summary-card .label { font-size: 10px; color: #666; margin-top: 2px; }
    .badge { display: inline-block; padding: 2px 8px; border-radius: 10px; font-size: 10px; font-weight: 600; }
    .badge-success { background: #d1fae5; color: #065f46; }
    .badge-warning { background: #fef3c7; color: #92400e; }
    .badge-danger { background: #fee2e2; color: #991b1b; }
    .print-actions { text-align: center; margin: 20px auto; max-width: 900px; }
    .print-actions button { background: #3b82f6; color: #fff; border: none; padding: 12px 32px; border-radius: 8px; font-size: 15px; font-weight: 600; cursor: pointer; }
    .print-actions button:hover { background: #2563eb; }
    .print-actions .hint { font-size: 12px; color: #888; margin-top: 8px; }
    @media print {
      body { background: #fff; padding: 0; }
      .report { box-shadow: none; border-radius: 0; padding: 0; }
      .print-actions { display: none !important; }
      @page { margin: 10mm; }
    }
  </style>
</head>
<body>
  <div class="print-actions">
    <button onclick="window.print()">🖨️ Imprimir / Guardar PDF</button>
    <div class="hint">Ctrl+P / Cmd+P → Guardar como PDF</div>
  </div>
  <div class="report">
    <h1>${title}</h1>
    <div class="subtitle">Generado el ${new Date().toLocaleDateString('es-BO', { day: '2-digit', month: '2-digit', year: 'numeric', hour: '2-digit', minute: '2-digit' })} — AyniPOS</div>
    ${bodyHtml}
  </div>
</body>
</html>`;
    const filePath = await saveReportHtml(html);
    await openPath(filePath);
  }

  // ─── CSV export functions ──────────────────────────────

  async function exportChartCsv() {
    if (chartData.length === 0) return;
    exporting = true;
    try {
      const rows = ['Período,Ventas (Bs),Transacciones'];
      for (const d of chartData) {
        rows.push(`"${d.label}",${d.total_sales.toFixed(2)},${d.transaction_count}`);
      }
      rows.push(`"TOTAL",${chartTotal().toFixed(2)},${chartTotalTransactions()}`);
      await doExportCsv(`ventas_${chartGroupBy}_${todayStr()}.csv`, rows.join('\n'));
    } catch { /* ignore cancel */ }
    exporting = false;
  }

  async function exportTopProductsCsv() {
    const data = sortedProducts();
    if (data.length === 0) return;
    exporting = true;
    try {
      const rows = ['Ranking,Producto,Cantidad,Ingresos (Bs)'];
      data.forEach((p, i) => {
        rows.push(`${i + 1},"${p.product_name}",${p.total_quantity.toFixed(0)},${p.total_revenue.toFixed(2)}`);
      });
      await doExportCsv(`productos_top_${todayStr()}.csv`, rows.join('\n'));
    } catch { /* ignore cancel */ }
    exporting = false;
  }

  async function exportMarginCsv() {
    const data = sortedMarginProducts();
    if (data.length === 0) return;
    exporting = true;
    try {
      const rows = ['Producto,P. Compra,P. Venta Prom.,Cantidad,Ingresos,Costo,Utilidad,Margen %'];
      for (const p of data) {
        rows.push(`"${p.product_name}",${p.purchase_price.toFixed(2)},${p.avg_sale_price.toFixed(2)},${p.total_quantity.toFixed(0)},${p.total_revenue.toFixed(2)},${p.total_cost.toFixed(2)},${p.gross_profit.toFixed(2)},${p.margin_percent.toFixed(1)}`);
      }
      rows.push(`"TOTAL",,,,${marginTotalRevenue().toFixed(2)},${marginTotalCost().toFixed(2)},${marginTotalProfit().toFixed(2)},${marginAvgPercent().toFixed(1)}`);
      await doExportCsv(`margen_ganancia_${todayStr()}.csv`, rows.join('\n'));
    } catch { /* ignore cancel */ }
    exporting = false;
  }

  async function exportInventoryCsv() {
    const data = sortedInvItems();
    if (data.length === 0) return;
    exporting = true;
    try {
      const rows = ['Producto,SKU,Categoría,Stock,P. Compra,P. Venta,Valor Costo,Valor Venta,Días sin mov.'];
      for (const p of data) {
        rows.push(`"${p.product_name}","${p.sku}","${p.category_name || ''}",${p.current_stock.toFixed(0)},${p.purchase_price.toFixed(2)},${p.sale_price.toFixed(2)},${p.stock_cost_value.toFixed(2)},${p.stock_sale_value.toFixed(2)},${p.days_without_movement ?? 'N/A'}`);
      }
      rows.push(`"TOTAL",,,,,,${invTotalCostValue().toFixed(2)},${invTotalSaleValue().toFixed(2)},`);
      await doExportCsv(`inventario_${todayStr()}.csv`, rows.join('\n'));
    } catch { /* ignore cancel */ }
    exporting = false;
  }

  // ─── PDF export functions ──────────────────────────────

  async function exportChartPdf() {
    if (chartData.length === 0) return;
    exporting = true;
    try {
      const summaryHtml = `
        <div class="summary-grid">
          <div class="summary-card"><div class="value">${fmt(chartTotal())}</div><div class="label">Total período</div></div>
          <div class="summary-card"><div class="value">${fmt(chartAvg())}</div><div class="label">Promedio</div></div>
          <div class="summary-card"><div class="value">${chartTotalTransactions()}</div><div class="label">Transacciones</div></div>
          <div class="summary-card"><div class="value">${chartBestLabel()}</div><div class="label">Mejor período</div></div>
        </div>`;
      const tableRows = chartData.map(d =>
        `<tr><td>${formatChartLabel(d.label)}</td><td class="num">${fmt(d.total_sales)}</td><td class="num">${d.transaction_count}</td></tr>`
      ).join('');
      const body = `${summaryHtml}
        <table>
          <thead><tr><th>Período</th><th class="num">Ventas</th><th class="num">Transacciones</th></tr></thead>
          <tbody>${tableRows}</tbody>
          <tfoot><tr><td class="bold">Total</td><td class="num">${fmt(chartTotal())}</td><td class="num">${chartTotalTransactions()}</td></tr></tfoot>
        </table>`;
      await doExportPdf('📈 Gráfico de Ventas', body);
    } catch { /* ignore */ }
    exporting = false;
  }

  async function exportTopProductsPdf() {
    const data = sortedProducts();
    if (data.length === 0) return;
    exporting = true;
    try {
      const tableRows = data.map((p, i) =>
        `<tr><td class="bold">${i === 0 ? '🥇' : i === 1 ? '🥈' : i === 2 ? '🥉' : (i+1)}</td><td>${p.product_name}</td><td class="num">${p.total_quantity.toFixed(0)}</td><td class="num">${fmt(p.total_revenue)}</td></tr>`
      ).join('');
      const body = `
        <table>
          <thead><tr><th>#</th><th>Producto</th><th class="num">Cantidad</th><th class="num">Ingresos</th></tr></thead>
          <tbody>${tableRows}</tbody>
        </table>`;
      await doExportPdf('🏆 Productos Más Vendidos', body);
    } catch { /* ignore */ }
    exporting = false;
  }

  async function exportMarginPdf() {
    const data = sortedMarginProducts();
    if (data.length === 0) return;
    exporting = true;
    try {
      const summaryHtml = `
        <div class="summary-grid">
          <div class="summary-card"><div class="value">${fmt(marginTotalRevenue())}</div><div class="label">Ingresos</div></div>
          <div class="summary-card"><div class="value">${fmt(marginTotalCost())}</div><div class="label">Costo</div></div>
          <div class="summary-card"><div class="value">${fmt(marginTotalProfit())}</div><div class="label">Utilidad bruta</div></div>
          <div class="summary-card"><div class="value">${marginAvgPercent().toFixed(1)}%</div><div class="label">Margen promedio</div></div>
        </div>`;
      const badgeClass = (pct: number) => pct >= 30 ? 'badge-success' : pct >= 15 ? 'badge-warning' : 'badge-danger';
      const tableRows = data.map(p =>
        `<tr><td class="bold">${p.product_name}</td><td class="num">${fmt(p.purchase_price)}</td><td class="num">${fmt(p.avg_sale_price)}</td><td class="num">${p.total_quantity.toFixed(0)}</td><td class="num">${fmt(p.total_revenue)}</td><td class="num">${fmt(p.total_cost)}</td><td class="num bold">${fmt(p.gross_profit)}</td><td class="num"><span class="badge ${badgeClass(p.margin_percent)}">${p.margin_percent.toFixed(1)}%</span></td></tr>`
      ).join('');
      const body = `${summaryHtml}
        <table>
          <thead><tr><th>Producto</th><th class="num">P. Compra</th><th class="num">P. Venta</th><th class="num">Uds</th><th class="num">Ingresos</th><th class="num">Costo</th><th class="num">Utilidad</th><th class="num">Margen</th></tr></thead>
          <tbody>${tableRows}</tbody>
          <tfoot><tr><td class="bold">Total</td><td></td><td></td><td></td><td class="num">${fmt(marginTotalRevenue())}</td><td class="num">${fmt(marginTotalCost())}</td><td class="num">${fmt(marginTotalProfit())}</td><td class="num"><span class="badge ${badgeClass(marginAvgPercent())}">${marginAvgPercent().toFixed(1)}%</span></td></tr></tfoot>
        </table>`;
      await doExportPdf('💰 Margen de Ganancia', body);
    } catch { /* ignore */ }
    exporting = false;
  }

  async function exportInventoryPdf() {
    const data = sortedInvItems();
    if (data.length === 0) return;
    exporting = true;
    try {
      const summaryHtml = `
        <div class="summary-grid">
          <div class="summary-card"><div class="value">${data.length}</div><div class="label">Productos</div></div>
          <div class="summary-card"><div class="value">${fmt(invTotalCostValue())}</div><div class="label">Valor a costo</div></div>
          <div class="summary-card"><div class="value">${fmt(invTotalSaleValue())}</div><div class="label">Valor a venta</div></div>
          <div class="summary-card"><div class="value">${invInactiveCount()}</div><div class="label">Sin mov. 30+ d</div></div>
        </div>`;
      const daysBadge = (d: number | null) => {
        if (d === null) return '<span class="badge">Sin mov.</span>';
        const cls = d >= 90 ? 'badge-danger' : d >= 30 ? 'badge-warning' : 'badge-success';
        return `<span class="badge ${cls}">${d}d</span>`;
      };
      const tableRows = data.map(p =>
        `<tr><td class="bold">${p.product_name}</td><td>${p.sku}</td><td>${p.category_name || '—'}</td><td class="num">${p.current_stock.toFixed(0)}</td><td class="num">${fmt(p.purchase_price)}</td><td class="num">${fmt(p.sale_price)}</td><td class="num bold">${fmt(p.stock_cost_value)}</td><td class="num bold">${fmt(p.stock_sale_value)}</td><td class="num">${daysBadge(p.days_without_movement)}</td></tr>`
      ).join('');
      const body = `${summaryHtml}
        <table>
          <thead><tr><th>Producto</th><th>SKU</th><th>Categoría</th><th class="num">Stock</th><th class="num">P. Compra</th><th class="num">P. Venta</th><th class="num">V. Costo</th><th class="num">V. Venta</th><th class="num">Inactividad</th></tr></thead>
          <tbody>${tableRows}</tbody>
          <tfoot><tr><td class="bold">Total</td><td></td><td></td><td class="num">${invTotalStock().toFixed(0)}</td><td></td><td></td><td class="num">${fmt(invTotalCostValue())}</td><td class="num">${fmt(invTotalSaleValue())}</td><td></td></tr></tfoot>
        </table>`;
      await doExportPdf('📦 Reporte de Inventario', body);
    } catch { /* ignore */ }
    exporting = false;
  }
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

  <!-- ─── Sales Chart ──────────────────────────────────── -->
  <div class="card" style="margin-bottom: var(--space-2xl);">
    <div class="top-header">
      <h3 style="font-weight: 700;">📈 Gráfico de ventas</h3>
      <div class="top-controls">
        <div class="btn-group">
          <button class="btn btn-sm {chartGroupBy === 'day' ? 'btn-primary' : 'btn-ghost'}" onclick={() => chartGroupBy = 'day'}>Diario</button>
          <button class="btn btn-sm {chartGroupBy === 'week' ? 'btn-primary' : 'btn-ghost'}" onclick={() => chartGroupBy = 'week'}>Semanal</button>
          <button class="btn btn-sm {chartGroupBy === 'month' ? 'btn-primary' : 'btn-ghost'}" onclick={() => chartGroupBy = 'month'}>Mensual</button>
        </div>
        {#if chartData.length > 0}
          <div class="export-btns">
            <button class="btn btn-sm btn-ghost" onclick={exportChartCsv} disabled={exporting} title="Exportar a CSV">📥 CSV</button>
            <button class="btn btn-sm btn-ghost" onclick={exportChartPdf} disabled={exporting} title="Exportar a PDF">📄 PDF</button>
          </div>
        {/if}
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
            class="btn btn-sm {chartPeriod === preset.key ? 'btn-primary' : 'btn-ghost'}"
            onclick={() => chartPeriod = preset.key as PeriodPreset}
          >{preset.label}</button>
        {/each}
      </div>
      {#if chartPeriod === 'custom'}
        <div class="custom-dates">
          <input type="date" class="input input-sm" bind:value={chartCustomFrom} />
          <span style="color: var(--text-muted);">—</span>
          <input type="date" class="input input-sm" bind:value={chartCustomTo} />
        </div>
      {/if}
    </div>

    {#if chartLoading}
      <div class="top-empty">
        <span class="text-muted">Cargando...</span>
      </div>
    {:else if chartData.length === 0}
      <div class="top-empty">
        <span style="font-size: 2rem;">📦</span>
        <span class="text-muted">No hay datos de ventas para este período</span>
      </div>
    {:else}
      <div class="chart-container">
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <svg viewBox="0 0 {CHART_W} {CHART_H}" class="chart-svg" onmouseleave={() => hoveredBar = null}>
          <defs>
            <linearGradient id="barGrad" x1="0" y1="0" x2="0" y2="1">
              <stop offset="0%" stop-color="#8b5cf6" />
              <stop offset="100%" stop-color="#3b82f6" />
            </linearGradient>
            <linearGradient id="barGradHover" x1="0" y1="0" x2="0" y2="1">
              <stop offset="0%" stop-color="#a78bfa" />
              <stop offset="100%" stop-color="#60a5fa" />
            </linearGradient>
          </defs>

          <!-- Grid lines -->
          {#each gridLines() as gl}
            <line
              x1={PAD.left} y1={PAD.top + INNER_H - (gl / chartMaxSales()) * INNER_H}
              x2={PAD.left + INNER_W} y2={PAD.top + INNER_H - (gl / chartMaxSales()) * INNER_H}
              class="chart-grid"
            />
            <text
              x={PAD.left - 10}
              y={PAD.top + INNER_H - (gl / chartMaxSales()) * INNER_H + 4}
              class="chart-label-y"
            >{fmtShort(gl)}</text>
          {/each}

          <!-- Bars -->
          {#each chartData as d, i}
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <rect
              x={barX(i)}
              y={barY(d)}
              width={chartBarWidth()}
              height={Math.max(0, barHeight(d))}
              rx="3"
              fill={hoveredBar === i ? 'url(#barGradHover)' : 'url(#barGrad)'}
              class="chart-bar"
              onmouseenter={() => hoveredBar = i}
            />
            <!-- X-axis label -->
            {#if chartData.length <= 31 || i % Math.ceil(chartData.length / 20) === 0}
              <text
                x={barX(i) + chartBarWidth() / 2}
                y={PAD.top + INNER_H + 18}
                class="chart-label-x"
                transform="rotate(-45, {barX(i) + chartBarWidth() / 2}, {PAD.top + INNER_H + 18})"
              >{formatChartLabel(d.label)}</text>
            {/if}
          {/each}

          <!-- Trend line -->
          {#if chartData.length >= 2}
            <polyline
              points={trendPoints()}
              class="chart-trend"
            />
            {#each chartData as d, i}
              <circle
                cx={barX(i) + chartBarWidth() / 2}
                cy={barY(d)}
                r={hoveredBar === i ? 5 : 3}
                class="chart-trend-dot"
              />
            {/each}
          {/if}

          <!-- Hover tooltip -->
          {#if hoveredBar !== null && chartData[hoveredBar]}
            {@const d = chartData[hoveredBar]}
            {@const tx = Math.min(Math.max(barX(hoveredBar) + chartBarWidth() / 2, PAD.left + 70), CHART_W - PAD.right - 70)}
            {@const ty = barY(d) - 12}
            <rect
              x={tx - 70}
              y={ty - 32}
              width="140"
              height="36"
              rx="6"
              class="chart-tooltip-bg"
            />
            <text x={tx} y={ty - 14} class="chart-tooltip-text">
              {fmt(d.total_sales)}
            </text>
            <text x={tx} y={ty} class="chart-tooltip-sub">
              {d.transaction_count} transacciones
            </text>
          {/if}
        </svg>
      </div>

      <!-- Chart summary -->
      <div class="chart-summary">
        <div class="chart-stat">
          <span class="chart-stat-value">{fmt(chartTotal())}</span>
          <span class="chart-stat-label">Total período</span>
        </div>
        <div class="chart-stat">
          <span class="chart-stat-value">{fmt(chartAvg())}</span>
          <span class="chart-stat-label">Promedio</span>
        </div>
        <div class="chart-stat">
          <span class="chart-stat-value">{chartTotalTransactions()}</span>
          <span class="chart-stat-label">Transacciones</span>
        </div>
        <div class="chart-stat">
          <span class="chart-stat-value">{chartBestLabel()}</span>
          <span class="chart-stat-label">Mejor período</span>
        </div>
      </div>
    {/if}
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
        {#if topProducts.length > 0}
          <div class="export-btns">
            <button class="btn btn-sm btn-ghost" onclick={exportTopProductsCsv} disabled={exporting} title="Exportar a CSV">📥 CSV</button>
            <button class="btn btn-sm btn-ghost" onclick={exportTopProductsPdf} disabled={exporting} title="Exportar a PDF">📄 PDF</button>
          </div>
        {/if}
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

  <!-- ─── Profit Margin ──────────────────────────────────── -->
  <div class="card" style="margin-bottom: var(--space-2xl);">
    <div class="top-header">
      <h3 style="font-weight: 700;">💰 Margen de Ganancia</h3>
      {#if marginProducts.length > 0}
        <div class="export-btns">
          <button class="btn btn-sm btn-ghost" onclick={exportMarginCsv} disabled={exporting} title="Exportar a CSV">📥 CSV</button>
          <button class="btn btn-sm btn-ghost" onclick={exportMarginPdf} disabled={exporting} title="Exportar a PDF">📄 PDF</button>
        </div>
      {/if}
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
            class="btn btn-sm {marginPeriod === preset.key ? 'btn-primary' : 'btn-ghost'}"
            onclick={() => marginPeriod = preset.key as PeriodPreset}
          >{preset.label}</button>
        {/each}
      </div>
      {#if marginPeriod === 'custom'}
        <div class="custom-dates">
          <input type="date" class="input input-sm" bind:value={marginCustomFrom} />
          <span style="color: var(--text-muted);">—</span>
          <input type="date" class="input input-sm" bind:value={marginCustomTo} />
        </div>
      {/if}
    </div>

    {#if marginLoading}
      <div class="top-empty">
        <span class="text-muted">Cargando...</span>
      </div>
    {:else if marginProducts.length === 0}
      <div class="top-empty">
        <span style="font-size: 2rem;">📦</span>
        <span class="text-muted">No hay datos de ventas para este período</span>
      </div>
    {:else}
      <!-- Summary cards -->
      <div class="card-grid card-grid-4" style="margin-bottom: var(--space-lg);">
        <div class="stat-card">
          <div class="stat-icon blue">💵</div>
          <div class="stat-content">
            <div class="stat-value">{fmt(marginTotalRevenue())}</div>
            <div class="stat-label">Ingresos</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon red">📦</div>
          <div class="stat-content">
            <div class="stat-value">{fmt(marginTotalCost())}</div>
            <div class="stat-label">Costo</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon green">📈</div>
          <div class="stat-content">
            <div class="stat-value">{fmt(marginTotalProfit())}</div>
            <div class="stat-label">Utilidad bruta</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon" style="background: {marginColor(marginAvgPercent())}22; color: {marginColor(marginAvgPercent())};">%</div>
          <div class="stat-content">
            <div class="stat-value" style="color: {marginColor(marginAvgPercent())};">{marginAvgPercent().toFixed(1)}%</div>
            <div class="stat-label">Margen promedio</div>
          </div>
        </div>
      </div>

      <!-- Table -->
      <div class="table-container" style="border: none;">
        <table>
          <thead>
            <tr>
              <th class="sortable-th" onclick={() => toggleMarginSort('product_name')}>Producto {marginSortCol === 'product_name' ? (marginSortAsc ? '↑' : '↓') : ''}</th>
              <th class="sortable-th text-right" onclick={() => toggleMarginSort('purchase_price')}>P. Compra {marginSortCol === 'purchase_price' ? (marginSortAsc ? '↑' : '↓') : ''}</th>
              <th class="sortable-th text-right" onclick={() => toggleMarginSort('avg_sale_price')}>P. Venta Prom. {marginSortCol === 'avg_sale_price' ? (marginSortAsc ? '↑' : '↓') : ''}</th>
              <th class="sortable-th text-right" onclick={() => toggleMarginSort('total_quantity')}>Uds {marginSortCol === 'total_quantity' ? (marginSortAsc ? '↑' : '↓') : ''}</th>
              <th class="sortable-th text-right" onclick={() => toggleMarginSort('total_revenue')}>Ingresos {marginSortCol === 'total_revenue' ? (marginSortAsc ? '↑' : '↓') : ''}</th>
              <th class="sortable-th text-right" onclick={() => toggleMarginSort('total_cost')}>Costo {marginSortCol === 'total_cost' ? (marginSortAsc ? '↑' : '↓') : ''}</th>
              <th class="sortable-th text-right" onclick={() => toggleMarginSort('gross_profit')}>Utilidad {marginSortCol === 'gross_profit' ? (marginSortAsc ? '↑' : '↓') : ''}</th>
              <th class="sortable-th text-right" onclick={() => toggleMarginSort('margin_percent')}>Margen % {marginSortCol === 'margin_percent' ? (marginSortAsc ? '↑' : '↓') : ''}</th>
            </tr>
          </thead>
          <tbody>
            {#each sortedMarginProducts() as p}
              <tr>
                <td style="font-weight: 600;">{p.product_name}</td>
                <td class="text-right">{fmt(p.purchase_price)}</td>
                <td class="text-right">{fmt(p.avg_sale_price)}</td>
                <td class="text-right">{p.total_quantity.toFixed(0)}</td>
                <td class="text-right" style="font-weight: 600;">{fmt(p.total_revenue)}</td>
                <td class="text-right">{fmt(p.total_cost)}</td>
                <td class="text-right" style="font-weight: 700; color: {marginColor(p.margin_percent)};">{fmt(p.gross_profit)}</td>
                <td class="text-right">
                  <span class="badge {marginBadgeClass(p.margin_percent)}">{p.margin_percent.toFixed(1)}%</span>
                </td>
              </tr>
            {/each}
          </tbody>
          <tfoot>
            <tr style="font-weight: 700; border-top: 2px solid var(--border-primary);">
              <td>Total</td>
              <td></td>
              <td></td>
              <td></td>
              <td class="text-right">{fmt(marginTotalRevenue())}</td>
              <td class="text-right">{fmt(marginTotalCost())}</td>
              <td class="text-right" style="color: {marginColor(marginAvgPercent())};">{fmt(marginTotalProfit())}</td>
              <td class="text-right">
                <span class="badge {marginBadgeClass(marginAvgPercent())}">{marginAvgPercent().toFixed(1)}%</span>
              </td>
            </tr>
          </tfoot>
        </table>
      </div>
    {/if}
  </div>

  <!-- ─── Inventory Report ──────────────────────────────── -->
  <div class="card" style="margin-bottom: var(--space-2xl);">
    <div class="top-header">
      <h3 style="font-weight: 700;">📦 Reporte de Inventario</h3>
      <div class="top-controls">
        <select class="select select-sm" onchange={(e) => {
          const v = (e.target as HTMLSelectElement).value;
          invInactiveDays = v === '' ? undefined : parseInt(v);
        }}>
          <option value="">Todos los productos</option>
          <option value="30">Sin movimiento 30+ días</option>
          <option value="60">Sin movimiento 60+ días</option>
          <option value="90">Sin movimiento 90+ días</option>
        </select>
        {#if invItems.length > 0}
          <div class="export-btns">
            <button class="btn btn-sm btn-ghost" onclick={exportInventoryCsv} disabled={exporting} title="Exportar a CSV">📥 CSV</button>
            <button class="btn btn-sm btn-ghost" onclick={exportInventoryPdf} disabled={exporting} title="Exportar a PDF">📄 PDF</button>
          </div>
        {/if}
      </div>
    </div>

    {#if invLoading}
      <div class="top-empty">
        <span class="text-muted">Cargando...</span>
      </div>
    {:else if invItems.length === 0}
      <div class="top-empty">
        <span style="font-size: 2rem;">📦</span>
        <span class="text-muted">No hay productos con los filtros seleccionados</span>
      </div>
    {:else}
      <!-- Summary cards -->
      <div class="card-grid card-grid-4" style="margin-bottom: var(--space-lg);">
        <div class="stat-card">
          <div class="stat-icon blue">📋</div>
          <div class="stat-content">
            <div class="stat-value">{invItems.length}</div>
            <div class="stat-label">Productos</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon red">💰</div>
          <div class="stat-content">
            <div class="stat-value">{fmt(invTotalCostValue())}</div>
            <div class="stat-label">Valor a costo</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon green">🏷️</div>
          <div class="stat-content">
            <div class="stat-value">{fmt(invTotalSaleValue())}</div>
            <div class="stat-label">Valor a venta</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon yellow">⏳</div>
          <div class="stat-content">
            <div class="stat-value">{invInactiveCount()}</div>
            <div class="stat-label">Sin movimiento 30+ d</div>
          </div>
        </div>
      </div>

      <!-- Table -->
      <div class="table-container" style="border: none;">
        <table>
          <thead>
            <tr>
              <th class="sortable-th" onclick={() => toggleInvSort('product_name')}>Producto {invSortCol === 'product_name' ? (invSortAsc ? '↑' : '↓') : ''}</th>
              <th class="sortable-th" onclick={() => toggleInvSort('sku')}>SKU {invSortCol === 'sku' ? (invSortAsc ? '↑' : '↓') : ''}</th>
              <th class="sortable-th" onclick={() => toggleInvSort('category_name')}>Categoría {invSortCol === 'category_name' ? (invSortAsc ? '↑' : '↓') : ''}</th>
              <th class="sortable-th text-right" onclick={() => toggleInvSort('current_stock')}>Stock {invSortCol === 'current_stock' ? (invSortAsc ? '↑' : '↓') : ''}</th>
              <th class="sortable-th text-right" onclick={() => toggleInvSort('purchase_price')}>P. Compra {invSortCol === 'purchase_price' ? (invSortAsc ? '↑' : '↓') : ''}</th>
              <th class="sortable-th text-right" onclick={() => toggleInvSort('sale_price')}>P. Venta {invSortCol === 'sale_price' ? (invSortAsc ? '↑' : '↓') : ''}</th>
              <th class="sortable-th text-right" onclick={() => toggleInvSort('stock_cost_value')}>Valor Costo {invSortCol === 'stock_cost_value' ? (invSortAsc ? '↑' : '↓') : ''}</th>
              <th class="sortable-th text-right" onclick={() => toggleInvSort('stock_sale_value')}>Valor Venta {invSortCol === 'stock_sale_value' ? (invSortAsc ? '↑' : '↓') : ''}</th>
              <th class="sortable-th text-right" onclick={() => toggleInvSort('days_without_movement')}>Inactividad {invSortCol === 'days_without_movement' ? (invSortAsc ? '↑' : '↓') : ''}</th>
            </tr>
          </thead>
          <tbody>
            {#each sortedInvItems() as item}
              <tr>
                <td style="font-weight: 600;">{item.product_name}</td>
                <td class="text-muted">{item.sku}</td>
                <td>{item.category_name || '—'}</td>
                <td class="text-right">{item.current_stock.toFixed(0)}</td>
                <td class="text-right">{fmt(item.purchase_price)}</td>
                <td class="text-right">{fmt(item.sale_price)}</td>
                <td class="text-right" style="font-weight: 600;">{fmt(item.stock_cost_value)}</td>
                <td class="text-right" style="font-weight: 600;">{fmt(item.stock_sale_value)}</td>
                <td class="text-right">
                  {#if item.days_without_movement !== null}
                    <span class="badge {inactiveBadgeClass(item.days_without_movement)}">{item.days_without_movement}d</span>
                  {:else}
                    <span class="badge badge-muted">Sin mov.</span>
                  {/if}
                </td>
              </tr>
            {/each}
          </tbody>
          <tfoot>
            <tr style="font-weight: 700; border-top: 2px solid var(--border-primary);">
              <td>Total</td>
              <td></td>
              <td></td>
              <td class="text-right">{invTotalStock().toFixed(0)}</td>
              <td></td>
              <td></td>
              <td class="text-right">{fmt(invTotalCostValue())}</td>
              <td class="text-right">{fmt(invTotalSaleValue())}</td>
              <td></td>
            </tr>
          </tfoot>
        </table>
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
  /* ─── Chart styles ──────────────────────────────────── */
  .chart-container {
    width: 100%;
    overflow-x: auto;
  }
  .chart-svg {
    width: 100%;
    height: auto;
    min-height: 280px;
  }
  .chart-grid {
    stroke: var(--border-primary);
    stroke-width: 0.5;
    stroke-dasharray: 4, 4;
  }
  .chart-label-y {
    fill: var(--text-muted);
    font-size: 11px;
    text-anchor: end;
    font-family: 'Inter', sans-serif;
  }
  .chart-label-x {
    fill: var(--text-muted);
    font-size: 10px;
    text-anchor: end;
    font-family: 'Inter', sans-serif;
  }
  .chart-bar {
    transition: opacity 0.15s;
    cursor: pointer;
  }
  .chart-bar:hover {
    opacity: 0.9;
  }
  .chart-trend {
    fill: none;
    stroke: #10b981;
    stroke-width: 2;
    stroke-linecap: round;
    stroke-linejoin: round;
  }
  .chart-trend-dot {
    fill: #10b981;
    stroke: var(--bg-primary);
    stroke-width: 2;
    transition: r 0.15s;
  }
  .chart-tooltip-bg {
    fill: var(--bg-tertiary);
    stroke: var(--border-primary);
    stroke-width: 1;
    filter: drop-shadow(0 2px 4px rgba(0,0,0,0.3));
  }
  .chart-tooltip-text {
    fill: var(--text-primary);
    font-size: 12px;
    font-weight: 700;
    text-anchor: middle;
    font-family: 'Inter', sans-serif;
  }
  .chart-tooltip-sub {
    fill: var(--text-muted);
    font-size: 10px;
    text-anchor: middle;
    font-family: 'Inter', sans-serif;
  }

  .chart-summary {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: var(--space-md);
    margin-top: var(--space-lg);
    padding-top: var(--space-lg);
    border-top: 1px solid var(--border-primary);
  }
  .chart-stat {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
  }
  .chart-stat-value {
    font-weight: 700;
    font-size: var(--font-size-md);
    color: var(--text-primary);
  }
  .chart-stat-label {
    font-size: var(--font-size-xs);
    color: var(--text-muted);
  }

  /* ─── Top Products styles ───────────────────────────── */
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

  /* ─── Margin table ────────────────────────────────────── */
  .sortable-th {
    cursor: pointer;
    user-select: none;
    white-space: nowrap;
    transition: color 0.15s;
  }
  .sortable-th:hover {
    color: var(--accent-primary);
  }
  .text-right {
    text-align: right;
  }

  /* ─── Export buttons ──────────────────────────────────── */
  .export-btns {
    display: flex;
    gap: 4px;
  }
</style>
