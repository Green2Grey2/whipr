# Whispr GNOME Overlay

This extension shows a bottom-center recording pill on GNOME Shell (X11 or Wayland).

## Install

```
./scripts/install-gnome-extension.sh
```

Optional: pass the absolute path to the Whispr binary:

```
./scripts/install-gnome-extension.sh /absolute/path/to/whispr
```

Enable the extension:

```
gnome-extensions enable whispr-overlay@greenuni
```

Reload GNOME Shell:

- X11: press `Alt` + `F2`, type `r`, press `Enter`.
- Wayland: log out and back in.

## Notes

- The extension reads `~/.local/state/whispr/overlay.json` to determine recording status.
- The overlay buttons call `whispr --toggle`, so the Whispr binary must be on your `PATH`.
- If Whispr is not on your `PATH`, set the binary in `~/.config/whispr/overlay-config.json`:

```
{"binary":"/absolute/path/to/whispr"}
```

- You can also override it per session with `WHISPR_BIN=/absolute/path/to/whispr`.

If you need to update the binary later:

```
./scripts/configure-gnome-extension.sh /absolute/path/to/whispr
```

## Uninstall

```
./scripts/uninstall-gnome-extension.sh
```
