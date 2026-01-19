#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
EXTENSION_UUID="whispr-overlay@greenuni"
SRC_DIR="${ROOT_DIR}/gnome-extension/${EXTENSION_UUID}"
DEST_DIR="${HOME}/.local/share/gnome-shell/extensions/${EXTENSION_UUID}"
CONFIG_DIR="${XDG_CONFIG_HOME:-${HOME}/.config}/whispr"
CONFIG_FILE="${CONFIG_DIR}/overlay-config.json"

json_escape() {
  local value="$1"
  value="${value//\\/\\\\}"
  value="${value//\"/\\\"}"
  printf '%s' "${value}"
}

if [[ ! -d "${SRC_DIR}" ]]; then
  echo "Extension source not found at ${SRC_DIR}" >&2
  exit 1
fi

mkdir -p "${DEST_DIR}"
cp -r "${SRC_DIR}/." "${DEST_DIR}/"

mkdir -p "${CONFIG_DIR}"

BIN_PATH="${1:-}"
if [[ -z "${BIN_PATH}" ]]; then
  if command -v whispr >/dev/null 2>&1; then
    BIN_PATH="$(command -v whispr)"
  fi
fi

if [[ -n "${BIN_PATH}" ]]; then
  ESCAPED_PATH="$(json_escape "${BIN_PATH}")"
  printf '{\"binary\":\"%s\"}\n' "${ESCAPED_PATH}" > "${CONFIG_FILE}"
  echo "Configured Whispr binary at ${BIN_PATH}."
else
  echo "Whispr binary not found on PATH. Overlay will use 'whispr' by default." >&2
  echo "You can set the binary later in ${CONFIG_FILE}." >&2
fi

if command -v gnome-extensions >/dev/null 2>&1; then
  gnome-extensions enable "${EXTENSION_UUID}" || true
  echo "Enabled ${EXTENSION_UUID}."
else
  echo "Installed ${EXTENSION_UUID}. Run: gnome-extensions enable ${EXTENSION_UUID}" >&2
fi

echo "Reload GNOME Shell: X11 uses Alt+F2 then 'r'; Wayland requires logout/login."
