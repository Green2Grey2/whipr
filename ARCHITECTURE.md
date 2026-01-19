# Whispr Linux Alternative - MVP Architecture

## Recommendation (MVP tech stack)
- Desktop framework: Tauri 2 (Rust backend, lightweight web UI).
- UI: Svelte + Vite for fast startup and small bundles.
- Audio capture: Rust `cpal` for device input and stream handling.
- Transcription engine: `whisper.cpp` via `whisper-rs` for CPU-first inference.
- Storage: SQLite via `rusqlite` for transcripts and settings.
- Hotkeys: `tauri-plugin-global-shortcut` for X11 global hotkeys (Wayland uses in-app controls).
- Clipboard and paste: `arboard` + `enigo` on X11, `wl-clipboard` + `wtype`/`ydotool` on Wayland.
- Notifications: Tauri notification plugin for status and errors.
- Packaging: AppImage for MVP distribution.

## Architecture overview
- UI layer: rendering, list/search, settings, and status.
- Core service (Rust): audio, transcription, hotkeys, paste automation, storage.
- Worker threads: audio capture and transcription run off the UI thread.
- Data store: local SQLite database for transcripts and preferences.

## Component breakdown
### UI (Svelte)
- Transcripts list, detail view, and search.
- Settings page for hotkeys, model, and automation.
- Record status indicator and action buttons.
- Receives events from core (recording state, progress, errors).

### Core service (Rust)
- Audio manager: device selection, stream start/stop, ring buffer.
- Transcription manager: model loading, inference, language selection.
- Hotkey manager: global shortcuts and conflicts.
- Automation manager: clipboard + paste injection.
- Storage manager: SQLite reads/writes, retention policy.

## Data flow
1) User hits hotkey.
2) Hotkey manager triggers recording start.
3) Audio manager streams PCM into buffer.
4) User hits stop hotkey.
5) Transcription manager runs inference and returns text.
6) Storage manager persists transcript and metadata.
7) UI updates list and shows notification.
8) If enabled, automation manager pastes into last focused field.

## Model management
- Default models are stored in `~/.local/share/whispr/models`.
- UI exposes model selection and download location.
- Core validates model files on load and reports issues to UI.
- Allow multiple models and quick switching.
- Default model: `whisper.cpp` small.en (English-only).
- Language is fixed to English for MVP; no auto-detect.
- Allow switching between English-only tiers (tiny.en/small.en/medium.en).
- Model manager supports download, activate, cycle, and delete actions from settings.
- Installed models are discovered from the model directory and tracked in a local manifest.

## Storage schema (MVP)
### transcripts
- id (integer, pk)
- created_at (unix ms)
- duration_ms (integer)
- text (string)
- language (string, nullable)
- tags (string, nullable)
- source (string, "mic")

### settings
- key (string, pk)
- value (string)

## OS integration notes
- X11 and Wayland sessions are supported; Wayland requires `wl-clipboard` + `wtype`/`ydotool`.
- Global hotkeys are X11-only; Wayland uses in-app controls.
- Focused window is sampled at stop for paste; fallback to last-known focus if needed.
- Audio stack assumes PipeWire + WirePlumber with ALSA devices available.
- Default input device is the system default capture device on first run.
- User-selected device is persisted; if missing, fall back to default and notify.

## Performance targets
- App cold start <2s, warm start <1s.
- Transcription for <=10s audio completes under 2x audio duration.
- Idle CPU <2% and memory <250MB.

## Risks and mitigations
- Wayland helper dependency: document `wl-clipboard` + `wtype`/`ydotool` and fallback to clipboard-only.
- Model size: allow location choice and clean-up tools.
- CPU load: default to small/medium models, optional GPU later.

## Future extensions
- GPU acceleration path (CUDA/ROCm) for faster transcription.
- Per-app auto-paste rules.
- Live partial transcription in the UI.
