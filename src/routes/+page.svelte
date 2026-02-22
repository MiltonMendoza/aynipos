<script lang="ts">
  import '../app.css';
  import type { AppRoute, User } from '$lib/types';
  import { canAccessRoute, getDefaultRoute, getRoleLabel, getRoleIcon } from '$lib/services/permissions';

  let { children } = $props();

  let currentRoute: AppRoute = $state('pos');
  let lowStockBadge = $state(0);
  let showShortcuts = $state(false);
  let currentUser: User | null = $state(null);

  const navItems: { route: AppRoute; icon: string; label: string; section?: string }[] = [
    { route: 'pos', icon: '🛒', label: 'Punto de Venta', section: 'Principal' },
    { route: 'sales', icon: '📋', label: 'Ventas' },
    { route: 'inventory', icon: '📦', label: 'Inventario', section: 'Gestión' },
    { route: 'customers', icon: '👥', label: 'Clientes' },
    { route: 'reports', icon: '📊', label: 'Reportes', section: 'Análisis' },
    { route: 'settings', icon: '⚙️', label: 'Configuración' },
  ];

  // Filter nav items by role permissions
  let filteredNavItems = $derived(
    navItems.filter(item => !currentUser || canAccessRoute(currentUser, item.route))
  );

  function navigate(route: AppRoute) {
    currentRoute = route;
  }

  function handleGlobalKeydown(e: KeyboardEvent) {
    if (e.key === 'F10') {
      e.preventDefault();
      showShortcuts = !showShortcuts;
    }
  }

  function handleLogin(user: User) {
    currentUser = user;
    currentRoute = getDefaultRoute(user);
  }

  function handleLogout() {
    currentUser = null;
    currentRoute = 'pos';
  }

  // Group items by section
  function getNavSections() {
    const sections: { label: string; items: typeof navItems }[] = [];
    let currentSection = { label: '', items: [] as typeof navItems };

    for (const item of filteredNavItems) {
      if (item.section) {
        if (currentSection.items.length > 0) {
          sections.push(currentSection);
        }
        currentSection = { label: item.section, items: [item] };
      } else {
        currentSection.items.push(item);
      }
    }
    if (currentSection.items.length > 0) {
      sections.push(currentSection);
    }
    return sections;
  }
</script>

<svelte:window onkeydown={handleGlobalKeydown} />

{#if !currentUser}
  {#await import('./login/LoginScreen.svelte') then { default: LoginScreen }}
    <LoginScreen onLogin={handleLogin} />
  {/await}
{:else}

<div class="app-layout">
  <!-- Sidebar -->
  <aside class="app-sidebar">
    <div class="sidebar-header">
      <div class="sidebar-logo">
        <div class="sidebar-logo-icon">A</div>
        <span class="sidebar-logo-text">AyniPOS</span>
      </div>
    </div>

    <nav class="sidebar-nav">
      {#each getNavSections() as section}
        <div class="sidebar-section-label">{section.label}</div>
        {#each section.items as item}
          <button
            class="nav-item"
            class:active={currentRoute === item.route}
            onclick={() => navigate(item.route)}
          >
            <span class="nav-icon">{item.icon}</span>
            <span>{item.label}</span>
            {#if item.route === 'inventory' && lowStockBadge > 0}
              <span class="nav-badge">{lowStockBadge}</span>
            {/if}
          </button>
        {/each}
      {/each}
    </nav>

    <!-- Sidebar footer -->
    <div style="padding: var(--space-lg); border-top: 1px solid var(--border-color);">
      <div class="flex items-center gap-md">
        <div style="width: 32px; height: 32px; background: var(--bg-hover); border-radius: var(--radius-full); display: flex; align-items: center; justify-content: center; font-size: var(--font-size-sm);">
          👤
        </div>
        <div class="flex-1">
          <div style="font-size: var(--font-size-sm); font-weight: 600;">{currentUser.name}</div>
          <div style="font-size: var(--font-size-xs); color: var(--text-muted);">{getRoleLabel(currentUser.role)}</div>
        </div>
      </div>
      <button
        class="btn btn-ghost btn-sm"
        style="width: 100%; margin-top: var(--space-sm); font-size: var(--font-size-xs);"
        onclick={handleLogout}
      >
        🚪 Cerrar Sesión
      </button>
      <button
        class="btn btn-ghost btn-sm"
        style="width: 100%; margin-top: var(--space-xs); font-size: var(--font-size-xs);"
        onclick={() => showShortcuts = !showShortcuts}
      >
        ⌨️ Atajos (F10)
      </button>
    </div>
  </aside>

  <!-- Main Content -->
  <main class="app-content">
    {#if currentRoute === 'pos'}
      {#await import('./pos/PosPage.svelte') then { default: PosPage }}
        <PosPage />
      {/await}
    {:else if currentRoute === 'sales'}
      {#await import('./sales/SalesPage.svelte') then { default: SalesPage }}
        <SalesPage {currentUser} />
      {/await}
    {:else if currentRoute === 'inventory'}
      {#await import('./inventory/InventoryPage.svelte') then { default: InventoryPage }}
        <InventoryPage />
      {/await}
    {:else if currentRoute === 'customers'}
      {#await import('./customers/CustomersPage.svelte') then { default: CustomersPage }}
        <CustomersPage />
      {/await}
    {:else if currentRoute === 'reports'}
      {#await import('./reports/ReportsPage.svelte') then { default: ReportsPage }}
        <ReportsPage {currentUser} />
      {/await}
    {:else if currentRoute === 'settings'}
      {#await import('./settings/SettingsPage.svelte') then { default: SettingsPage }}
        <SettingsPage {currentUser} />
      {/await}
    {/if}
  </main>
</div>

<!-- Shortcuts Cheat Sheet -->
{#if showShortcuts}
  <div class="shortcuts-panel">
    <h4>
      ⌨️ Atajos de Teclado
      <button class="btn btn-ghost btn-sm" style="padding: 0; width: 20px; height: 20px; font-size: var(--font-size-xs);" onclick={() => showShortcuts = false}>✕</button>
    </h4>
    <div class="shortcut-row"><span>Buscar producto</span><span class="shortcut-key">F1</span></div>
    <div class="shortcut-row"><span>Cobrar</span><span class="shortcut-key">F2</span></div>
    <div class="shortcut-row"><span>Limpiar carrito</span><span class="shortcut-key">F4</span></div>
    <div class="shortcut-row"><span>Confirmar venta</span><span class="shortcut-key">Enter</span></div>
    <div class="shortcut-row"><span>Cerrar modal</span><span class="shortcut-key">Esc</span></div>
    <div class="shortcut-row"><span>Cantidad +1 / −1</span><span class="shortcut-key">+ / −</span></div>
    <div class="shortcut-row"><span>Mostrar/ocultar atajos</span><span class="shortcut-key">F10</span></div>
  </div>
{/if}

{/if}
