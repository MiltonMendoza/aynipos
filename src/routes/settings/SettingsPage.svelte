<script lang="ts">
  import { onMount } from 'svelte';
  import type { Setting, CashRegister, User, CreateUser } from '$lib/types';
  import { getSettings, updateSetting, getCurrentCashRegister, openCashRegister, closeCashRegister, getCashRegisterReport, getUsers, createUser, updateUser, deleteUser } from '$lib/services/api';
  import { extractBusinessInfo, type BusinessInfo } from '$lib/services/receipt';
  import { printCashReport } from '$lib/services/cashReportPrint';
  import { getRoleLabel, getRoleIcon, hasPermission } from '$lib/services/permissions';

  let { currentUser }: { currentUser: User | null } = $props();

  let settings: Setting[] = $state([]);
  let cashRegister: CashRegister | null = $state(null);
  let showOpenCash = $state(false);
  let showCloseCash = $state(false);
  let openingAmount = $state(0);
  let closingAmount = $state(0);
  let closingNotes = $state('');
  let lastClosedRegisterId: string | null = $state(null);
  let businessInfo: BusinessInfo = $state({ name: '', nit: '', address: '', phone: '', city: '' });

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

  // Users management
  let users: User[] = $state([]);
  let showUserModal = $state(false);
  let editingUser: User | null = $state(null);
  let userName = $state('');
  let userPin = $state('');
  let userPinConfirm = $state('');
  let userRole = $state('cashier');
  let userErrors: Record<string, string> = $state({});

  onMount(async () => {
    try {
      settings = await getSettings();
      cashRegister = await getCurrentCashRegister();
      businessInfo = extractBusinessInfo(settings);
      for (const s of settings) {
        if (s.key === 'business_name') businessName = s.value;
        if (s.key === 'business_nit') businessNit = s.value;
        if (s.key === 'business_address') businessAddress = s.value;
        if (s.key === 'business_phone') businessPhone = s.value;
        if (s.key === 'business_city') businessCity = s.value;
      }
      users = await getUsers();
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
      lastClosedRegisterId = result.id;
      businessInfo = extractBusinessInfo(settings);
      const report = await getCashRegisterReport(result.id);
      await printCashReport(report, businessInfo);
    } catch (e) { alert('Error: ' + e); }
  }

  async function handleReprintReport() {
    if (!lastClosedRegisterId) return;
    try {
      businessInfo = extractBusinessInfo(settings);
      const report = await getCashRegisterReport(lastClosedRegisterId);
      await printCashReport(report, businessInfo);
    } catch (e) { alert('Error al generar reporte: ' + e); }
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

  // ─── Users Management ───────────────────────────────────
  function openNewUserModal() {
    editingUser = null;
    userName = '';
    userPin = '';
    userPinConfirm = '';
    userRole = 'cashier';
    userErrors = {};
    showUserModal = true;
  }

  function openEditUserModal(user: User) {
    editingUser = user;
    userName = user.name;
    userPin = '';
    userPinConfirm = '';
    userRole = user.role;
    userErrors = {};
    showUserModal = true;
  }

  function validateUser(): boolean {
    const e: Record<string, string> = {};
    if (!userName.trim()) e.name = 'El nombre es requerido';
    if (!editingUser) {
      // Creating — PIN required
      if (!userPin) e.pin = 'El PIN es requerido';
      else if (userPin.length < 4 || userPin.length > 6) e.pin = 'El PIN debe tener 4-6 dígitos';
      else if (!/^\d+$/.test(userPin)) e.pin = 'El PIN debe ser solo números';
      if (userPin !== userPinConfirm) e.pinConfirm = 'Los PINs no coinciden';
    } else if (userPin) {
      // Editing — PIN optional, but validate if provided
      if (userPin.length < 4 || userPin.length > 6) e.pin = 'El PIN debe tener 4-6 dígitos';
      else if (!/^\d+$/.test(userPin)) e.pin = 'El PIN debe ser solo números';
      if (userPin !== userPinConfirm) e.pinConfirm = 'Los PINs no coinciden';
    }
    userErrors = e;
    return Object.keys(e).length === 0;
  }

  async function handleSaveUser() {
    if (!validateUser()) return;
    try {
      if (editingUser) {
        await updateUser({
          id: editingUser.id,
          name: userName.trim(),
          pin: userPin || undefined,
          role: userRole,
        });
      } else {
        await createUser({
          name: userName.trim(),
          pin: userPin,
          role: userRole,
        });
      }
      users = await getUsers();
      showUserModal = false;
    } catch (e) {
      alert('Error: ' + e);
    }
  }

  async function handleDeleteUser(user: User) {
    if (!confirm(`¿Eliminar al usuario "${user.name}"?`)) return;
    try {
      await deleteUser(user.id);
      users = await getUsers();
    } catch (e) {
      alert('Error: ' + e);
    }
  }
</script>

<div class="page">
  <div class="page-header">
    <div>
      <h1 class="page-title">⚙️ Configuración</h1>
      <p class="page-subtitle">Datos del negocio, caja y usuarios</p>
    </div>
  </div>

  <div style="display: grid; grid-template-columns: {hasPermission(currentUser, 'manage_settings') ? '1fr 1fr' : '1fr'}; gap: var(--space-xl); max-width: 900px;">
    <!-- Business info -->
    {#if hasPermission(currentUser, 'manage_settings')}
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
    {/if}

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
        <div style="display: flex; flex-direction: column; gap: var(--space-sm);">
          <button class="btn btn-success btn-block" onclick={openCashModal}>🔓 Abrir Caja</button>
          {#if lastClosedRegisterId}
            <button class="btn btn-ghost btn-block" onclick={handleReprintReport}>📊 Ver último cierre</button>
          {/if}
        </div>
      {/if}
    </div>
  </div>

  <!-- Users Management -->
  {#if hasPermission(currentUser, 'manage_users')}
  <div style="max-width: 900px; margin-top: var(--space-xl);">
    <div class="card">
      <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: var(--space-lg);">
        <h3 style="font-weight: 700; margin: 0;">👥 Usuarios</h3>
        <button class="btn btn-primary btn-sm" onclick={openNewUserModal}>➕ Nuevo Usuario</button>
      </div>

      {#if users.length === 0}
        <p class="text-muted" style="text-align: center; padding: var(--space-xl);">No hay usuarios registrados</p>
      {:else}
        <div class="table-container">
          <table>
            <thead>
              <tr>
                <th>Nombre</th>
                <th>Rol</th>
                <th>Creado</th>
                <th style="width: 120px;">Acciones</th>
              </tr>
            </thead>
            <tbody>
              {#each users as user}
                <tr>
                  <td>
                    <div style="display: flex; align-items: center; gap: var(--space-sm);">
                      <span style="width: 28px; height: 28px; background: var(--bg-hover); border-radius: var(--radius-full); display: flex; align-items: center; justify-content: center; font-size: var(--font-size-xs);">
                        {getRoleIcon(user.role)}
                      </span>
                      {user.name}
                    </div>
                  </td>
                  <td>
                    <span class="badge" class:badge-warning={user.role === 'admin'} class:badge-info={user.role !== 'admin'}>
                      {getRoleLabel(user.role)}
                    </span>
                  </td>
                  <td class="text-muted text-sm">
                    {user.created_at ? new Date(user.created_at + 'Z').toLocaleDateString('es-BO') : '—'}
                  </td>
                  <td>
                    <div style="display: flex; gap: var(--space-xs);">
                      <button class="btn btn-ghost btn-sm" onclick={() => openEditUserModal(user)} title="Editar">✏️</button>
                      <button class="btn btn-ghost btn-sm" style="color: var(--accent-danger);" onclick={() => handleDeleteUser(user)} title="Eliminar">🗑️</button>
                    </div>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}
    </div>
  </div>
  {/if}
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

{#if showUserModal}
  <div class="modal-overlay" onclick={() => showUserModal = false}>
    <div class="modal" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h3 class="modal-title">{editingUser ? '✏️ Editar Usuario' : '➕ Nuevo Usuario'}</h3>
      </div>
      <div class="modal-body">
        <div style="display: flex; flex-direction: column; gap: var(--space-lg);">
          <div class="input-group">
            <label class="input-label">Nombre *</label>
            <input class="input" class:input-error={userErrors.name} bind:value={userName} placeholder="Nombre del usuario" oninput={() => { if (userErrors.name) { const { name, ...rest } = userErrors; userErrors = rest; } }} />
            {#if userErrors.name}<span class="field-error">{userErrors.name}</span>{/if}
          </div>

          <div class="input-group">
            <label class="input-label">{editingUser ? 'Nuevo PIN (dejar vacío para no cambiar)' : 'PIN *'}</label>
            <input class="input" class:input-error={userErrors.pin} type="password" inputmode="numeric" pattern="[0-9]*" maxlength="6" bind:value={userPin} placeholder="4-6 dígitos" oninput={() => { if (userErrors.pin) { const { pin, ...rest } = userErrors; userErrors = rest; } }} />
            {#if userErrors.pin}<span class="field-error">{userErrors.pin}</span>{/if}
          </div>

          <div class="input-group">
            <label class="input-label">Confirmar PIN</label>
            <input class="input" class:input-error={userErrors.pinConfirm} type="password" inputmode="numeric" pattern="[0-9]*" maxlength="6" bind:value={userPinConfirm} placeholder="Repetir PIN" oninput={() => { if (userErrors.pinConfirm) { const { pinConfirm, ...rest } = userErrors; userErrors = rest; } }} />
            {#if userErrors.pinConfirm}<span class="field-error">{userErrors.pinConfirm}</span>{/if}
          </div>

          <div class="input-group">
            <label class="input-label">Rol</label>
            <select class="select" bind:value={userRole}>
              <option value="cashier">Cajero</option>
              <option value="inventory">Inventarista</option>
              <option value="admin">Administrador</option>
            </select>
          </div>
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-ghost" onclick={() => showUserModal = false}>Cancelar</button>
        <button class="btn btn-primary" onclick={handleSaveUser}>
          {editingUser ? '💾 Guardar' : '➕ Crear'}
        </button>
      </div>
    </div>
  </div>
{/if}
