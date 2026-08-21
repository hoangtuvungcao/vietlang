# VietLang Visual Studio Code Setup & Development Guide

This guide provides instructions for setting up Visual Studio Code for writing, running, and debugging VietLang code.

---

## 1. Installing the VietLang VS Code Extension

The extension package is located in `editors/vscode/`. You can install it locally directly into your VS Code environment.

### Quick Install (Local Linking)

```bash
# Linux / macOS
mkdir -p ~/.vscode/extensions/
cp -r editors/vscode ~/.vscode/extensions/vietlang-0.1.0
```

### Packaging into a .vsix Bundle

If you have `vsce` installed:

```bash
cd editors/vscode
npx @vscode/vsce package
code --install-extension vietlang-0.1.0.vsix
```

---

## 2. Features Included in the Extension

- **Full Syntax Highlighting**: Keywords (`let`, `mut`, `fn`, `struct`, `match`, `try`, `catch`), built-in types (`Int`, `String`, `Map`), operators (`+=`, `-=`, `=>`, `->`), strings, numbers, comments.
- **Auto-Closing Pairs & Bracket Matching**: Parentheses, brackets, curly braces, and double quotes.
- **Code Snippets**:
  - `fn` -> Function definition template
  - `struct` -> Struct template
  - `route` -> HTTP backend route handler template
  - `test` -> `std.test` unit test template
  - `try` -> Try-catch error handling block
  - `match` -> Pattern matching block

---

## 3. Configuring VS Code Build & Run Tasks

Create a `.vscode/tasks.json` in your project root to run VietLang files with a single shortcut (`Ctrl+Shift+B` or `F5`):

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "VietLang: Run Current File",
      "type": "shell",
      "command": "vietlang ${file}",
      "group": {
        "kind": "build",
        "isDefault": true
      },
      "presentation": {
        "reveal": "always",
        "panel": "shared",
        "clear": true
      },
      "problemMatcher": []
    },
    {
      "label": "VietLang: Run Package Tests",
      "type": "shell",
      "command": "vietlang examples/community_modules_demo.vl",
      "group": "test",
      "presentation": {
        "reveal": "always",
        "panel": "shared"
      },
      "problemMatcher": []
    }
  ]
}
```

---

## 4. Debugging & Formatting Tips

- **Check AST**: Use `vietlang --ast <file.vl>` to inspect the syntax tree.
- **Check Tokens**: Use `vietlang --tokens <file.vl>` to inspect the lexical token stream.
- **Run Package Manager**: Use `vietlang vpm.vl <command>` to manage modules.
