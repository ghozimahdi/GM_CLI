#!/bin/sh
# GM CLI installer (macOS).
#
#   curl -fsSL https://ghozimahdi.github.io/GM_CLI/install.sh | sh
#
# Options (environment variables):
#   GM_VERSION=v4.0.1        install a specific release tag (default: latest)
#   GM_INSTALL_DIR=~/bin     install directory (default: /usr/local/bin if
#                            writable, otherwise ~/.local/bin)
#
# The binary is downloaded from GitHub Releases and verified against the
# release's SHA256SUMS before it is installed.
set -eu

REPO="ghozimahdi/GM_CLI"
BIN="gm"

say() { printf '%s\n' "$*"; }
err() { printf 'gm-install: %s\n' "$*" >&2; exit 1; }
need() { command -v "$1" >/dev/null 2>&1 || err "'$1' is required but not installed"; }

main() {
  need curl
  need tar
  need uname
  need shasum

  [ "$(uname -s)" = "Darwin" ] || err "prebuilt binaries are macOS only. On other systems build from source: https://github.com/${REPO}#build-from-source"

  case "$(uname -m)" in
    arm64 | aarch64) target="aarch64-apple-darwin" ;;
    x86_64) target="x86_64-apple-darwin" ;;
    *) err "unsupported CPU architecture: $(uname -m)" ;;
  esac

  version="${GM_VERSION:-latest}"
  if [ "$version" = "latest" ]; then
    base="https://github.com/${REPO}/releases/latest/download"
  else
    case "$version" in v*) ;; *) version="v${version}" ;; esac
    base="https://github.com/${REPO}/releases/download/${version}"
  fi
  asset="${BIN}-${target}.tar.gz"

  if [ -n "${GM_INSTALL_DIR:-}" ]; then
    dir="$GM_INSTALL_DIR"
  elif [ -d /usr/local/bin ] && [ -w /usr/local/bin ]; then
    dir="/usr/local/bin"
  else
    dir="${HOME}/.local/bin"
  fi

  tmp="$(mktemp -d)"
  trap 'rm -rf "$tmp"' EXIT INT TERM

  say "Downloading GM CLI (${version}, ${target})..."
  curl -fsSL "${base}/${asset}" -o "${tmp}/${asset}" || err "download failed: ${base}/${asset}"
  curl -fsSL "${base}/SHA256SUMS" -o "${tmp}/SHA256SUMS" || err "could not download SHA256SUMS"

  expected="$(awk -v f="$asset" '$2 == f || $2 == "*" f { print $1 }' "${tmp}/SHA256SUMS")"
  [ -n "$expected" ] || err "no checksum for ${asset} in SHA256SUMS"
  actual="$(shasum -a 256 "${tmp}/${asset}" | awk '{ print $1 }')"
  [ "$expected" = "$actual" ] || err "checksum mismatch for ${asset} (expected ${expected}, got ${actual})"
  say "Checksum verified."

  tar -xzf "${tmp}/${asset}" -C "$tmp"
  [ -f "${tmp}/${BIN}" ] || err "archive does not contain '${BIN}'"

  mkdir -p "$dir" || err "cannot create ${dir}"
  if [ -w "$dir" ]; then
    install -m 0755 "${tmp}/${BIN}" "${dir}/${BIN}"
  else
    err "${dir} is not writable. Re-run with GM_INSTALL_DIR set to a writable directory"
  fi

  say "Installed ${dir}/${BIN}"

  if command -v brew >/dev/null 2>&1 && brew list --formula gm-cli >/dev/null 2>&1; then
    say "Note: the Homebrew formula 'gm-cli' is also installed; whichever comes first in PATH wins."
  fi

  case ":${PATH}:" in
    *":${dir}:"*) say "Run 'gm help' to get started." ;;
    *)
      say ""
      say "${dir} is not on your PATH. Add it, for example:"
      say "  echo 'export PATH=\"${dir}:\$PATH\"' >> ~/.zshrc && source ~/.zshrc"
      ;;
  esac
}

main "$@"
