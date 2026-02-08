#!/usr/bin/env bash
set -euo pipefail

# Auto-detect CUDA location: prefer /usr/local/cuda-* installs over system package.
if [ -z "${CUDA_HOME:-}" ]; then
  if [ -d "/usr/local/cuda/bin" ]; then
    export CUDA_HOME="/usr/local/cuda"
  elif [ -x "/usr/bin/nvcc" ]; then
    export CUDA_HOME="/usr"
  else
    echo "::error::Cannot find CUDA installation"
    exit 1
  fi
fi

export CUDAToolkit_ROOT="${CUDA_HOME}"
export PATH="${CUDA_HOME}/bin:${PATH}"
export LD_LIBRARY_PATH="${CUDA_HOME}/lib64:${CUDA_HOME}/lib/x86_64-linux-gnu:${LD_LIBRARY_PATH:-}"

if [ -n "${CMAKE_CUDA_ARCHITECTURES:-}" ]; then
  echo "Using CMAKE_CUDA_ARCHITECTURES=${CMAKE_CUDA_ARCHITECTURES}"
elif [ -n "${CUDA_ARCH:-}" ]; then
  export CMAKE_CUDA_ARCHITECTURES="${CUDA_ARCH}"
  echo "Using CUDA_ARCH override: ${CMAKE_CUDA_ARCHITECTURES}"
else
  mapfile -t supported_archs < <(nvcc --list-gpu-arch 2>/dev/null | sed "s/compute_//")

  if [ "${#supported_archs[@]}" -eq 0 ]; then
    echo "::error::nvcc --list-gpu-arch returned no architectures. Is CUDA installed correctly?"
    exit 1
  fi

  detected_arch=""

  if command -v nvidia-smi >/dev/null 2>&1; then
    cap_raw="$(nvidia-smi --query-gpu=compute_cap --format=csv,noheader 2>/dev/null | head -n1 | tr -d ' ')"
    if [ -n "${cap_raw}" ]; then
      detected_arch="$(echo "${cap_raw}" | tr -d '.')"
    fi
  fi

  if [ -n "${detected_arch}" ]; then
    if printf '%s\n' "${supported_archs[@]}" | grep -qx "${detected_arch}"; then
      export CMAKE_CUDA_ARCHITECTURES="${detected_arch}"
      echo "Detected compute capability ${detected_arch}; using CMAKE_CUDA_ARCHITECTURES=${CMAKE_CUDA_ARCHITECTURES}"
    else
      fallback_arch="${supported_archs[$((${#supported_archs[@]} - 1))]}"
      export CMAKE_CUDA_ARCHITECTURES="${fallback_arch}"
      echo "Detected compute capability ${detected_arch}, but nvcc does not support it. Falling back to ${fallback_arch}."
      echo "For best results, upgrade CUDA or set CUDA_ARCH to a supported value."
    fi
  else
    fallback_arch="${supported_archs[$((${#supported_archs[@]} - 1))]}"
    export CMAKE_CUDA_ARCHITECTURES="${fallback_arch}"
    echo "Could not detect GPU compute capability; using ${fallback_arch}."
    echo "Set CUDA_ARCH to override (example: CUDA_ARCH=89)."
  fi
fi

npm run tauri -- build --features cuda
