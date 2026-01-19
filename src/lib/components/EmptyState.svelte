<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let icon: 'microphone' | 'search' | 'bookmark' = 'microphone';
  export let title = 'Nothing here yet';
  export let description = '';
  export let actionLabel = '';

  const dispatch = createEventDispatcher<{ action: void }>();

  const icons = {
    microphone: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
      <path d="M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z"/>
      <path d="M19 10v2a7 7 0 0 1-14 0v-2"/>
      <line x1="12" y1="19" x2="12" y2="23"/>
      <line x1="8" y1="23" x2="16" y2="23"/>
    </svg>`,
    search: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
      <circle cx="11" cy="11" r="8"/>
      <line x1="21" y1="21" x2="16.65" y2="16.65"/>
    </svg>`,
    bookmark: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
      <path d="M6 3h12a1 1 0 0 1 1 1v17l-7-4-7 4V4a1 1 0 0 1 1-1z"/>
    </svg>`,
  };
</script>

<div class="empty-state" role="status">
  <div class="empty-icon" aria-hidden="true">
    {@html icons[icon]}
  </div>
  <h3 class="empty-title">{title}</h3>
  {#if description}
    <p class="empty-description">{description}</p>
  {/if}
  {#if actionLabel}
    <button class="btn-primary" on:click={() => dispatch('action')}>
      {actionLabel}
    </button>
  {/if}
</div>
