# VietLang

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

### Install

```bash
# Build from source (Requirements: Rust 1.70+)
git clone https://github.com/hoangtuvungcao/vietlang.git
cd vietlang
cargo build --release
sudo cp target/release/vietlang /usr/local/bin/
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

## Package Manager (vpm)

VietLang comes with `vpm.vl`, a full package manager written in pure VietLang:

```bash
# Initialize a new package
vietlang vpm.vl init my_api

# List installed modules
vietlang vpm.vl list

# View package info
vietlang vpm.vl info
```

## Self-Hosting Bootstrap

VietLang is designed for complete self-hosting (writing the VietLang compiler in VietLang itself):

- `bootstrap/lexer.vl`: 100% VietLang lexer that tokenizes VietLang source code (including itself)
- `bootstrap/parser.vl`: 100% VietLang parser that produces an Abstract Syntax Tree (AST)

```bash
# Run self-hosted lexer on its own source
vietlang bootstrap/lexer.vl

# Run self-hosted parser
vietlang bootstrap/parser.vl
```

## Project Structure

```
vietlang/
├── src/
│   ├── main.rs              # CLI and REPL
│   ├── error.rs             # Error types & control flow signals
│   ├── stdlib.rs            # Built-in standard library
│   ├── lexer/               # Lexer & Token definitions
│   ├── parser/              # Parser & AST definitions
│   └── interpreter/         # Tree-walking interpreter & environment
├── std/                     # Pure VietLang Community Standard Libraries
│   ├── test.vl              # Unit testing framework
│   ├── strings.vl           # String utilities
│   ├── jwt.vl               # JWT signing & verification
│   └── http_router.vl       # Web router & framework
├── bootstrap/               # Self-Hosting Compiler in VietLang
│   ├── lexer.vl             # Self-hosted lexer
│   └── parser.vl            # Self-hosted parser
├── examples/                # Example applications
│   ├── demo.vl
│   ├── backend_demo.vl
│   ├── http_server.vl
│   ├── database.vl
│   ├── concurrency.vl
│   ├── file_io.vl
│   ├── json_demo.vl
│   ├── new_features_test.vl
│   └── community_modules_demo.vl
├── docs/                    # Complete documentation
│   ├── getting-started.md
│   ├── language-reference.md
│   └── stdlib-reference.md
├── vpm.vl                   # VietLang Package Manager
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

# Run all examples and demos
make demo
```

## Roadmap

- [x] Phase 1: Core Language (Lexer, Parser, AST, Tree-walking Interpreter)
- [x] Phase 2: Memory & Concurrency (Channels, Spawn, Mutex, Control Flow Signals)
- [x] Phase 3: Standard Library (HTTP, DB, JSON, File I/O, Crypto, Logging, Env, Time, Collections)
- [x] Phase 4: Extended Syntax (Compound Assignment += -= *= /= %=, Short-circuit && ||, Try/Catch)
- [x] Phase 5: Community Standard Library in VietLang (std.test, std.strings, std.jwt, std.http_router)
- [x] Phase 6: Package Manager (`vpm.vl`)
- [x] Phase 7: Self-Hosting Bootstrap (Stage 1: `bootstrap/lexer.vl`, Stage 2: `bootstrap/parser.vl`)
- [ ] Phase 8: Bytecode VM / Native Backend

## License

MIT License - see [LICENSE](LICENSE) for details.
