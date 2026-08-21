# VietLang Multi-Platform Installation & VS Code Extension Publishing Guide

---

## 1. Multi-Platform One-Line Installation (Auto-Set PATH & Built-in Modules)

VietLang provides official one-line installers for Linux, macOS, and Windows that automatically install the binary, set up the **49 Standard Library Modules**, and configure your shell `PATH` variable:

### 🐧 Linux & 🍎 macOS (Bash, Zsh, Fish)

```bash
curl -fsSL https://raw.githubusercontent.com/hoangtuvungcao/vietlang/main/install.sh | bash
```

**What the installer does automatically:**
1. Detects CPU architecture (`x86_64` vs `arm64` / Apple Silicon M1-M4).
2. Installs binary to `~/.vietlang/bin/vietlang`.
3. Pre-installs and syncs all **49 Pure Standard Library Modules** into `~/.vietlang/std`.
4. Automatically writes `export PATH="$HOME/.vietlang/bin:$PATH"` into `~/.bashrc`, `~/.zshrc`, and `~/.profile`.
5. Ready to use immediately without requiring `sudo`!

---

### 🪟 Windows (PowerShell)

```powershell
iex (irm https://raw.githubusercontent.com/hoangtuvungcao/vietlang/main/install.ps1)
```

**What the Windows installer does automatically:**
1. Downloads `vietlang-windows-x64.exe` to `$HOME\.vietlang\bin\vietlang.exe`.
2. Syncs all standard modules to `$HOME\.vietlang\std`.
3. Sets user environment `PATH` permanently via `[Environment]::SetEnvironmentVariable`.

---

## 2. VS Code Extension Packaging & Marketplace Publishing Guide

The official VS Code extension for VietLang is located in `editors/vscode/`. It is 100% compliant with Microsoft VS Code Marketplace and Open VSX validation rules:

### 📦 Extension Assets & Structure

```
editors/vscode/
├── package.json               # Marketplace metadata (categories, keywords, icon, galleryBanner)
├── language-configuration.json# Bracket auto-closing and block comments
├── syntaxes/
│   └── vietlang.tmLanguage.json # Full TextMate syntax highlighting grammar
├── snippets/
│   └── vietlang.json         # 20+ backend productivity snippets
├── images/
│   └── icon.png              # 128x128 high-res logo
├── README.md                  # Extension overview & quickstart
├── CHANGELOG.md               # Version history
├── LICENSE                    # MIT License
└── vietlang-0.1.0.vsix        # Pre-packaged production VSIX archive
```

---

### 🚀 How to Publish to Microsoft VS Code Marketplace

#### Step 1: Create a Publisher Account
1. Visit [https://marketplace.visualstudio.com/manage](https://marketplace.visualstudio.com/manage).
2. Sign in with your GitHub / Microsoft account and create a publisher ID (e.g. `hoangtuvungcao`).

#### Step 2: Generate a Personal Access Token (PAT)
1. Go to Azure DevOps (`https://dev.azure.com/`).
2. Click **User settings** > **Personal access tokens**.
3. Create a token with **Marketplace (Manage)** scope.

#### Step 3: Publish via CLI or Web Dashboard

**Option A: Instant Web Upload**
1. Open [https://marketplace.visualstudio.com/manage](https://marketplace.visualstudio.com/manage).
2. Click **+ New Extension** > **Visual Studio Code**.
3. Drag and drop `editors/vscode/vietlang-0.1.0.vsix`.

**Option B: Command Line Publishing**
```bash
cd editors/vscode
npx -y @vscode/vsce login hoangtuvungcao
npx -y @vscode/vsce publish
```

---

## 3. GitHub Actions Automated CI/CD Release

The workflow in `.github/workflows/build.yml` automatically triggers on every release tag (e.g. `git tag v0.1.0 && git push --tags`) or via manual dispatch:
1. Runs full test suite (Cargo unit tests + VietLang integration tests).
2. Compiles cross-platform binaries:
   - `vietlang-linux-x64`
   - `vietlang-linux-arm64`
   - `vietlang-macos-x64`
   - `vietlang-macos-arm64`
   - `vietlang-windows-x64.exe`
3. Packages `vietlang-0.1.0.vsix`.
4. Automatically attaches all binaries, standard library, and VSIX extension to the GitHub Release!
