<script lang="ts">
  import type { DataTableState } from '../utils/datatable.svelte';

  let { table }: { table: DataTableState<any> } = $props();

  let pages = $derived(getVisiblePages(table.currentPage, table.totalPages));

  function getVisiblePages(current: number, total: number) {
    const range = [];
    const maxVisible = 5;
    let start = Math.max(1, current - Math.floor(maxVisible / 2));
    let end = Math.min(total, start + maxVisible - 1);

    if (end - start + 1 < maxVisible) {
      start = Math.max(1, end - maxVisible + 1);
    }

    for (let i = start; i <= end; i++) {
      range.push(i);
    }
    return range;
  }
</script>

<div class="datatable-pagination">
  <!-- Left side: Page size and count -->
  <div class="pagination-info">
    <span class="text-muted">Mostrar</span>
    <select
      class="select select-compact"
      bind:value={table.pageSize}
      onchange={() => (table.currentPage = 1)}
    >
      <option value={10}>10</option>
      <option value={20}>20</option>
      <option value={50}>50</option>
      <option value={100}>100</option>
    </select>
    <span class="text-muted">
      registros (Mostrando {table.startIndex}-{table.endIndex} de {table.sorted.length})
    </span>
  </div>

  <!-- Right side: Buttons -->
  <div class="pagination-buttons">
    <button
      class="pagination-btn"
      disabled={table.currentPage === 1}
      onclick={() => (table.currentPage = 1)}
      title="Primera página"
    >
      ⏮
    </button>
    <button
      class="pagination-btn"
      disabled={table.currentPage === 1}
      onclick={() => (table.currentPage = Math.max(1, table.currentPage - 1))}
      title="Anterior"
    >
      ◀
    </button>

    {#if pages[0] > 1}
      <button class="pagination-btn" onclick={() => (table.currentPage = 1)}>1</button>
      {#if pages[0] > 2}
        <span class="pagination-ellipsis">...</span>
      {/if}
    {/if}

    {#each pages as page}
      <button
        class="pagination-btn"
        class:active={table.currentPage === page}
        onclick={() => (table.currentPage = page)}
      >
        {page}
      </button>
    {/each}

    {#if pages[pages.length - 1] < table.totalPages}
      {#if pages[pages.length - 1] < table.totalPages - 1}
        <span class="pagination-ellipsis">...</span>
      {/if}
      <button class="pagination-btn" onclick={() => (table.currentPage = table.totalPages)}>
        {table.totalPages}
      </button>
    {/if}

    <button
      class="pagination-btn"
      disabled={table.currentPage === table.totalPages}
      onclick={() => (table.currentPage = Math.min(table.totalPages, table.currentPage + 1))}
      title="Siguiente"
    >
      ▶
    </button>
    <button
      class="pagination-btn"
      disabled={table.currentPage === table.totalPages}
      onclick={() => (table.currentPage = table.totalPages)}
      title="Última página"
    >
      ⏭
    </button>
  </div>
</div>

<style>
  .datatable-pagination {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: var(--space-md);
    margin-top: var(--space-md);
    padding: var(--space-xs) 0;
    border-top: 1px solid var(--border-color);
    font-size: var(--font-size-xs);
    flex-wrap: wrap;
  }

  .pagination-info {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
  }

  .select-compact {
    padding: 2px 24px 2px 8px;
    height: 26px;
    font-size: var(--font-size-xs);
    border-radius: var(--radius-sm);
    width: auto;
  }

  .pagination-buttons {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .pagination-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    height: 26px;
    min-width: 26px;
    padding: 0 6px;
    border: 1px solid var(--border-color);
    background: var(--bg-card);
    color: var(--text-primary);
    font-size: var(--font-size-xs);
    font-weight: 500;
    border-radius: var(--radius-sm);
    cursor: pointer;
    transition: all 0.15s ease;
    user-select: none;
  }

  .pagination-btn:hover:not(:disabled) {
    background: var(--bg-hover);
    border-color: var(--text-muted);
  }

  .pagination-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
    background: var(--bg-muted);
  }

  .pagination-btn.active {
    background: var(--accent-primary);
    color: white;
    border-color: var(--accent-primary);
  }

  .pagination-ellipsis {
    color: var(--text-muted);
    padding: 0 4px;
    user-select: none;
  }
</style>
