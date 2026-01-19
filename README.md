# Whispr (Linux-first + macOS Apple Silicon)

A Linux-first desktop transcription app with local English-only models, global hotkeys, history, and auto-paste. macOS (Apple Silicon) builds are supported.

## Prereqs
- Rust (stable)
- Node.js + npm (or pnpm/yarn)
- Linux: X11 session, or Wayland with `wl-clipboard` + `wtype` (or `ydotool`)
- Linux: Global hotkeys are limited on Wayland; use the app buttons instead.
- macOS (Apple Silicon): Xcode Command Line Tools

## Wayland hotkeys (GNOME)
GNOME on Wayland does not allow apps to register global hotkeys directly. Use GNOME custom shortcuts:
1) Settings -> Keyboard -> View and Customize Shortcuts -> Custom Shortcuts
2) Add shortcuts that run:
   - `whispr --toggle` (start/stop recording)
   - `whispr --paste-last` (paste last transcript)
   - `whispr --show` (show the app window)
The commands target the running instance; if it is not running, they will start it.

## Dev
```bash
npm install
npm run dev
```

## Tauri dev
```bash
npm install
npm run tauri dev
```

## macOS (Apple Silicon) build
```bash
npm install
npm run tauri:build:mac
```
Builds use the `metal` feature via `src-tauri/tauri.macos.conf.json`. If bundling fails due to missing macOS icons, run `npx tauri icon` to generate `icons/icon.icns`.

## Linux AppImage (CUDA)
```bash
npm install
npm run build
npm run tauri:build:cuda
```
The CUDA build script exports:
`CUDAToolkit_ROOT=/usr`, `CUDA_HOME=/usr`, and adds `/usr/lib/x86_64-linux-gnu` to `LD_LIBRARY_PATH`.
It also sets `CMAKE_CUDA_ARCHITECTURES` based on detected GPU capability when possible.
To override, set `CUDA_ARCH` (example: `CUDA_ARCH=89 npm run tauri:build:cuda`).
If you see `Unsupported gpu architecture 'compute_120'`, upgrade CUDA or set `CUDA_ARCH` to a supported value.

## Notes
- Default model: `small.en` (English-only)
- Models are stored under `~/.local/share/whispr/models` on Linux and `~/Library/Application Support/whispr/models` on macOS.
- Transcription runs locally via `whisper-rs` (whisper.cpp); download a model in Settings before recording.
- GPU acceleration is available when built with cargo features (e.g. `cuda`, `vulkan`, `metal`, `hipblas`, `intel-sycl`).
