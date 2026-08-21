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
 ║              Backend-First Language v0.1.0    |___/        ║
 ║              Type 'help' for usage, 'exit' to quit         ║
 ╚════════════════════════════════════════════════════════════╝
```

<div align="center">

**A Backend-First Programming Language**

[![Build](https://github.com/hoangtuvungcao/vietlang/actions/workflows/build.yml/badge.svg)](https://github.com/hoangtuvungcao/vietlang/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/hoangtuvungcao/vietlang/releases)

[Getting Started](docs/getting-started.md) | [Language Reference](docs/language-reference.md) | [Stdlib Reference](docs/stdlib-reference.md) | [Contributing](CONTRIBUTING.md)

</div>

---

## Why VietLang?

VietLang is designed from the ground up for **backend development**. It combines the best features from Go, Rust, Python, and Kotlin into a single, cohesive language.

| Feature | VietLang | Go | Rust | Python | Node.js |
|:---|:---:|:---:|:---:|:---:|:---:|
| **Built-in HTTP Server** | Yes | Yes | No | No | No |
| **Built-in Database Driver** | Yes | No | No | No | No |
| **Built-in JSON** | Yes | Yes | No | Yes | Yes |
| **Pattern Matching** | Yes | No | Yes | Partial | No |
| **Null Safety** | Yes | No | Yes | No | No |
| **Result / Error Handling** | Yes | No | Yes | No | No |
| **Green Threads & Channels** | Yes | Yes | No | No | No |
| **Single Binary** | Yes | Yes | Yes | No | No |
| **Self-Hosting Ready** | Yes | Yes | Yes | No | No |
| **Learning Curve** | Low | Low | High | Low | Medium |

## Quick Start

### Install (1-Line Quickstart)

```bash
# Linux & macOS (Bash, Zsh, Fish) — Auto-detects CPU & sets PATH
curl -fsSL https://raw.githubusercontent.com/hoangtuvungcao/vietlang/main/install.sh | bash

# macOS (via Homebrew)
brew tap hoangtuvungcao/vietlang && brew install vietlang

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
let doubled = [1, 2, 3].map(fn(x) { return x * 2 })
let evens = [1, 2, 3, 4].filter(fn(x) { return x % 2 == 0 })
```

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

### Built-in HTTP Server

```rust
// Zero-dependency HTTP server
http_listen(8080, handler)
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

// Spawn lightweight tasks
spawn(process_task)

// Thread-safe state
let counter = mutex_new(0)
```

## Community Modules (std.*)

VietLang includes standard modules written directly in VietLang:

- `std.test`: Complete unit testing framework (`suite`, `test`, `assert_eq`, `assert_true`)
- `std.strings`: String utilities (`pad_left`, `pad_right`, `snake_case`, `camel_case`, `slugify`)
- `std.jwt`: Pure VietLang JWT token generator and validator (HMAC SHA-256)
- `std.http_router`: Router with middleware support and JSON response helpers

```rust
import std.test
import std.strings
import std.jwt
import std.http_router

suite("API Test Suite")
test("Token generation", fn() {
    let token = jwt_sign(map_set(map_new(), "user_id", 1), "secret")
    assert_true(token.len() > 0)
})
test_summary()
```

## Package Manager & Central Registry

VietLang provides a **native package manager built directly into the runtime binary** with a unified **Central Community Registry** and npm-style script execution:

```bash
# Initialize a new package (templates: lib | api | microservice)
vietlang init my_package api

# Run npm-style scripts from vietlang.json
vietlang start                     # Run the 'start' script or 'main' entrypoint
vietlang dev                       # Run the 'dev' development server script
vietlang run build                 # Run the 'build' standalone compilation script
vietlang run <script_name>         # Run any custom defined script in vietlang.json

# Search & install package from Central Registry (by name or explicit version lock)
vietlang search redis
vietlang install redis@1.2.0
vietlang install auth@3.0.0

# Interactive Documentation & API Inspector
vietlang doc                       # Browse all 54 standard library modules
vietlang doc std.vietqr            # Inspect signatures, types, and comments for VietQR
vietlang doc std.vnpay             # Inspect VNPay gateway docs
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

## Standalone AOT Binary Compilation (`vietlang build`)

Compile any VietLang source file into an independent, self-contained native executable binary with zero external dependencies:

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

// 1. Thread-safe Channels & Green Threads (spawn)
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

## Vietnamese Fintech & Cloud Integrations

```rust
import std.vietqr
import std.vnpay
import std.momo
import std.zalo

// 1. VietQR (50+ Vietnamese Commercial Banks)
let qr = vietqr_create_payment("MB", "0901234567", 250000, "Thanh toan #101")

// 2. VNPay 2.1.0 Gateway URL + HMAC-SHA512
let vnp = vnpay_client("TMN_CODE", "HASH_SECRET", "https://sandbox.vnpayment.vn/paymentv2/vpcpay.html")
let url = vnpay_create_payment_url(vnp, 1001, 500000, "Nap vi", "https://mysite.vn/return", "127.0.0.1")

// 3. MoMo E-Wallet Gateway Payload + HMAC-SHA256
let momo = momo_client("PARTNER_CODE", "ACCESS_KEY", "SECRET_KEY", "https://payment.momo.vn/v2/gateway/api/create")
let payload = momo_create_payment_payload(momo, 1002, 150000, "Don hang", "https://site.vn/return", "https://site.vn/ipn", "")
```

## Self-Hosting Bootstrap & Bytecode Virtual Machine (VM)

VietLang provides both a Tree-walking Interpreter and a high-performance stack-based Bytecode Virtual Machine (VM):

- `bootstrap/lexer.vl`: 100% VietLang lexer that tokenizes VietLang source code (including itself, 3,304 tokens)
- `bootstrap/parser.vl`: 100% VietLang parser that produces an Abstract Syntax Tree (AST)
- `--vm`: Execute source files using the Bytecode VM

```bash
# Run self-hosted lexer on its own source
vietlang bootstrap/lexer.vl

# Run self-hosted parser
vietlang bootstrap/parser.vl

# Execute via the Bytecode Virtual Machine
vietlang --vm examples/bytecode_vm_demo.vl
```

## Project Structure

```
vietlang/
├── src/
│   ├── main.rs              # CLI, REPL, Compiler & Subcommand Dispatcher
│   ├── pm.rs                # Package Manager, Central Registry & DocGen
│   ├── error.rs             # Error types & control flow signals
│   ├── stdlib.rs            # Built-in native standard library & CSP channels
│   ├── lexer/               # Lexer & Token definitions
│   ├── parser/              # Parser & AST definitions
│   ├── interpreter/         # Tree-walking interpreter & environment
│   └── vm/                  # Bytecode VM & compiler (OpCode stack machine)
├── registry/                # Central Community Package Registry
│   ├── shards/              # GitOps decentralized shard catalog
│   └── index.json           # Unified Community Package Catalog
├── std/                     # 54 Pure VietLang Standard Libraries (Zero-Cost Modularity)
│   ├── vietqr.vl            # Napas 247 VietQR standard for 50+ banks
│   ├── vnpay.vl             # VNPay 2.1.0 gateway with HMAC-SHA512
│   ├── momo.vl              # MoMo E-Wallet with HMAC-SHA256 signature
│   ├── zalo.vl              # Zalo OA & ZNS customer notification service
│   ├── concurrency.vl       # WorkerPool and parallel_map execution
│   ├── pagination.vl        # REST API pagination & query slicing
│   ├── email.vl             # Generic transactional email builder
│   ├── otp.vl               # 6-digit OTP generation & 2FA verification
│   ├── file_storage.vl      # MIME validator & safe file utilities
│   ├── ws.vl                # Opt-in WebSocket RFC 6455
│   ├── rate_limiter.vl      # Sliding-window rate limiter
│   ├── logger.vl            # Structured JSON logger
│   ├── session.vl           # High-performance session store
│   ├── queue.vl             # Async background job queue
│   ├── db_sqlite.vl         # In-memory & file-backed SQLite relational engine
│   ├── db_mysql.vl          # MySQL protocol and connection pool
│   ├── db_postgres.vl       # PostgreSQL client and schema tools
│   ├── multipart.vl         # Multipart form-data & file upload parser
│   ├── saga.vl              # Distributed SAGA transaction coordinator
│   ├── retry.vl             # Exponential backoff retry engine
│   ├── kv_store.vl          # Redis-level in-memory KV engine
│   ├── stream.vl            # Kafka-level partitioned stream broker
│   ├── http_pipeline.vl     # Onion middleware & security headers
│   ├── jwt.vl               # JWT signing & RBAC verification
│   ├── http_router.vl       # Web router & framework
│   └── ...                  # 54 modules total
├── bootstrap/               # Self-Hosting Compiler in VietLang
│   ├── lexer.vl             # Self-hosted lexer
│   └── parser.vl            # Self-hosted parser
├── examples/                # Real-World Enterprise Backend Demos
│   ├── agricultural_ecommerce_platform/ # Nông Sản Việt (Clean Architecture, Multi-Page, ACID)
│   ├── fintech_and_concurrency_test.vl  # VietQR, VNPay, MoMo, Zalo & Parallel Map Test
│   ├── native_concurrency_demo.vl       # Channel Streaming & Goroutines Test
│   ├── enterprise_ecosystem_demo.vl     # 14 Framework Integration Tests
│   └── multi_database_demo.vl           # SQLite, MySQL, Postgres Relational Tests
├── docs/                    # Complete documentation
│   ├── api/                 # Auto-generated API docs for all 54 modules
│   ├── standard-library-ecosystem.md
│   ├── getting-started.md
│   ├── language-reference.md
│   ├── stdlib-reference.md
│   ├── backend-cookbook.md
│   └── community-module-guide.md
├── Makefile
├── CONTRIBUTING.md
└── LICENSE
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
- [x] Phase 5: Community Standard Library in VietLang (54 pure modules in `std.*`)
- [x] Phase 6: Central Package Manager & Community Registry (`vietlang install`, `publish`, `search`)
- [x] Phase 7: Standalone AOT Binary Compiler (`vietlang build` for Linux ELF & Windows `.exe`)
- [x] Phase 8: Vietnamese Fintech Ecosystem (`std.vietqr`, `std.vnpay`, `std.momo`, `std.zalo`)
- [x] Phase 9: Self-Hosting Bootstrap (Stage 1: `bootstrap/lexer.vl`, Stage 2: `bootstrap/parser.vl`)
- [x] Phase 10: Bytecode VM / Native Stack-based Execution (`vietlang --vm`)

## License

MIT License - see [LICENSE](LICENSE) for details.
