# Architecture

This document describes the technical architecture of Whipr.

## Overview

Whipr is built with [Tauri 2](https://tauri.app), combining a Rust backend for performance-critical operations with a Svelte frontend for the user interface. The architecture prioritizes local processing, low latency, and cross-platform compatibility.

```
┌─────────────────────────────────────────────────────────────┐
│                        Frontend (Svelte)                     │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ Transcripts │  │  Settings   │  │   Recording State   │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└────────────────────────────┬────────────────────────────────┘
                             │ Tauri IPC
┌────────────────────────────┴────────────────────────────────┐
│                        Backend (Rust)                        │
│  ┌─────────┐ ┌──────────────┐ ┌────────┐ ┌──────────────┐   │
│  │  Audio  │ │ Transcription│ │ Hotkeys│ │  Automation  │   │
│  └─────────┘ └──────────────┘ └────────┘ └──────────────┘   │
│  ┌─────────┐ ┌──────────────┐ ┌────────────────────────┐    │
│  │ Storage │ │    Models    │ │    System Tray         │    │
│  └─────────┘ └──────────────┘ └────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

## Tech Stack

| Layer | Technology | Purpose |
|-------|------------|---------|
| Framework | Tauri 2 | Desktop app shell, IPC, system integration |
| Frontend | Svelte + Vite | UI components, state management |
| Backend | Rust | Core logic, performance-critical code |
| Audio | cpal | Cross-platform audio capture |
| Transcription | whisper-rs (whisper.cpp) | Local speech-to-text |
| Storage | SQLite (rusqlite) | Transcripts and settings |
| Clipboard | arboard | Cross-platform clipboard |
| Input Simulation | enigo | Keystroke injection (X11) |
| Hotkeys | tauri-plugin-global-shortcut | Global keyboard shortcuts |

## Core Modules

### Audio (`src-tauri/src/core/audio.rs`)

Handles microphone input using the `cpal` library:

- Device enumeration and selection
- Real-time PCM capture at 16kHz mono
- Ring buffer for audio data
- Input level monitoring for UI feedback

### Transcription (`src-tauri/src/core/transcription.rs`)

Manages whisper.cpp integration via `whisper-rs`:

- Model loading and validation
- Inference execution on captured audio
- GPU acceleration detection (CUDA, Metal, Vulkan, etc.)
- Thread pool management for background processing

### Storage (`src-tauri/src/core/storage.rs`)

SQLite-based persistence layer:

- Transcript CRUD operations
- Settings key-value store
- Full-text search on transcript content
- Data retention and cleanup

### Automation (`src-tauri/src/core/automation.rs`)

Handles paste injection after transcription:

- Clipboard management via `arboard`
- Keystroke simulation via `enigo` (X11)
- Wayland support via `wl-clipboard` + `wtype`/`ydotool`
- Focus window tracking

### Hotkeys (`src-tauri/src/core/hotkeys.rs`)

Global keyboard shortcut handling:

- Registration via `tauri-plugin-global-shortcut`
- Conflict detection
- Platform-specific key mapping

### Models (`src-tauri/src/core/models.rs`)

Whisper model management:

- Download from Hugging Face
- Local manifest tracking
- Model validation and activation
- Storage in platform-specific directories

## Data Flow

### Recording Flow

```
1. User triggers recording (hotkey or button)
2. Audio module starts capture stream
3. PCM samples buffered in ring buffer
4. UI receives level updates via events
5. User stops recording
6. Audio data passed to transcription
7. whisper.cpp processes audio
8. Text result stored in SQLite
9. UI updates transcript list
10. If auto-paste enabled, text injected to focused app
```

### Settings Flow

```
1. User changes setting in UI
2. Frontend calls Tauri command
3. Backend validates and stores in SQLite
4. Change applied immediately where possible
5. Some changes (model, hotkeys) require reload
```

## Database Schema

### transcripts

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER | Primary key |
| created_at | INTEGER | Unix timestamp (ms) |
| duration_ms | INTEGER | Recording duration |
| text | TEXT | Transcribed content |
| source | TEXT | Origin (mic, import) |

### settings

| Column | Type | Description |
|--------|------|-------------|
| key | TEXT | Setting identifier |
| value | TEXT | JSON-encoded value |

## Platform Considerations

### Linux

- **X11**: Full support for global hotkeys and paste injection
- **Wayland**: Hotkeys limited; paste requires `wl-clipboard` + `wtype`/`ydotool`
- **Audio**: Requires ALSA; works with PipeWire/PulseAudio via ALSA plugin

### macOS

- Global hotkeys require Accessibility permissions
- Metal GPU acceleration on Apple Silicon
- Standard clipboard and input simulation APIs

### Windows

- Full global hotkey support
- CUDA/Vulkan GPU acceleration available
- Standard Win32 clipboard and input APIs

## GPU Acceleration

Whisper.cpp supports multiple GPU backends via Cargo features:

| Feature | Backend | Platforms |
|---------|---------|-----------|
| `cuda` | NVIDIA CUDA | Linux, Windows |
| `metal` | Apple Metal | macOS |
| `vulkan` | Vulkan | All |
| `hipblas` | AMD ROCm | Linux |
| `intel-sycl` | Intel oneAPI | Linux, Windows |

Build with: `cargo build --features cuda`

## Performance Targets

| Metric | Target |
|--------|--------|
| Cold start | < 2 seconds |
| Warm start | < 1 second |
| Transcription (10s audio) | < 2x real-time |
| Idle CPU | < 2% |
| Idle memory | < 250 MB |

## Security

- All processing is local by default
- No telemetry or network calls (except model downloads)
- SQLite database stored in user data directory
- No sensitive data in logs
