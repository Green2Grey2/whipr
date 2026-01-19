<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { AudioDevice, RuntimeInfo, Settings } from '../api';
  import { onboardingSteps as steps } from '../onboarding';

  export let open = false;
  export let step = 0;
  export let settings: Settings | null = null;
  export let runtimeInfo: RuntimeInfo | null = null;
  export let audioDevices: AudioDevice[] = [];

  const dispatch = createEventDispatcher<{
    next: void;
    back: void;
    skipStep: void;
    skipAll: void;
    finish: void;
    openSettings: { section: 'audio' | 'hotkeys' | 'automation' | 'app' };
  }>();

  const totalSteps = steps.length;
  const recordHotkey = settings?.hotkeys.record_toggle ?? 'Ctrl+Alt+Space';
  const pasteHotkey = settings?.hotkeys.paste_last ?? 'Ctrl+Alt+V';
</script>

{#if open}
  <div class="onboard-backdrop" role="presentation">
    <div class="onboard-card" role="dialog" aria-modal="true" aria-labelledby="onboard-title">
      <header class="onboard-header">
        <div>
          <p class="onboard-kicker">Quick Start</p>
          <h2 id="onboard-title">{steps[step].title}</h2>
          <p class="onboard-subtitle">{steps[step].subtitle}</p>
        </div>
        <button class="onboard-skip" type="button" on:click={() => dispatch('skipAll')}>
          Skip tour
        </button>
      </header>

      <div class="onboard-progress">
        <span>Step {step + 1} of {totalSteps}</span>
        <div class="onboard-dots">
          {#each Array(totalSteps) as _, index}
            <span class={`onboard-dot ${index === step ? 'active' : ''}`}></span>
          {/each}
        </div>
      </div>

      <div class="onboard-body">
        {#key step}
          <div class="onboard-step">
            {#if step === 0}
              <div class="onboard-grid">
                <div>
                  <h3>What Whispr does</h3>
                  <ul>
                    <li>Record audio with a single shortcut.</li>
                    <li>Transcribe locally on your device.</li>
                    <li>Copy and format your notes instantly.</li>
                  </ul>
                </div>
                <div class="onboard-example">
                  <p class="onboard-example-title">Example transcript</p>
                  <p>“Meeting Notes: [Topic] - [Date]”</p>
                  <p>“Action items: send invoice, schedule a follow‑up.”</p>
                </div>
              </div>
            {:else if step === 1}
              <div class="onboard-grid">
                <div>
                  <h3>Recommended setup</h3>
                  <p>Use a headset or an external mic when possible.</p>
                  <button
                    class="btn-secondary"
                    type="button"
                    on:click={() => dispatch('openSettings', { section: 'audio' })}
                  >
                    Open audio settings
                  </button>
                </div>
                <div class="onboard-example">
                  <p class="onboard-example-title">Detected devices</p>
                  {#if audioDevices.length > 0}
                    <ul>
                      {#each audioDevices.slice(0, 4) as device}
                        <li>{device.name}{device.is_default ? ' (default)' : ''}</li>
                      {/each}
                    </ul>
                  {:else}
                    <p>No devices yet — open settings to refresh.</p>
                  {/if}
                </div>
              </div>
            {:else if step === 2}
              <div class="onboard-grid">
                <div>
                  <h3>Speed with hotkeys</h3>
                  <p>Start/stop recording with <strong>{recordHotkey}</strong>.</p>
                  <p>Paste last transcript with <strong>{pasteHotkey}</strong>.</p>
                  <button
                    class="btn-secondary"
                    type="button"
                    on:click={() => dispatch('openSettings', { section: 'hotkeys' })}
                  >
                    Edit shortcuts
                  </button>
                </div>
                <div class="onboard-example">
                  <p class="onboard-example-title">System note</p>
                  {#if runtimeInfo?.session_type === 'wayland'}
                    <p>Wayland limits hotkeys and paste helpers on some systems.</p>
                  {:else if runtimeInfo?.session_type === 'windows'}
                    <p>Windows tray supports click‑to‑record and quick access.</p>
                  {:else}
                    <p>Shortcuts work best on X11 and Windows.</p>
                  {/if}
                </div>
              </div>
            {:else}
              <div class="onboard-grid">
                <div>
                  <h3>Make it yours</h3>
                  <p>Pick copy formats: plain, markdown, or bullet list.</p>
                  <p>Enable auto‑paste for instant delivery.</p>
                  <button
                    class="btn-secondary"
                    type="button"
                    on:click={() => dispatch('openSettings', { section: 'automation' })}
                  >
                    Review automation
                  </button>
                </div>
                <div class="onboard-example">
                  <p class="onboard-example-title">Quick workflow</p>
                  <ol>
                    <li>Record</li>
                    <li>Preview</li>
                    <li>Copy or paste</li>
                  </ol>
                </div>
              </div>
            {/if}
          </div>
        {/key}
      </div>

      <footer class="onboard-footer">
        <div class="onboard-actions">
          <button class="btn-secondary" type="button" on:click={() => dispatch('skipStep')}>
            Skip step
          </button>
          <button class="btn-secondary" type="button" on:click={() => dispatch('back')} disabled={step === 0}>
            Back
          </button>
        </div>
        <div class="onboard-actions">
          {#if step < totalSteps - 1}
            <button class="btn-primary" type="button" on:click={() => dispatch('next')}>
              Next
            </button>
          {:else}
            <button class="btn-primary" type="button" on:click={() => dispatch('finish')}>
              Finish setup
            </button>
          {/if}
        </div>
      </footer>
    </div>
  </div>
{/if}
