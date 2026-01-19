<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let open = false;
  export let title = 'Confirm Action';
  export let message = 'Are you sure?';
  export let confirmLabel = 'Confirm';
  export let cancelLabel = 'Cancel';
  export let destructive = false;

  const dispatch = createEventDispatcher<{
    confirm: void;
    cancel: void;
  }>();

  function handleConfirm() {
    dispatch('confirm');
    open = false;
  }

  function handleCancel() {
    dispatch('cancel');
    open = false;
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      handleCancel();
    }
  }

  function handleBackdropClick(event: MouseEvent) {
    if (event.target === event.currentTarget) {
      handleCancel();
    }
  }
</script>

<svelte:window on:keydown={open ? handleKeydown : undefined} />

{#if open}
  <div
    class="dialog-backdrop"
    on:click={handleBackdropClick}
    role="presentation"
  >
    <div
      class="dialog-card"
      role="alertdialog"
      aria-modal="true"
      aria-labelledby="dialog-title"
      aria-describedby="dialog-message"
    >
      <h3 id="dialog-title">{title}</h3>
      <p id="dialog-message">{message}</p>
      <div class="dialog-actions">
        <button class="btn-secondary" on:click={handleCancel}>{cancelLabel}</button>
        <button
          class={destructive ? 'btn-danger' : 'btn-primary'}
          on:click={handleConfirm}
        >
          {confirmLabel}
        </button>
      </div>
    </div>
  </div>
{/if}
