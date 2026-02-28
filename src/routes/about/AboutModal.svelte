<script lang="ts">
  import { getVersion } from '@tauri-apps/api/app';

  let { onClose } = $props<{ onClose: () => void }>();

  let appVersion = $state('...');

  $effect(() => {
    getVersion().then(v => (appVersion = v));
  });

  function handleBackdrop(e: MouseEvent) {
    if ((e.target as HTMLElement).classList.contains('modal-overlay')) {
      onClose();
    }
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
<div class="modal-overlay" onclick={handleBackdrop}>
  <div class="modal about-modal">
    <!-- Header con logo -->
    <div class="about-header">
      <div class="about-logo">
        <div class="about-logo-icon">A</div>
      </div>
      <div class="about-title-block">
        <h2 class="about-app-name">AyniPOS</h2>
        <span class="about-version-badge">v{appVersion}</span>
      </div>
      <button class="modal-close-btn" onclick={onClose} aria-label="Cerrar">✕</button>
    </div>

    <p class="about-tagline">Sistema de Punto de Venta para pequeñas y medianas empresas · Hecho por <strong style="color: var(--text-primary);">Valnex Tech</strong></p>

    <div class="about-divider"></div>

    <!-- Info del desarrollador -->
    <div class="about-section">
      <div class="about-info-row">
        <span class="about-info-icon">🏢</span>
        <div>
          <div class="about-info-label">Desarrollado por</div>
          <div class="about-info-value">Valnex Tech</div>
        </div>
      </div>
      <div class="about-info-row">
        <span class="about-info-icon">🌐</span>
        <div>
          <div class="about-info-label">Sitio web</div>
          <div class="about-info-value about-link">valnextech.com</div>
        </div>
      </div>
      <div class="about-info-row">
        <span class="about-info-icon">📞</span>
        <div>
          <div class="about-info-label">Soporte (WhatsApp)</div>
          <div class="about-info-value about-link">+591 — — — — — —</div>
        </div>
      </div>
      <div class="about-info-row">
        <span class="about-info-icon">✉️</span>
        <div>
          <div class="about-info-label">Correo</div>
          <div class="about-info-value about-link">contacto@valnextech.com</div>
        </div>
      </div>
    </div>

    <div class="about-divider"></div>

    <div class="about-footer">
      <span>© {new Date().getFullYear()} Valnex Tech · Todos los derechos reservados</span>
    </div>

    <button class="btn btn-ghost btn-sm" style="width: 100%; margin-top: var(--space-md);" onclick={onClose}>
      Cerrar
    </button>
  </div>
</div>

<style>
  .about-modal {
    width: 420px;
    max-width: 95vw;
  }

  .about-header {
    display: flex;
    align-items: center;
    gap: var(--space-md);
    margin-bottom: var(--space-sm);
    position: relative;
  }

  .about-logo {
    flex-shrink: 0;
  }

  .about-logo-icon {
    width: 52px;
    height: 52px;
    border-radius: var(--radius-lg);
    background: linear-gradient(135deg, var(--accent-blue), #6366f1);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.75rem;
    font-weight: 800;
    color: white;
    box-shadow: 0 4px 16px rgba(59, 130, 246, 0.35);
  }

  .about-title-block {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .about-app-name {
    margin: 0;
    font-size: var(--font-size-xl);
    font-weight: 700;
    color: var(--text-primary);
    line-height: 1;
  }

  .about-version-badge {
    display: inline-block;
    background: rgba(59, 130, 246, 0.15);
    color: var(--accent-blue);
    border: 1px solid rgba(59, 130, 246, 0.3);
    border-radius: var(--radius-full);
    padding: 2px 10px;
    font-size: var(--font-size-xs);
    font-weight: 600;
    letter-spacing: 0.05em;
    width: fit-content;
  }

  .modal-close-btn {
    position: absolute;
    top: 0;
    right: 0;
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    font-size: var(--font-size-sm);
    padding: 4px 8px;
    border-radius: var(--radius-sm);
    transition: color 0.15s, background 0.15s;
  }

  .modal-close-btn:hover {
    color: var(--text-primary);
    background: var(--bg-hover);
  }

  .about-tagline {
    color: var(--text-muted);
    font-size: var(--font-size-sm);
    margin: 0 0 var(--space-md);
    line-height: 1.5;
  }

  .about-divider {
    height: 1px;
    background: var(--border-color);
    margin: var(--space-md) 0;
  }

  .about-section {
    display: flex;
    flex-direction: column;
    gap: var(--space-sm);
  }

  .about-info-row {
    display: flex;
    align-items: flex-start;
    gap: var(--space-md);
    padding: var(--space-sm) var(--space-md);
    border-radius: var(--radius-md);
    transition: background 0.15s;
  }

  .about-info-row:hover {
    background: var(--bg-hover);
  }

  .about-info-icon {
    font-size: 1.1rem;
    margin-top: 1px;
    flex-shrink: 0;
  }

  .about-info-label {
    font-size: var(--font-size-xs);
    color: var(--text-muted);
    margin-bottom: 2px;
  }

  .about-info-value {
    font-size: var(--font-size-sm);
    font-weight: 500;
    color: var(--text-primary);
  }

  .about-link {
    color: var(--accent-blue);
    cursor: pointer;
  }

  .about-footer {
    text-align: center;
    font-size: var(--font-size-xs);
    color: var(--text-muted);
  }
</style>
