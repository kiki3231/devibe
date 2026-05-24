#!/usr/bin/env bash
# Install devibe on Linux/macOS
set -euo pipefail

REPO="kiki3231/devibe"
VERSION="${1:-latest}"

if [ "$VERSION" = "latest" ]; then
    API_URL="https://api.github.com/repos/$REPO/releases/latest"
else
    API_URL="https://api.github.com/repos/$REPO/releases/tags/v$VERSION"
fi

echo "Downloading devibe $VERSION..."

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux)
        case "$ARCH" in
            x86_64)  ASSET="devibe-linux-x86_64.tar.gz" ;;
            aarch64) ASSET="devibe-linux-x86_64-musl.tar.gz" ;;
            *)       echo "Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    Darwin)
        case "$ARCH" in
            x86_64) ASSET="devibe-macos-x86_64.tar.gz" ;;
            arm64)  ASSET="devibe-macos-arm64.tar.gz" ;;
            *)      echo "Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    *)
        echo "Unsupported OS: $OS. Please build from source: https://github.com/$REPO"
        exit 1
        ;;
esac

DOWNLOAD_URL=$(curl -s "$API_URL" | grep "browser_download_url.*$ASSET" | cut -d '"' -f 4)

if [ -z "$DOWNLOAD_URL" ]; then
    echo "Asset $ASSET not found in release. Falling back to cargo install..."
    if command -v cargo &> /dev/null; then
        cargo install devibe
    else
        echo "Please install Rust: https://rustup.rs"
        exit 1
    fi
    exit 0
fi

TMPDIR=$(mktemp -d)
curl -sL "$DOWNLOAD_URL" -o "$TMPDIR/devibe.tar.gz"
tar xzf "$TMPDIR/devibe.tar.gz" -C "$TMPDIR"

INSTALL_DIR="${HOME}/.local/bin"
mkdir -p "$INSTALL_DIR"
cp "$TMPDIR/devibe" "$INSTALL_DIR/devibe"
chmod +x "$INSTALL_DIR/devibe"
rm -rf "$TMPDIR"

echo "devibe installed to $INSTALL_DIR/devibe"

# Check if install dir is in PATH
if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    echo
    echo "Note: Add $INSTALL_DIR to your PATH:"
    echo "  echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.bashrc"
    echo "  source ~/.bashrc"
fi

echo
echo "Run: devibe --scan ~/projects"
