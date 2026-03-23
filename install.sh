#!/usr/bin/env sh
set -eu

# enx installer (binary install)
# External dependency: curl

if ! command -v curl >/dev/null 2>&1; then
    echo "error: curl is required but was not found in PATH" >&2
    exit 1
fi

# Override if installing from a fork:
#   ENX_REPO="owner/repo" sh install.sh
ENX_REPO="${ENX_REPO:-enxilium/enx-cli}"

# Channel/tag to install from. CI publishes rolling binaries to this tag.
ENX_CHANNEL="${ENX_CHANNEL:-nightly}"

INSTALL_DIR="${ENX_INSTALL_DIR:-$HOME/.local/bin}"
TMP_FILE="${TMPDIR:-/tmp}/enx-install.$$"

OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

BIN_NAME="enx"

validate_windows_binary() {
    file_path="$1"
    magic=$(dd if="$file_path" bs=1 count=2 2>/dev/null | od -An -tx1 | tr -d ' \n')

    if [ "$magic" != "4d5a" ]; then
        echo "error: downloaded windows asset is not a valid PE executable (expected MZ header)" >&2
        echo "error: refusing to install to avoid an infinite launcher loop" >&2
        exit 1
    fi
}

strip_enx_source_lines() {
    file_path="$1"
    [ -f "$file_path" ] || return 0

    tmp_file="${TMPDIR:-/tmp}/enx-rc-clean.$$"
    awk '
        !($0 ~ /source .*enx[\/]+shell[\/]+init\.(sh|bash|zsh|fish)/)
    ' "$file_path" > "$tmp_file"
    mv "$tmp_file" "$file_path"
}

case "$OS" in
    linux)
        case "$ARCH" in
            x86_64|amd64) ASSET="enx-linux-x86_64" ;;
            aarch64|arm64) ASSET="enx-linux-aarch64" ;;
            *)
                echo "error: unsupported linux architecture: $ARCH" >&2
                exit 1
                ;;
        esac
        ;;
    darwin)
        case "$ARCH" in
            x86_64|amd64) ASSET="enx-macos-x86_64" ;;
            arm64|aarch64) ASSET="enx-macos-aarch64" ;;
            *)
                echo "error: unsupported macos architecture: $ARCH" >&2
                exit 1
                ;;
        esac
        ;;
    msys*|mingw*|cygwin*)
        case "$ARCH" in
            x86_64|amd64) ASSET="enx-windows-x86_64.exe" ;;
            *)
                echo "error: unsupported windows architecture: $ARCH" >&2
                echo "windows installer support currently requires x86_64" >&2
                exit 1
                ;;
        esac
        BIN_NAME="enx.exe"
        ;;
    *)
        echo "error: unsupported operating system: $OS" >&2
        echo "use a POSIX shell environment (bash/zsh) on Linux, macOS, or Windows (Git Bash/MSYS2/Cygwin/WSL)" >&2
        exit 1
        ;;
esac

DOWNLOAD_URL="https://github.com/$ENX_REPO/releases/download/$ENX_CHANNEL/$ASSET"

echo "==> downloading $ASSET"
if ! curl -fsSL "$DOWNLOAD_URL" -o "$TMP_FILE"; then
    echo "error: failed to download binary from $DOWNLOAD_URL" >&2
    echo "if this is a fork, set ENX_REPO=owner/repo" >&2
    exit 1
fi

if [ "$BIN_NAME" = "enx.exe" ]; then
    validate_windows_binary "$TMP_FILE"
fi

echo "==> installing to $INSTALL_DIR/$BIN_NAME"
mkdir -p "$INSTALL_DIR"

if [ "$BIN_NAME" = "enx.exe" ]; then
    # Remove legacy shim + prior binary first so corrupted or linked files
    # cannot survive the reinstall.
    rm -f "$INSTALL_DIR/enx" "$INSTALL_DIR/enx.exe"
fi

chmod +x "$TMP_FILE"
mv "$TMP_FILE" "$INSTALL_DIR/$BIN_NAME"

ENX_BIN="$INSTALL_DIR/$BIN_NAME"

# Remove stale shell integration entries/files so `enx setup` can fully
# regenerate a clean configuration.
strip_enx_source_lines "$HOME/.bashrc"

if [ -n "${ZDOTDIR:-}" ]; then
    strip_enx_source_lines "$ZDOTDIR/.zshrc"
else
    strip_enx_source_lines "$HOME/.zshrc"
fi

SHELL_DIR="$HOME/.config/enx/shell"
rm -f "$SHELL_DIR/init.sh" "$SHELL_DIR/init.bash" "$SHELL_DIR/init.zsh" "$SHELL_DIR/init.fish"

echo ""
echo "install finished"
echo "cleared previous enx shell integration files and source lines"
echo "if '$INSTALL_DIR' is not in PATH, add it to your shell profile"
echo "run 'enx setup' from the shell you want integrated (bash or zsh)"
echo "if shell integration does not apply immediately, restart that shell"
