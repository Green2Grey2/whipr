<script lang="ts">
  import { theme, type ThemePreference } from '../theme';

  let current: ThemePreference;
  theme.subscribe((value) => (current = value));

  function handleChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    theme.setTheme(target.value as ThemePreference);
  }
</script>

<div class="settings-row">
  <div class="settings-label">
    <label for="theme-select">Theme</label>
  </div>
  <div class="settings-control">
    <select id="theme-select" class="select-compact" value={current} on:change={handleChange}>
      <option value="system">System</option>
      <option value="light">Light</option>
      <option value="dark">Dark</option>
    </select>
  </div>
</div>
