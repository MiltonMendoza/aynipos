/**
 * Receipt printing service.
 *
 * Generates an HTML receipt, saves it as a temp file via Rust,
 * and opens it in the system browser where the user can print
 * with Cmd+P or use the built-in print button.
 *
 * This approach works reliably with Tauri because we don't
 * depend on the webview's print capabilities.
 */

import type { Sale, SaleItem } from '$lib/types';
import { saveReceiptHtml } from '$lib/services/api';
import { openPath } from '@tauri-apps/plugin-opener';

export interface BusinessInfo {
  name: string;
  nit: string;
  address: string;
  phone: string;
  city: string;
}

/**
 * Extract business info from the settings key-value array.
 */
export function extractBusinessInfo(settings: { key: string; value: string }[]): BusinessInfo {
  const map = new Map(settings.map(s => [s.key, s.value]));
  return {
    name: map.get('business_name') || 'Mi Negocio',
    nit: map.get('business_nit') || '',
    address: map.get('business_address') || '',
    phone: map.get('business_phone') || '',
    city: map.get('business_city') || '',
  };
}

function formatCurrency(amount: number): string {
  return `Bs ${amount.toFixed(2)}`;
}

function formatDate(dateStr: string | null | undefined): string {
  if (!dateStr) {
    // Use current date for just-created sales
    return new Date().toLocaleDateString('es-BO', {
      day: '2-digit', month: '2-digit', year: 'numeric',
      hour: '2-digit', minute: '2-digit',
    });
  }
  const d = new Date(dateStr);
  return d.toLocaleDateString('es-BO', {
    day: '2-digit', month: '2-digit', year: 'numeric',
    hour: '2-digit', minute: '2-digit',
  });
}

function paymentMethodLabel(method: string): string {
  switch (method) {
    case 'efectivo': return 'Efectivo';
    case 'tarjeta': return 'Tarjeta';
    case 'qr': return 'QR';
    default: return method;
  }
}

/**
 * Build the full standalone HTML document for the receipt.
 * Includes its own styles, a print button, and auto-print script.
 */
function buildReceiptHTML(sale: Sale, items: SaleItem[], biz: BusinessInfo): string {
  const itemRows = items.map(item => {
    const hasDiscount = item.discount > 0;
    return `
      <tr>
        <td class="item-name">${item.product_name}</td>
        <td class="item-qty">${item.quantity}</td>
        <td class="item-price">${formatCurrency(item.unit_price)}</td>
        <td class="item-total">${formatCurrency(item.total)}</td>
      </tr>
      ${hasDiscount ? `
      <tr class="discount-row">
        <td colspan="3" class="discount-label">Desc. ítem</td>
        <td class="discount-amount">-${formatCurrency(item.discount)}</td>
      </tr>` : ''}
    `;
  }).join('');

  const totalItemDiscounts = items.reduce((sum, i) => sum + i.discount, 0);

  return `<!DOCTYPE html>
<html lang="es">
<head>
  <meta charset="UTF-8">
  <title>Recibo #${sale.sale_number} — ${biz.name}</title>
  <style>
    *, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }

    body {
      font-family: 'Courier New', Courier, monospace;
      font-size: 12px;
      line-height: 1.4;
      color: #000;
      background: #f5f5f5;
      margin: 0;
      padding: 20px;
    }

    .receipt {
      width: 80mm;
      margin: 0 auto;
      padding: 12px;
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

    .divider { border: none; border-top: 1px dashed #000; margin: 6px 0; }
    .divider-double { border: none; border-top: 2px solid #000; margin: 6px 0; }

    .sale-info { font-size: 11px; margin-bottom: 4px; }
    .sale-info .row { display: flex; justify-content: space-between; }
    .sale-info .label { color: #555; }

    .items-table { width: 100%; border-collapse: collapse; margin: 4px 0; }
    .items-table th {
      text-align: left; font-size: 10px; font-weight: 700;
      text-transform: uppercase; border-bottom: 1px solid #000; padding: 2px 0;
    }
    .items-table th:nth-child(2),
    .items-table th:nth-child(3),
    .items-table th:nth-child(4) { text-align: right; }
    .items-table td { padding: 2px 0; font-size: 11px; }
    .item-name { max-width: 120px; word-wrap: break-word; }
    .item-qty, .item-price, .item-total { text-align: right; }
    .discount-row td { font-size: 10px; color: #666; font-style: italic; }
    .discount-label { text-align: right; padding-right: 4px; }
    .discount-amount { text-align: right; }

    .totals { font-size: 11px; margin: 4px 0; }
    .totals .row { display: flex; justify-content: space-between; padding: 1px 0; }
    .totals .row.grand-total { font-size: 16px; font-weight: 900; padding: 4px 0; }
    .totals .discount-val { color: #333; }

    .payment-info { font-size: 11px; margin: 4px 0; }
    .notes { font-size: 10px; color: #555; font-style: italic; margin: 4px 0; word-wrap: break-word; }

    .footer { text-align: center; font-size: 11px; margin-top: 8px; }
    .footer .thanks { font-weight: 700; font-size: 12px; margin-bottom: 2px; }
    .footer .disclaimer { font-size: 9px; color: #777; margin-top: 4px; }

    /* Print actions — visible on screen only */
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
      .receipt { border: none; box-shadow: none; border-radius: 0; padding: 0; margin: 0; }
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
    <button onclick="window.print()">🖨️ Imprimir Recibo</button>
    <div class="hint">También puedes usar Ctrl+P / Cmd+P</div>
  </div>

  <div class="receipt">
    <div class="header">
      <div class="biz-name">${biz.name}</div>
      ${biz.nit ? `<div class="biz-detail">NIT: ${biz.nit}</div>` : ''}
      ${biz.address ? `<div class="biz-detail">${biz.address}</div>` : ''}
      ${biz.phone ? `<div class="biz-detail">Tel: ${biz.phone}</div>` : ''}
      ${biz.city ? `<div class="biz-detail">${biz.city}, Bolivia</div>` : ''}
    </div>

    <hr class="divider-double">

    <div class="sale-info">
      <div class="row">
        <span class="label">Venta #:</span>
        <span><strong>${sale.sale_number}</strong></span>
      </div>
      <div class="row">
        <span class="label">Fecha:</span>
        <span>${formatDate(sale.created_at)}</span>
      </div>
      ${sale.customer_name && sale.customer_name !== 'Sin Nombre' ? `
      <div class="row">
        <span class="label">Cliente:</span>
        <span>${sale.customer_name}</span>
      </div>` : ''}
    </div>

    <hr class="divider">

    <table class="items-table">
      <thead>
        <tr>
          <th>Producto</th>
          <th>Cant</th>
          <th>P.Unit</th>
          <th>Total</th>
        </tr>
      </thead>
      <tbody>
        ${itemRows}
      </tbody>
    </table>

    <hr class="divider">

    <div class="totals">
      <div class="row">
        <span>Subtotal</span>
        <span>${formatCurrency(sale.subtotal)}</span>
      </div>
      ${totalItemDiscounts > 0 ? `
      <div class="row">
        <span>Desc. por ítems</span>
        <span class="discount-val">-${formatCurrency(totalItemDiscounts)}</span>
      </div>` : ''}
      ${sale.discount_amount > 0 ? `
      <div class="row">
        <span>Desc. global</span>
        <span class="discount-val">-${formatCurrency(sale.discount_amount)}</span>
      </div>` : ''}
      ${sale.tax_amount > 0 ? `
      <div class="row">
        <span>Débito Fiscal</span>
        <span>${formatCurrency(sale.tax_amount)}</span>
      </div>` : ''}
      <hr class="divider-double">
      <div class="row grand-total">
        <span>TOTAL</span>
        <span>${formatCurrency(sale.total)}</span>
      </div>
    </div>

    <hr class="divider">

    <div class="payment-info">
      <div class="row">
        <span>Método de pago:</span>
        <span><strong>${paymentMethodLabel(sale.payment_method)}</strong></span>
      </div>
    </div>

    ${sale.notes ? `<div class="notes">📝 ${sale.notes}</div>` : ''}

    <hr class="divider">

    <div class="footer">
      <div class="thanks">¡Gracias por su compra!</div>
      <div class="disclaimer">Este recibo no es una factura.</div>
    </div>
  </div>

</body>
</html>`;
}

/**
 * Generate receipt HTML, save to temp file, and open in system browser.
 * The browser provides reliable printing via Cmd+P or the print button.
 */
export async function printReceipt(sale: Sale, items: SaleItem[], businessInfo: BusinessInfo): Promise<void> {
  const html = buildReceiptHTML(sale, items, businessInfo);

  // Save HTML to temp file via Rust command
  const filePath = await saveReceiptHtml(html);

  // Open in system default browser
  await openPath(filePath);
}
