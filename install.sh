#!/bin/bash
# Installation script for chronos CLI

set -e

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case $OS in
    "Linux")
        case $ARCH in
            "x86_64") ASSET="chronos-linux-x86_64.tar.gz" ;;
            "aarch64"|"arm64") ASSET="chronos-linux-aarch64.tar.gz" ;;
            *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    "Darwin")
        case $ARCH in
            "x86_64") ASSET="chronos-macos-x86_64.tar.gz" ;;
            "arm64") ASSET="chronos-macos-aarch64.tar.gz" ;;
            *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

# Download and install
REPO="bitshyam/chronos"
INSTALL_DIR="/usr/local/bin"
TEMP_DIR=$(mktemp -d)

echo "Downloading chronos for $OS $ARCH..."
curl -sL "https://github.com/$REPO/releases/latest/download/$ASSET" | tar -xz -C "$TEMP_DIR"

echo "Installing chronos to $INSTALL_DIR..."
sudo mv "$TEMP_DIR/chronos" "$INSTALL_DIR/chronos"
sudo chmod +x "$INSTALL_DIR/chronos"

# Clean up
rm -rf "$TEMP_DIR"

echo "chronos installed successfully!"
echo "Try: chronos --help"
