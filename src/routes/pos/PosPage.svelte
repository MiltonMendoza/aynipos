<script lang="ts">
  import { onMount } from 'svelte';
  import type { ProductWithStock, CartItem, Customer, CreateCustomer, Sale, SaleItem, User } from '$lib/types';
  import { getProducts, getProductByBarcode, createSale, getSaleItems, cancelSale, getCurrentCashRegister, getDashboardStats, getCustomers, createCustomer, getSettings, logAction } from '$lib/services/api';
  import { DataTableState } from '$lib/utils/datatable.svelte';
  import TablePagination from '$lib/components/TablePagination.svelte';

  let { currentUser }: { currentUser: User | null } = $props();
  import { playAddSound, playErrorSound, playSuccessSound, playScanSound } from '$lib/services/sounds';
  import { printReceipt, extractBusinessInfo, type BusinessInfo } from '$lib/services/receipt';

  let products: ProductWithStock[] = $state([]);
  let viewMode = $state<'grid' | 'table'>('grid');
  let posTable = new DataTableState<ProductWithStock>([], [
    'product.sku',
    'product.name',
    'product.dose',
    'category_name',
    'supplier_name',
    'product.sale_price',
    'current_stock',
    'nearest_expiry_date'
  ]);

  function viewModeKey() {
    return currentUser ? `pos_view_mode_${currentUser.id}` : 'pos_view_mode';
  }

  function setViewMode(mode: 'grid' | 'table') {
    viewMode = mode;
    localStorage.setItem(viewModeKey(), mode);
  }

  function getCardStyle(ps: ProductWithStock) {
    const isExpired = ps.expiry_status === 'expired';
    const isLowStock = ps.current_stock <= ps.product.min_stock && ps.product.min_stock > 0;
    const isExpiring = ps.expiry_status === 'expiring' && !isLowStock;

    if (isExpired || isLowStock) {
      return {
        bg: 'rgba(239, 68, 68, 0.07)',
        border: 'var(--accent-danger)',
        hoverBg: 'rgba(239, 68, 68, 0.12)'
      };
    } else if (isExpiring) {
      return {
        bg: 'rgba(245, 158, 11, 0.07)',
        border: 'var(--accent-warning)',
        hoverBg: 'rgba(245, 158, 11, 0.12)'
      };
    }
    return {
      bg: 'var(--bg-secondary)',
      border: 'var(--border-color)',
      hoverBg: 'var(--bg-hover)'
    };
  }
  let cart: CartItem[] = $state([]);
  let searchQuery = $state('');
  let cashRegisterOpen = $state(false);
  let cashReceived = $state(0);
  let toast = $state({ show: false, message: '', type: 'success' as 'success' | 'error' | 'warning' });
  let toastTimer: ReturnType<typeof setTimeout> | null = null;
  
  // Sale success feedback in empty cart area
  let saleFeedback = $state({
    show: false,
    message: '',
    amount: 0,
    undoSaleId: null as string | null,
    progress: 100
  });
  let saleFeedbackTimer: ReturnType<typeof setTimeout> | null = null;
  let saleFeedbackProgressTimer: ReturnType<typeof setInterval> | null = null;

  let savedCart: CartItem[] = []; // snapshot for undo
  let searchInputRef: HTMLInputElement | undefined = $state(undefined);
  let f4PendingConfirm = $state(false);

  // Sale notes
  let saleNotes = $state('');
  let showNotes = $state(false);

  // Barcode detection — try exact barcode match before normal search
  let searchTimeout: ReturnType<typeof setTimeout> | null = null;
  let isScanning = $state(false);
  const BARCODE_MIN_LENGTH = 4;

  // Discounts
  let editingItemDiscount: number | null = $state(null);
  let itemDiscountType: 'percent' | 'fixed' = $state('percent');
  let itemDiscountInput: number = $state(0);
  let globalDiscountType: 'percent' | 'fixed' = $state('fixed');
  let globalDiscountInput: number = $state(0);
  let showGlobalDiscount = $state(false);

  // Customer selection
  let selectedCustomer: Customer | null = $state(null);
  let customerSearch = $state('');
  let customerResults: Customer[] = $state([]);
  let showCustomerSearch = $state(false);
  let showCreateCustomer = $state(false);
  let customerSearchTimeout: ReturnType<typeof setTimeout> | null = null;
  let newCustomer: CreateCustomer = $state({ name: '', nit: '' });
  let customerErrors: Record<string, string> = $state({});
  let customerSearchInputRef: HTMLInputElement | undefined = $state(undefined);

  // Dashboard quick stats
  let stats = $state({ total_sales_today: 0, total_transactions_today: 0, total_products: 0, low_stock_count: 0, expiring_soon_count: 0, total_capital: 0 });

  // Feedback animations
  let lastSaleTotal = $state(0);
  let animatingCartItems: Set<string> = $state(new Set());

  // Receipt printing
  let businessInfo: BusinessInfo = $state({ name: 'Mi Negocio', nit: '', address: '', phone: '', city: '' });
  let lastCompletedSale: Sale | null = $state(null);
  let lastCompletedSaleItems: SaleItem[] = $state([]);

  // Unified search handler — debounces input, checks barcode first
  function handleSearchInput() {
    if (searchTimeout) clearTimeout(searchTimeout);

    const query = searchQuery.trim();

    // Empty query — load all products immediately
    if (!query) {
      loadProducts('');
      isScanning = false;
      return;
    }

    // Show scanning indicator while waiting
    if (query.length >= BARCODE_MIN_LENGTH) {
      isScanning = true;
    }

    // Wait for input to stabilize, then check barcode or search
    searchTimeout = setTimeout(async () => {
      const finalQuery = searchQuery.trim();
      if (!finalQuery) {
        isScanning = false;
        loadProducts('');
        return;
      }

      // Try exact barcode match first (if long enough)
      if (finalQuery.length >= BARCODE_MIN_LENGTH) {
        try {
          const ps = await getProductByBarcode(finalQuery);
          if (ps) {
            console.log(`[Barcode] Match found: "${finalQuery}" → ${ps.product.name}`);
            addToCart(ps, true);
            playScanSound();
            showToast(`📦 Escaneado: ${ps.product.name}`, 'success');
            searchQuery = '';
            isScanning = false;
            return;
          }
        } catch (err) {
          console.error('[Barcode] Error:', err);
        }
      }

      // No barcode match — do normal product search
      console.log(`[Search] Searching for: "${finalQuery}"`);
      isScanning = false;
      loadProducts(finalQuery);
    }, 400);
  }

  onMount(async () => {
    const saved = localStorage.getItem(viewModeKey());
    if (saved === 'grid' || saved === 'table') {
      viewMode = saved;
    }
    try {
      const cr = await getCurrentCashRegister();
      cashRegisterOpen = cr !== null;
      stats = await getDashboardStats();
      // Load business info for receipts
      const allSettings = await getSettings();
      businessInfo = extractBusinessInfo(allSettings);
    } catch { /* first run, no data */ }
    await loadProducts('');
    // Auto-focus search input for barcode scanner
    searchInputRef?.focus();
  });

  // Keep search input focused for barcode scanner
  function refocusSearch() {
    setTimeout(() => searchInputRef?.focus(), 100);
  }

  async function loadProducts(search: string) {
    try {
      products = await getProducts(search || undefined);
      posTable.data = products;
      posTable.currentPage = 1;
    } catch {
      products = [];
      posTable.data = [];
    }
  }

  // ─── Customer Search ───
  function handleCustomerSearch() {
    if (customerSearchTimeout) clearTimeout(customerSearchTimeout);
    const query = customerSearch.trim();
    if (!query) { customerResults = []; return; }
    customerSearchTimeout = setTimeout(async () => {
      try {
        customerResults = await getCustomers(customerSearch.trim());
      } catch { customerResults = []; }
    }, 300);
  }

  function selectCustomer(customer: Customer) {
    selectedCustomer = customer;
    showCustomerSearch = false;
    showCreateCustomer = false;
    customerSearch = '';
    customerResults = [];
  }

  function clearCustomer() {
    selectedCustomer = null;
  }

  function toggleCustomerSearch() {
    showCustomerSearch = !showCustomerSearch;
    showCreateCustomer = false;
    customerSearch = '';
    customerResults = [];
    if (showCustomerSearch) {
      setTimeout(() => customerSearchInputRef?.focus(), 100);
    }
  }

  function validateNewCustomer(): boolean {
    const e: Record<string, string> = {};
    if (!newCustomer.name.trim()) e.name = 'El nombre es requerido';
    customerErrors = e;
    return Object.keys(e).length === 0;
  }

  async function handleCreateCustomer() {
    if (!validateNewCustomer()) return;
    try {
      const created = await createCustomer(newCustomer);
      selectCustomer(created);
      showCreateCustomer = false;
      newCustomer = { name: '', nit: '' };
      customerErrors = {};
      showToast(`👤 Cliente "${created.name}" creado`, 'success');
    } catch (e) {
      showToast(`❌ Error: ${e}`, 'error');
    }
  }

  // ─── Keyboard Shortcuts ───
  function handleKeydown(e: KeyboardEvent) {
    const tag = (e.target as HTMLElement)?.tagName;
    const isInput = tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT';

    // Escape — close any open modal/panel
    if (e.key === 'Escape') {
      return;
    }

    // F1 — focus search
    if (e.key === 'F1') {
      e.preventDefault();
      searchInputRef?.focus();
      searchInputRef?.select();
      return;
    }

    // F2 — cobro rápido con efectivo
    if (e.key === 'F2') {
      e.preventDefault();
      completeSale('efectivo');
      return;
    }

    // F3 — toggle customer selector
    if (e.key === 'F3') {
      e.preventDefault();
      toggleCustomerSearch();
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

    // F8 — deshacer última venta
    if (e.key === 'F8') {
      if (saleFeedback.show && saleFeedback.undoSaleId) {
        e.preventDefault();
        undoLastSale();
      }
      return;
    }
  }

  function addToCart(ps: ProductWithStock, fromBarcode = false) {
    if (saleFeedback.show) {
      clearSaleFeedbackTimers();
      saleFeedback.show = false;
      savedCart = [];
    }

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

    // Audio & visual feedback (skip sound if barcode — playScanSound handles it)
    if (!fromBarcode) playAddSound();

    // Animate the cart item
    const pid = ps.product.id;
    animatingCartItems = new Set([...animatingCartItems, pid]);
    setTimeout(() => {
      animatingCartItems = new Set([...animatingCartItems].filter(id => id !== pid));
    }, 300);
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

  async function completeSale(method: string) {
    if (cart.length === 0) return;

    // Validate: cash register must be open
    if (!cashRegisterOpen) {
      showToast('⚠️ Abre la caja registradora antes de cobrar (Configuración → Abrir Caja)', 'warning');
      return;
    }

    const gd = computedGlobalDiscount();
    try {
      const saleTotal = cartTotal();
      const completedSale = await createSale({
        customer_id: selectedCustomer?.id || 'default-consumer',
        items: cart.map(c => ({
          product_id: c.product.product.id,
          quantity: c.quantity,
          discount: c.discount > 0 ? c.discount : undefined,
        })),
        payment_method: method,
        discount_amount: gd > 0 ? gd : undefined,
        notes: saleNotes.trim() || undefined,
        user_id: currentUser?.id,
      });

      // Save sale data for receipt printing
      lastCompletedSale = completedSale;
      lastCompletedSaleItems = await getSaleItems(completedSale.id);

      // Audit log
      if (currentUser) {
        logAction(currentUser.id, currentUser.name, 'sale_created', 'sale', completedSale.id, `Venta #${completedSale.sale_number} por Bs ${saleTotal.toFixed(2)}`);
      }

      // Success feedback — save cart snapshot for undo BEFORE clearing
      const cartSnapshot = [...cart.map(c => ({ ...c }))];
      const saleNotesSnapshot = saleNotes;

      lastSaleTotal = saleTotal;
      playSuccessSound();
      clearCart();
      stats = await getDashboardStats();
      await loadProducts(searchQuery);
      refocusSearch();

      showSaleToast(`Venta registrada con éxito`, completedSale.id, cartSnapshot, saleNotesSnapshot, saleTotal);
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
    selectedCustomer = null;
    showCustomerSearch = false;
    showCreateCustomer = false;
    saleNotes = '';
    showNotes = false;
  }

  function clearToastTimers() {
    if (toastTimer) { clearTimeout(toastTimer); toastTimer = null; }
  }

  function clearSaleFeedbackTimers() {
    if (saleFeedbackTimer) { clearTimeout(saleFeedbackTimer); saleFeedbackTimer = null; }
    if (saleFeedbackProgressTimer) { clearInterval(saleFeedbackProgressTimer); saleFeedbackProgressTimer = null; }
  }

  function showToast(message: string, type: 'success' | 'error' | 'warning') {
    clearToastTimers();
    toast = { show: true, message, type };
    if (type === 'error') playErrorSound();
    toastTimer = setTimeout(() => { toast.show = false; }, 3000);
  }

  const UNDO_DURATION = 10000; // ms

  function showSaleToast(message: string, saleId: string, cartSnapshot: CartItem[], notesSnapshot: string, totalAmount: number) {
    clearSaleFeedbackTimers();
    savedCart = cartSnapshot;
    saleFeedback = {
      show: true,
      message,
      amount: totalAmount,
      undoSaleId: saleId,
      progress: 100
    };

    const step = 100 / (UNDO_DURATION / 50);
    saleFeedbackProgressTimer = setInterval(() => {
      saleFeedback.progress = Math.max(0, saleFeedback.progress - step);
      if (saleFeedback.progress <= 0) { clearSaleFeedbackTimers(); }
    }, 50);

    saleFeedbackTimer = setTimeout(() => {
      saleFeedback.show = false;
      savedCart = [];
    }, UNDO_DURATION);
  }

  async function undoLastSale() {
    const saleId = saleFeedback.undoSaleId;
    if (!saleId) return;
    clearSaleFeedbackTimers();
    saleFeedback.show = false;
    try {
      await cancelSale(saleId);
      // Restore cart
      cart = savedCart;
      savedCart = [];
      stats = await getDashboardStats();
      await loadProducts(searchQuery);
      showToast('↩ Venta deshecha correctamente', 'warning');
    } catch (e) {
      showToast(`❌ No se pudo deshacer: ${e}`, 'error');
    }
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
            oninput={handleSearchInput}
          />
          {#if isScanning}
            <div style="
              position: absolute;
              right: 12px;
              top: 50%;
              transform: translateY(-50%);
              display: flex;
              align-items: center;
              gap: var(--space-xs);
              background: var(--accent-primary);
              color: white;
              padding: 2px 10px;
              border-radius: var(--radius-full);
              font-size: var(--font-size-xs);
              font-weight: 600;
              animation: pulse 1s ease-in-out infinite;
            ">
              📡 Escaneando...
            </div>
          {/if}
        </div>
        {#if !cashRegisterOpen}
          <span class="badge badge-warning">⚠️ Caja cerrada</span>
        {/if}
      </div>

      <!-- Quick stats and view toggle -->
      <div class="flex items-center justify-between" style="margin-top: var(--space-md);">
        <div class="flex gap-lg">
          <span class="text-sm text-muted">📊 Hoy: <strong style="color: var(--accent-success)">{formatCurrency(stats.total_sales_today)}</strong></span>
          <span class="text-sm text-muted">🧾 {stats.total_transactions_today} ventas</span>
          {#if stats.low_stock_count > 0}
            <span class="badge badge-danger">{stats.low_stock_count} bajo stock</span>
          {/if}
        </div>

        <!-- Toggle view mode -->
        <div class="flex gap-xs" style="background: var(--bg-tertiary); padding: 2px; border-radius: var(--radius-md); border: 1px solid var(--border-color);">
          <button
            class="btn btn-sm"
            style="padding: 2px 8px; height: 24px; font-size: 11px; border: none; cursor: pointer; background: {viewMode === 'grid' ? 'var(--accent-primary)' : 'transparent'}; color: {viewMode === 'grid' ? 'white' : 'var(--text-muted)'};"
            onclick={(e) => { e.stopPropagation(); setViewMode('grid'); }}
          >
            🎴 Cuadrícula
          </button>
          <button
            class="btn btn-sm"
            style="padding: 2px 8px; height: 24px; font-size: 11px; border: none; cursor: pointer; background: {viewMode === 'table' ? 'var(--accent-primary)' : 'transparent'}; color: {viewMode === 'table' ? 'white' : 'var(--text-muted)'};"
            onclick={(e) => { e.stopPropagation(); setViewMode('table'); }}
          >
            📋 Lista
          </button>
        </div>
      </div>
    </div>

    <!-- Products grid -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div style="flex: 1; overflow-y: auto; padding: var(--space-lg);" onclick={refocusSearch}>
      {#if products.length === 0}
        <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%; gap: var(--space-lg); color: var(--text-muted);">
          <div style="font-size: 3rem; opacity: 0.5;">📦</div>
          <p>No se encontraron productos</p>
          <p class="text-sm">Agrega productos desde el menú de Inventario</p>
        </div>
      {:else}
        {#if viewMode === 'grid'}
          <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(180px, 1fr)); gap: var(--space-md);">
            {#each products as ps}
              {@const style = getCardStyle(ps)}
              {@const noStock = ps.current_stock <= 0}
              {@const lowStock = !noStock && ps.current_stock <= ps.product.min_stock && ps.product.min_stock > 0}
              <button
                class="product-card"
                onclick={() => addToCart(ps)}
                disabled={ps.current_stock <= 0}
                style="
                  background: {style.bg};
                  border: 1px solid {style.border};
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
                onmouseenter={(e) => { if (ps.current_stock > 0) { (e.currentTarget as HTMLElement).style.borderColor = 'var(--accent-primary)'; (e.currentTarget as HTMLElement).style.background = style.hoverBg; (e.currentTarget as HTMLElement).style.transform = 'translateY(-2px)'; (e.currentTarget as HTMLElement).style.boxShadow = 'var(--shadow-glow-blue)'; }}}
                onmouseleave={(e) => { (e.currentTarget as HTMLElement).style.borderColor = style.border; (e.currentTarget as HTMLElement).style.background = style.bg; (e.currentTarget as HTMLElement).style.transform = 'none'; (e.currentTarget as HTMLElement).style.boxShadow = 'none'; }}
              >
                <!-- Fila 1: SKU + badge Proveedor resaltado -->
                <div class="flex items-center justify-between" style="gap: var(--space-xs); flex-wrap: wrap;">
                  <span class="text-xs text-muted">{ps.product.sku}</span>
                  {#if ps.supplier_name}
                    <span style="
                      font-size: var(--font-size-sm); font-weight: 700;
                      padding: 3px 10px; border-radius: 999px;
                      background: var(--accent-primary-glow);
                      color: var(--accent-primary);
                      border: 1px solid var(--accent-primary);
                      letter-spacing: 0.02em;
                    ">{ps.supplier_name}</span>
                  {/if}
                </div>
                <!-- Nombre — resaltado principal -->
                <div style="font-weight: 800; font-size: var(--font-size-xl); line-height: 1.2; letter-spacing: -0.01em;" class="truncate">
                  {ps.product.name}
                </div>
                <!-- Dosis -->
                {#if ps.product.dose}
                  <div><span class="badge badge-info" style="font-size: var(--font-size-xs);">{ps.product.dose}</span></div>
                {/if}
                <!-- Categoría -->
                {#if ps.category_name}
                  <div class="text-xs text-muted truncate">{ps.category_name}</div>
                {/if}
                <!-- Vencimiento — chip con color según estado -->
                {#if ps.nearest_expiry_date}
                  {@const isExpired = ps.expiry_status === 'expired'}
                  {@const isExpiring = ps.expiry_status === 'expiring'}
                  <div style="
                    display: inline-flex; align-items: center; gap: 4px;
                    font-size: var(--font-size-xs); font-weight: 600;
                    padding: 2px 8px; border-radius: 999px;
                    background: {isExpired ? 'var(--accent-danger-glow)' : isExpiring ? 'var(--accent-warning-glow)' : 'var(--bg-tertiary)'};
                    color: {isExpired ? 'var(--accent-danger)' : isExpiring ? 'var(--accent-warning)' : 'var(--text-muted)'};
                    align-self: flex-start;
                  ">
                    {isExpired ? '⛔' : isExpiring ? '⚠️' : '📅'}
                    Vence: {new Date(ps.nearest_expiry_date + 'T12:00:00').toLocaleDateString('es-BO', { day: '2-digit', month: 'short', year: 'numeric' })}
                  </div>
                {/if}
                <!-- Footer: Precio resaltado + Stock chip -->
                <div class="flex items-center justify-between" style="margin-top: auto; padding-top: var(--space-sm); border-top: 1px solid var(--border-color);">
                  <span style="font-weight: 900; color: var(--accent-primary); font-size: 1.6rem; letter-spacing: -0.02em; line-height: 1;">
                    {formatCurrency(ps.product.sale_price)}
                  </span>
                  <span style="
                    font-size: var(--font-size-xs); font-weight: 600;
                    padding: 2px 8px; border-radius: 999px;
                    background: {noStock ? 'var(--accent-danger-glow)' : lowStock ? 'var(--accent-warning-glow)' : 'var(--bg-tertiary)'};
                    color: {noStock ? 'var(--accent-danger)' : lowStock ? 'var(--accent-warning)' : 'var(--text-muted)'};
                  ">
                    {noStock ? '⛔' : lowStock ? '⚠️' : '📦'} Stock: {ps.current_stock}
                  </span>
                </div>
              </button>
            {/each}
          </div>
        {:else if viewMode === 'table'}
          <div style="display: flex; flex-direction: column; gap: var(--space-md); height: 100%;">
            <div class="table-container" style="flex: 1; overflow-y: auto;">
              <table class="table">
                <thead>
                  <tr>
                    <th onclick={() => posTable.sortBy('product.sku')} style="cursor: pointer; user-select: none;">
                      SKU {posTable.sortColumn === 'product.sku' ? (posTable.sortDirection === 'asc' ? '↑' : '↓') : ''}
                    </th>
                    <th onclick={() => posTable.sortBy('product.name')} style="cursor: pointer; user-select: none;">
                      Producto {posTable.sortColumn === 'product.name' ? (posTable.sortDirection === 'asc' ? '↑' : '↓') : ''}
                    </th>
                    <th onclick={() => posTable.sortBy('product.dose')} style="cursor: pointer; user-select: none;">
                      Dosis {posTable.sortColumn === 'product.dose' ? (posTable.sortDirection === 'asc' ? '↑' : '↓') : ''}
                    </th>
                    <th onclick={() => posTable.sortBy('category_name')} style="cursor: pointer; user-select: none;">
                      Categoría {posTable.sortColumn === 'category_name' ? (posTable.sortDirection === 'asc' ? '↑' : '↓') : ''}
                    </th>
                    <th onclick={() => posTable.sortBy('supplier_name')} style="cursor: pointer; user-select: none;">
                      Proveedor {posTable.sortColumn === 'supplier_name' ? (posTable.sortDirection === 'asc' ? '↑' : '↓') : ''}
                    </th>
                    <th onclick={() => posTable.sortBy('product.sale_price')} style="cursor: pointer; user-select: none;">
                      Precio {posTable.sortColumn === 'product.sale_price' ? (posTable.sortDirection === 'asc' ? '↑' : '↓') : ''}
                    </th>
                    <th onclick={() => posTable.sortBy('current_stock')} style="cursor: pointer; user-select: none;">
                      Stock {posTable.sortColumn === 'current_stock' ? (posTable.sortDirection === 'asc' ? '↑' : '↓') : ''}
                    </th>
                    <th onclick={() => posTable.sortBy('nearest_expiry_date')} style="cursor: pointer; user-select: none;">
                      Vencimiento {posTable.sortColumn === 'nearest_expiry_date' ? (posTable.sortDirection === 'asc' ? '↑' : '↓') : ''}
                    </th>
                  </tr>
                </thead>
                <tbody>
                  {#if posTable.paginated.length === 0}
                    <tr><td colspan="9" class="text-center text-muted" style="padding: var(--space-3xl);">Sin productos</td></tr>
                  {:else}
                    {#each posTable.paginated as ps}
                      <tr
                        class:row-expired={ps.expiry_status === 'expired'}
                        class:row-low-stock={ps.current_stock <= ps.product.min_stock && ps.product.min_stock > 0}
                        class:row-expiring={ps.expiry_status === 'expiring' && !(ps.current_stock <= ps.product.min_stock && ps.product.min_stock > 0)}
                        style="cursor: {ps.current_stock <= 0 ? 'not-allowed' : 'pointer'}; opacity: {ps.current_stock <= 0 ? '0.5' : '1'};"
                        onclick={() => { if (ps.current_stock > 0) addToCart(ps); }}
                      >
                        <td class="font-mono text-sm">{ps.product.sku}</td>
                        <td style="font-weight: 600;">{ps.product.name}</td>
                        <td class="text-muted">
                          {#if ps.product.dose}
                            <span class="badge badge-info" style="font-size: var(--font-size-xs);">{ps.product.dose}</span>
                          {:else}—{/if}
                        </td>
                        <td class="text-muted">{ps.category_name || '—'}</td>
                        <td class="text-muted">{ps.supplier_name || '—'}</td>
                        <td style="font-weight: 600; color: var(--accent-primary);">{formatCurrency(ps.product.sale_price)}</td>
                        <td style="font-weight: 700;">
                          {ps.current_stock}
                          {#if ps.product.min_stock > 0}
                            <span style="font-weight: 400; font-size: var(--font-size-xs); color: var(--text-muted); margin-left: 2px;">
                              (Min: {ps.product.min_stock})
                            </span>
                          {/if}
                        </td>
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
            <TablePagination table={posTable} />
          </div>
        {/if}
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

    <!-- Customer section -->
    <div style="padding: var(--space-sm) var(--space-lg); border-bottom: 1px solid var(--border-color); background: var(--bg-tertiary);">
      {#if !showCustomerSearch && !showCreateCustomer}
        <!-- Selected customer display -->
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-sm" style="min-width: 0;">
            <span style="font-size: var(--font-size-sm);">👤</span>
            <div style="min-width: 0;">
              <div style="font-weight: 600; font-size: var(--font-size-sm);" class="truncate">
                {selectedCustomer ? selectedCustomer.name : 'Sin Nombre'}
              </div>
              <div class="text-xs text-muted">
                NIT: {selectedCustomer?.nit || '0'}
              </div>
            </div>
          </div>
          <div class="flex items-center gap-xs">
            {#if selectedCustomer}
              <button
                class="btn btn-ghost btn-sm"
                style="padding: 2px 6px; font-size: var(--font-size-xs); color: var(--accent-danger);"
                onclick={clearCustomer}
                title="Quitar cliente"
              >✕</button>
            {/if}
            <button
              class="btn btn-ghost btn-sm"
              style="padding: 2px 8px; font-size: var(--font-size-xs);"
              onclick={toggleCustomerSearch}
            >
              {selectedCustomer ? 'Cambiar' : '+ Cliente'} (F3)
            </button>
          </div>
        </div>
      {:else if showCustomerSearch}
        <!-- Customer search dropdown -->
        <div style="display: flex; flex-direction: column; gap: var(--space-sm); animation: slideDown var(--transition-fast);">
          <div class="flex items-center gap-sm">
            <input
              bind:this={customerSearchInputRef}
              class="input"
              style="flex: 1; padding: var(--space-xs) var(--space-sm); font-size: var(--font-size-sm);"
              placeholder="🔍 Buscar por nombre, NIT o teléfono..."
              bind:value={customerSearch}
              oninput={handleCustomerSearch}
              onkeydown={(e) => { if (e.key === 'Escape') { toggleCustomerSearch(); e.stopPropagation(); }}}
            />
            <button
              class="btn btn-ghost btn-sm"
              style="padding: 2px 6px; font-size: var(--font-size-xs);"
              onclick={toggleCustomerSearch}
            >✕</button>
          </div>

          <!-- Search results -->
          {#if customerResults.length > 0}
            <div style="
              max-height: 180px;
              overflow-y: auto;
              display: flex;
              flex-direction: column;
              gap: 2px;
              border-radius: var(--radius-sm);
            ">
              {#each customerResults.slice(0, 6) as customer}
                <button
                  class="btn btn-ghost"
                  style="
                    width: 100%;
                    justify-content: flex-start;
                    text-align: left;
                    padding: var(--space-xs) var(--space-sm);
                    font-size: var(--font-size-sm);
                    border-radius: var(--radius-sm);
                    gap: var(--space-sm);
                  "
                  onclick={() => selectCustomer(customer)}
                >
                  <span>👤</span>
                  <div style="min-width: 0; flex: 1;">
                    <div style="font-weight: 600;" class="truncate">{customer.name}</div>
                    <div class="text-xs text-muted">
                      {#if customer.nit}NIT: {customer.nit}{/if}
                      {#if customer.phone}{customer.nit ? ' · ' : ''}Tel: {customer.phone}{/if}
                    </div>
                  </div>
                </button>
              {/each}
            </div>
          {:else if customerSearch.trim().length > 0}
            <div class="text-xs text-muted" style="text-align: center; padding: var(--space-sm);">
              No se encontraron clientes
            </div>
          {/if}

          <!-- Create new customer button -->
          <button
            class="btn btn-ghost btn-sm"
            style="font-size: var(--font-size-xs); color: var(--accent-primary); align-self: flex-start; padding: 2px var(--space-sm);"
            onclick={() => { showCreateCustomer = true; showCustomerSearch = false; newCustomer = { name: customerSearch.trim() || '', nit: '' }; customerErrors = {}; }}
          >
            ➕ Crear nuevo cliente
          </button>
        </div>
      {:else if showCreateCustomer}
        <!-- Quick create customer form -->
        <div style="display: flex; flex-direction: column; gap: var(--space-sm); animation: slideDown var(--transition-fast);">
          <div class="flex items-center justify-between">
            <span style="font-weight: 600; font-size: var(--font-size-sm);">➕ Nuevo Cliente</span>
            <button
              class="btn btn-ghost btn-sm"
              style="padding: 2px 6px; font-size: var(--font-size-xs);"
              onclick={() => { showCreateCustomer = false; customerErrors = {}; }}
            >✕</button>
          </div>
          <div>
            <input
              class="input"
              class:input-error={customerErrors.name}
              style="padding: var(--space-xs) var(--space-sm); font-size: var(--font-size-sm);"
              placeholder="Nombre del cliente *"
              bind:value={newCustomer.name}
              oninput={() => { if (customerErrors.name) { const { name: _, ...rest } = customerErrors; customerErrors = rest; } }}
              onkeydown={(e) => { if (e.key === 'Enter') { e.preventDefault(); handleCreateCustomer(); } if (e.key === 'Escape') { showCreateCustomer = false; e.stopPropagation(); }}}
            />
            {#if customerErrors.name}<span class="field-error">{customerErrors.name}</span>{/if}
          </div>
          <input
            class="input"
            style="padding: var(--space-xs) var(--space-sm); font-size: var(--font-size-sm);"
            placeholder="NIT / CI (opcional)"
            bind:value={newCustomer.nit}
            onkeydown={(e) => { if (e.key === 'Enter') { e.preventDefault(); handleCreateCustomer(); } if (e.key === 'Escape') { showCreateCustomer = false; e.stopPropagation(); }}}
          />
          <div class="flex gap-sm">
            <button
              class="btn btn-ghost btn-sm"
              style="flex: 1; font-size: var(--font-size-xs);"
              onclick={() => { showCreateCustomer = false; customerErrors = {}; }}
            >Cancelar</button>
            <button
              class="btn btn-primary btn-sm"
              style="flex: 1; font-size: var(--font-size-xs);"
              onclick={handleCreateCustomer}
            >Guardar</button>
          </div>
        </div>
      {/if}
    </div>

    <!-- Cart items -->
    <div style="flex: 1; overflow-y: auto; padding: var(--space-md);">
      {#if cart.length === 0}
        {#if saleFeedback.show}
          <div style="
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            height: 100%;
            padding: var(--space-lg);
            animation: fadeIn var(--transition-base);
          ">
            <div style="
              background: var(--bg-elevated);
              border: 1px solid var(--border-color);
              border-radius: var(--radius-lg);
              padding: var(--space-xl);
              width: 100%;
              max-width: 320px;
              box-shadow: var(--shadow-md);
              position: relative;
              overflow: hidden;
              display: flex;
              flex-direction: column;
              align-items: center;
              gap: var(--space-md);
              text-align: center;
              border-top: 4px solid var(--accent-success);
            ">
              <!-- Close button -->
              <button
                onclick={() => { clearSaleFeedbackTimers(); saleFeedback.show = false; savedCart = []; }}
                style="
                  position: absolute;
                  top: var(--space-sm);
                  right: var(--space-sm);
                  border: none;
                  background: transparent;
                  color: var(--text-muted);
                  font-size: var(--font-size-md);
                  font-weight: 700;
                  cursor: pointer;
                  opacity: 0.6;
                  transition: opacity var(--transition-fast);
                "
                onmouseenter={(e) => (e.currentTarget as HTMLButtonElement).style.opacity = '1'}
                onmouseleave={(e) => (e.currentTarget as HTMLButtonElement).style.opacity = '0.6'}
                title="Cerrar confirmación"
              >✕</button>

              <!-- Success check icon -->
              <div style="
                width: 48px;
                height: 48px;
                border-radius: var(--radius-full);
                background: var(--accent-success-glow);
                color: var(--accent-success);
                display: flex;
                align-items: center;
                justify-content: center;
                font-size: 1.5rem;
                font-weight: 700;
                margin-bottom: var(--space-xs);
              ">
                ✓
              </div>

              <div>
                <h3 style="font-weight: 700; font-size: var(--font-size-md); color: var(--text-primary); margin-bottom: 2px;">
                  ¡Venta Completada!
                </h3>
                <p style="font-size: var(--font-size-xs); color: var(--text-muted);">
                  {saleFeedback.message}
                </p>
              </div>

              <div style="
                font-size: 1.6rem;
                font-weight: 900;
                color: var(--accent-success);
                margin: var(--space-xs) 0;
              ">
                {formatCurrency(saleFeedback.amount)}
              </div>

              {#if saleFeedback.undoSaleId}
                <button
                  class="btn btn-ghost"
                  onclick={undoLastSale}
                  style="
                    width: 100%;
                    padding: var(--space-sm);
                    font-size: var(--font-size-sm);
                    font-weight: 700;
                    border: 1px dashed var(--accent-danger);
                    color: var(--accent-danger);
                    border-radius: var(--radius-md);
                    cursor: pointer;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    gap: var(--space-xs);
                    transition: all var(--transition-fast);
                  "
                  onmouseenter={(e) => {
                    const el = e.currentTarget as HTMLButtonElement;
                    el.style.background = 'var(--accent-danger-glow)';
                  }}
                  onmouseleave={(e) => {
                    const el = e.currentTarget as HTMLButtonElement;
                    el.style.background = 'transparent';
                  }}
                  title="Deshacer venta (F8)"
                >
                  <span>↩</span> Deshacer Venta (F8)
                </button>
              {/if}

              <!-- Progress bar countdown -->
              <div style="
                position: absolute;
                bottom: 0; left: 0;
                height: 4px;
                width: {saleFeedback.progress}%;
                background: var(--accent-success);
                transition: width 0.05s linear;
              "></div>
            </div>
          </div>
        {:else}
          <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%; color: var(--text-muted); gap: var(--space-md);">
            <div style="font-size: 2.5rem; opacity: 0.4;">🛒</div>
            <p class="text-sm">El carrito está vacío</p>
            <p class="text-xs">Haz clic en un producto para agregarlo</p>
          </div>
        {/if}
      {:else}
        <div style="display: flex; flex-direction: column; gap: var(--space-sm);">
          {#each cart as item, index}
            <div
              class:cart-item-pop={animatingCartItems.has(item.product.product.id)}
              style="
                background: var(--bg-tertiary);
                border-radius: var(--radius-md);
                padding: var(--space-md);
                display: flex;
                flex-direction: column;
                gap: var(--space-sm);
                animation: slideDown var(--transition-fast);
                transition: background var(--transition-fast);
              "
            >
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
                    style="width: 34px; height: 34px; padding: 0; font-size: 1.1rem;"
                    onclick={() => updateQuantity(index, item.quantity - 1)}
                  >−</button>
                  <input
                    type="number"
                    class="input"
                    style="width: 72px; height: 34px; text-align: center; padding: var(--space-xs) var(--space-sm); font-weight: 700; font-size: var(--font-size-lg);"
                    value={item.quantity}
                    onchange={(e) => updateQuantity(index, parseFloat((e.target as HTMLInputElement).value) || 1)}
                    min="1"
                  />
                  <button
                    class="btn btn-ghost btn-sm"
                    style="width: 34px; height: 34px; padding: 0; font-size: 1.1rem;"
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
        {#if cartTax() > 0}
          <div class="flex justify-between text-sm">
            <span class="text-muted">Débito Fiscal</span>
            <span>{formatCurrency(cartTax())}</span>
          </div>
        {/if}

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

        <!-- Sale Notes -->
        {#if !showNotes}
          <button
            class="btn btn-ghost btn-sm"
            style="font-size: var(--font-size-xs); align-self: flex-start; padding: 2px var(--space-sm); color: var(--text-muted);"
            onclick={() => showNotes = true}
          >
            📝 {saleNotes.trim() ? 'Editar nota' : '+ Agregar nota'}
            {#if saleNotes.trim()}
              <span style="color: var(--accent-primary); margin-left: var(--space-xs);">✓</span>
            {/if}
          </button>
        {:else}
          <div style="
            background: var(--bg-elevated);
            border-radius: var(--radius-sm);
            padding: var(--space-sm) var(--space-md);
            display: flex;
            flex-direction: column;
            gap: var(--space-xs);
            animation: slideDown var(--transition-fast);
          ">
            <div class="flex items-center justify-between">
              <span class="text-xs text-muted">📝 Nota de la venta</span>
              <button
                class="btn btn-ghost btn-sm"
                style="padding: 2px 6px; font-size: var(--font-size-xs);"
                onclick={() => showNotes = false}
              >✕</button>
            </div>
            <textarea
              class="input"
              style="
                padding: var(--space-xs) var(--space-sm);
                font-size: var(--font-size-sm);
                resize: vertical;
                min-height: 48px;
                max-height: 100px;
                font-family: var(--font-family);
              "
              placeholder="Observaciones de la venta..."
              bind:value={saleNotes}
              onkeydown={(e) => { if (e.key === 'Escape') { showNotes = false; e.stopPropagation(); }}}
            ></textarea>
          </div>
        {/if}

        <div style="height: 1px; background: var(--border-color); margin: var(--space-xs) 0;"></div>
        <div class="flex justify-between" style="font-size: var(--font-size-xl); font-weight: 800;">
          <span>Total</span>
          <span style="color: var(--accent-success);">{formatCurrency(cartTotal())}</span>
        </div>
      </div>

      <div style="display: grid; gap: var(--space-sm);">
        <div style="text-align: center; font-size: var(--font-size-xs); text-transform: uppercase; letter-spacing: 0.08em; color: var(--text-muted); font-weight: 600; margin-bottom: var(--space-xs);">Cobrar con</div>
        <div style="display: grid; grid-template-columns: 2fr 1fr; gap: var(--space-sm);">
          <button
            class="btn btn-success"
            onclick={() => completeSale('efectivo')}
            disabled={cart.length === 0}
            style="flex-direction: column; gap: 2px; padding: var(--space-sm) 0; font-size: var(--font-size-sm); font-weight: 700;"
          >
            <span style="font-size: 1.3rem;">💵</span>
            <span>Efectivo</span>
            <span style="font-size: var(--font-size-xs); opacity: 0.8; font-weight: 400;">(F2)</span>
          </button>
          <button
            class="btn"
            onclick={() => completeSale('qr')}
            disabled={cart.length === 0}
            style="flex-direction: column; gap: 2px; padding: var(--space-sm) 0; font-size: var(--font-size-sm); font-weight: 700; background: var(--bg-elevated); border: 1px solid var(--border-color);"
          >
            <span style="font-size: 1.3rem;">📱</span>
            <span>QR</span>
          </button>
        </div>
      </div>
    </div>
  </div>
</div>



<!-- Toast Notification -->
{#if toast.show}
  <div class="toast"
    class:toast-success={toast.type === 'success'}
    class:toast-error={toast.type === 'error'}
    class:toast-shake={toast.type === 'error'}
    style={toast.type === 'warning' ? 'border-left: 3px solid var(--accent-warning);' : ''}
  >
    <span>{toast.message}</span>
  </div>
{/if}

