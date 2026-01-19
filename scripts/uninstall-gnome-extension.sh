#!/usr/bin/env bash
set -euo pipefail

EXTENSION_UUID="whispr-overlay@greenuni"
DEST_DIR="${HOME}/.local/share/gnome-shell/extensions/${EXTENSION_UUID}"
CONFIG_DIR="${XDG_CONFIG_HOME:-${HOME}/.config}/whispr"
CONFIG_FILE="${CONFIG_DIR}/overlay-config.json"
STATE_DIR="${XDG_STATE_HOME:-${HOME}/.local/state}/whispr"
STATE_FILE="${STATE_DIR}/overlay.json"

if command -v gnome-extensions >/dev/null 2>&1; then
  gnome-extensions disable "${EXTENSION_UUID}" || true
fi

if [[ -d "${DEST_DIR}" ]]; then
  rm -rf "${DEST_DIR}"
  echo "Removed ${DEST_DIR}."
else
  echo "Extension directory not found at ${DEST_DIR}."
fi

if [[ -f "${CONFIG_FILE}" ]]; then
  rm -f "${CONFIG_FILE}"
  echo "Removed ${CONFIG_FILE}."
fi

if [[ -f "${STATE_FILE}" ]]; then
  rm -f "${STATE_FILE}"
  echo "Removed ${STATE_FILE}."
fi

if [[ -d "${CONFIG_DIR}" ]] && [[ -z "$(ls -A "${CONFIG_DIR}")" ]]; then
  rmdir "${CONFIG_DIR}"
fi

if [[ -d "${STATE_DIR}" ]] && [[ -z "$(ls -A "${STATE_DIR}")" ]]; then
  rmdir "${STATE_DIR}"
fi

echo "Reload GNOME Shell: X11 uses Alt+F2 then 'r'; Wayland requires logout/login."
