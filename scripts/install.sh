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
        echo "use a POSIX shell environment (bash/zsh/fish) on Linux, macOS, or Windows (Git Bash/MSYS2/Cygwin/WSL)" >&2
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

echo "==> installing to $INSTALL_DIR/$BIN_NAME"
mkdir -p "$INSTALL_DIR"
chmod +x "$TMP_FILE"
mv "$TMP_FILE" "$INSTALL_DIR/$BIN_NAME"

if [ "$BIN_NAME" = "enx.exe" ]; then
    cat > "$INSTALL_DIR/enx" <<'EOF'
#!/usr/bin/env sh
SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
exec "$SCRIPT_DIR/enx.exe" "$@"
EOF
    chmod +x "$INSTALL_DIR/enx"
fi

ENX_BIN="$INSTALL_DIR/enx"

echo "==> running enx setup"
"$ENX_BIN" setup

echo ""
echo "setup finished"
echo "if '$INSTALL_DIR' is not in PATH, add it to your shell profile"
echo "if shell integration does not apply immediately, restart your shell"
