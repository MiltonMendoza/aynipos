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
  let toast = $state({ show: false, message: '', type: 'success' as 'success' | 'error' | 'warning' });
  let paymentErrors: Record<string, string> = $state({});
  let searchInputRef: HTMLInputElement | undefined = $state(undefined);
  let f4PendingConfirm = $state(false);

  // Discounts
  let editingItemDiscount: number | null = $state(null);
  let itemDiscountType: 'percent' | 'fixed' = $state('percent');
  let itemDiscountInput: number = $state(0);
  let globalDiscountType: 'percent' | 'fixed' = $state('fixed');
  let globalDiscountInput: number = $state(0);
  let showGlobalDiscount = $state(false);

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

  // ─── Keyboard Shortcuts ───
  function handleKeydown(e: KeyboardEvent) {
    const tag = (e.target as HTMLElement)?.tagName;
    const isInput = tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT';

    // Escape — close any open modal
    if (e.key === 'Escape') {
      if (showPaymentModal) { showPaymentModal = false; e.preventDefault(); }
      return;
    }

    // F1 — focus search
    if (e.key === 'F1') {
      e.preventDefault();
      searchInputRef?.focus();
      searchInputRef?.select();
      return;
    }

    // F2 — open payment / cobrar
    if (e.key === 'F2') {
      e.preventDefault();
      openPayment();
      return;
    }

    // Enter — confirm sale when payment modal is open
    if (e.key === 'Enter' && showPaymentModal) {
      e.preventDefault();
      completeSale();
      return;
    }

    // F4 — clear cart (double-press to confirm)
    if (e.key === 'F4') {
      e.preventDefault();
      if (cart.length > 0) {
        if (f4PendingConfirm) {
          clearCart();
          f4PendingConfirm = false;
          showToast('🗑️ Carrito limpiado', 'success');
        } else {
          f4PendingConfirm = true;
          showToast('⚠️ Presiona F4 de nuevo para limpiar el carrito', 'warning');
          setTimeout(() => { f4PendingConfirm = false; }, 3000);
        }
      }
      return;
    }

    // +/- — adjust last item quantity (only when not in an input)
    if (!isInput && cart.length > 0) {
      const lastIdx = cart.length - 1;
      if (e.key === '+' || e.key === '=') {
        e.preventDefault();
        updateQuantity(lastIdx, cart[lastIdx].quantity + 1);
      } else if (e.key === '-') {
        e.preventDefault();
        updateQuantity(lastIdx, cart[lastIdx].quantity - 1);
      }
    }
  }

  function addToCart(ps: ProductWithStock) {
    // Validate: stock = 0
    if (ps.current_stock <= 0) {
      showToast(`❌ Sin stock disponible para "${ps.product.name}"`, 'error');
      return;
    }

    const existing = cart.find(c => c.product.product.id === ps.product.id);
    const currentQty = existing ? existing.quantity : 0;

    // Validate: would exceed available stock
    if (currentQty + 1 > ps.current_stock) {
      showToast(`⚠️ Solo hay ${ps.current_stock} unidades disponibles de "${ps.product.name}"`, 'warning');
      return;
    }

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

  function toggleItemDiscount(index: number) {
    if (editingItemDiscount === index) {
      editingItemDiscount = null;
    } else {
      editingItemDiscount = index;
      const item = cart[index];
      // Pre-fill with existing discount
      if (item.discount > 0) {
        itemDiscountType = 'fixed';
        itemDiscountInput = item.discount;
      } else {
        itemDiscountType = 'percent';
        itemDiscountInput = 0;
      }
    }
  }

  function applyItemDiscount(index: number) {
    const item = cart[index];
    const lineTotal = item.product.product.sale_price * item.quantity;
    let discountAmount = 0;

    if (itemDiscountType === 'percent') {
      const pct = Math.min(Math.max(itemDiscountInput, 0), 100);
      discountAmount = lineTotal * (pct / 100);
    } else {
      discountAmount = Math.min(Math.max(itemDiscountInput, 0), lineTotal);
    }

    cart[index].discount = Math.round(discountAmount * 100) / 100;
    cart[index].subtotal = lineTotal - cart[index].discount;
    cart = [...cart];
  }

  function removeItemDiscount(index: number) {
    const item = cart[index];
    cart[index].discount = 0;
    cart[index].subtotal = item.product.product.sale_price * item.quantity;
    itemDiscountInput = 0;
    cart = [...cart];
  }

  function removeFromCart(index: number) {
    cart = cart.filter((_, i) => i !== index);
  }

  function updateQuantity(index: number, qty: number) {
    if (qty <= 0) {
      removeFromCart(index);
      return;
    }

    const item = cart[index];
    const available = item.product.current_stock;

    // Validate: exceeds stock
    if (qty > available) {
      showToast(`⚠️ Solo hay ${available} unidades disponibles`, 'warning');
      return;
    }

    // Validate: unusually large quantity
    if (qty > 50) {
      if (!confirm(`¿Seguro que desea agregar ${qty} unidades de "${item.product.product.name}"?`)) return;
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

  function computedGlobalDiscount(): number {
    if (globalDiscountInput <= 0) return 0;
    const sub = cartSubtotal();
    if (globalDiscountType === 'percent') {
      const pct = Math.min(globalDiscountInput, 100);
      return Math.round(sub * (pct / 100) * 100) / 100;
    }
    return Math.min(Math.round(globalDiscountInput * 100) / 100, sub);
  }

  function cartTotal(): number {
    return cartSubtotal() - computedGlobalDiscount();
  }

  function totalItemDiscounts(): number {
    return cart.reduce((sum, item) => sum + item.discount, 0);
  }

  function change(): number {
    return Math.max(0, cashReceived - cartTotal());
  }

  function openPayment() {
    if (cart.length === 0) return;

    // Validate: cash register must be open
    if (!cashRegisterOpen) {
      showToast('⚠️ Abre la caja registradora antes de cobrar (Configuración → Abrir Caja)', 'warning');
      return;
    }

    cashReceived = cartTotal();
    paymentErrors = {};
    showPaymentModal = true;
  }

  function validatePayment(): boolean {
    const e: Record<string, string> = {};
    if (paymentMethod === 'efectivo' && cashReceived < cartTotal()) {
      e.cash = 'El monto recibido es menor al total';
    }
    paymentErrors = e;
    return Object.keys(e).length === 0;
  }

  async function completeSale() {
    if (!validatePayment()) return;
    const gd = computedGlobalDiscount();
    try {
      await createSale({
        items: cart.map(c => ({
          product_id: c.product.product.id,
          quantity: c.quantity,
          discount: c.discount > 0 ? c.discount : undefined,
        })),
        payment_method: paymentMethod,
        discount_amount: gd > 0 ? gd : undefined,
      });
      showToast('✅ Venta completada exitosamente', 'success');
      clearCart();
      showPaymentModal = false;
      stats = await getDashboardStats();
      await loadProducts(searchQuery);
    } catch (e) {
      showToast(`❌ Error: ${e}`, 'error');
    }
  }

  function clearCart() {
    cart = [];
    globalDiscountInput = 0;
    globalDiscountType = 'fixed';
    showGlobalDiscount = false;
    editingItemDiscount = null;
  }

  function showToast(message: string, type: 'success' | 'error' | 'warning') {
    toast = { show: true, message, type };
    setTimeout(() => { toast.show = false; }, 3000);
  }

  function formatCurrency(amount: number): string {
    return `Bs ${amount.toFixed(2)}`;
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div style="display: flex; height: 100vh; overflow: hidden;">
  <!-- Left: Product Search -->
  <div style="flex: 1; display: flex; flex-direction: column; border-right: 1px solid var(--border-color);">
    <!-- Search bar -->
    <div style="padding: var(--space-lg); border-bottom: 1px solid var(--border-color); background: var(--bg-secondary);">
      <div class="flex items-center gap-md">
        <div style="position: relative; flex: 1;">
          <input
            bind:this={searchInputRef}
            class="input input-lg"
            placeholder="🔍 Buscar producto por nombre, SKU o código de barras... (F1)"
            bind:value={searchQuery}
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
              disabled={ps.current_stock <= 0}
              style="
                background: var(--bg-secondary);
                border: 1px solid {ps.current_stock <= 0 ? 'var(--accent-danger)' : 'var(--border-color)'};
                border-radius: var(--radius-lg);
                padding: var(--space-lg);
                cursor: {ps.current_stock <= 0 ? 'not-allowed' : 'pointer'};
                text-align: left;
                transition: all var(--transition-fast);
                display: flex;
                flex-direction: column;
                gap: var(--space-sm);
                color: var(--text-primary);
                font-family: var(--font-family);
                opacity: {ps.current_stock <= 0 ? '0.5' : '1'};
              "
              onmouseenter={(e) => { if (ps.current_stock > 0) { (e.currentTarget as HTMLElement).style.borderColor = 'var(--accent-primary)'; (e.currentTarget as HTMLElement).style.transform = 'translateY(-2px)'; (e.currentTarget as HTMLElement).style.boxShadow = 'var(--shadow-glow-blue)'; }}}
              onmouseleave={(e) => { (e.currentTarget as HTMLElement).style.borderColor = ps.current_stock <= 0 ? 'var(--accent-danger)' : 'var(--border-color)'; (e.currentTarget as HTMLElement).style.transform = 'none'; (e.currentTarget as HTMLElement).style.boxShadow = 'none'; }}
            >
              <div class="flex items-center justify-between">
                <span class="text-xs text-muted">{ps.product.sku}</span>
                {#if ps.current_stock <= 0}
                  <span class="badge badge-danger">Sin stock</span>
                {:else if ps.current_stock <= ps.product.min_stock && ps.product.min_stock > 0}
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
                <span class="text-xs" style="color: {ps.current_stock <= 0 ? 'var(--accent-danger)' : 'var(--text-muted)'};">
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
          <button class="btn btn-ghost btn-sm" onclick={clearCart}>Limpiar (F4)</button>
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
                <div class="flex items-center gap-xs">
                  <button
                    class="btn btn-ghost btn-sm"
                    style="padding: 2px 6px; font-size: var(--font-size-xs); {item.discount > 0 ? 'color: var(--accent-warning);' : ''}"
                    onclick={() => toggleItemDiscount(index)}
                    title="Descuento por ítem"
                  >
                    {item.discount > 0 ? `−${formatCurrency(item.discount)}` : '% Desc.'}
                  </button>
                  <button
                    class="btn btn-ghost btn-sm"
                    style="width: 24px; height: 24px; padding: 0; font-size: var(--font-size-xs); border-radius: var(--radius-full);"
                    onclick={() => removeFromCart(index)}
                  >
                    ✕
                  </button>
                </div>
              </div>

              {#if editingItemDiscount === index}
                <div style="
                  background: var(--bg-elevated);
                  border-radius: var(--radius-sm);
                  padding: var(--space-sm) var(--space-md);
                  display: flex;
                  align-items: center;
                  gap: var(--space-sm);
                  animation: slideDown var(--transition-fast);
                ">
                  <div style="display: flex; border-radius: var(--radius-sm); overflow: hidden; border: 1px solid var(--border-color);">
                    <button
                      class="btn btn-sm"
                      style="padding: 2px 8px; border-radius: 0; font-size: var(--font-size-xs); {itemDiscountType === 'percent' ? 'background: var(--accent-primary); color: white;' : 'background: transparent; color: var(--text-secondary);'}"
                      onclick={() => { itemDiscountType = 'percent'; itemDiscountInput = 0; removeItemDiscount(index); }}
                    >%</button>
                    <button
                      class="btn btn-sm"
                      style="padding: 2px 8px; border-radius: 0; font-size: var(--font-size-xs); {itemDiscountType === 'fixed' ? 'background: var(--accent-primary); color: white;' : 'background: transparent; color: var(--text-secondary);'}"
                      onclick={() => { itemDiscountType = 'fixed'; itemDiscountInput = 0; removeItemDiscount(index); }}
                    >Bs</button>
                  </div>
                  <input
                    type="number"
                    class="input"
                    style="width: 70px; padding: 2px var(--space-sm); text-align: center; font-size: var(--font-size-sm);"
                    bind:value={itemDiscountInput}
                    oninput={() => applyItemDiscount(index)}
                    min="0"
                    max={itemDiscountType === 'percent' ? 100 : undefined}
                    step={itemDiscountType === 'percent' ? 1 : 0.5}
                    placeholder={itemDiscountType === 'percent' ? '0%' : '0.00'}
                  />
                  {#if item.discount > 0}
                    <button
                      class="btn btn-ghost btn-sm"
                      style="padding: 2px 6px; font-size: var(--font-size-xs); color: var(--accent-danger);"
                      onclick={() => removeItemDiscount(index)}
                      title="Quitar descuento"
                    >✕</button>
                  {/if}
                </div>
              {/if}

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
                {#if item.discount > 0}
                  <span style="color: var(--accent-warning); margin-left: var(--space-sm);">· Desc. −{formatCurrency(item.discount)}</span>
                {/if}
                {#if item.quantity >= item.product.current_stock}
                  <span style="color: var(--accent-warning); margin-left: var(--space-sm);">· Stock máximo</span>
                {/if}
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
        {#if totalItemDiscounts() > 0}
          <div class="flex justify-between text-sm">
            <span class="text-muted">Desc. por ítems</span>
            <span style="color: var(--accent-warning);">−{formatCurrency(totalItemDiscounts())}</span>
          </div>
        {/if}
        <div class="flex justify-between text-sm">
          <span class="text-muted">IVA (13%)</span>
          <span>{formatCurrency(cartTax())}</span>
        </div>

        <!-- Global Discount -->
        {#if !showGlobalDiscount}
          <button
            class="btn btn-ghost btn-sm"
            style="font-size: var(--font-size-xs); align-self: flex-start; padding: 2px var(--space-sm); color: var(--accent-warning);"
            onclick={() => showGlobalDiscount = true}
          >
            + Agregar descuento global
          </button>
        {:else}
          <div style="
            background: var(--bg-elevated);
            border-radius: var(--radius-sm);
            padding: var(--space-sm) var(--space-md);
            display: flex;
            align-items: center;
            gap: var(--space-sm);
            animation: slideDown var(--transition-fast);
          ">
            <span class="text-xs text-muted" style="white-space: nowrap;">Desc. global</span>
            <div style="display: flex; border-radius: var(--radius-sm); overflow: hidden; border: 1px solid var(--border-color);">
              <button
                class="btn btn-sm"
                style="padding: 2px 8px; border-radius: 0; font-size: var(--font-size-xs); {globalDiscountType === 'percent' ? 'background: var(--accent-primary); color: white;' : 'background: transparent; color: var(--text-secondary);'}"
                onclick={() => { globalDiscountType = 'percent'; globalDiscountInput = 0; }}
              >%</button>
              <button
                class="btn btn-sm"
                style="padding: 2px 8px; border-radius: 0; font-size: var(--font-size-xs); {globalDiscountType === 'fixed' ? 'background: var(--accent-primary); color: white;' : 'background: transparent; color: var(--text-secondary);'}"
                onclick={() => { globalDiscountType = 'fixed'; globalDiscountInput = 0; }}
              >Bs</button>
            </div>
            <input
              type="number"
              class="input"
              style="width: 70px; padding: 2px var(--space-sm); text-align: center; font-size: var(--font-size-sm);"
              bind:value={globalDiscountInput}
              min="0"
              max={globalDiscountType === 'percent' ? 100 : undefined}
              step={globalDiscountType === 'percent' ? 1 : 0.5}
              placeholder={globalDiscountType === 'percent' ? '0%' : '0.00'}
            />
            <button
              class="btn btn-ghost btn-sm"
              style="padding: 2px 6px; font-size: var(--font-size-xs); color: var(--accent-danger);"
              onclick={() => { showGlobalDiscount = false; globalDiscountInput = 0; }}
              title="Quitar descuento global"
            >✕</button>
          </div>
          {#if computedGlobalDiscount() > 0}
            <div class="flex justify-between text-sm">
              <span class="text-muted">Descuento global</span>
              <span style="color: var(--accent-warning);">−{formatCurrency(computedGlobalDiscount())}</span>
            </div>
          {/if}
        {/if}

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
        💰 Cobrar {formatCurrency(cartTotal())} (F2)
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
              onclick={() => { paymentMethod = method.value; paymentErrors = {}; }}
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
            <label class="input-label" for="cash-input">Efectivo recibido</label>
            <input
              id="cash-input"
              class="input input-lg"
              class:input-error={paymentErrors.cash}
              type="number"
              bind:value={cashReceived}
              oninput={() => { if (paymentErrors.cash) paymentErrors = {}; }}
              min={cartTotal()}
              step="0.5"
              style="font-size: var(--font-size-xl); font-weight: 700; text-align: center;"
            />
            {#if paymentErrors.cash}<span class="field-error">{paymentErrors.cash}</span>{/if}
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
        >
          ✅ Confirmar Venta (Enter)
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Toast Notification -->
{#if toast.show}
  <div class="toast"
    class:toast-success={toast.type === 'success'}
    class:toast-error={toast.type === 'error'}
    style={toast.type === 'warning' ? 'border-left: 3px solid var(--accent-warning);' : ''}
  >
    {toast.message}
  </div>
{/if}
