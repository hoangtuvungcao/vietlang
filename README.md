# VietLang

```text
 ╔════════════════════════════════════════════════════════════╗
 ║       __      ___      _   _                               ║
 ║       \ \    / (_)    | | | |                              ║
 ║        \ \  / / _  ___| |_| |     __ _ _ __   __ _         ║
 ║         \ \/ / | |/ _ \ __| |    / _` | '_ \ / _` |        ║
 ║          \  /  | |  __/ |_| |___| (_| | | | | (_| |        ║
 ║           \/   |_|\___|\__|______\__,_|_| |_|\__, |        ║
 ║                                               __/ |        ║
 ║           Backend-First Language v0.3.0-alpha.1 |___/      ║
 ║              Type 'help' for usage, 'exit' to quit         ║
 ╚════════════════════════════════════════════════════════════╝
```

<div align="center">

**A Backend-First Programming Language**

[![Build](https://github.com/hoangtuvungcao/vietlang/actions/workflows/build.yml/badge.svg)](https://github.com/hoangtuvungcao/vietlang/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-0.3.0--alpha.1-blue.svg)](https://github.com/hoangtuvungcao/vietlang/releases)

[Getting Started](docs/getting-started.md) | [Draft Specification](docs/language-specification.md) | [Stdlib Reference](docs/stdlib-reference.md) | [Contributing](CONTRIBUTING.md)

</div>

---

> [!WARNING]
> **Experimental status:** VietLang 0.3.0-alpha.1 is a language/runtime prerelease. It is
> suitable for learning, local demos, and non-sensitive experiments, but it has
> not completed a security audit, protocol conformance program, or production
> hardening. Do not use it for real authentication, payments, secrets, or public
> production services yet.

## Why VietLang?

VietLang is designed from the ground up for **backend development**. It combines the best features from Go, Rust, Python, and Kotlin into a single, cohesive language.

| Feature | VietLang | Go | Rust | Python | Node.js |
|:---|:---:|:---:|:---:|:---:|:---:|
| **Built-in HTTP Server** | Yes | Yes | No | No | No |
| **Built-in Database Driver** | Yes | No | No | No | No |
| **Built-in JSON** | Yes | Yes | No | Yes | Yes |
| **Pattern Matching** | Yes | No | Yes | Partial | No |
| **Type Annotations** | Gradual/local checks | Yes | Yes | Dynamic | Dynamic |
| **Result / Error Handling** | Yes | No | Yes | No | No |
| **OS Threads & Channels** | Yes | Yes | Yes | Yes | Yes |
| **Standalone Bundle** | Yes | Yes | Yes | No | No |
| **Self-Hosting Bootstrap** | Partial | Yes | Yes | No | No |
| **Learning Curve** | Low | Low | High | Low | Medium |

## Quick Start

### Install (1-Line Quickstart)

```bash
# Linux & macOS (Bash, Zsh, Fish) — Auto-detects CPU & sets PATH
curl -fsSL https://raw.githubusercontent.com/hoangtuvungcao/vietlang/main/install.sh | bash

# Windows (PowerShell)
iex (irm https://raw.githubusercontent.com/hoangtuvungcao/vietlang/main/install.ps1)
```

### Hello World

```rust
// hello.vl
println("Hello, VietLang!")
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

## Language Features

### Variables and Types

```rust
let name = "VietLang"              // String (immutable)
let mut counter = 0                // Mutable variable
counter += 1                       // Compound assignment (+=, -=, *=, /=, %=)
let pi: Float = 3.14159           // Type annotation
let items = [1, 2, 3, 4, 5]      // Array
let active = true                  // Boolean
```

### Functions and Closures

```rust
fn add(a: Int, b: Int) -> Int {
    return a + b
}

fn greet(name: String) {
    println("Hello, " + name + "!")
}

// Higher-order functions & Lambdas
let doubled = [1, 2, 3].map(fn(x: Int) -> Int { return x * 2 })
let evens = [1, 2, 3, 4].filter(fn(x) { return x % 2 == 0 })
```

Closures use lexical scope, survive after their declaring block returns, and
sibling closures share captured mutable bindings. Named functions support
recursion. Captured state moved to `spawn` is synchronized, but multi-step
read/modify/write protocols still require an explicit mutex or channel.

### Error Handling with Try/Catch

```rust
try {
    let res = risky_operation()
    println(res)
} catch err {
    println("Caught error: " + err)
}
```

### Structs and Methods

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

let user = User { name: "Trong", email: "trong@example.com", age: 25 }
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

### Built-in HTTP/HTTPS Server

```rust
// HTTP/1.1 + HTTP/2. Add tls_cert_file/tls_key_file for HTTPS.
let mut config = map_new()
config = map_set(config, "port", 8080)
config = map_set(config, "max_concurrency", 256)
config = map_set(config, "max_body_bytes", 1048576)
http_listen(config, handler)
```

### Database Operations

```rust
// Query Builder & Raw SQL
db_query("SELECT * FROM users WHERE active = true")
db_table("users")
```

### JSON Processing

```rust
// Parse JSON
let data = json_parse("{\"name\": \"VietLang\", \"version\": \"0.1.0\"}")

// Stringify with pretty print
let output = json_stringify(data, true)
```

### File and Directory I/O

```rust
// Read/Write files
file_write("config.json", json_stringify(config, true))
let content = file_read("config.json")

// Directory operations
dir_create("my-project/src")
let files = dir_list("my-project")
```

### Cryptography and Security

```rust
let hash = sha256("password123")
let id = uuid()
let token = base64_encode("user:password")
```

### Concurrency

```rust
// Channels
let ch = channel(100)

// Spawn OS-thread-based tasks
spawn(process_task)

// Thread-safe state
let counter = mutex_new(0)
```

## Community Modules (std.*)

VietLang includes standard modules written directly in VietLang:

- `std.test`: Complete unit testing framework (`suite`, `test`, `assert_eq`, `assert_true`)
- `std.strings`: String utilities (`pad_left`, `pad_right`, `snake_case`, `camel_case`, `slugify`)
- `std.jwt`: disabled legacy compatibility module; do not use for new authentication
- `std.http_router`: Router with middleware support and JSON response helpers

```rust
import std.test
import std.strings
import std.http_router

suite("API Test Suite")
test("Router prerequisite", fn() {
    assert_true(true)
})
test_summary()
```

`vietlang check file.vl` runs lexing, parsing, name resolution, and the gradual
semantic/type checks. It currently validates locally known bindings, calls,
returns, structs, methods, enum payloads, and Bool/enum match exhaustiveness.
Imported and native APIs without machine-readable signatures remain `Unknown`;
passing `check` is therefore not a whole-program safety proof.

## Package Manager & Central Registry

VietLang includes an experimental package manager and npm-style scripts. The
installer resolves semver to an exact version, requires Ed25519 registry
signatures and SHA-256 content checks, activates from an isolated staging
directory, and records the immutable Git revision in `vietlang.lock`. Legacy
unsigned registry entries fail closed:

```bash
# 1-Command Backend Project Scaffolding (Clean Architecture, SQLite, Services & REST Routes)
vietlang new my_backend_service

# Run npm-style scripts from vietlang.json
vietlang dev                       # Run development server (http://localhost:8080)
vietlang start                     # Run the 'start' script or 'main' entrypoint
vietlang run build                 # Bundle source + runtime for Linux (ELF)
vietlang run build:win             # Bundle source + runtime for Windows (.exe)
vietlang run <script_name>         # Run any custom defined script in vietlang.json

# Search & install a development package (version request is not yet a lockfile)
vietlang search redis
vietlang install redis@1.2.0
vietlang install auth@3.0.0

# Interactive Documentation & API Inspector
vietlang doc                       # Browse standard library modules
vietlang doc std.vietqr            # Inspect signatures, types, and comments for VietQR
vietlang doc my_package            # Inspect custom or community package docs
vietlang doc --all                 # Generate complete Markdown docs into docs/api/

# Verify package syntax and unit tests
vietlang verify
vietlang test

# Update or remove dependencies
vietlang update redis
vietlang update redis@2.0.0
vietlang remove redis

# Publish module to Central Community Registry
vietlang publish
```

## Standalone Source Bundle (`vietlang build`)

Bundle a VietLang source file with the Rust runtime into an independent,
self-contained executable. The embedded source is parsed and interpreted at
startup; this is not ahead-of-time native compilation of VietLang code.

```bash
# 🐧 Build standalone Linux binary (ELF)
vietlang build src/main.vl -o my_service
./my_service

# 🪟 Build standalone Windows binary (.exe)
vietlang build src/main.vl -o my_service.exe --target windows

# 🍎 Build standalone macOS binary
vietlang build src/main.vl -o my_service_macos --target macos
```

## Native Concurrency & Channels (CSP Model)

```rust
import std.concurrency

// 1. Thread-safe Channels & OS Threads (spawn)
let ch = channel_new(10)

spawn(fn() {
    println("[Worker] Processing task...")
    channel_send(ch, "Task Done")
})

let result = channel_recv(ch)
println("[Main] Got: " + to_string(result))

// 2. High-Level Parallel Map across Worker Pool
let numbers = [10, 20, 30, 40, 50]
let squared = parallel_map(numbers, fn(n) { return n * n })
// [100, 400, 900, 1600, 2500] (executed in parallel across threads)
```

## Provider integrations belong in community packages

The core standard library provides crypto, HTTP/HTTPS, JSON, and database
building blocks. Provider-specific payment behavior such as MoMo and VNPay is
not part of core security conformance. Applications should install or build a
versioned community package and validate it against the provider contract they
actually use. The legacy `std.momo` and `std.vnpay` compatibility modules remain
disabled by default and must not be used for live transactions.

```rust
import std.vietqr
import std.zalo

// 1. VietQR (50+ Vietnamese Commercial Banks)
let qr = vietqr_create_payment("MB", "0901234567", 250000, "Thanh toan #101")

// Provider SDKs should be installed as versioned community packages.
```

## Self-Hosting Bootstrap & Bytecode Virtual Machine (VM)

VietLang provides a tree-walking interpreter and an experimental stack-based Bytecode Virtual Machine (VM):

- `bootstrap/lexer.vl`: 100% VietLang lexer that tokenizes VietLang source code (including itself, 3,304 tokens)
- `bootstrap/parser.vl`: 100% VietLang parser that produces an Abstract Syntax Tree (AST)
- `--vm`: Execute source files using the Bytecode VM
- The covered VM subset is checked by differential tests against the interpreter;
  unsupported AST nodes fail compilation explicitly.

```bash
# Run self-hosted lexer on its own source
vietlang bootstrap/lexer.vl

# Run self-hosted parser
vietlang bootstrap/parser.vl

# Execute via the Bytecode Virtual Machine
vietlang --vm examples/bytecode_vm_demo.vl
```

## Project Structure

```text
vietlang/
├── src/                     # Core Rust Engine (Parser, Lexer, Interpreter, VM, Package Manager)
├── std/                     # 60+ Standard Library Modules (including experimental/disabled integrations)
├── registry/                # Central Community Package Registry & GitOps Shards
├── bootstrap/               # Self-Hosting Compiler (Lexer & Parser written in VietLang)
├── examples/                # Curated Showcase Examples & Full Applications
│   ├── 01_basics/           # Core Syntax, Structs, Arrays, Maps, File I/O & JSON
│   │   ├── hello_world.vl
│   │   ├── data_structures.vl
│   │   └── file_and_json.vl
│   ├── 02_backend/          # HTTP/1.1 + HTTP/2 backend examples, SQLite & WebSockets
│   │   ├── sqlite_database.vl
│   │   ├── http_rest_api.vl
│   │   └── websocket_realtime.vl
│   ├── 03_fintech_and_concurrency/ # Multi-Channel Fintech & Concurrency Thread Pool
│   │   ├── vietnam_fintech.vl
│   │   └── concurrency_csp.vl
│   └── 04_full_apps/        # Experimental Full-Stack Example Projects
│       ├── agricultural_ecommerce/ # Nông Sản Việt (Clean Architecture E-Commerce Platform)
│       └── viet_fintech_gateway/   # VietFintech Payment Gateway Microservice
└── docs/                    # Architecture guides, API reference & Developer tutorials
```

## Building and Testing

```bash
# Build release binary
make build

# Run all unit tests
make test

# Run all examples, backend enterprise suites, and demos
make demo
```

## Roadmap

- [x] Phase 1: Core Language (Lexer, Parser, AST, Tree-walking Interpreter)
- [x] Phase 2: Memory & Concurrency (CSP Channels, Spawn, Mutex, Control Flow Signals)
- [x] Phase 3: Standard Library (HTTP, DB, JSON, File I/O, Crypto, Logging, Env, Time, Collections)
- [x] Phase 4: Extended Syntax (Compound Assignment += -= *= /= %=, Short-circuit && ||, Try/Catch)
- [x] Phase 5 prototype library catalog (60+ modules in `std.*`, with legacy modules clearly marked)
- [x] Phase 6: Central Package Manager & Community Registry (`vietlang install`, `publish`, `search`)
- [x] Phase 7: Standalone Source Bundler (`vietlang build` for Linux ELF & Windows `.exe`)
- [x] Phase 8 prototype archive: VietQR/Zalo helpers and disabled legacy payment adapters
- [x] Phase 9: Self-Hosting Bootstrap (Stage 1: `bootstrap/lexer.vl`, Stage 2: `bootstrap/parser.vl`)
- [x] Phase 10: Bytecode VM / Native Stack-based Execution (`vietlang --vm`)
- [x] Phase 11 initial: semantic analyzer for scopes, arity, local annotations,
      structs, methods, enum payloads, returns, and match exhaustiveness
- [x] Phase 12 initial: lexical closures and interpreter/VM differential
      conformance for the documented bytecode subset

## License

MIT License - see [LICENSE](LICENSE) for details.
