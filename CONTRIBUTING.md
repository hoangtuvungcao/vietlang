# Contributing to VietLang 🇻🇳

Thank you for your interest in contributing to VietLang! We welcome contributions from everyone.

## Getting Started

### Prerequisites
- **Rust 1.70+**: Install from [rustup.rs](https://rustup.rs)
- **Git**: For version control

### Setup

```bash
# Fork and clone
git clone https://github.com/YOUR-USERNAME/vietlang.git
cd vietlang

# Build
cargo build

# Run tests
cargo test

# Run an example
cargo run -- examples/demo.vl
```

## How to Contribute

### 1. Report Bugs
- Use [GitHub Issues](https://github.com/hoangtuvungcao/vietlang/issues)
- Include: VietLang version, OS, steps to reproduce, expected vs actual behavior
- Include a minimal `.vl` file that reproduces the issue

### 2. Suggest Features
- Open an issue with the `[Feature]` prefix
- Describe the use case and proposed syntax

### 3. Submit Code

```bash
# Create a branch
git checkout -b feature/my-feature

# Make changes
# ... edit files ...

# Test
cargo test
cargo run -- examples/demo.vl

# Commit
git add .
git commit -m "feat: add my feature"

# Push and create PR
git push origin feature/my-feature
```

## Code Style

- Follow standard Rust conventions (`rustfmt`)
- Use meaningful variable names
- Add comments for complex logic
- Add tests for new features

## Project Structure

```
src/
├── main.rs            # CLI entry point + REPL
├── error.rs           # Error types
├── stdlib.rs          # Standard library builtins
├── lexer/
│   ├── mod.rs         # Tokenizer
│   └── token.rs       # Token types
├── parser/
│   ├── mod.rs         # Parser
│   └── ast.rs         # AST nodes
└── interpreter/
    ├── mod.rs         # Interpreter
    ├── value.rs       # Runtime values
    └── environment.rs # Variable scoping
```

## Adding a Built-in Function

1. Add the function implementation in `src/stdlib.rs`
2. Register it in `src/interpreter/mod.rs` → `register_builtins()`
3. Add the dispatch in `call_builtin()` match block
4. Add documentation in `docs/stdlib-reference.md`
5. Add an example usage

## Commit Message Convention

```
feat: add new feature
fix: fix a bug
docs: update documentation
test: add tests
refactor: code refactoring
chore: build, CI, etc.
```

## Code of Conduct

Be respectful, inclusive, and constructive. We're building something together! 🤝

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
