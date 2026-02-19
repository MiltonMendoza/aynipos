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

  // Editable settings
  let businessName = $state('');
  let businessNit = $state('');
  let businessAddress = $state('');
  let businessPhone = $state('');
  let businessCity = $state('');

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
    alert('✅ Configuración guardada');
  }

  async function handleOpenCash() {
    try {
      cashRegister = await openCashRegister(openingAmount);
      showOpenCash = false;
    } catch (e) { alert('Error: ' + e); }
  }

  async function handleCloseCash() {
    try {
      const result = await closeCashRegister(closingAmount, closingNotes || undefined);
      cashRegister = null;
      showCloseCash = false;
      const diff = (result.closing_amount ?? 0) - (result.expected_amount ?? 0);
      alert(`Caja cerrada.\nEsperado: Bs ${result.expected_amount?.toFixed(2)}\nReal: Bs ${result.closing_amount?.toFixed(2)}\nDiferencia: Bs ${diff.toFixed(2)}`);
    } catch (e) { alert('Error: ' + e); }
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
        <button class="btn btn-primary btn-block" onclick={saveBusiness}>💾 Guardar</button>
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
        <button class="btn btn-danger btn-block" onclick={() => showCloseCash = true}>🔒 Cerrar Caja</button>
      {:else}
        <div style="background: var(--bg-tertiary); border-radius: var(--radius-lg); padding: var(--space-xl); text-align: center; margin-bottom: var(--space-lg);">
          <div class="badge badge-warning">● Caja Cerrada</div>
        </div>
        <button class="btn btn-success btn-block" onclick={() => showOpenCash = true}>🔓 Abrir Caja</button>
      {/if}
    </div>
  </div>
</div>

{#if showOpenCash}
  <div class="modal-overlay" onclick={() => showOpenCash = false}>
    <div class="modal" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header"><h3 class="modal-title">🔓 Abrir Caja</h3></div>
      <div class="modal-body">
        <div class="input-group"><label class="input-label">Monto inicial (Bs)</label><input class="input input-lg" type="number" bind:value={openingAmount} min="0" step="0.5" /></div>
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
        <div class="input-group"><label class="input-label">Monto en caja (Bs)</label><input class="input input-lg" type="number" bind:value={closingAmount} min="0" step="0.5" /></div>
        <div class="input-group"><label class="input-label">Notas</label><input class="input" bind:value={closingNotes} placeholder="Observaciones..." /></div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-ghost" onclick={() => showCloseCash = false}>Cancelar</button>
        <button class="btn btn-danger" onclick={handleCloseCash}>🔒 Cerrar</button>
      </div>
    </div>
  </div>
{/if}
