# Whispr Linux Alternative - Settings Schema (MVP)

## Scope
- English-only transcription for MVP (`small.en` default).
- X11 hotkeys; Wayland paste automation via helper tools.
- Hotkeys and paste delay are configurable in Settings.

## Storage format
- SQLite `settings` table: `key` TEXT PRIMARY KEY, `value` TEXT.
- `value` is JSON-encoded to allow typed values and future expansion.

## Keys
| Key | Type | Default | Notes |
| --- | --- | --- | --- |
| audio.input_device_id | string | "default" | Use system default capture device on first run. |
| audio.sample_rate_hz | integer | 16000 | Record at 16kHz for `whisper.cpp` compatibility. |
| audio.channels | integer | 1 | Mono input for speech. |
| audio.input_gain_db | number | 0.0 | Optional software gain; 0 disables. |
| audio.noise_gate_enabled | boolean | false | Ignore audio below threshold while recording. |
| audio.noise_gate_threshold | number | 0.02 | RMS threshold for noise gate. |
| audio.vad_enabled | boolean | false | Auto-pause/resume capture when speech is detected. |
| audio.vad_threshold | number | 0.02 | RMS threshold for VAD detection. |
| audio.vad_silence_ms | integer | 800 | Silence duration before pausing capture. |
| audio.vad_resume_ms | integer | 200 | Speech duration before resuming capture. |
| hotkey.record_toggle | string | "CmdOrCtrl+Alt+Space" | Global start/stop recording; configurable to avoid conflicts. |
| hotkey.paste_last | string | "CmdOrCtrl+Alt+V" | Paste last transcript; configurable to avoid conflicts. |
| hotkey.open_app | string | "CmdOrCtrl+Alt+O" | Focus or show the main window; configurable to avoid conflicts. |
| transcription.model | string | "small.en" | English-only tier selection: tiny.en/small.en/medium.en. |
| transcription.model_dir | string | "~/.local/share/whispr/models" | Model storage directory. |
| transcription.threads | integer | 0 | 0 = auto. |
| transcription.language | string | "en" | Fixed for MVP; UI can hide this setting. |
| transcription.custom_vocab | string | "" | Bias phrases, acronyms, or names to improve accuracy. |
| transcription.use_gpu | boolean | false | Use GPU acceleration when available. |
| automation.auto_paste_enabled | boolean | true | Paste after transcription finishes (default behavior). |
| automation.paste_delay_ms | integer | 250 | Configurable delay before paste to allow focus to settle. |
| automation.copy_to_clipboard | boolean | true | Always keep transcript in clipboard. |
| automation.paste_method | string | "auto" | Auto-detects X11 vs Wayland; supports `x11_ctrl_v`, `wayland_wtype`, `wayland_ydotool`, `clipboard_only`. |
| storage.data_dir | string | "~/.local/share/whispr" | Root data directory for DB and cache. |
| storage.keep_audio | boolean | false | Keep raw audio files for debugging. |
| storage.retention_days | integer | 0 | 0 = keep forever. |
| app.launch_on_login | boolean | false | Launch on login. |
| app.start_in_tray | boolean | true | Launch minimized to tray. |
| app.close_to_tray | boolean | true | Closing the window hides the app instead of quitting (Windows). |
| ui.list_compact | boolean | false | Compact transcript list layout. |
| ui.onboarding_seen | boolean | false | Hide first-run onboarding. |
