<script lang="ts">
  import { getLicenseStatus, activateLicense } from '$lib/services/api';
  import type { LicenseStatus } from '$lib/types';

  let { onActivated }: { onActivated: () => void } = $props();

  let licenseKey = $state('');
  let error = $state('');
  let loading = $state(false);
  let machineId = $state('');
  let copied = $state(false);

  // Load machine ID on mount
  $effect(() => {
    getLicenseStatus().then(status => {
      machineId = status.machine_id;
    });
  });

  async function handleActivate() {
    if (!licenseKey.trim()) {
      error = 'Ingrese una clave de licencia';
      return;
    }
    loading = true;
    error = '';
    try {
      await activateLicense(licenseKey.trim());
      onActivated();
    } catch (e) {
      error = String(e);
    }
    loading = false;
  }

  async function copyMachineId() {
    try {
      await navigator.clipboard.writeText(machineId);
      copied = true;
      setTimeout(() => { copied = false; }, 2000);
    } catch {
      // Fallback
      const input = document.createElement('input');
      input.value = machineId;
      document.body.appendChild(input);
      input.select();
      document.execCommand('copy');
      document.body.removeChild(input);
      copied = true;
      setTimeout(() => { copied = false; }, 2000);
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      e.preventDefault();
      handleActivate();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="activation-screen">
  <div class="activation-container">
    <!-- Logo -->
    <div class="activation-logo">
      <div class="activation-logo-icon">A</div>
      <h1 class="activation-title">AyniPOS</h1>
      <p class="activation-subtitle">Activación de Licencia</p>
    </div>

    <!-- Expired notice -->
    <div class="activation-notice">
      <span class="notice-icon">⚠️</span>
      <span>Su periodo de prueba ha finalizado. Active una licencia para continuar usando AyniPOS.</span>
    </div>

    <!-- Machine ID -->
    <div class="machine-id-section">
      <label class="machine-id-label">Código de Máquina</label>
      <div class="machine-id-row">
        <code class="machine-id-code">{machineId || '...'}</code>
        <button
          class="btn-copy"
          onclick={copyMachineId}
          title="Copiar código de máquina"
        >
          {copied ? '✅' : '📋'}
        </button>
      </div>
      <p class="machine-id-hint">Envíe este código a soporte para obtener su licencia</p>
    </div>

    <!-- License key input -->
    <div class="license-input-section">
      <label class="license-label" for="license-key">Clave de Licencia</label>
      <textarea
        id="license-key"
        class="license-input"
        placeholder="Pegue aquí su clave de licencia..."
        bind:value={licenseKey}
        rows="3"
      ></textarea>
    </div>

    <!-- Error message -->
    {#if error}
      <div class="activation-error">{error}</div>
    {/if}

    <!-- Activate button -->
    <button
      class="btn btn-primary activation-submit"
      onclick={handleActivate}
      disabled={!licenseKey.trim() || loading}
    >
      {loading ? '⏳ Verificando...' : '🔓 Activar Licencia'}
    </button>

    <p class="activation-contact">
      📞 Contacte a soporte: <strong>+591 XXXXXXX</strong>
    </p>
  </div>
</div>

<style>
  .activation-screen {
    position: fixed;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-tertiary) 50%, var(--bg-primary) 100%);
    z-index: 9999;
  }

  .activation-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1.25rem;
    padding: 2.5rem;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 1.5rem;
    box-shadow: var(--shadow-xl), 0 0 60px var(--accent-danger-glow);
    width: 420px;
    max-width: 95vw;
  }

  .activation-logo {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
  }

  .activation-logo-icon {
    width: 64px;
    height: 64px;
    background: linear-gradient(135deg, var(--accent-danger), var(--accent-warning));
    border-radius: 1rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 2rem;
    font-weight: 800;
    color: white;
    box-shadow: 0 8px 24px var(--accent-danger-glow);
  }

  .activation-title {
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--text-primary);
    margin: 0;
  }

  .activation-subtitle {
    font-size: 0.85rem;
    color: var(--text-muted);
    margin: 0;
  }

  .activation-notice {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    background: var(--accent-warning-glow);
    border: 1px solid color-mix(in srgb, var(--accent-warning) 25%, transparent);
    border-radius: 0.75rem;
    font-size: 0.8rem;
    color: var(--accent-warning);
    line-height: 1.4;
    width: 100%;
  }

  .notice-icon {
    font-size: 1.2rem;
    flex-shrink: 0;
  }

  .machine-id-section {
    width: 100%;
    text-align: center;
  }

  .machine-id-label {
    font-size: 0.75rem;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    font-weight: 600;
  }

  .machine-id-row {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    margin-top: 0.5rem;
  }

  .machine-id-code {
    font-size: 1.5rem;
    font-weight: 700;
    letter-spacing: 0.15em;
    color: var(--accent-primary);
    background: var(--accent-primary-glow);
    padding: 0.5rem 1rem;
    border-radius: 0.5rem;
    border: 1px solid color-mix(in srgb, var(--accent-primary) 25%, transparent);
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
  }

  .btn-copy {
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: 0.5rem;
    padding: 0.5rem;
    font-size: 1.2rem;
    cursor: pointer;
    transition: all 0.15s ease;
    line-height: 1;
  }

  .btn-copy:hover {
    background: var(--bg-hover);
    border-color: var(--accent-primary);
  }

  .machine-id-hint {
    font-size: 0.7rem;
    color: var(--text-muted);
    margin: 0.5rem 0 0;
    opacity: 0.7;
  }

  .license-input-section {
    width: 100%;
  }

  .license-label {
    font-size: 0.8rem;
    font-weight: 600;
    color: var(--text-secondary);
    display: block;
    margin-bottom: 0.5rem;
  }

  .license-input {
    width: 100%;
    padding: 0.75rem;
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: 0.5rem;
    color: var(--text-primary);
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    font-size: 0.8rem;
    resize: none;
    transition: border-color 0.2s ease;
    box-sizing: border-box;
  }

  .license-input:focus {
    outline: none;
    border-color: var(--accent-primary);
    box-shadow: 0 0 0 2px color-mix(in srgb, var(--accent-primary) 22%, transparent);
  }

  .license-input::placeholder {
    color: var(--text-muted);
    opacity: 0.6;
  }

  .activation-error {
    color: var(--accent-danger);
    font-size: 0.8rem;
    font-weight: 500;
    padding: 0.5rem 1rem;
    background: var(--accent-danger-glow);
    border-radius: 0.5rem;
    border: 1px solid color-mix(in srgb, var(--accent-danger) 25%, transparent);
    width: 100%;
    text-align: center;
  }

  .activation-submit {
    width: 100%;
    height: 48px;
    font-size: 1rem;
    font-weight: 600;
    border-radius: 0.75rem;
  }

  .activation-contact {
    font-size: 0.75rem;
    color: var(--text-muted);
    margin: 0;
    opacity: 0.6;
  }
</style>
