import type { Setting } from '$lib/types';

export const UI_THEME_SETTING_KEY = 'ui_theme';
export const DEFAULT_UI_THEME = 'light-modern';

const VALID_UI_THEMES = new Set([DEFAULT_UI_THEME]);

export function resolveUiTheme(theme: string | null | undefined): string {
  if (!theme) return DEFAULT_UI_THEME;
  return VALID_UI_THEMES.has(theme) ? theme : DEFAULT_UI_THEME;
}

export function applyUiTheme(theme: string | null | undefined): string {
  const resolved = resolveUiTheme(theme);
  document.documentElement.dataset.theme = resolved;
  document.documentElement.style.colorScheme = 'light';
  return resolved;
}

export function getUiThemeFromSettings(settings: Setting[]): {
  resolvedTheme: string;
  needsPersist: boolean;
} {
  const rawTheme = settings.find((s) => s.key === UI_THEME_SETTING_KEY)?.value;
  const resolvedTheme = resolveUiTheme(rawTheme);
  const needsPersist = !rawTheme || rawTheme !== resolvedTheme;

  return { resolvedTheme, needsPersist };
}
