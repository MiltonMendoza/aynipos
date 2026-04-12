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
    background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-tertiary) 50%, var(--bg-primary) 100%);
    z-index: 9999;
  }

  .login-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1.5rem;
    padding: 2.5rem;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 1.5rem;
    box-shadow: var(--shadow-xl), 0 0 60px var(--accent-primary-glow);
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
    background: var(--gradient-brand);
    border-radius: 1rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 2rem;
    font-weight: 800;
    color: white;
    box-shadow: 0 8px 24px var(--accent-primary-glow);
  }

  .login-title {
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--text-primary);
    margin: 0;
  }

  .login-subtitle {
    font-size: 0.85rem;
    color: var(--text-muted);
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
    border: 2px solid var(--border-color);
    background: transparent;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .pin-dot.filled {
    background: var(--accent-primary);
    border-color: var(--accent-primary);
    box-shadow: 0 0 8px color-mix(in srgb, var(--accent-primary) 35%, transparent);
    transform: scale(1.1);
  }

  .pin-dot.active {
    border-color: var(--accent-primary);
    animation: pulse 1.5s infinite;
  }

  @keyframes pulse {
    0%, 100% { box-shadow: 0 0 0 0 color-mix(in srgb, var(--accent-primary) 28%, transparent); }
    50% { box-shadow: 0 0 0 6px transparent; }
  }

  .login-error {
    color: var(--accent-danger);
    font-size: 0.85rem;
    font-weight: 500;
    padding: 0.4rem 1rem;
    background: var(--accent-danger-glow);
    border-radius: 0.5rem;
    border: 1px solid color-mix(in srgb, var(--accent-danger) 28%, transparent);
  }

  .numpad {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 0.5rem;
    width: 100%;
  }

  .numpad-btn {
    height: 56px;
    border: 1px solid var(--border-color);
    border-radius: 0.75rem;
    background: var(--bg-tertiary);
    color: var(--text-primary);
    font-size: 1.25rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.15s ease;
    user-select: none;
    -webkit-user-select: none;
  }

  .numpad-btn:hover:not(:disabled) {
    background: var(--bg-hover);
    border-color: var(--accent-primary);
    transform: scale(1.02);
  }

  .numpad-btn:active:not(:disabled) {
    transform: scale(0.96);
    background: var(--accent-primary-glow);
  }

  .numpad-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .numpad-fn {
    font-size: 0.9rem;
    color: var(--text-muted);
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
    color: var(--text-muted);
    margin: 0;
    opacity: 0.6;
  }
</style>
