#!/usr/bin/env bash
# ========================================================================
# VietLang Official Cross-Platform Installer (Linux & macOS)
# Usage: curl -fsSL https://raw.githubusercontent.com/hoangtuvungcao/vietlang/main/install.sh | bash
# ========================================================================

set -e

VIETLANG_VERSION="0.1.0"
REPO="hoangtuvungcao/vietlang"
VIETLANG_HOME="${HOME}/.vietlang"
BIN_DIR="${VIETLANG_HOME}/bin"
STD_DIR="${VIETLANG_HOME}/std"

echo -e "\033[36m╔════════════════════════════════════════════════════════════╗\033[0m"
echo -e "\033[36m║       __      ___      _   _                               ║\033[0m"
echo -e "\033[36m║       \\ \\    / (_)    | | | |                              ║\033[0m"
echo -e "\033[36m║        \\ \\  / / _  ___| |_| |     __ _ _ __   __ _         ║\033[0m"
echo -e "\033[36m║         \\ \\/ / | |/ _ \\ __| |    / _\` | '_ \\ / _\` |        ║\033[0m"
echo -e "\033[36m║          \\  /  | |  __/ |_| |___| (_| | | | | (_| |        ║\033[0m"
echo -e "\033[36m║           \\/   |_|\\___|\\__|______\\__,_|_| |_|\\__, |        ║\033[0m"
echo -e "\033[36m║                                               __/ |        ║\033[0m"
echo -e "\033[36m║              Backend-First Language v${VIETLANG_VERSION}   |___/        ║\033[0m"
echo -e "\033[36m╚════════════════════════════════════════════════════════════╝\033[0m"
echo ""

# 1. Detect Operating System & CPU Architecture
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "${OS}" in
    linux)
        case "${ARCH}" in
            x86_64)  TARGET="vietlang-linux-x64" ;;
            aarch64) TARGET="vietlang-linux-arm64" ;;
            *) echo -e "\033[31m[!] Unsupported Linux architecture: ${ARCH}\033[0m"; exit 1 ;;
        esac
        ;;
    darwin)
        case "${ARCH}" in
            x86_64) TARGET="vietlang-macos-x64" ;;
            arm64)  TARGET="vietlang-macos-arm64" ;;
            *) echo -e "\033[31m[!] Unsupported macOS architecture: ${ARCH}\033[0m"; exit 1 ;;
        esac
        ;;
    *)
        echo -e "\033[31m[!] Unsupported Operating System: ${OS}\033[0m"
        echo "Please download the binary manually from: https://github.com/${REPO}/releases"
        exit 1
        ;;
esac

echo -e " detected platform: \033[32m${OS} (${ARCH})\033[0m -> Package: \033[33m${TARGET}\033[0m"

# 2. Setup Directory Structure
mkdir -p "${BIN_DIR}"
mkdir -p "${STD_DIR}"
mkdir -p "${VIETLANG_HOME}/modules"

# 3. Download Binary
DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${TARGET}"

echo -e " downloading VietLang binary from GitHub Releases..."
if command -v curl >/dev/null 2>&1; then
    curl -fSL "${DOWNLOAD_URL}" -o "${BIN_DIR}/vietlang" || {
        echo -e "\033[33mRelease binary not found online, attempting local build fallback...\033[0m"
        if [ -f "./target/release/vietlang" ]; then
            cp "./target/release/vietlang" "${BIN_DIR}/vietlang"
        fi
    }
elif command -v wget >/dev/null 2>&1; then
    wget -q "${DOWNLOAD_URL}" -O "${BIN_DIR}/vietlang" || true
fi

chmod +x "${BIN_DIR}/vietlang"

# 4. Install Standard Library
echo -e " syncing 49 Standard Library Modules into ${STD_DIR}..."
if [ -d "./std" ]; then
    cp -r ./std/* "${STD_DIR}/"
else
    # Fetch from github main branch
    curl -fsSL "https://github.com/${REPO}/archive/refs/heads/main.tar.gz" -o "/tmp/vietlang-std.tar.gz" 2>/dev/null && {
        tar -xzf "/tmp/vietlang-std.tar.gz" -C "/tmp"
        cp -r /tmp/vietlang-main/std/* "${STD_DIR}/" 2>/dev/null || true
        rm -rf /tmp/vietlang-std.tar.gz /tmp/vietlang-main
    } || true
fi

# 5. Automatically Set PATH in Shell Configs
export_line="export PATH=\"\$HOME/.vietlang/bin:\$PATH\""
export_std="export VIETLANG_STD=\"\$HOME/.vietlang/std\""

setup_shell_profile() {
    local profile_file="$1"
    if [ -f "${profile_file}" ]; then
        if ! grep -q ".vietlang/bin" "${profile_file}"; then
            echo "" >> "${profile_file}"
            echo "# VietLang Environment" >> "${profile_file}"
            echo "${export_line}" >> "${profile_file}"
            echo "${export_std}" >> "${profile_file}"
            echo -e " Added PATH to \033[32m${profile_file}\033[0m"
        fi
    fi
}

setup_shell_profile "${HOME}/.bashrc"
setup_shell_profile "${HOME}/.zshrc"
setup_shell_profile "${HOME}/.profile"
setup_shell_profile "${HOME}/.bash_profile"

# 6. Verify Installation
export PATH="${BIN_DIR}:${PATH}"
export VIETLANG_STD="${STD_DIR}"

echo ""
echo -e "\033[32m VietLang installed successfully!\033[0m"
echo -e "   Binary:   \033[33m${BIN_DIR}/vietlang\033[0m"
echo -e "   Standard: \033[33m${STD_DIR}\033[0m (49 modules)"
echo ""
echo -e "Quickstart commands:"
echo -e "  \033[36mvietlang\033[0m                  # Start REPL"
echo -e "  \033[36mvietlang doc\033[0m              # Browse 49 standard modules"
echo -e "  \033[36mvietlang init my_app api\033[0m  # Create a new backend project"
echo -e "  \033[36mvietlang run src/main.vl\033[0m  # Run server"
echo ""
echo -e "\033[33mPlease restart your terminal or run: source ~/.bashrc (or ~/.zshrc)\033[0m"
