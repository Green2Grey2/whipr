# Whispr Linux Alternative - PRD

## Summary
Build a Linux-native desktop app for fast voice transcription with a polished UI. It must support hotkeys, show a transcript history, provide a settings page, and optionally auto-paste text into the last focused field. Transcription should run locally using a fast model.

## Goals
- Linux-first, low-latency transcription and paste.
- Simple, attractive UI with transcript history and settings.
- Reliable hotkey capture for start/stop and quick actions.
- Local, offline transcription by default.
- Noticeably faster than cloud workflows for short dictations.

## Non-goals
- Cloud-only transcription.
- Mobile apps.
- Cross-platform parity on day one (focus Linux).
- Real-time live captions (MVP is stop-to-transcribe).
- Multi-language transcription (English-only MVP).

## Assumptions and constraints
- Target environments are GNOME and KDE on X11 or Wayland.
- Wayland auto-paste relies on `wl-clipboard` plus `wtype`/`ydotool`.
- Global hotkeys are X11-only until a portal-based solution is added.
- Local model files are large and must be user-managed (download path, cleanup).
- Microphone access and background execution must be granted by the OS.

## Target users
- Developers and writers who dictate short snippets.
- Support agents who need rapid transcription into forms.
- Power users who want hotkeys and automation.

## User stories
- As a user, I can press a hotkey and start/stop recording without switching windows.
- As a user, I can see and copy past transcripts from a history list.
- As a user, I can configure auto-paste into the last focused field.
- As a user, I can choose a local transcription model and quality/perf mode.
- As a user, I can disable telemetry and keep all data local.

## User journeys (detailed)
1) Quick dictation with auto-paste
   - Pre: app running in tray, hotkey set, auto-paste enabled.
   - Steps: press hotkey -> record -> press hotkey -> transcript generated -> auto-paste.
   - Success: text appears in the last focused field within a short delay.
2) Dictation without auto-paste
   - Pre: app running, auto-paste disabled.
   - Steps: hotkey record -> stop -> transcript appears -> user copies.
   - Success: transcript is saved in history and copy works.
3) Review and reuse history
   - Pre: prior transcripts exist.
   - Steps: open app -> search/filter -> select item -> copy or paste.
   - Success: search returns relevant items and copy matches the stored text.
4) First-time setup
   - Steps: open settings -> select mic -> choose model -> set hotkeys -> test.
   - Success: settings persist after restart and recording works.
5) Error recovery
   - Pre: mic blocked or model missing.
   - Steps: attempt record -> see error -> follow guidance -> retry.
   - Success: user can resolve without manual config edits.

## Core requirements
### Functional
- Global hotkeys: start/stop recording (toggle), quick paste last transcript, open app.
- Transcription: local model inference, fast enough for short snippets (<10s).
- History: list view with timestamp, duration, optional tags, search/filter, click to copy.
- Settings: hotkeys, auto-paste, model selection, storage location, startup.
- Auto-paste: when enabled, paste into the focused field immediately after stop.
- Record toggle: same hotkey starts recording and stops to paste.
- Model management: download, activate, cycle, delete models in settings.
- Export: copy transcript, save as text, or quick paste.
- Errors: user-friendly error states for mic/model access.

### Non-functional
- Startup <2s on a typical Linux dev machine.
- Transcription turnaround <2x audio duration for small snippets.
- Offline by default; no network required.
- Works on GNOME and KDE on X11 or Wayland (with helpers).
- Minimal CPU/GPU use while idle.
- X11 and Wayland helper dependencies are documented for hotkeys/paste injection.

## UX and UI
- Single-window UI with two main tabs: "Transcripts" and "Settings".
- Transcripts: left list + right detail pane, or a single list with expandable items.
- Settings: grouped sections (Audio, Hotkeys, Transcription, Automation, Privacy).
- Clear recording state with timer and waveform/level indicator.
- Old transcripts searchable and sortable by date.

## Information architecture
- Home/Transcripts
  - Record button + status
  - Recent list
  - Search/filter
- Settings
  - Audio: input device, gain, noise suppression
  - Hotkeys: start/stop, paste last, open app
  - Transcription: English-only model tier (tiny/small/medium)
  - Model management: download, activate, delete, cycle installed models
  - Automation: auto-paste enable, delay, target app behavior
  - Storage: data location, retention policy
  - Privacy: telemetry toggle, local-only confirmation

## Key flows
1) Hotkey dictation
   - User hits hotkey -> app records -> user hits same hotkey -> transcript appears -> auto-paste if enabled.
2) Review history
   - User opens app -> sees list -> clicks item to copy -> optional paste.
3) Configuration
   - User opens settings -> chooses model -> sets hotkeys -> enables auto-paste.

## Local transcription
- Default model should be fast and lightweight (`whisper.cpp` small.en).
- MVP is English-only; no language auto-detect.
- Supported tiers: tiny.en (fast), small.en (balanced default), medium.en (accuracy).
- Support CPU-only, optional GPU acceleration if available.

## Storage and data
- Store transcripts locally in a lightweight DB (SQLite).
- Optional retention policy and manual delete.
- Keep audio recordings optional and disabled by default.

## Privacy and security
- Local processing by default.
- No data leaves device unless explicitly enabled.
- Clear data deletion controls.

## Error handling
- Microphone permission denied -> guidance and retry.
- Model missing or incompatible -> prompt to download/select.
- Hotkey conflict -> show conflict and allow reassignment.

## Success metrics
- Time from hotkey to paste <2x audio length.
- Crash-free sessions >99%.
- 90% of users can set up hotkeys without assistance.

## Open questions
- Which Linux frameworks are preferred (Electron, Tauri, native)?
- Should auto-paste be optional per-app?
- Do we want portal-based hotkeys for Wayland?
- Do we want multi-language support after MVP?

## Acceptance criteria
### Recording and hotkeys
- Start/stop hotkey works from any foreground app on X11; on Wayland, use the app buttons.
- Recording state is obvious (visual + optional sound).
- Hotkey conflicts are detected and require reassignment.

### Transcription
- For <=10s recordings, transcript appears within 2x audio duration or 5s, whichever is greater.
- Failed transcriptions show a clear error and retry option.
- Language is fixed to English for MVP.

### History
- Transcripts persist across restarts in local storage.
- Each item shows timestamp and duration.
- Search/filter returns matching items in under 200ms for 1,000 items.
- Users can delete single items or clear all.
- Clicking a transcript copies it to the clipboard and shows a confirmation.

### Settings
- Changes persist and are applied immediately where possible.
- Model selection updates the active engine after download/load completes.
- Hotkey changes are validated before save.
- Model management allows download, activate, cycle, and delete from settings.

### Auto-paste
- When enabled, text is inserted into the focused field after transcription (Wayland requires helpers).
- If paste fails, text is still placed in the clipboard and user is notified.
- User can configure a short delay before paste.

### Performance and reliability
- Idle CPU under 2% after startup stabilization.
- Memory usage under 250MB for UI-only idle state.
- Crash-free sessions exceed 99% in internal testing.

## Milestones
1) Prototype: hotkey -> record -> local transcript -> UI list.
2) MVP: settings, history, auto-paste, basic errors.
3) Beta: performance optimizations, polish, Wayland hotkeys/portal coverage.
