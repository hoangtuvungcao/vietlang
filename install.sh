#!/bin/bash
# VietLang Installer
# Usage: curl -sSL https://raw.githubusercontent.com/hoangtuvungcao/vietlang/main/install.sh | bash

set -e

VERSION="0.1.0"
REPO="hoangtuvungcao/vietlang"
INSTALL_DIR="/usr/local/bin"

echo "🇻🇳 Installing VietLang v${VERSION}..."
echo ""

# Detect OS and architecture
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "${OS}" in
    linux)
        case "${ARCH}" in
            x86_64) BINARY="vietlang-linux-x64" ;;
            aarch64) BINARY="vietlang-linux-arm64" ;;
            *) echo "❌ Unsupported architecture: ${ARCH}"; exit 1 ;;
        esac
        ;;
    darwin)
        case "${ARCH}" in
            x86_64) BINARY="vietlang-macos-x64" ;;
            arm64) BINARY="vietlang-macos-arm64" ;;
            *) echo "❌ Unsupported architecture: ${ARCH}"; exit 1 ;;
        esac
        ;;
    *)
        echo "❌ Unsupported OS: ${OS}"
        echo "   Please download manually from: https://github.com/${REPO}/releases"
        exit 1
        ;;
esac

DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${BINARY}"

echo "📦 Downloading ${BINARY}..."
if command -v curl &> /dev/null; then
    curl -sSL "${DOWNLOAD_URL}" -o /tmp/vietlang
elif command -v wget &> /dev/null; then
    wget -q "${DOWNLOAD_URL}" -O /tmp/vietlang
else
    echo "❌ Neither curl nor wget found. Please install one."
    exit 1
fi

chmod +x /tmp/vietlang

echo "📁 Installing to ${INSTALL_DIR}/vietlang..."
if [ -w "${INSTALL_DIR}" ]; then
    mv /tmp/vietlang "${INSTALL_DIR}/vietlang"
else
    sudo mv /tmp/vietlang "${INSTALL_DIR}/vietlang"
fi

echo ""
echo "✅ VietLang installed successfully!"
echo ""
echo "   Run: vietlang --version"
echo "   REPL: vietlang"
echo "   File: vietlang hello.vl"
echo ""

vietlang --version 2>/dev/null || echo "   (restart your terminal to use)"
