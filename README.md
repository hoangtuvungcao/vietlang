# VietLang 🇻🇳

<div align="center">

```
 __      ___      _   _
 \ \    / (_)    | | | |
  \ \  / / _  ___| |_| |     __ _ _ __   __ _
   \ \/ / | |/ _ \ __| |    / _` | '_ \ / _` |
    \  /  | |  __/ |_| |___| (_| | | | | (_| |
     \/   |_|\___|\__|______\__,_|_| |_|\__, |
                                         __/ |
                                        |___/
```

**A Backend-First Programming Language**

[![Build](https://github.com/hoangtuvungcao/vietlang/actions/workflows/build.yml/badge.svg)](https://github.com/hoangtuvungcao/vietlang/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/hoangtuvungcao/vietlang/releases)

[Getting Started](docs/getting-started.md) • [Language Reference](docs/language-reference.md) • [Stdlib Reference](docs/stdlib-reference.md) • [Contributing](CONTRIBUTING.md)

</div>

---

## ✨ Why VietLang?

VietLang is designed from the ground up for **backend development**. It combines the best features from Go, Rust, Python, and Kotlin into a single, cohesive language.

| Feature | VietLang | Go | Rust | Python | Node.js |
|:---|:---:|:---:|:---:|:---:|:---:|
| **Built-in HTTP Server** | ✅ | ✅ | ❌ | ❌ | ❌ |
| **Built-in Database** | ✅ | ❌ | ❌ | ❌ | ❌ |
| **Built-in JSON** | ✅ | ✅ | ❌ | ✅ | ✅ |
| **Pattern Matching** | ✅ | ❌ | ✅ | ⚠️ | ❌ |
| **Null Safety** | ✅ | ❌ | ✅ | ❌ | ❌ |
| **Result Types** | ✅ | ❌ | ✅ | ❌ | ❌ |
| **Green Threads** | ✅ | ✅ | ❌ | ❌ | ❌ |
| **Single Binary** | ✅ | ✅ | ✅ | ❌ | ❌ |
| **Learning Curve** | 🟢 Low | 🟢 Low | 🔴 High | 🟢 Low | 🟡 Medium |

## 🚀 Quick Start

### Install

```bash
# Linux / macOS
curl -sSL https://raw.githubusercontent.com/hoangtuvungcao/vietlang/main/install.sh | bash

# Or build from source
git clone https://github.com/hoangtuvungcao/vietlang.git
cd vietlang
cargo build --release
sudo cp target/release/vietlang /usr/local/bin/
```

### Hello World

```rust
// hello.vl
println("Hello, VietLang! 🇻🇳")
```

```bash
vietlang hello.vl
```

### Interactive REPL

```bash
$ vietlang
vl:1 > let name = "VietLang"
vl:2 > println("Hello, " + name + "!")
Hello, VietLang!
vl:3 > 2 + 3 * 4
  = 14
```

## 📖 Language Features

### Variables & Types

```rust
let name = "VietLang"              // String (immutable)
let mut counter = 0                // Mutable variable
let pi: Float = 3.14159           // Type annotation
let items = [1, 2, 3, 4, 5]      // Array
let active = true                  // Boolean
```

### Functions

```rust
fn add(a: Int, b: Int) -> Int {
    return a + b
}

fn greet(name: String) {
    println("Hello, " + name + "! 👋")
}

// Higher-order functions
let doubled = [1, 2, 3].map(fn(x) { return x * 2 })
let evens = [1, 2, 3, 4].filter(fn(x) { return x % 2 == 0 })
```

### Structs & Methods

```rust
struct User {
    name: String,
    email: String,
    age: Int
}

impl User {
    pub fn display(self) {
        println(self.name + " <" + self.email + ">")
    }
}

let user = User { name: "Trong", email: "trong@vietlang.dev", age: 25 }
```

### Pattern Matching

```rust
fn http_status(code: Int) -> String {
    return match code {
        200 => "OK",
        201 => "Created",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Unknown"
    }
}
```

### 🌐 Built-in HTTP Server

```rust
// Zero-dependency HTTP server
http_listen(8080, handler)
```

### 🗄️ Database Operations

```rust
// Query Builder
db_query("SELECT * FROM users WHERE active = true")
db_table("users")

// Connection pool built-in
log_info("Pool: min=5, max=20, health_check=5s")
```

### 📋 JSON Processing

```rust
// Parse JSON
let data = json_parse("{\"name\": \"VietLang\", \"version\": \"0.1.0\"}")

// Stringify with pretty print
let output = json_stringify(data, true)
```

### 📁 File I/O

```rust
// Read/Write files
file_write("config.json", json_stringify(config, true))
let content = file_read("config.json")

// Directory operations
dir_create("my-project/src")
let files = dir_list("my-project")
```

### 🔐 Crypto & Security

```rust
let hash = sha256("password123")
let id = uuid()
let token = base64_encode("user:password")
```

### ⚡ Concurrency

```rust
// Channels
let ch = channel(100)

// Spawn tasks
spawn(process_task)

// Thread-safe state
let counter = mutex_new(0)
```

## 📂 Project Structure

```
vietlang/
├── src/
│   ├── main.rs              # CLI & REPL
│   ├── error.rs             # Error types
│   ├── stdlib.rs            # Standard library (HTTP, DB, JSON, IO, Crypto...)
│   ├── lexer/
│   │   ├── mod.rs           # Tokenizer
│   │   └── token.rs         # Token types
│   ├── parser/
│   │   ├── mod.rs           # Recursive descent parser
│   │   └── ast.rs           # AST definitions
│   └── interpreter/
│       ├── mod.rs           # Tree-walking interpreter
│       ├── value.rs         # Runtime values
│       └── environment.rs   # Variable scoping
├── examples/
│   ├── demo.vl              # Feature showcase
│   ├── backend_demo.vl      # Backend patterns
│   ├── http_server.vl       # HTTP server demo
│   ├── database.vl          # Database demo
│   ├── concurrency.vl       # Concurrency demo
│   ├── file_io.vl           # File I/O demo
│   └── json_demo.vl         # JSON processing demo
├── docs/
│   ├── getting-started.md   # Installation & tutorial
│   ├── language-reference.md # Full language reference
│   └── stdlib-reference.md  # Standard library API
├── .github/workflows/
│   └── build.yml            # CI/CD
├── Makefile                  # Build automation
├── CONTRIBUTING.md           # How to contribute
└── LICENSE                   # MIT License
```

## 🛠️ Building from Source

```bash
# Requirements: Rust 1.70+
git clone https://github.com/hoangtuvungcao/vietlang.git
cd vietlang

# Build
make build          # or: cargo build --release

# Run tests
make test           # or: cargo test

# Install
make install        # copies to /usr/local/bin

# Run examples
make demo
```

## 🗺️ Roadmap

- [x] **Phase 1**: Core Language (Lexer, Parser, Interpreter)
- [x] **Phase 2**: Concurrency (Channels, Spawn, Mutex)
- [x] **Phase 3**: Standard Library (HTTP, DB, JSON, IO, Crypto, Logging)
- [x] **Phase 4**: Toolchain (CLI, REPL, Debug modes)
- [ ] **Phase 5**: Package Manager (`vpm`)
- [ ] **Phase 6**: Static Type Checker
- [ ] **Phase 7**: LLVM Backend (compile to native)
- [ ] **Phase 8**: Self-Hosting (VietLang written in VietLang)

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

```bash
# Fork, clone, create branch
git checkout -b feature/my-feature

# Make changes, test
cargo test

# Submit PR
```

## 📄 License

MIT License — see [LICENSE](LICENSE) for details.

## 💬 Community

- **GitHub Issues**: Bug reports & feature requests
- **Discussions**: Questions & ideas

---

<div align="center">
  <strong>Made with ❤️ in Vietnam 🇻🇳</strong>
</div>
