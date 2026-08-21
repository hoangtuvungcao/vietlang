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

VietLang includes **30 pure-VietLang standard modules**:

1. `std.db_sqlite`: SQLite in-memory and file-backed database engine with ACID transactions.
2. `std.db_mysql`: MySQL connection protocol, pooling, and prepared queries.
3. `std.db_postgres`: PostgreSQL client, connection management, and health checks.
4. `std.multipart`: Streaming multipart form-data and file upload parser.
5. `std.saga`: Distributed SAGA Transaction Coordinator with compensating rollbacks.
6. `std.retry`: Exponential backoff retry policy with jitter.
7. `std.cron`: Enterprise job scheduling engine.
8. `std.cache_lru`: Fixed-capacity Least-Recently-Used cache with eviction.
9. `std.sql_builder`: Multi-table SQL Query Builder (INNER/LEFT JOIN, GROUP BY).
10. `std.metrics`: Prometheus Metrics Exporter (Counters, Gauges).
11. `std.security`: Password hashing, constant-time compare, CSRF, XSS filter.
12. `std.crypto_advanced`: Webhook HMAC-SHA256 signature verification and payload encryption.
13. `std.kv_store`: In-memory Redis engine (Atomic `INCR`, Hashes, TTL).
14. `std.stream`: Kafka-like partitioned stream engine with consumer offsets.
15. `std.http_pipeline`: Onion-model middleware with automated Security Headers (CSP, HSTS).
16. `std.websocket`: WebSocket RFC 6455 framing and room broadcaster.
17. `std.socket`: Raw TCP/UDP low-level socket client.
18. `std.jwt`: JWT authentication with Role-Based Access Control (RBAC).
19. `std.http_router`: High-level web routing and JSON responses.
20. `std.validator`: Request payload validation rules.
21. `std.orm`: SQL query builder and data layer.
22. `std.migration`: Database schema migration versioning.
23. `std.rate_limiter`: Token bucket DDoS protection.
24. `std.circuit_breaker`: Fault-tolerance circuit breaker pattern.
25. `std.telemetry`: OpenTelemetry trace context and header propagation.
26. `std.health`: Kubernetes `/healthz` and `/readyz` probe checkers.
27. `std.event_bus`: In-memory Pub/Sub event bus.
28. `std.queue`: Asynchronous task queue with Dead-Letter Queue (DLQ).
29. `std.config`: Environment variables and `.env` loader.
30. `std.test`: Complete unit testing assertions framework.

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

## 10. Package Management with VPM (`vpm.vl`)

VietLang features a **decentralized, serverless package manager**:

```bash
# Initialize a new package (templates: lib | api | microservice)
vietlang vpm.vl init my_service microservice

# Search community packages
vietlang vpm.vl search redis
vietlang vpm.vl search postgres

# Install any Git repository directly as a module
vietlang vpm.vl install https://github.com/user/my_module.git

# Inspect exported functions without opening code
vietlang vpm.vl docs my_module

# Verify code syntax and automated test suites
vietlang vpm.vl verify

# Update or remove modules
vietlang vpm.vl update my_module
vietlang vpm.vl remove my_module

# Validate package before publishing
vietlang vpm.vl publish
```
