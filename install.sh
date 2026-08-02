#!/bin/sh
set -eu

repository="https://github.com/orYoffe/tsborrow"
install_dir="${TSBORROW_INSTALL_DIR:-$HOME/.local/bin}"

case "$(uname -s)-$(uname -m)" in
  Linux-x86_64|Linux-amd64) asset="tsborrow-linux-x64" ;;
  Darwin-x86_64|Darwin-amd64) asset="tsborrow-darwin-x64" ;;
  Darwin-arm64|Darwin-aarch64) asset="tsborrow-darwin-arm64" ;;
  *) echo "tsborrow: unsupported platform $(uname -s)-$(uname -m)" >&2; exit 2 ;;
esac

if [ -n "${TSBORROW_VERSION:-}" ]; then
  release="${repository}/releases/download/v${TSBORROW_VERSION}"
else
  release="${repository}/releases/latest/download"
fi

temporary="$(mktemp -d)"
trap 'rm -rf "$temporary"' EXIT HUP INT TERM

download() {
  if command -v wget >/dev/null 2>&1; then
    wget -q --https-only -O "$2" "$1"
  elif command -v curl >/dev/null 2>&1; then
    curl --proto '=https' --tlsv1.2 --fail --location --silent --show-error -o "$2" "$1"
  else
    echo "tsborrow: wget or curl is required" >&2
    exit 2
  fi
}

download "${release}/${asset}" "${temporary}/${asset}"
download "${release}/${asset}.sha256" "${temporary}/${asset}.sha256"

if command -v sha256sum >/dev/null 2>&1; then
  (cd "$temporary" && sha256sum -c "${asset}.sha256")
else
  expected="$(awk '{print $1}' "${temporary}/${asset}.sha256")"
  actual="$(shasum -a 256 "${temporary}/${asset}" | awk '{print $1}')"
  [ "$actual" = "$expected" ] || { echo "tsborrow: checksum mismatch" >&2; exit 1; }
fi

mkdir -p "$install_dir"
install -m 0755 "${temporary}/${asset}" "${install_dir}/tsborrow"
echo "installed tsborrow to ${install_dir}/tsborrow"
