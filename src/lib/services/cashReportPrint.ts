/**
 * Cash Register Closing Report — Print Service
 *
 * Generates an HTML report for a cash register closing session,
 * saves it as a temp file via Rust, and opens it in the system browser.
 * Same pattern as receipt.ts.
 */

import type { CashRegisterReport } from '$lib/types';
import type { BusinessInfo } from '$lib/services/receipt';
import { saveReceiptHtml } from '$lib/services/api';
import { openPath } from '@tauri-apps/plugin-opener';

function formatCurrency(amount: number): string {
  return `Bs ${amount.toFixed(2)}`;
}

function formatDate(dateStr: string | null | undefined): string {
  if (!dateStr) return '—';
  const d = new Date(dateStr);
  return d.toLocaleDateString('es-BO', {
    day: '2-digit', month: '2-digit', year: 'numeric',
    hour: '2-digit', minute: '2-digit',
  });
}

function buildCashReportHTML(report: CashRegisterReport, biz: BusinessInfo): string {
  const r = report.register;
  const diffClass = report.difference >= 0 ? 'positive' : 'negative';
  const diffSign = report.difference >= 0 ? '+' : '';

  // Payment method rows — only show methods with transactions
  const paymentRows: string[] = [];
  if (report.count_cash > 0) {
    paymentRows.push(`<tr><td>💵 Efectivo</td><td class="num">${report.count_cash}</td><td class="num">${formatCurrency(report.sales_cash)}</td></tr>`);
  }
  if (report.count_card > 0) {
    paymentRows.push(`<tr><td>💳 Tarjeta</td><td class="num">${report.count_card}</td><td class="num">${formatCurrency(report.sales_card)}</td></tr>`);
  }
  if (report.count_qr > 0) {
    paymentRows.push(`<tr><td>📱 QR</td><td class="num">${report.count_qr}</td><td class="num">${formatCurrency(report.sales_qr)}</td></tr>`);
  }
  if (report.count_mixed > 0) {
    paymentRows.push(`<tr><td>🔀 Mixto</td><td class="num">${report.count_mixed}</td><td class="num">${formatCurrency(report.sales_mixed)}</td></tr>`);
  }
  if (paymentRows.length === 0) {
    paymentRows.push(`<tr><td colspan="3" class="empty">Sin ventas en este turno</td></tr>`);
  }

  return `<!DOCTYPE html>
<html lang="es">
<head>
  <meta charset="UTF-8">
  <title>Cierre de Caja — ${biz.name}</title>
  <style>
    *, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }

    body {
      font-family: 'Courier New', Courier, monospace;
      font-size: 12px;
      line-height: 1.5;
      color: #000;
      background: #f5f5f5;
      margin: 0;
      padding: 20px;
    }

    .report {
      width: 80mm;
      margin: 0 auto;
      padding: 14px;
      background: #fff;
      border: 1px solid #ddd;
      border-radius: 6px;
      box-shadow: 0 2px 12px rgba(0,0,0,0.08);
    }

    .header { text-align: center; margin-bottom: 8px; }
    .header .biz-name {
      font-size: 16px;
      font-weight: 900;
      text-transform: uppercase;
      letter-spacing: 1px;
    }
    .header .biz-detail { font-size: 11px; color: #333; margin-top: 2px; }
    .header .report-title {
      font-size: 14px;
      font-weight: 700;
      margin-top: 8px;
      padding: 4px 8px;
      background: #000;
      color: #fff;
      display: inline-block;
      letter-spacing: 1px;
    }

    .divider { border: none; border-top: 1px dashed #000; margin: 8px 0; }
    .divider-double { border: none; border-top: 2px solid #000; margin: 8px 0; }

    .section-title {
      font-size: 11px;
      font-weight: 700;
      text-transform: uppercase;
      letter-spacing: 0.5px;
      margin-bottom: 4px;
    }

    .info-row { display: flex; justify-content: space-between; font-size: 11px; padding: 1px 0; }
    .info-row .label { color: #555; }

    .payment-table { width: 100%; border-collapse: collapse; margin: 4px 0; }
    .payment-table th {
      text-align: left;
      font-size: 10px;
      font-weight: 700;
      text-transform: uppercase;
      border-bottom: 1px solid #000;
      padding: 3px 0;
    }
    .payment-table th.num { text-align: right; }
    .payment-table td { padding: 3px 0; font-size: 11px; }
    .payment-table td.num { text-align: right; font-weight: 600; }
    .payment-table td.empty { text-align: center; color: #999; font-style: italic; padding: 8px 0; }
    .payment-table tfoot td { border-top: 1px solid #000; font-weight: 900; font-size: 12px; padding-top: 4px; }

    .totals .row { display: flex; justify-content: space-between; font-size: 11px; padding: 2px 0; }
    .totals .row.grand { font-size: 14px; font-weight: 900; padding: 4px 0; }

    .reconciliation { margin: 4px 0; }
    .reconciliation .row { display: flex; justify-content: space-between; font-size: 11px; padding: 2px 0; }
    .reconciliation .diff { font-size: 14px; font-weight: 900; padding: 6px 0; }
    .reconciliation .positive { color: #16a34a; }
    .reconciliation .negative { color: #dc2626; }

    .notes { font-size: 10px; color: #555; font-style: italic; margin: 4px 0; word-wrap: break-word; }

    .footer { text-align: center; font-size: 10px; color: #777; margin-top: 8px; }

    .print-actions {
      text-align: center;
      margin: 20px auto;
      width: 80mm;
    }
    .print-actions button {
      background: #3b82f6;
      color: #fff;
      border: none;
      padding: 12px 32px;
      border-radius: 8px;
      font-size: 15px;
      font-weight: 600;
      cursor: pointer;
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
      transition: background 0.2s;
    }
    .print-actions button:hover { background: #2563eb; }
    .print-actions .hint {
      font-size: 12px;
      color: #888;
      margin-top: 8px;
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
    }

    @media print {
      body { background: #fff; padding: 0; margin: 0; }
      .report { border: none; box-shadow: none; border-radius: 0; padding: 0; margin: 0; }
      .print-actions { display: none !important; }
      @page {
        size: 80mm auto;
        margin: 4mm;
      }
    }
  </style>
</head>
<body>
  <div class="print-actions">
    <button onclick="window.print()">🖨️ Imprimir Reporte</button>
    <div class="hint">También puedes usar Ctrl+P / Cmd+P</div>
  </div>

  <div class="report">
    <div class="header">
      <div class="biz-name">${biz.name}</div>
      ${biz.nit ? `<div class="biz-detail">NIT: ${biz.nit}</div>` : ''}
      ${biz.address ? `<div class="biz-detail">${biz.address}</div>` : ''}
      ${biz.phone ? `<div class="biz-detail">Tel: ${biz.phone}</div>` : ''}
      ${biz.city ? `<div class="biz-detail">${biz.city}, Bolivia</div>` : ''}
      <div class="report-title">CIERRE DE CAJA</div>
    </div>

    <hr class="divider-double">

    <div class="section-title">📅 Período</div>
    <div class="info-row">
      <span class="label">Apertura:</span>
      <span>${formatDate(r.opened_at)}</span>
    </div>
    <div class="info-row">
      <span class="label">Cierre:</span>
      <span>${formatDate(r.closed_at)}</span>
    </div>

    <hr class="divider">

    <div class="section-title">💳 Ventas por Método de Pago</div>
    <table class="payment-table">
      <thead>
        <tr>
          <th>Método</th>
          <th class="num"># Ventas</th>
          <th class="num">Total</th>
        </tr>
      </thead>
      <tbody>
        ${paymentRows.join('\n        ')}
      </tbody>
      <tfoot>
        <tr>
          <td>TOTAL</td>
          <td class="num">${report.total_transactions}</td>
          <td class="num">${formatCurrency(report.total_sales)}</td>
        </tr>
      </tfoot>
    </table>

    ${report.cancelled_transactions > 0 ? `
    <div class="info-row" style="margin-top: 4px;">
      <span class="label">❌ Ventas anuladas:</span>
      <span>${report.cancelled_transactions}</span>
    </div>
    ` : ''}

    <hr class="divider">

    <div class="section-title">📊 Resumen</div>
    <div class="totals">
      ${report.total_discount > 0 ? `
      <div class="row">
        <span>Descuentos</span>
        <span>-${formatCurrency(report.total_discount)}</span>
      </div>` : ''}
      <div class="row">
        <span>IVA incluido</span>
        <span>${formatCurrency(report.total_tax)}</span>
      </div>
      <hr class="divider-double">
      <div class="row grand">
        <span>TOTAL VENTAS</span>
        <span>${formatCurrency(report.total_sales)}</span>
      </div>
    </div>

    <hr class="divider">

    <div class="section-title">💰 Reconciliación</div>
    <div class="reconciliation">
      <div class="row">
        <span>Monto apertura</span>
        <span>${formatCurrency(r.opening_amount)}</span>
      </div>
      <div class="row">
        <span>Monto esperado</span>
        <span>${formatCurrency(r.expected_amount ?? 0)}</span>
      </div>
      <div class="row">
        <span>Monto real</span>
        <span>${formatCurrency(r.closing_amount ?? 0)}</span>
      </div>
      <hr class="divider-double">
      <div class="diff ${diffClass}">
        <div class="row">
          <span>DIFERENCIA</span>
          <span>${diffSign}${formatCurrency(report.difference)}</span>
        </div>
      </div>
    </div>

    ${r.notes ? `
    <hr class="divider">
    <div class="notes">📝 ${r.notes}</div>
    ` : ''}

    <hr class="divider">

    <div class="footer">
      Reporte generado el ${formatDate(new Date().toISOString())}
    </div>
  </div>

</body>
</html>`;
}

/**
 * Generate cash register report HTML, save to temp file, and open in system browser.
 */
export async function printCashReport(report: CashRegisterReport, businessInfo: BusinessInfo): Promise<void> {
  const html = buildCashReportHTML(report, businessInfo);
  const filePath = await saveReceiptHtml(html);
  await openPath(filePath);
}
