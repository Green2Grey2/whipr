#!/usr/bin/env bash
set -euo pipefail

CONFIG_DIR="${XDG_CONFIG_HOME:-${HOME}/.config}/whispr"
CONFIG_FILE="${CONFIG_DIR}/overlay-config.json"

json_escape() {
  local value="$1"
  value="${value//\\/\\\\}"
  value="${value//\"/\\\"}"
  printf '%s' "${value}"
}

BIN_PATH="${1:-}"
if [[ -z "${BIN_PATH}" ]]; then
  if command -v whispr >/dev/null 2>&1; then
    BIN_PATH="$(command -v whispr)"
  fi
fi

if [[ -z "${BIN_PATH}" ]]; then
  echo "Usage: $0 /absolute/path/to/whispr" >&2
  exit 1
fi

mkdir -p "${CONFIG_DIR}"
ESCAPED_PATH="$(json_escape "${BIN_PATH}")"
printf '{\"binary\":\"%s\"}\n' "${ESCAPED_PATH}" > "${CONFIG_FILE}"

echo "Configured Whispr binary at ${BIN_PATH}."
