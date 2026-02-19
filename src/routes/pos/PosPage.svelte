<script lang="ts">
  import { onMount } from 'svelte';
  import type { ProductWithStock, CartItem } from '$lib/types';
  import { getProducts, createSale, getCurrentCashRegister, getDashboardStats } from '$lib/services/api';

  let products: ProductWithStock[] = $state([]);
  let cart: CartItem[] = $state([]);
  let searchQuery = $state('');
  let cashRegisterOpen = $state(false);
  let showPaymentModal = $state(false);
  let paymentMethod = $state('efectivo');
  let cashReceived = $state(0);
  let toast = $state({ show: false, message: '', type: 'success' as 'success' | 'error' });

  // Dashboard quick stats
  let stats = $state({ total_sales_today: 0, total_transactions_today: 0, total_products: 0, low_stock_count: 0, expiring_soon_count: 0 });

  $effect(() => {
    loadProducts(searchQuery);
  });

  onMount(async () => {
    try {
      const cr = await getCurrentCashRegister();
      cashRegisterOpen = cr !== null;
      stats = await getDashboardStats();
    } catch { /* first run, no data */ }
    await loadProducts('');
  });

  async function loadProducts(search: string) {
    try {
      products = await getProducts(search || undefined);
    } catch { products = []; }
  }

  function addToCart(ps: ProductWithStock) {
    const existing = cart.find(c => c.product.product.id === ps.product.id);
    if (existing) {
      existing.quantity += 1;
      existing.subtotal = existing.quantity * existing.product.product.sale_price - existing.discount;
      cart = [...cart];
    } else {
      cart = [...cart, {
        product: ps,
        quantity: 1,
        discount: 0,
        subtotal: ps.product.sale_price
      }];
    }
  }

  function removeFromCart(index: number) {
    cart = cart.filter((_, i) => i !== index);
  }

  function updateQuantity(index: number, qty: number) {
    if (qty <= 0) {
      removeFromCart(index);
      return;
    }
    cart[index].quantity = qty;
    cart[index].subtotal = qty * cart[index].product.product.sale_price - cart[index].discount;
    cart = [...cart];
  }

  function cartSubtotal(): number {
    return cart.reduce((sum, item) => sum + item.subtotal, 0);
  }

  function cartTax(): number {
    return cart.reduce((sum, item) => sum + (item.subtotal * item.product.product.tax_rate), 0);
  }

  function cartTotal(): number {
    return cartSubtotal();
  }

  function change(): number {
    return Math.max(0, cashReceived - cartTotal());
  }

  function openPayment() {
    if (cart.length === 0) return;
    cashReceived = cartTotal();
    showPaymentModal = true;
  }

  async function completeSale() {
    try {
      await createSale({
        items: cart.map(c => ({
          product_id: c.product.product.id,
          quantity: c.quantity,
          discount: c.discount > 0 ? c.discount : undefined,
        })),
        payment_method: paymentMethod,
      });
      showToast('✅ Venta completada exitosamente', 'success');
      cart = [];
      showPaymentModal = false;
      stats = await getDashboardStats();
      await loadProducts(searchQuery);
    } catch (e) {
      showToast(`❌ Error: ${e}`, 'error');
    }
  }

  function clearCart() {
    cart = [];
  }

  function showToast(message: string, type: 'success' | 'error') {
    toast = { show: true, message, type };
    setTimeout(() => { toast.show = false; }, 3000);
  }

  function formatCurrency(amount: number): string {
    return `Bs ${amount.toFixed(2)}`;
  }
</script>

<div style="display: flex; height: 100vh; overflow: hidden;">
  <!-- Left: Product Search -->
  <div style="flex: 1; display: flex; flex-direction: column; border-right: 1px solid var(--border-color);">
    <!-- Search bar -->
    <div style="padding: var(--space-lg); border-bottom: 1px solid var(--border-color); background: var(--bg-secondary);">
      <div class="flex items-center gap-md">
        <div style="position: relative; flex: 1;">
          <input
            class="input input-lg"
            placeholder="🔍 Buscar producto por nombre, SKU o código de barras..."
            bind:value={searchQuery}
            autofocus
          />
        </div>
        {#if !cashRegisterOpen}
          <span class="badge badge-warning">⚠️ Caja cerrada</span>
        {/if}
      </div>

      <!-- Quick stats -->
      <div class="flex gap-lg" style="margin-top: var(--space-md);">
        <span class="text-sm text-muted">📊 Hoy: <strong style="color: var(--accent-success)">{formatCurrency(stats.total_sales_today)}</strong></span>
        <span class="text-sm text-muted">🧾 {stats.total_transactions_today} ventas</span>
        {#if stats.low_stock_count > 0}
          <span class="badge badge-danger">{stats.low_stock_count} bajo stock</span>
        {/if}
      </div>
    </div>

    <!-- Products grid -->
    <div style="flex: 1; overflow-y: auto; padding: var(--space-lg);">
      {#if products.length === 0}
        <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%; gap: var(--space-lg); color: var(--text-muted);">
          <div style="font-size: 3rem; opacity: 0.5;">📦</div>
          <p>No se encontraron productos</p>
          <p class="text-sm">Agrega productos desde el menú de Inventario</p>
        </div>
      {:else}
        <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(180px, 1fr)); gap: var(--space-md);">
          {#each products as ps}
            <button
              class="product-card"
              onclick={() => addToCart(ps)}
              style="
                background: var(--bg-secondary);
                border: 1px solid var(--border-color);
                border-radius: var(--radius-lg);
                padding: var(--space-lg);
                cursor: pointer;
                text-align: left;
                transition: all var(--transition-fast);
                display: flex;
                flex-direction: column;
                gap: var(--space-sm);
                color: var(--text-primary);
                font-family: var(--font-family);
              "
              onmouseenter={(e) => { (e.currentTarget as HTMLElement).style.borderColor = 'var(--accent-primary)'; (e.currentTarget as HTMLElement).style.transform = 'translateY(-2px)'; (e.currentTarget as HTMLElement).style.boxShadow = 'var(--shadow-glow-blue)'; }}
              onmouseleave={(e) => { (e.currentTarget as HTMLElement).style.borderColor = 'var(--border-color)'; (e.currentTarget as HTMLElement).style.transform = 'none'; (e.currentTarget as HTMLElement).style.boxShadow = 'none'; }}
            >
              <div class="flex items-center justify-between">
                <span class="text-xs text-muted">{ps.product.sku}</span>
                {#if ps.current_stock <= ps.product.min_stock && ps.product.min_stock > 0}
                  <span class="badge badge-danger">Bajo</span>
                {/if}
              </div>
              <div style="font-weight: 600; font-size: var(--font-size-base); line-height: 1.3;" class="truncate">
                {ps.product.name}
              </div>
              {#if ps.category_name}
                <div class="text-xs text-muted">{ps.category_name}</div>
              {/if}
              <div class="flex items-center justify-between" style="margin-top: auto;">
                <span style="font-weight: 800; color: var(--accent-primary); font-size: var(--font-size-md);">
                  {formatCurrency(ps.product.sale_price)}
                </span>
                <span class="text-xs text-muted">
                  Stock: {ps.current_stock}
                </span>
              </div>
            </button>
          {/each}
        </div>
      {/if}
    </div>
  </div>

  <!-- Right: Cart -->
  <div style="width: 380px; min-width: 380px; display: flex; flex-direction: column; background: var(--bg-secondary);">
    <!-- Cart header -->
    <div style="padding: var(--space-lg); border-bottom: 1px solid var(--border-color);">
      <div class="flex items-center justify-between">
        <h2 style="font-size: var(--font-size-lg); font-weight: 700;">
          🛒 Carrito
          {#if cart.length > 0}
            <span class="badge badge-info" style="margin-left: var(--space-sm);">{cart.length}</span>
          {/if}
        </h2>
        {#if cart.length > 0}
          <button class="btn btn-ghost btn-sm" onclick={clearCart}>Limpiar</button>
        {/if}
      </div>
    </div>

    <!-- Cart items -->
    <div style="flex: 1; overflow-y: auto; padding: var(--space-md);">
      {#if cart.length === 0}
        <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%; color: var(--text-muted); gap: var(--space-md);">
          <div style="font-size: 2.5rem; opacity: 0.4;">🛒</div>
          <p class="text-sm">El carrito está vacío</p>
          <p class="text-xs">Haz clic en un producto para agregarlo</p>
        </div>
      {:else}
        <div style="display: flex; flex-direction: column; gap: var(--space-sm);">
          {#each cart as item, index}
            <div style="
              background: var(--bg-tertiary);
              border-radius: var(--radius-md);
              padding: var(--space-md);
              display: flex;
              flex-direction: column;
              gap: var(--space-sm);
              animation: slideDown var(--transition-fast);
            ">
              <div class="flex items-center justify-between">
                <span style="font-weight: 600; font-size: var(--font-size-sm);" class="truncate">
                  {item.product.product.name}
                </span>
                <button
                  class="btn btn-ghost btn-sm"
                  style="width: 24px; height: 24px; padding: 0; font-size: var(--font-size-xs); border-radius: var(--radius-full);"
                  onclick={() => removeFromCart(index)}
                >
                  ✕
                </button>
              </div>
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-sm">
                  <button
                    class="btn btn-ghost btn-sm"
                    style="width: 28px; height: 28px; padding: 0;"
                    onclick={() => updateQuantity(index, item.quantity - 1)}
                  >−</button>
                  <input
                    type="number"
                    class="input"
                    style="width: 50px; text-align: center; padding: var(--space-xs) var(--space-sm); font-weight: 700;"
                    value={item.quantity}
                    onchange={(e) => updateQuantity(index, parseFloat((e.target as HTMLInputElement).value) || 1)}
                    min="1"
                  />
                  <button
                    class="btn btn-ghost btn-sm"
                    style="width: 28px; height: 28px; padding: 0;"
                    onclick={() => updateQuantity(index, item.quantity + 1)}
                  >+</button>
                </div>
                <span style="font-weight: 700; color: var(--accent-primary);">
                  {formatCurrency(item.subtotal)}
                </span>
              </div>
              <div class="text-xs text-muted">
                {formatCurrency(item.product.product.sale_price)} × {item.quantity}
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>

    <!-- Cart totals & pay -->
    <div style="padding: var(--space-lg); border-top: 1px solid var(--border-color); background: var(--bg-tertiary);">
      <div style="display: flex; flex-direction: column; gap: var(--space-sm); margin-bottom: var(--space-lg);">
        <div class="flex justify-between text-sm">
          <span class="text-muted">Subtotal</span>
          <span>{formatCurrency(cartSubtotal())}</span>
        </div>
        <div class="flex justify-between text-sm">
          <span class="text-muted">IVA (13%)</span>
          <span>{formatCurrency(cartTax())}</span>
        </div>
        <div style="height: 1px; background: var(--border-color); margin: var(--space-xs) 0;"></div>
        <div class="flex justify-between" style="font-size: var(--font-size-xl); font-weight: 800;">
          <span>Total</span>
          <span style="color: var(--accent-success);">{formatCurrency(cartTotal())}</span>
        </div>
      </div>

      <button
        class="btn btn-success btn-xl btn-block"
        onclick={openPayment}
        disabled={cart.length === 0}
        style="position: relative; overflow: hidden;"
      >
        💰 Cobrar {formatCurrency(cartTotal())}
      </button>
    </div>
  </div>
</div>

<!-- Payment Modal -->
{#if showPaymentModal}
  <div class="modal-overlay" onclick={() => showPaymentModal = false}>
    <div class="modal" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h3 class="modal-title">💳 Método de Pago</h3>
        <button class="btn btn-ghost btn-sm" onclick={() => showPaymentModal = false}>✕</button>
      </div>

      <div class="modal-body">
        <div style="display: flex; gap: var(--space-md);">
          {#each [
            { value: 'efectivo', icon: '💵', label: 'Efectivo' },
            { value: 'tarjeta', icon: '💳', label: 'Tarjeta' },
            { value: 'qr', icon: '📱', label: 'QR' }
          ] as method}
            <button
              class="btn"
              class:btn-primary={paymentMethod === method.value}
              class:btn-ghost={paymentMethod !== method.value}
              style="flex: 1; flex-direction: column; padding: var(--space-lg); gap: var(--space-sm);"
              onclick={() => paymentMethod = method.value}
            >
              <span style="font-size: 1.5rem;">{method.icon}</span>
              <span>{method.label}</span>
            </button>
          {/each}
        </div>

        <div style="background: var(--bg-tertiary); border-radius: var(--radius-lg); padding: var(--space-xl); text-align: center;">
          <div class="text-sm text-muted" style="margin-bottom: var(--space-sm);">Total a cobrar</div>
          <div style="font-size: var(--font-size-3xl); font-weight: 800; color: var(--accent-success);">
            {formatCurrency(cartTotal())}
          </div>
        </div>

        {#if paymentMethod === 'efectivo'}
          <div class="input-group">
            <label class="input-label">Efectivo recibido</label>
            <input
              class="input input-lg"
              type="number"
              bind:value={cashReceived}
              min={cartTotal()}
              step="0.5"
              style="font-size: var(--font-size-xl); font-weight: 700; text-align: center;"
            />
          </div>

          {#if change() > 0}
            <div style="background: var(--accent-success-glow); border-radius: var(--radius-lg); padding: var(--space-lg); text-align: center;">
              <div class="text-sm" style="color: var(--accent-success); margin-bottom: var(--space-xs);">Cambio</div>
              <div style="font-size: var(--font-size-2xl); font-weight: 800; color: var(--accent-success);">
                {formatCurrency(change())}
              </div>
            </div>
          {/if}
        {/if}
      </div>

      <div class="modal-footer">
        <button class="btn btn-ghost" onclick={() => showPaymentModal = false}>Cancelar</button>
        <button
          class="btn btn-success btn-lg"
          onclick={completeSale}
          disabled={paymentMethod === 'efectivo' && cashReceived < cartTotal()}
        >
          ✅ Confirmar Venta
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Toast Notification -->
{#if toast.show}
  <div class="toast" class:toast-success={toast.type === 'success'} class:toast-error={toast.type === 'error'}>
    {toast.message}
  </div>
{/if}
