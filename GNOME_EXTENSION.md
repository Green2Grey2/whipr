# GNOME Shell Extension

Whipr includes an optional GNOME Shell extension that provides a recording indicator overlay and quick controls. This is particularly useful on Wayland where global hotkeys are limited.

## Features

- Bottom-center recording pill indicator
- One-click start/stop recording
- Works on both X11 and Wayland
- Reads recording state from Whipr

## Installation

### Quick Install

```bash
./scripts/install-gnome-extension.sh
gnome-extensions enable whispr-overlay@greenuni
```

### With Custom Binary Path

If Whipr is not in your `PATH`:

```bash
./scripts/install-gnome-extension.sh /path/to/whipr
```

### Reload GNOME Shell

After enabling the extension:

- **X11**: Press `Alt+F2`, type `r`, press Enter
- **Wayland**: Log out and log back in

## Configuration

### Binary Path

The extension needs to know where the Whipr binary is located. Configure it in one of these ways:

1. **Add to PATH** (recommended): Install Whipr to a directory in your PATH
2. **Config file**: Create `~/.config/whispr/overlay-config.json`:
   ```json
   {"binary": "/path/to/whipr"}
   ```
3. **Environment variable**: Set `WHISPR_BIN=/path/to/whipr`

### Update Binary Path

To change the binary path after installation:

```bash
./scripts/configure-gnome-extension.sh /new/path/to/whipr
```

## How It Works

1. The extension monitors `~/.local/state/whispr/overlay.json` for recording state
2. When recording starts, a pill-shaped indicator appears at the bottom of the screen
3. Clicking the overlay triggers `whipr --toggle` to start/stop recording

## Uninstallation

```bash
./scripts/uninstall-gnome-extension.sh
```

Or manually:

```bash
gnome-extensions disable whispr-overlay@greenuni
rm -rf ~/.local/share/gnome-shell/extensions/whispr-overlay@greenuni
```

## Troubleshooting

### Extension not appearing

1. Verify it's enabled: `gnome-extensions list --enabled`
2. Check GNOME Shell version compatibility
3. Look for errors: `journalctl -f -o cat /usr/bin/gnome-shell`

### Recording button not working

1. Verify Whipr binary path is correct
2. Test manually: `whipr --toggle`
3. Check overlay config: `cat ~/.config/whispr/overlay-config.json`

## Requirements

- GNOME Shell 45+
- Whipr binary accessible via PATH or configured path
