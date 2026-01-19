import { register, unregisterAll } from '@tauri-apps/plugin-global-shortcut';
import { getCurrentWindow } from '@tauri-apps/api/window';
import type { Settings } from './api';

export type HotkeyHandlers = {
  onToggle: () => Promise<void>;
  onPasteLast: () => Promise<void>;
};

const modifierKeys = new Set(['CmdOrCtrl', 'CommandOrControl', 'Cmd', 'Ctrl', 'Control', 'Alt', 'Shift']);

const normalizeHotkey = (value: string) =>
  value
    .split('+')
    .map((part) => {
      const normalized = part.trim();
      if (normalized.toLowerCase() === 'ctrl' || normalized.toLowerCase() === 'control') {
        return 'CmdOrCtrl';
      }
      return normalized;
    })
    .filter(Boolean)
    .join('+');

const hasNonModifier = (value: string) => {
  const parts = value.split('+').map((part) => part.trim()).filter(Boolean);
  return parts.some((part) => !modifierKeys.has(part));
};

export const validateHotkeys = (settings: Settings): string | null => {
  const entries: Array<[string, string]> = [
    ['Record toggle', normalizeHotkey(settings.hotkeys.record_toggle)],
    ['Paste last', normalizeHotkey(settings.hotkeys.paste_last)],
    ['Open app', normalizeHotkey(settings.hotkeys.open_app)],
  ];

  for (const [label, combo] of entries) {
    if (!combo) {
      return `${label} hotkey is required.`;
    }
    if (!hasNonModifier(combo)) {
      return `${label} hotkey needs a non-modifier key.`;
    }
  }

  const map = new Map<string, string[]>();
  for (const [label, combo] of entries) {
    if (!map.has(combo)) map.set(combo, []);
    map.get(combo)?.push(label);
  }

  for (const [combo, labels] of map.entries()) {
    if (labels.length > 1) {
      return `Duplicate hotkeys detected (${combo}): ${labels.join(' and ')}.`;
    }
  }

  return null;
};

export const registerHotkeys = async (settings: Settings, handlers: HotkeyHandlers) => {
  const validationError = validateHotkeys(settings);
  if (validationError) {
    throw new Error(validationError);
  }

  await unregisterAll();

  await register(normalizeHotkey(settings.hotkeys.record_toggle), async (event) => {
    if (event.state !== 'Pressed') return;
    await handlers.onToggle();
  });

  await register(normalizeHotkey(settings.hotkeys.paste_last), async (event) => {
    if (event.state !== 'Pressed') return;
    await handlers.onPasteLast();
  });

  await register(normalizeHotkey(settings.hotkeys.open_app), async (event) => {
    if (event.state !== 'Pressed') return;
    const window = getCurrentWindow();
    await window.show();
    await window.setFocus();
  });
};
