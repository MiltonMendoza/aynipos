<script lang="ts">
  import '../app.css';
  import type { AppRoute } from '$lib/types';

  let { children } = $props();

  let currentRoute: AppRoute = $state('pos');
  let lowStockBadge = $state(0);

  const navItems: { route: AppRoute; icon: string; label: string; section?: string }[] = [
    { route: 'pos', icon: '🛒', label: 'Punto de Venta', section: 'Principal' },
    { route: 'sales', icon: '📋', label: 'Ventas' },
    { route: 'inventory', icon: '📦', label: 'Inventario', section: 'Gestión' },
    { route: 'customers', icon: '👥', label: 'Clientes' },
    { route: 'reports', icon: '📊', label: 'Reportes', section: 'Análisis' },
    { route: 'settings', icon: '⚙️', label: 'Configuración' },
  ];

  function navigate(route: AppRoute) {
    currentRoute = route;
  }

  // Group items by section
  function getNavSections() {
    const sections: { label: string; items: typeof navItems }[] = [];
    let currentSection = { label: '', items: [] as typeof navItems };

    for (const item of navItems) {
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
          💊
        </div>
        <div class="flex-1">
          <div style="font-size: var(--font-size-sm); font-weight: 600;">Mi Farmacia</div>
          <div style="font-size: var(--font-size-xs); color: var(--text-muted);">v0.1.0</div>
        </div>
      </div>
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
        <SalesPage />
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
        <ReportsPage />
      {/await}
    {:else if currentRoute === 'settings'}
      {#await import('./settings/SettingsPage.svelte') then { default: SettingsPage }}
        <SettingsPage />
      {/await}
    {/if}
  </main>
</div>
