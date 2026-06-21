import type { Setting } from '$lib/types';

export const UI_THEME_SETTING_KEY = 'ui_theme';
export const DEFAULT_UI_THEME = 'dark'; // Tema oscuro por defecto

export type AppTheme = 'dark' | 'light-modern';

const VALID_UI_THEMES = new Set<AppTheme>(['dark', 'light-modern']);

/** Clave de localStorage por usuario */
export function userThemeKey(userId: string): string {
  return `app_theme_${userId}`;
}

/** Lee el tema guardado para un usuario */
export function getUserTheme(userId: string): AppTheme {
  const saved = localStorage.getItem(userThemeKey(userId));
  if (saved === 'dark' || saved === 'light-modern') return saved;
  return DEFAULT_UI_THEME;
}

/** Guarda el tema elegido por el usuario */
export function saveUserTheme(userId: string, theme: AppTheme): void {
  localStorage.setItem(userThemeKey(userId), theme);
}

export function resolveUiTheme(theme: string | null | undefined): AppTheme {
  if (!theme) return DEFAULT_UI_THEME;
  return VALID_UI_THEMES.has(theme as AppTheme) ? (theme as AppTheme) : DEFAULT_UI_THEME;
}

export function applyUiTheme(theme: string | null | undefined): AppTheme {
  const resolved = resolveUiTheme(theme);
  document.documentElement.dataset.theme = resolved;
  document.documentElement.style.colorScheme = resolved === 'dark' ? 'dark' : 'light';
  return resolved;
}

export function getUiThemeFromSettings(settings: Setting[]): {
  resolvedTheme: AppTheme;
  needsPersist: boolean;
} {
  const rawTheme = settings.find((s) => s.key === UI_THEME_SETTING_KEY)?.value;
  const resolvedTheme = resolveUiTheme(rawTheme);
  const needsPersist = !rawTheme || rawTheme !== resolvedTheme;

  return { resolvedTheme, needsPersist };
}
