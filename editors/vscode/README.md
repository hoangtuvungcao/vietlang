# VietLang Visual Studio Code Extension

Official Visual Studio Code extension providing rich language support, syntax highlighting, custom file icons, 1-click code execution, real-time diagnostics linter, and developer snippets for **VietLang** — The Backend-First Programming Language.

---

## ✨ Features

- 🎨 **Rich Syntax Highlighting**: Full TextMate grammar for functions, variables, keywords, control flow, pattern matching, closures, types, and strings.
- 📁 **Custom File Icons**: Beautiful emerald-green `.vl` file icons in the File Explorer.
- ▶ **1-Click Run Button**: Click the ▶ icon in the top-right editor titlebar or press `Ctrl+Alt+V` (`Cmd+Alt+V` on macOS) to instantly run active `.vl` files in the integrated terminal.
- 🔍 **Real-Time Diagnostics (Linter)**: Syntax error detection with red squiggly underlines and direct error reporting in the **Problems** tab (`Ctrl+Shift+M`).
- ⚡ **Fintech & Backend Snippets**:
  - `vietqr`: Napas 247 VietQR generation for 50+ Vietnamese commercial banks
  - `vnpay`: VNPay 2.1.0 gateway session with HMAC-SHA512
  - `momo`: MoMo E-Wallet HMAC-SHA256 signature payload
  - `zalo`: Zalo ZNS & OA customer notification dispatch
  - `http-server`: Experimental HTTP/1.1 REST server bootstrap
  - `ws-server`: Real-time WebSocket broadcaster
  - `sqlite-crud`: SQLite ACID relational operations
  - `concurrency`: OS-thread-based `spawn` tasks and channels
  - `jwt-auth`: JWT signing & authorization verification
- 🔧 **Command Palette Integration (`Ctrl+Shift+P`)**:
  - `VietLang: Run Active File`
  - `VietLang: Check Syntax (Linter)`
  - `VietLang: Build Standalone Binary`
  - `VietLang: Start Interactive REPL`
  - `VietLang: Browse Standard Library Docs`

---

## 📥 Installation

### Method 1: Install from VSIX file
```bash
code --install-extension vietlang-0.1.1.vsix
```
Or in VS Code:
1. Open Extensions view (`Ctrl+Shift+X`).
2. Click the `...` menu in the top right.
3. Select **Install from VSIX...** and choose `vietlang-0.1.1.vsix`.

### Method 2: Install from VS Code Marketplace
Search for **VietLang** in the Extensions tab (`Ctrl+Shift+X`) and click **Install**.

---

## 📖 Links

- **Repository**: [https://github.com/hoangtuvungcao/vietlang](https://github.com/hoangtuvungcao/vietlang)
- **Documentation**: [https://github.com/hoangtuvungcao/vietlang/tree/main/docs](https://github.com/hoangtuvungcao/vietlang/tree/main/docs)

---

## 📄 License

MIT License — see [LICENSE](LICENSE) for details.
