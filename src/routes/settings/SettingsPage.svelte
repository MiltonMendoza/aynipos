<script lang="ts">
  import { onMount } from 'svelte';
  import type { Setting, CashRegister, User, CreateUser, AuditLogEntry } from '$lib/types';
  import { getSettings, updateSetting, getCurrentCashRegister, openCashRegister, closeCashRegister, getCashRegisterReport, getUsers, createUser, updateUser, deleteUser, logAction, getAuditLog } from '$lib/services/api';
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

  // Audit log
  let auditLog: AuditLogEntry[] = $state([]);
  let auditLoading = $state(false);
  let auditFilterUser = $state('');
  let auditFilterAction = $state('');
  let auditFilterDateFrom = $state('');
  let auditFilterDateTo = $state('');
  let auditLimit = $state(50);

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
      if (currentUser) {
        logAction(currentUser.id, currentUser.name, 'cash_register_opened', 'cash_register', cashRegister.id, `Caja abierta con Bs ${openingAmount.toFixed(2)}`);
      }
      showOpenCash = false;
      openCashErrors = {};
    } catch (e) { alert('Error: ' + e); }
  }

  async function handleCloseCash() {
    if (!validateCloseCash()) return;
    try {
      const result = await closeCashRegister(closingAmount, closingNotes || undefined);
      if (currentUser) {
        logAction(currentUser.id, currentUser.name, 'cash_register_closed', 'cash_register', result.id, `Caja cerrada. Real: Bs ${closingAmount.toFixed(2)}`);
      }
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
        if (currentUser) {
          const changes: string[] = [];
          if (editingUser.name !== userName.trim()) changes.push(`nombre: "${userName.trim()}"`);
          if (editingUser.role !== userRole) changes.push(`rol: ${getRoleLabel(userRole)}`);
          if (userPin) changes.push('PIN cambiado');
          logAction(currentUser.id, currentUser.name, 'user_updated', 'user', editingUser.id, `Usuario "${userName.trim()}" actualizado (${changes.join(', ') || 'sin cambios'})`);
        }
      } else {
        const created = await createUser({
          name: userName.trim(),
          pin: userPin,
          role: userRole,
        });
        if (currentUser) {
          logAction(currentUser.id, currentUser.name, 'user_created', 'user', created.id, `Usuario "${userName.trim()}" creado (${getRoleLabel(userRole)})`);
        }
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
      if (currentUser) {
        logAction(currentUser.id, currentUser.name, 'user_deleted', 'user', user.id, `Usuario "${user.name}" desactivado`);
      }
      users = await getUsers();
    } catch (e) {
      alert('Error: ' + e);
    }
  }

  // ─── Audit Log ────────────────────────────────────────
  async function loadAuditLog() {
    auditLoading = true;
    try {
      auditLog = await getAuditLog(
        auditFilterUser || undefined,
        auditFilterAction || undefined,
        auditFilterDateFrom || undefined,
        auditFilterDateTo || undefined,
        auditLimit
      );
    } catch { auditLog = []; }
    auditLoading = false;
  }

  function loadMoreAudit() {
    auditLimit += 50;
    loadAuditLog();
  }

  function actionIcon(action: string): string {
    switch (action) {
      case 'sale_created': return '💰';
      case 'sale_cancelled': return '🚫';
      case 'inventory_adjusted': return '📦';
      case 'product_created': return '➕';
      case 'product_updated': return '✏️';
      case 'product_deleted': return '🗑️';
      case 'cash_register_opened': return '🔓';
      case 'cash_register_closed': return '🔒';
      case 'user_created': return '👤';
      case 'user_updated': return '🔄';
      case 'user_deleted': return '❌';
      case 'user_login': return '🔑';
      default: return '📝';
    }
  }

  function actionLabel(action: string): string {
    switch (action) {
      case 'sale_created': return 'Venta';
      case 'sale_cancelled': return 'Anulación';
      case 'inventory_adjusted': return 'Ajuste inv.';
      case 'product_created': return 'Producto+';
      case 'product_updated': return 'Producto✏️';
      case 'product_deleted': return 'Producto−';
      case 'cash_register_opened': return 'Caja abierta';
      case 'cash_register_closed': return 'Caja cerrada';
      case 'user_created': return 'Usuario+';
      case 'user_updated': return 'Usuario✏️';
      case 'user_deleted': return 'Usuario−';
      case 'user_login': return 'Login';
      default: return action;
    }
  }

  function actionBadgeClass(action: string): string {
    if (action.includes('created') || action === 'cash_register_opened') return 'badge-success';
    if (action.includes('cancelled') || action.includes('deleted')) return 'badge-danger';
    if (action.includes('updated') || action.includes('adjusted')) return 'badge-warning';
    return 'badge-info';
  }

  function formatDateTime(d: string | null): string {
    if (!d) return '-';
    const date = new Date(d + 'Z');
    return date.toLocaleDateString('es-BO', { day: '2-digit', month: '2-digit', year: 'numeric', hour: '2-digit', minute: '2-digit' });
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

  <!-- Audit Log -->
  {#if hasPermission(currentUser, 'view_audit_log')}
  <div style="max-width: 900px; margin-top: var(--space-xl);">
    <div class="card">
      <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: var(--space-lg);">
        <h3 style="font-weight: 700; margin: 0;">📋 Registro de Actividad</h3>
        <button class="btn btn-ghost btn-sm" onclick={loadAuditLog}>{auditLoading ? '⏳...' : '🔄 Actualizar'}</button>
      </div>

      <!-- Filters -->
      <div style="display: flex; gap: var(--space-md); margin-bottom: var(--space-lg); flex-wrap: wrap; align-items: flex-end;">
        <div style="display: flex; flex-direction: column; gap: var(--space-xs);">
          <label style="font-size: var(--font-size-xs); color: var(--text-muted); text-transform: uppercase; font-weight: 600;">Acción</label>
          <select class="select" style="width: 160px; height: 36px; font-size: var(--font-size-sm);" bind:value={auditFilterAction} onchange={loadAuditLog}>
            <option value="">Todas</option>
            <option value="sale_created">Ventas</option>
            <option value="sale_cancelled">Anulaciones</option>
            <option value="inventory_adjusted">Ajustes inventario</option>
            <option value="product_created">Producto creado</option>
            <option value="product_updated">Producto editado</option>
            <option value="cash_register_opened">Caja abierta</option>
            <option value="cash_register_closed">Caja cerrada</option>
            <option value="user_login">Logins</option>
          </select>
        </div>
        <div style="display: flex; flex-direction: column; gap: var(--space-xs);">
          <label style="font-size: var(--font-size-xs); color: var(--text-muted); text-transform: uppercase; font-weight: 600;">Desde</label>
          <input type="date" class="input" style="width: 150px; height: 36px; font-size: var(--font-size-sm);" bind:value={auditFilterDateFrom} onchange={loadAuditLog} />
        </div>
        <div style="display: flex; flex-direction: column; gap: var(--space-xs);">
          <label style="font-size: var(--font-size-xs); color: var(--text-muted); text-transform: uppercase; font-weight: 600;">Hasta</label>
          <input type="date" class="input" style="width: 150px; height: 36px; font-size: var(--font-size-sm);" bind:value={auditFilterDateTo} onchange={loadAuditLog} />
        </div>
      </div>

      {#if auditLog.length === 0 && !auditLoading}
        <div style="text-align: center; padding: var(--space-2xl); color: var(--text-muted);">
          <div style="font-size: 2rem; margin-bottom: var(--space-md); opacity: 0.5;">📋</div>
          <p>Presiona "🔄 Actualizar" para cargar el registro de actividad</p>
        </div>
      {:else}
        <div class="table-container" style="max-height: 500px; overflow-y: auto;">
          <table>
            <thead>
              <tr>
                <th>Fecha</th>
                <th>Usuario</th>
                <th>Acción</th>
                <th>Detalle</th>
              </tr>
            </thead>
            <tbody>
              {#each auditLog as entry}
                <tr>
                  <td class="text-sm" style="white-space: nowrap;">{formatDateTime(entry.created_at)}</td>
                  <td style="font-weight: 600; font-size: var(--font-size-sm);">{entry.user_name}</td>
                  <td>
                    <span class="badge {actionBadgeClass(entry.action)}">
                      {actionIcon(entry.action)} {actionLabel(entry.action)}
                    </span>
                  </td>
                  <td class="text-sm text-muted" style="max-width: 300px;">
                    <div class="truncate">{entry.details || '—'}</div>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
        {#if auditLog.length >= auditLimit}
          <div style="text-align: center; margin-top: var(--space-lg);">
            <button class="btn btn-ghost" onclick={loadMoreAudit}>📄 Cargar más</button>
          </div>
        {/if}
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
