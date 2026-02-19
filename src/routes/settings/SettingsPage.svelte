<script lang="ts">
  import { onMount } from 'svelte';
  import type { Setting, CashRegister } from '$lib/types';
  import { getSettings, updateSetting, getCurrentCashRegister, openCashRegister, closeCashRegister } from '$lib/services/api';

  let settings: Setting[] = $state([]);
  let cashRegister: CashRegister | null = $state(null);
  let showOpenCash = $state(false);
  let showCloseCash = $state(false);
  let openingAmount = $state(0);
  let closingAmount = $state(0);
  let closingNotes = $state('');

  // Validation errors
  let openCashErrors: Record<string, string> = $state({});
  let closeCashErrors: Record<string, string> = $state({});

  // Editable settings
  let businessName = $state('');
  let businessNit = $state('');
  let businessAddress = $state('');
  let businessPhone = $state('');
  let businessCity = $state('');

  // Save feedback
  let saveSuccess = $state(false);

  onMount(async () => {
    try {
      settings = await getSettings();
      cashRegister = await getCurrentCashRegister();
      for (const s of settings) {
        if (s.key === 'business_name') businessName = s.value;
        if (s.key === 'business_nit') businessNit = s.value;
        if (s.key === 'business_address') businessAddress = s.value;
        if (s.key === 'business_phone') businessPhone = s.value;
        if (s.key === 'business_city') businessCity = s.value;
      }
    } catch {}
  });

  async function saveBusiness() {
    await updateSetting('business_name', businessName);
    await updateSetting('business_nit', businessNit);
    await updateSetting('business_address', businessAddress);
    await updateSetting('business_phone', businessPhone);
    await updateSetting('business_city', businessCity);
    saveSuccess = true;
    setTimeout(() => { saveSuccess = false; }, 3000);
  }

  function validateOpenCash(): boolean {
    const e: Record<string, string> = {};
    if (openingAmount < 0) e.amount = 'El monto debe ser mayor o igual a 0';
    openCashErrors = e;
    return Object.keys(e).length === 0;
  }

  function validateCloseCash(): boolean {
    const e: Record<string, string> = {};
    if (closingAmount < 0) e.amount = 'El monto debe ser mayor o igual a 0';
    closeCashErrors = e;
    return Object.keys(e).length === 0;
  }

  async function handleOpenCash() {
    if (!validateOpenCash()) return;
    try {
      cashRegister = await openCashRegister(openingAmount);
      showOpenCash = false;
      openCashErrors = {};
    } catch (e) { alert('Error: ' + e); }
  }

  async function handleCloseCash() {
    if (!validateCloseCash()) return;
    try {
      const result = await closeCashRegister(closingAmount, closingNotes || undefined);
      cashRegister = null;
      showCloseCash = false;
      closeCashErrors = {};
      const diff = (result.closing_amount ?? 0) - (result.expected_amount ?? 0);
      alert(`Caja cerrada.\nEsperado: Bs ${result.expected_amount?.toFixed(2)}\nReal: Bs ${result.closing_amount?.toFixed(2)}\nDiferencia: Bs ${diff.toFixed(2)}`);
    } catch (e) { alert('Error: ' + e); }
  }

  function openCashModal() {
    openingAmount = 0;
    openCashErrors = {};
    showOpenCash = true;
  }

  function closeCashModal() {
    closingAmount = 0;
    closingNotes = '';
    closeCashErrors = {};
    showCloseCash = true;
  }
</script>

<div class="page">
  <div class="page-header">
    <div>
      <h1 class="page-title">⚙️ Configuración</h1>
      <p class="page-subtitle">Datos del negocio y caja</p>
    </div>
  </div>

  <div style="display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-xl); max-width: 900px;">
    <!-- Business info -->
    <div class="card">
      <h3 style="font-weight: 700; margin-bottom: var(--space-lg);">🏪 Datos del Negocio</h3>
      <div style="display: flex; flex-direction: column; gap: var(--space-lg);">
        <div class="input-group"><label class="input-label">Razón Social</label><input class="input" bind:value={businessName} /></div>
        <div class="input-group"><label class="input-label">NIT</label><input class="input" bind:value={businessNit} /></div>
        <div class="input-group"><label class="input-label">Dirección</label><input class="input" bind:value={businessAddress} /></div>
        <div class="input-group"><label class="input-label">Teléfono</label><input class="input" bind:value={businessPhone} /></div>
        <div class="input-group"><label class="input-label">Ciudad</label><input class="input" bind:value={businessCity} /></div>
        <button class="btn btn-primary btn-block" onclick={saveBusiness}>
          {saveSuccess ? '✅ Guardado' : '💾 Guardar'}
        </button>
      </div>
    </div>

    <!-- Cash Register -->
    <div class="card">
      <h3 style="font-weight: 700; margin-bottom: var(--space-lg);">💰 Caja Registradora</h3>
      {#if cashRegister}
        <div style="background: var(--accent-success-glow); border-radius: var(--radius-lg); padding: var(--space-xl); text-align: center; margin-bottom: var(--space-lg);">
          <div class="badge badge-success" style="margin-bottom: var(--space-sm);">● Caja Abierta</div>
          <div class="text-sm text-muted">Monto inicial: Bs {cashRegister.opening_amount.toFixed(2)}</div>
        </div>
        <button class="btn btn-danger btn-block" onclick={closeCashModal}>🔒 Cerrar Caja</button>
      {:else}
        <div style="background: var(--bg-tertiary); border-radius: var(--radius-lg); padding: var(--space-xl); text-align: center; margin-bottom: var(--space-lg);">
          <div class="badge badge-warning">● Caja Cerrada</div>
        </div>
        <button class="btn btn-success btn-block" onclick={openCashModal}>🔓 Abrir Caja</button>
      {/if}
    </div>
  </div>
</div>

{#if showOpenCash}
  <div class="modal-overlay" onclick={() => showOpenCash = false}>
    <div class="modal" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header"><h3 class="modal-title">🔓 Abrir Caja</h3></div>
      <div class="modal-body">
        <div class="input-group">
          <label class="input-label">Monto inicial (Bs) *</label>
          <input class="input input-lg" class:input-error={openCashErrors.amount} type="number" bind:value={openingAmount} oninput={() => { if (openCashErrors.amount) openCashErrors = {}; }} min="0" step="0.5" />
          {#if openCashErrors.amount}<span class="field-error">{openCashErrors.amount}</span>{/if}
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-ghost" onclick={() => showOpenCash = false}>Cancelar</button>
        <button class="btn btn-success" onclick={handleOpenCash}>✅ Abrir</button>
      </div>
    </div>
  </div>
{/if}

{#if showCloseCash}
  <div class="modal-overlay" onclick={() => showCloseCash = false}>
    <div class="modal" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header"><h3 class="modal-title">🔒 Cerrar Caja</h3></div>
      <div class="modal-body">
        <div class="input-group">
          <label class="input-label">Monto en caja (Bs) *</label>
          <input class="input input-lg" class:input-error={closeCashErrors.amount} type="number" bind:value={closingAmount} oninput={() => { if (closeCashErrors.amount) closeCashErrors = {}; }} min="0" step="0.5" />
          {#if closeCashErrors.amount}<span class="field-error">{closeCashErrors.amount}</span>{/if}
        </div>
        <div class="input-group"><label class="input-label">Notas</label><input class="input" bind:value={closingNotes} placeholder="Observaciones..." /></div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-ghost" onclick={() => showCloseCash = false}>Cancelar</button>
        <button class="btn btn-danger" onclick={handleCloseCash}>🔒 Cerrar</button>
      </div>
    </div>
  </div>
{/if}
