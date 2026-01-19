import { writable } from 'svelte/store';

export type ThemePreference = 'system' | 'light' | 'dark';

const STORAGE_KEY = 'whispr-theme';

function getInitialTheme(): ThemePreference {
  if (typeof localStorage === 'undefined') return 'system';
  const stored = localStorage.getItem(STORAGE_KEY);
  if (stored === 'light' || stored === 'dark' || stored === 'system') {
    return stored;
  }
  return 'system';
}

function getEffectiveTheme(preference: ThemePreference): 'light' | 'dark' {
  if (preference !== 'system') {
    return preference;
  }
  if (typeof window === 'undefined') return 'light';
  return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
}

function applyTheme(theme: 'light' | 'dark') {
  if (typeof document === 'undefined') return;
  document.documentElement.setAttribute('data-theme', theme);
}

function createThemeStore() {
  const initial = getInitialTheme();
  const { subscribe, set } = writable<ThemePreference>(initial);

  // Apply initial theme
  applyTheme(getEffectiveTheme(initial));

  // Listen for system preference changes
  if (typeof window !== 'undefined') {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    mediaQuery.addEventListener('change', (e) => {
      const current = localStorage.getItem(STORAGE_KEY) as ThemePreference;
      if (current === 'system' || !current) {
        applyTheme(e.matches ? 'dark' : 'light');
      }
    });
  }

  return {
    subscribe,
    setTheme: (preference: ThemePreference) => {
      if (typeof localStorage !== 'undefined') {
        localStorage.setItem(STORAGE_KEY, preference);
      }
      applyTheme(getEffectiveTheme(preference));
      set(preference);
    },
  };
}

export const theme = createThemeStore();
