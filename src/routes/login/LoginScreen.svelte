<script lang="ts">
  import { loginWithPin } from '$lib/services/api';
  import type { User } from '$lib/types';

  let { onLogin }: { onLogin: (user: User) => void } = $props();

  let pin = $state('');
  let error = $state('');
  let shake = $state(false);
  let loading = $state(false);
  const maxDigits = 6;

  function addDigit(digit: string) {
    if (pin.length >= maxDigits) return;
    pin += digit;
    error = '';
  }

  function removeDigit() {
    pin = pin.slice(0, -1);
    error = '';
  }

  function clearPin() {
    pin = '';
    error = '';
  }

  async function submitPin() {
    if (pin.length < 4) {
      error = 'El PIN debe tener al menos 4 dígitos';
      triggerShake();
      return;
    }
    loading = true;
    try {
      const user = await loginWithPin(pin);
      onLogin(user);
    } catch {
      showError();
    }
    loading = false;
  }

  function showError() {
    error = 'PIN incorrecto';
    triggerShake();
    pin = '';
  }

  function triggerShake() {
    shake = true;
    setTimeout(() => { shake = false; }, 500);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key >= '0' && e.key <= '9') {
      e.preventDefault();
      addDigit(e.key);
    } else if (e.key === 'Backspace') {
      e.preventDefault();
      removeDigit();
    } else if (e.key === 'Enter') {
      e.preventDefault();
      submitPin();
    } else if (e.key === 'Escape') {
      e.preventDefault();
      clearPin();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="login-screen">
  <div class="login-container" class:shake>
    <!-- Logo -->
    <div class="login-logo">
      <div class="login-logo-icon">A</div>
      <h1 class="login-title">AyniPOS</h1>
      <p class="login-subtitle">Ingrese su PIN para continuar</p>
    </div>

    <!-- PIN dots -->
    <div class="pin-display">
      {#each Array(maxDigits) as _, i}
        <div class="pin-dot" class:filled={i < pin.length} class:active={i === pin.length}></div>
      {/each}
    </div>

    <!-- Error message -->
    {#if error}
      <div class="login-error">{error}</div>
    {/if}

    <!-- Numeric keypad -->
    <div class="numpad">
      {#each ['1','2','3','4','5','6','7','8','9'] as digit}
        <button class="numpad-btn" onclick={() => addDigit(digit)} disabled={loading}>
          {digit}
        </button>
      {/each}
      <button class="numpad-btn numpad-fn" onclick={clearPin} disabled={loading}>
        C
      </button>
      <button class="numpad-btn" onclick={() => addDigit('0')} disabled={loading}>
        0
      </button>
      <button class="numpad-btn numpad-fn" onclick={removeDigit} disabled={loading}>
        ⌫
      </button>
    </div>

    <!-- Submit button -->
    <button
      class="btn btn-primary login-submit"
      onclick={submitPin}
      disabled={pin.length < 4 || loading}
    >
      {loading ? '⏳ Verificando...' : '🔓 Ingresar'}
    </button>

    <p class="login-hint">PIN por defecto: 1234</p>
  </div>
</div>

<style>
  .login-screen {
    position: fixed;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, #0f1117 0%, #1a1d2e 50%, #0f1117 100%);
    z-index: 9999;
  }

  .login-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1.5rem;
    padding: 2.5rem;
    background: var(--bg-secondary, #1a1d2e);
    border: 1px solid var(--border-color, #2a2d3e);
    border-radius: 1.5rem;
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5), 0 0 80px rgba(59, 130, 246, 0.08);
    width: 340px;
    transition: transform 0.1s;
  }

  .login-container.shake {
    animation: shake 0.5s ease-in-out;
  }

  @keyframes shake {
    0%, 100% { transform: translateX(0); }
    10%, 30%, 50%, 70%, 90% { transform: translateX(-6px); }
    20%, 40%, 60%, 80% { transform: translateX(6px); }
  }

  .login-logo {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
  }

  .login-logo-icon {
    width: 64px;
    height: 64px;
    background: linear-gradient(135deg, #3b82f6, #8b5cf6);
    border-radius: 1rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 2rem;
    font-weight: 800;
    color: white;
    box-shadow: 0 8px 24px rgba(59, 130, 246, 0.3);
  }

  .login-title {
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--text-primary, #e2e8f0);
    margin: 0;
  }

  .login-subtitle {
    font-size: 0.85rem;
    color: var(--text-muted, #64748b);
    margin: 0;
  }

  .pin-display {
    display: flex;
    gap: 0.75rem;
    padding: 0.75rem 0;
  }

  .pin-dot {
    width: 16px;
    height: 16px;
    border-radius: 50%;
    border: 2px solid var(--border-color, #2a2d3e);
    background: transparent;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .pin-dot.filled {
    background: #3b82f6;
    border-color: #3b82f6;
    box-shadow: 0 0 8px rgba(59, 130, 246, 0.4);
    transform: scale(1.1);
  }

  .pin-dot.active {
    border-color: #3b82f6;
    animation: pulse 1.5s infinite;
  }

  @keyframes pulse {
    0%, 100% { box-shadow: 0 0 0 0 rgba(59, 130, 246, 0.3); }
    50% { box-shadow: 0 0 0 6px rgba(59, 130, 246, 0); }
  }

  .login-error {
    color: #ef4444;
    font-size: 0.85rem;
    font-weight: 500;
    padding: 0.4rem 1rem;
    background: rgba(239, 68, 68, 0.1);
    border-radius: 0.5rem;
    border: 1px solid rgba(239, 68, 68, 0.2);
  }

  .numpad {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 0.5rem;
    width: 100%;
  }

  .numpad-btn {
    height: 56px;
    border: 1px solid var(--border-color, #2a2d3e);
    border-radius: 0.75rem;
    background: var(--bg-tertiary, #252836);
    color: var(--text-primary, #e2e8f0);
    font-size: 1.25rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.15s ease;
    user-select: none;
    -webkit-user-select: none;
  }

  .numpad-btn:hover:not(:disabled) {
    background: var(--bg-hover, #2a2d3e);
    border-color: #3b82f6;
    transform: scale(1.02);
  }

  .numpad-btn:active:not(:disabled) {
    transform: scale(0.96);
    background: rgba(59, 130, 246, 0.15);
  }

  .numpad-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .numpad-fn {
    font-size: 0.9rem;
    color: var(--text-muted, #64748b);
  }

  .login-submit {
    width: 100%;
    height: 48px;
    font-size: 1rem;
    font-weight: 600;
    border-radius: 0.75rem;
  }

  .login-hint {
    font-size: 0.75rem;
    color: var(--text-muted, #64748b);
    margin: 0;
    opacity: 0.6;
  }
</style>
