# VietLang Language & Runtime Handbook

The definitive guide to learning and mastering VietLang for backend systems engineering.

---

## Table of Contents

1. [Introduction & Design Philosophy](#1-introduction--design-philosophy)
2. [Variables, Types, and Immutability](#2-variables-types-and-immutability)
3. [Operators and Expressions](#3-operators-and-expressions)
4. [Functions and Closures](#4-functions-and-closures)
5. [Control Flow and Pattern Matching](#5-control-flow-and-pattern-matching)
6. [Error Handling & Exceptions](#6-error-handling--exceptions)
7. [Structs, Enums, and Methods](#7-structs-enums-and-methods)
8. [Standard Library & Built-in Modules](#8-standard-library--built-in-modules)
9. [Concurrency & Network I/O](#9-concurrency--network-io)
10. [Package Management with VPM](#10-package-management-with-vpm)

---

## 1. Introduction & Design Philosophy

VietLang is a **backend-first** programming language. Unlike general-purpose languages that require dozens of external dependencies to start a simple HTTP server or query a database, VietLang provides:

- Built-in zero-dependency HTTP server and client.
- Built-in connection-pooled database engine.
- Fast JSON serialization and deserialization.
- First-class lightweight green threads, channels, and synchronization primitives.
- Self-hosting toolchain and standalone single binary distribution.

---

## 2. Variables, Types, and Immutability

### Variable Bindings

In VietLang, variables are immutable by default:

```rust
let port = 8080               // Immutable Int
let host = "0.0.0.0"          // Immutable String
let debug_mode = true         // Immutable Bool
let initial_weight = 72.5     // Immutable Float
```

To create a mutable variable, use the `mut` keyword:

```rust
let mut request_counter = 0
request_counter += 1
```

### Supported Primitives

| Type | Description | Example |
|---|---|---|
| `Int` | 64-bit signed integer | `42`, `-100`, `1_000_000` |
| `Float` | 64-bit floating point | `3.14159`, `-0.005` |
| `String` | UTF-8 string | `"Hello, VietLang"` |
| `Bool` | Boolean truth values | `true`, `false` |
| `None` | Null / absence of value | `none` |
| `Array` | Dynamic array | `[1, 2, 3, "mixed"]` |
| `Map` | Key-value associative store | `map_set(map_new(), "key", "val")` |

---

## 3. Operators and Expressions

### Arithmetic & Compound Assignment

```rust
let a = 10 + 5 * 2   // 20
let mut x = 100
x += 10              // 110
x -= 5               // 105
x *= 2               // 210
x /= 3               // 70
x %= 4               // 2
```

### Short-Circuit Logical Operators

```rust
// If left is false, right is never evaluated (safe against out-of-bounds)
if index < arr.len() && arr[index] == target {
    println("Found target")
}
```

---

## 4. Functions and Closures

### Named Functions

```rust
fn calculate_tax(amount: Float, rate: Float = 0.1) -> Float {
    return amount * rate
}
```

### Higher-Order Functions and Lambdas

```rust
let numbers = [1, 2, 3, 4, 5]

// Map transformation
let squared = numbers.map(fn(x) { return x * x })

// Filter
let evens = numbers.filter(fn(x) { return x % 2 == 0 })
```

---

## 5. Control Flow and Pattern Matching

### If-Else Statements

```rust
if status == 200 {
    println("Success")
} else if status == 404 {
    println("Resource not found")
} else {
    println("Other status: " + to_string(status))
}
```

### Loops

```rust
// While loop
let mut i = 0
while i < 5 {
    println(i)
    i += 1
}

// For-in loop
for user in user_list {
    println("Processing: " + user.name)
}
```

### Pattern Matching

```rust
fn route_status(code: Int) -> String {
    return match code {
        200 => "OK",
        201 => "Created",
        400 => "Bad Request",
        401 => "Unauthorized",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Unknown Status"
    }
}
```

---

## 6. Error Handling & Exceptions

```rust
try {
    let raw_config = file_read("config.json")
    let config = json_parse(raw_config)
} catch err {
    log_error("Failed to load configuration: " + err)
    let config = map_new()
}

// Throwing custom errors
if user_id <= 0 {
    throw("Invalid user identifier")
}
```

---

## 7. Structs, Enums, and Methods

```rust
struct User {
    id: Int,
    name: String,
    email: String
}

impl User {
    pub fn format_display(self) -> String {
        return "#" + to_string(self.id) + ": " + self.name + " (" + self.email + ")"
    }
}

let u = User { id: 1, name: "Admin", email: "admin@example.com" }
println(u.format_display())
```

---

## 8. Standard Library & Built-in Modules

- `std.http_router`: High-level web framework and routing.
- `std.validator`: Request payload validation.
- `std.jwt`: JWT authentication tokens.
- `std.cache`: In-memory TTL caching.
- `std.orm`: SQL query builder and data layer.
- `std.strings`: String transformation tools.
- `std.test`: Unit and integration testing framework.

---

## 9. Concurrency & Network I/O

```rust
// Create channel
let ch = channel(50)

// Spawn background worker
spawn(fn() {
    println("Worker running in background task")
})

// Thread-safe mutex state
let state = mutex_new(0)
```

---

## 10. Package Management with VPM

```bash
# Create a new project
vietlang vpm.vl init my_service

# Run project
vietlang src/main.vl
```
