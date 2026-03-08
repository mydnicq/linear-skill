#!/usr/bin/env bash
set -euo pipefail

REPO="mydnicq/linear-skill"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

os="$(uname -s)"
arch="$(uname -m)"

case "$os" in
  Linux)
    case "$arch" in
      x86_64) artifact="linear-skill-linux-amd64" ;;
      *) echo "Unsupported Linux architecture: $arch" >&2; exit 1 ;;
    esac
    binary_name="linear-skill"
    ;;
  Darwin)
    case "$arch" in
      arm64)  artifact="linear-skill-macos-arm64" ;;
      x86_64) artifact="linear-skill-macos-amd64" ;;
      *) echo "Unsupported macOS architecture: $arch" >&2; exit 1 ;;
    esac
    binary_name="linear-skill"
    ;;
  MINGW*|MSYS*|CYGWIN*|Windows_NT)
    artifact="linear-skill-windows-amd64.exe"
    binary_name="linear-skill.exe"
    ;;
  *)
    echo "Unsupported OS: $os" >&2
    exit 1
    ;;
esac

tag="$(curl -sfL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | head -1 | cut -d '"' -f 4)"
if [ -z "$tag" ]; then
  echo "Failed to fetch latest release tag from GitHub." >&2
  exit 1
fi

url="https://github.com/${REPO}/releases/download/${tag}/${artifact}"
dest="${SCRIPT_DIR}/../${binary_name}"

echo "Downloading ${artifact} (${tag})..."
curl -sfL -o "$dest" "$url"

if [ "$binary_name" != "linear-skill.exe" ]; then
  chmod +x "$dest"
fi

echo "Installed to ${dest}"
