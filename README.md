<p align="center">
  <img src="src-tauri/icons/icon.png" alt="Whipr" width="128" height="128">
</p>

<h1 align="center">Whipr</h1>

<p align="center">
  <strong>Fast, private voice transcription for your desktop</strong>
</p>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#installation">Installation</a> •
  <a href="#usage">Usage</a> •
  <a href="#configuration">Configuration</a> •
  <a href="#development">Development</a> •
  <a href="#license">License</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-blue" alt="Platform">
  <img src="https://img.shields.io/badge/license-MIT-green" alt="License">
  <img src="https://img.shields.io/badge/rust-stable-orange" alt="Rust">
</p>

---

Whipr is a desktop application for fast, local voice transcription. Press a hotkey, speak, and your words are transcribed and pasted directly into any application. All processing happens on your machine—no cloud, no subscriptions, no data leaves your device.

Built with [Tauri](https://tauri.app), [Svelte](https://svelte.dev), and [whisper.cpp](https://github.com/ggerganov/whisper.cpp).

## Features

- **Global Hotkeys** — Start/stop recording from anywhere with customizable keyboard shortcuts
- **Local Transcription** — Powered by whisper.cpp with support for multiple model sizes
- **Auto-Paste** — Transcribed text is automatically inserted into your focused application
- **Transcript History** — Search and browse past transcriptions with full-text search
- **GPU Acceleration** — Optional CUDA, Metal, Vulkan, ROCm, and Intel oneAPI support
- **Privacy First** — Everything runs locally; no internet connection required
- **Cross-Platform** — Native builds for Linux, macOS, and Windows

## Installation

### Linux

#### AppImage (Recommended)

Download the latest `.AppImage` from [Releases](https://github.com/green2grey/whipr/releases), then:

```bash
chmod +x Whipr_*.AppImage
./Whipr_*.AppImage
```

#### Build from Source

**Prerequisites:**
- Rust (stable)
- Node.js 18+ and npm
- Build essentials: `build-essential`, `pkg-config`, `libssl-dev`
- Audio libraries: `libasound2-dev`
- WebKit: `libwebkit2gtk-4.1-dev`, `libgtk-3-dev`
- Additional: `libayatana-appindicator3-dev`, `librsvg2-dev`

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev libasound2-dev \
  libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

**Fedora:**
```bash
sudo dnf install -y gcc pkg-config openssl-devel alsa-lib-devel \
  webkit2gtk4.1-devel gtk3-devel libappindicator-gtk3-devel librsvg2-devel
```

**Arch Linux:**
```bash
sudo pacman -S --needed base-devel pkg-config openssl alsa-lib \
  webkit2gtk-4.1 gtk3 libappindicator-gtk3 librsvg
```

**Build:**
```bash
git clone https://github.com/green2grey/whipr.git
cd whipr
npm install
npm run tauri build
```

The AppImage will be in `src-tauri/target/release/bundle/appimage/`.

#### Linux with CUDA

For NVIDIA GPU acceleration:

```bash
# Ensure CUDA toolkit is installed
npm install
npm run tauri:build:cuda
```

Set `CUDA_ARCH` to override auto-detection: `CUDA_ARCH=89 npm run tauri:build:cuda`

#### Wayland Notes

On Wayland, install clipboard and input helpers for auto-paste:

```bash
# For wtype (recommended)
sudo apt install wl-clipboard wtype

# Or ydotool
sudo apt install wl-clipboard ydotool
```

Global hotkeys are limited on Wayland. Use GNOME custom shortcuts or the in-app buttons instead. See [GNOME Extension](#gnome-extension) for an overlay solution.

---

### macOS

#### DMG (Apple Silicon)

Download the latest `.dmg` from [Releases](https://github.com/green2grey/whipr/releases) and drag Whipr to Applications.

#### Build from Source

**Prerequisites:**
- Xcode Command Line Tools: `xcode-select --install`
- Rust (stable): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Node.js 18+: `brew install node`

**Build:**
```bash
git clone https://github.com/green2grey/whipr.git
cd whipr
npm install
npm run tauri:build:mac
```

The `.app` and `.dmg` will be in `src-tauri/target/release/bundle/`.

> **Note:** macOS builds use Metal for GPU acceleration automatically on Apple Silicon.

---

### Windows

#### Installer

Download the latest `.msi` or `.exe` installer from [Releases](https://github.com/green2grey/whipr/releases).

#### Build from Source

**Prerequisites:**
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/) with C++ workload
- [Rust](https://rustup.rs)
- [Node.js 18+](https://nodejs.org)
- [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (usually pre-installed on Windows 10/11)

**Build:**
```powershell
git clone https://github.com/green2grey/whipr.git
cd whipr
npm install
npm run tauri:build:win
```

Installers will be in `src-tauri\target\release\bundle\`.

---

## Usage

### Quick Start

1. **Launch Whipr** — The app starts minimized to the system tray
2. **Download a Model** — Open Settings and download a transcription model (small.en recommended)
3. **Configure Hotkeys** — Set your preferred keyboard shortcuts
4. **Start Recording** — Press your hotkey or click the record button
5. **Speak** — Your audio is captured locally
6. **Stop Recording** — Press the hotkey again; text is transcribed and pasted

### Hotkeys

| Action | Default | Description |
|--------|---------|-------------|
| Toggle Recording | `Ctrl+Alt+Space` | Start/stop recording |
| Paste Last | `Ctrl+Alt+V` | Paste the last transcript |
| Show App | `Ctrl+Alt+O` | Bring the app window to focus |

On macOS, use `Cmd` instead of `Ctrl`.

### CLI Commands

Whipr supports CLI commands for integration with system shortcuts:

```bash
whipr --toggle       # Start/stop recording
whipr --paste-last   # Paste the last transcript
whipr --show         # Show the app window
```

### GNOME Extension

For GNOME on Wayland, an optional overlay extension provides a recording indicator and quick controls:

```bash
./scripts/install-gnome-extension.sh
gnome-extensions enable whispr-overlay@greenuni
```

See [GNOME_EXTENSION.md](GNOME_EXTENSION.md) for details.

---

## Configuration

### Models

Whipr uses whisper.cpp models for transcription. Available models:

| Model | Size | Speed | Accuracy | Use Case |
|-------|------|-------|----------|----------|
| `tiny.en` | ~75 MB | Fastest | Good | Quick notes, low-end hardware |
| `small.en` | ~460 MB | Fast | Better | **Recommended default** |
| `medium.en` | ~1.5 GB | Moderate | Best | High accuracy requirements |

Models are downloaded to:
- **Linux:** `~/.local/share/whispr/models`
- **macOS:** `~/Library/Application Support/whispr/models`
- **Windows:** `%APPDATA%\whispr\models`

### Settings

All settings are accessible from the Settings tab:

| Category | Options |
|----------|---------|
| **Audio** | Input device, sample rate, gain, noise gate, VAD |
| **Hotkeys** | Customize all keyboard shortcuts |
| **Transcription** | Model selection, GPU acceleration, custom vocabulary |
| **Automation** | Auto-paste, paste delay, clipboard behavior |
| **Storage** | Data location, audio retention, history cleanup |

### GPU Acceleration

Enable GPU acceleration in Settings for faster transcription:

| Platform | Backend | Build Flag |
|----------|---------|------------|
| NVIDIA | CUDA | `--features cuda` |
| AMD | ROCm/HIP | `--features hipblas` |
| Intel | oneAPI | `--features intel-sycl` |
| Apple Silicon | Metal | `--features metal` (default on macOS) |
| Cross-platform | Vulkan | `--features vulkan` |

---

## Development

### Prerequisites

- Rust (stable): https://rustup.rs
- Node.js 18+: https://nodejs.org
- Platform-specific dependencies (see [Installation](#installation))

### Setup

```bash
git clone https://github.com/green2grey/whipr.git
cd whipr
npm install
```

### Development Server

```bash
# Frontend only (hot reload)
npm run dev

# Full Tauri app (recommended)
npm run tauri dev
```

### Project Structure

```
whipr/
├── src/                    # Svelte frontend
│   ├── lib/               # Components and utilities
│   └── App.svelte         # Main application
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── core/          # Audio, transcription, storage
│   │   ├── commands.rs    # Tauri IPC commands
│   │   └── main.rs        # Application entry
│   └── Cargo.toml
├── gnome-extension/       # GNOME Shell overlay
└── scripts/               # Build and install scripts
```

### Building

```bash
# Linux (AppImage)
npm run tauri build

# Linux (CUDA)
npm run tauri:build:cuda

# macOS (DMG)
npm run tauri:build:mac

# Windows (MSI/NSIS)
npm run tauri:build:win
```

---

## Troubleshooting

### Common Issues

**No audio input detected**
- Check that your microphone is not muted
- Verify the correct input device is selected in Settings
- On Linux, ensure PipeWire/PulseAudio is running

**Transcription is slow**
- Try a smaller model (tiny.en)
- Enable GPU acceleration if available
- Check CPU usage; close resource-heavy applications

**Auto-paste not working**
- On Wayland: Install `wl-clipboard` and `wtype`
- On X11: Ensure the target application accepts `Ctrl+V`
- Increase the paste delay in Settings

**Model download fails**
- Check your internet connection
- Verify write permissions to the models directory
- Try downloading manually and placing in the models folder

---

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

---

<p align="center">
  Made with Rust, Svelte, and whisper.cpp
</p>
