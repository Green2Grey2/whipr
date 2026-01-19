# Whispr Linux Alternative - MVP Plan and Tickets

## Milestones
### M0 - Discovery and design (1 week)
- Define UX wireframes and interaction model.
- Confirm target desktops and X11 dependencies.
- Validate model choices and CPU performance.

### M1 - Prototype (2 weeks)
- Hotkey -> record -> transcript -> history list.
- Basic settings and persistence.

### M2 - MVP (3-4 weeks)
- Full settings, auto-paste, model management.
- Error handling, notifications, and packaging.

### M3 - Beta (2-3 weeks)
- Performance tuning and UX polish.
- Expanded desktop coverage and QA.

## Tickets
### T-001 - App scaffold
- Description: Create Tauri app with Svelte UI and Rust core modules.
- Acceptance: App builds, launches, and shows a basic window with navigation.

### T-002 - Audio capture pipeline
- Description: Capture microphone audio and buffer PCM frames.
- Acceptance: User can start/stop recording and see input level changes.

### T-003 - Global hotkeys
- Description: Register hotkeys for start/stop toggle and open app.
- Acceptance: Hotkeys work from any foreground app on X11 and conflicts are detected; stop triggers auto-paste if enabled.

### T-004 - Recording UI state
- Description: Show recording status, timer, and error state in the UI.
- Acceptance: UI reflects idle, recording, and transcribing states with clear signals.

### T-005 - Transcription engine integration
- Description: Integrate `whisper.cpp` with English-only model loading and inference.
- Acceptance: Short recordings transcribe locally with correct text output using `small.en` by default.

### T-006 - Model management
- Description: Select, download, and store English-only models with a configurable download path.
- Acceptance: User can download, activate, cycle, and delete English-only tiers (tiny.en/small.en/medium.en) and the app validates availability.

### T-007 - Storage schema and data layer
- Description: Create SQLite schema and CRUD for transcripts and settings.
- Acceptance: Transcripts persist across restarts and settings save/load.

### T-008 - Transcript history UI
- Description: Build list view with timestamps and detail panel.
- Acceptance: New transcripts appear immediately; clicking an item copies it to clipboard.

### T-009 - Search and filter
- Description: Implement search and date filter in history.
- Acceptance: Search returns matching entries within 200ms for 1,000 items.

### T-010 - Settings UI and validation
- Description: Settings page for audio, hotkeys, English-only transcription, automation.
- Acceptance: Changes persist, invalid hotkeys show a validation error, and model management actions are available.

### T-011 - Auto-paste engine
- Description: Copy transcript to clipboard and issue paste keystroke.
- Acceptance: When enabled, text is inserted into the last focused field on X11.

### T-012 - Focus tracking
- Description: Track last focused window at record start.
- Acceptance: Auto-paste targets the correct window 90% of the time on X11.

### T-013 - Error handling and notifications
- Description: Surface mic and model errors with actionable messages.
- Acceptance: Errors are visible, retryable, and do not crash the app.

### T-014 - Performance instrumentation
- Description: Add basic timing metrics for record-to-transcript.
- Acceptance: P50 and P95 latency metrics are available in logs.

### T-015 - Packaging (AppImage)
- Description: Produce an AppImage for Linux distribution.
- Acceptance: AppImage runs on at least one GNOME and one KDE system.

### T-016 - QA matrix and release checks
- Description: Define and run basic QA across GNOME and KDE on X11.
- Acceptance: QA checklist completed with documented issues and mitigations.
