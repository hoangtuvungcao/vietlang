---
name: vietlang-developer
description: Comprehensive expert syntax guide, project architecture manual, standard library catalog, and backend programming reference for AI agents and developers building with VietLang.
---

# VietLang — Master Language Syntax, Project Architecture & Full-Stack Backend Manual

VietLang is a **backend-first, statically-analyzed, high-throughput programming language** designed specifically for production REST APIs, microservices, real-time WebSocket systems, transactional ACID databases, and native multi-threading concurrency.

GitHub Repository: [https://github.com/hoangtuvungcao/vietlang](https://github.com/hoangtuvungcao/vietlang)

---

## 1. Quick Installation & Multi-Platform Setup

### 🐧 Linux & 🍎 macOS (Apple Silicon M1-M4 & Intel x64) & 📱 Android (Termux)
```bash
curl -fsSL https://raw.githubusercontent.com/hoangtuvungcao/vietlang/main/install.sh | bash
```
- **Auto-Configures PATH** in `~/.bashrc`, `~/.zshrc`, and `~/.profile`.
- **Pre-installs all 54 Standard Library Modules** into `~/.vietlang/std`.
- **Zero sudo required**: Binary installed directly to `~/.vietlang/bin/vietlang`.

### 🪟 Windows (PowerShell)
```powershell
iex (irm https://raw.githubusercontent.com/hoangtuvungcao/vietlang/main/install.ps1)
```
- Installs to `$HOME\.vietlang\bin\vietlang.exe` and sets user environment `PATH`.

### 🍺 Homebrew for macOS
```bash
brew tap hoangtuvungcao/vietlang
brew install vietlang
```

---

## 2. Master Project Architecture & Directory Layout

When creating a production-grade VietLang service, AI agents and developers **MUST** follow this clean, modular architecture:

```text
my_enterprise_app/
├── vpm.json                      # Project manifest (dependencies, version, author)
├── README.md                     # Documentation & API endpoints
├── data/                         # Persistent database files (SQLite)
│   └── app.sqlite
├── public/                       # Static frontend assets
│   ├── index.html
│   ├── style.css
│   └── app.js
└── src/
    ├── config/
    │   └── database.vl           # DB Connection pool & standardized HTTP response helpers
    ├── models/
    │   └── schema.vl             # Struct models, DTOs & Table auto-migrations
    ├── services/
    │   ├── auth_service.vl       # Authentication, JWT tokens, bcrypt/HMAC hashing
    │   └── payment_service.vl    # VietQR, VNPay, MoMo, ZaloPay transaction workflows
    ├── routes/
    │   └── router.vl             # REST API routing, query/body parser & WebSocket handlers
    └── main.vl                   # Application entrypoint (server config, HTTP/2 & HTTP/3 listeners)
```

---

## 3. Core Language Syntax Reference

### 3.1 Variables & Mutability
```rust
// Immutable variable (default)
let port = 8080
let host = "0.0.0.0"
let is_active = true

// Mutable variable (MUST use 'let mut')
let mut total_requests = 0
let mut status = "INIT"
status = "READY"

// Compound Assignments (+=, -=, *=, /=, %=)
total_requests += 1
total_requests -= 5
total_requests *= 2
total_requests /= 2
total_requests %= 10

// Optional Type Annotations
let user_id: Int = 101
let amount: Float = 450000.0
let title: String = "Nông Sản Việt"
let is_verified: Bool = true
```

> ⚠️ **RULE**: Always use `snake_case` or lowercase names for variables and functions (e.g. `let order_list = []`, `let user_data = map_new()`). Avoid all-uppercase variable names in expressions as they can be interpreted as Struct constructors.

---

### 3.2 Control Flow & Pattern Matching

#### A. Conditional `if / else`
`if` is a statement block. To conditionally mutate a variable, declare it with `let mut` beforehand:
```rust
let mut discount = 0
if total_amount > 500000 {
    discount = 50000
} else if total_amount > 200000 {
    discount = 20000
} else {
    discount = 0
}
```

#### B. Short-Circuit Logical Operators (`&&`, `||`, `!`)
```rust
if user != none && map_get(user, "role") == "ADMIN" {
    println("Access granted to admin console")
}
```

#### C. Pattern Matching (`match`)
`match` is a first-class expression that returns a value. Each pattern arm is written as `pattern => value`:
```rust
let http_status_text = match status_code {
    200 => "OK",
    201 => "Created",
    400 => "Bad Request",
    401 => "Unauthorized",
    403 => "Forbidden",
    404 => "Not Found",
    500 => "Internal Server Error",
    _ => "Unknown Error"
}
```

#### D. Loops: `while` and `for..in`
```rust
// While loop
let mut counter = 0
while counter < 5 {
    println("Request: " + to_string(counter))
    counter += 1
}

// For..in loop over array
let fruits = ["Gạo ST25", "Xoài Cát", "Sầu Riêng Ri6"]
for item in fruits {
    println("Sản phẩm: " + item)
}

// Loop controls: break & continue
let mut i = 0
while i < 10 {
    i += 1
    if i == 3 { continue }
    if i == 8 { break }
}
```

---

### 3.3 Functions, Closures & Higher-Order Lambdas

```rust
// 1. Standard Function with typed params and default values
fn calculate_fee(amount: Int, rate: Float = 0.05) -> Int {
    let fee = to_int(amount * rate)
    return fee
}

// 2. Anonymous Functions (Lambdas) & Closures
let multiplier = fn(x: Int) -> Int {
    return x * 2
}

// 3. Higher-Order Array Operations
let numbers = [1, 2, 3, 4, 5]
let doubled = numbers.map(fn(x) { return x * 2 })
let evens = numbers.filter(fn(x) { return x % 2 == 0 })
```

---

### 3.4 Data Structures: Arrays, Maps, and JSON

#### A. Arrays
```rust
let mut list = [10, 20, 30]
let count = len(list)              // 3
let first = list[0]                 // 10
list = push(list, 40)               // [10, 20, 30, 40]
```

#### B. Maps (Key-Value Dictionaries)
```rust
let mut user = map_new()
user = map_set(user, "id", 101)
user = map_set(user, "name", "Nguyễn Văn A")
user = map_set(user, "role", "ADMIN")

let name = map_get(user, "name")     // "Nguyễn Văn A"
let has_phone = map_has(user, "phone")// false
let keys = map_keys(user)            // ["id", "name", "role"]
let values = map_values(user)        // [101, "Nguyễn Văn A", "ADMIN"]
```

#### C. JSON Serialization & Parsing
```rust
// Parse JSON string -> VietLang Map/Array
let json_str = "{\"code\":200,\"message\":\"Success\"}"
let parsed = json_parse(json_str)

// Serialize Map/Array -> JSON string
let mut payload = map_new()
payload = map_set(payload, "order_id", "DH_101")
payload = map_set(payload, "total", 350000)

let json_compact = json_stringify(payload, false)
let json_pretty = json_stringify(payload, true)
```

---

### 3.5 String Manipulation Utilities
```rust
let text = "  Nông Sản Sạch Việt Nam  "
let clean = trim(text)                       // "Nông Sản Sạch Việt Nam"
let upper = to_uppercase(clean)              // "NÔNG SẢN SẠCH VIỆT NAM"
let lower = to_lowercase(clean)              // "nông sản sạch việt nam"
let has_clean = contains(clean, "Sạch")      // true
let is_nong = starts_with(clean, "Nông")     // true
let is_nam = ends_with(clean, "Nam")         // true
let sub = substring(clean, 0, 8)             // "Nông Sản"
let length = len(clean)                      // Length of string
```

---

### 3.6 Native Concurrency & Channels (CSP Model)

VietLang integrates native multithreading with Goroutines (`spawn`) and Thread-Safe Channels (`channel_new`):

```rust
// 1. Thread-Safe Communication via Channels
let ch = channel_new(10) // Bounded channel with capacity 10 (0 for unbounded)

// 2. Spawn Background Green Thread
spawn(fn() {
    println("[Worker] Background task running...")
    channel_send(ch, "Order #99281 Processed")
})

// 3. Receive message in Main Thread (blocking)
let result = channel_recv(ch)
println("[Main] Received: " + to_string(result))

// 4. Non-blocking Receive check
let try_res = channel_try_recv(ch)
if map_get(try_res, "ok") == true {
    println("Got item: " + to_string(map_get(try_res, "value")))
}
```

#### High-Level Worker Pool & Parallel Map (`std.concurrency`)
```rust
import std.concurrency

let numbers = [10, 20, 30, 40, 50]
let squared = parallel_map(numbers, fn(n) {
    return n * n
})
// squared = [100, 400, 900, 1600, 2500] (executed across Worker Threads in parallel)
```

---

### 3.7 Vietnam Fintech Modules Reference

#### A. VietQR Generation (`std.vietqr`)
```rust
import std.vietqr

// Generate Napas 247 Instant Transfer VietQR
let qr_res = vietqr_create_payment("MB", "0901234567", 250000, "Thanh toan don hang #101")
let qr_image_url = map_get(qr_res, "qr_url")
```

#### B. VNPay Payment Gateway 2.1.0 (`std.vnpay`)
```rust
import std.vnpay

let client = vnpay_client("TMN_CODE", "HASH_SECRET", "https://sandbox.vnpayment.vn/paymentv2/vpcpay.html")
let pay_url = vnpay_create_payment_url(client, 1001, 500000, "Thanh toan", "https://site.vn/return", "127.0.0.1")
```

#### C. MoMo E-Wallet Gateway (`std.momo`)
```rust
import std.momo

let momo = momo_client("PARTNER_CODE", "ACCESS_KEY", "SECRET_KEY", "https://payment.momo.vn/v2/gateway/api/create")
let payload = momo_create_payment_payload(momo, 1002, 150000, "Don hang", "https://site.vn/return", "https://site.vn/ipn", "")
```

#### D. Zalo OA & ZNS Notifications (`std.zalo`)
```rust
import std.zalo

let normalized_phone = zalo_normalize_phone("0901234567") // "84901234567"
let zns_data = zalo_create_zns_payload(normalized_phone, "TEMPLATE_ID_101", map_new(), "TRACK_99")
```

---

## 4. Standalone AOT Binary Compilation (`vietlang build`)

Compile any VietLang source file into a **self-contained standalone executable binary** with zero external dependencies:

```bash
# 🐧 Build standalone Linux binary (ELF)
vietlang build src/main.vl -o my_service
./my_service

# 🪟 Build standalone Windows binary (.exe)
vietlang build src/main.vl -o my_service.exe --target windows

# 🍎 Build standalone macOS binary
vietlang build src/main.vl -o my_service_macos --target macos
```

---

## 5. Complete Production REST API Blueprint

Here is a full, working reference of a production REST API service in VietLang:

```rust
import std.http_router
import std.db_sqlite
import std.json
import std.jwt

// 1. Initialize Relational Database
let db = sqlite_open("data/app.sqlite")
sqlite_exec(db, "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, email TEXT, created_at INTEGER)")

// 2. Standardized HTTP Response Helper
fn json_response(status_code: Int, data) {
    let mut payload = map_new()
    payload = map_set(payload, "status", status_code)
    payload = map_set(payload, "data", data)
    payload = map_set(payload, "timestamp", time_now())
    
    let mut res = map_new()
    res = map_set(res, "status_code", status_code)
    res = map_set(res, "content_type", "application/json; charset=utf-8")
    res = map_set(res, "body", json_stringify(payload, false))
    return res
}

// 3. HTTP Server Config (HTTP/1.1, HTTP/2, HTTP/3 ALPN Ready)
let server_config = map_new()
server_config = map_set(server_config, "port", 8080)
server_config = map_set(server_config, "workers", 100)

println("Server running on http://localhost:8080")

http_listen(server_config, fn(req) {
    let method = to_string(map_get(req, "method"))
    let path = to_string(map_get(req, "path"))
    let body = to_string(map_get(req, "body"))

    // Route: GET /api/health
    if method == "GET" && path == "/api/health" {
        let mut health = map_new()
        health = map_set(health, "status", "UP")
        health = map_set(health, "uptime_s", time_now())
        return json_response(200, health)
    }

    // Route: GET /api/users
    if method == "GET" && path == "/api/users" {
        let users = sqlite_query(db, "SELECT id, name, email, created_at FROM users ORDER BY id DESC")
        return json_response(200, users)
    }

    // Route: POST /api/users
    if method == "POST" && path == "/api/users" {
        let parsed = json_parse(body)
        let name = to_string(map_get(parsed, "name"))
        let email = to_string(map_get(parsed, "email"))
        
        let query = "INSERT INTO users (name, email, created_at) VALUES ('" + name + "', '" + email + "', " + to_string(time_now()) + ")"
        sqlite_exec(db, query)
        
        let mut created = map_new()
        created = map_set(created, "message", "User created successfully")
        return json_response(201, created)
    }

    // 404 Not Found
    return json_response(404, "Endpoint not found: " + path)
})
```

---

## 6. Top 10 Golden Rules for AI Agents Writing VietLang

1. **Mutability**: Always declare variables that will be reassigned with `let mut`.
2. **Naming**: Use lowercase / `snake_case` for all variable and function names. Capitalized identifiers (e.g. `User`, `WorkerPool`) are reserved for Struct definitions.
3. **Dictionaries (Maps)**: Always manipulate dictionaries with `map_new()`, `map_set(m, k, v)`, `map_get(m, k)`, `map_has(m, k)`.
4. **Arrays**: Append to arrays using `list = push(list, item)` and measure length with `len(list)`.
5. **Pattern Matching**: Write single-pattern arms in `match`: `"GET" => 1, "POST" => 2, _ => 0`.
6. **No Inline `if` Expressions**: In VietLang, `if` is a statement block. Initialize `let mut val = default; if cond { val = new_val; }`.
7. **Concurrency**: Use `spawn(fn() { ... })` for background tasks, and communicate via `channel_new()`, `channel_send()`, and `channel_recv()`.
8. **String Utilities**: Use built-in `trim()`, `to_uppercase()`, `to_lowercase()`, `starts_with()`, `ends_with()`, `contains()`, `substring()`.
9. **Zero-Cost Modularity**: Only `import std.module_name` when you actually call functions in that module.
10. **Single Binary Deployment**: Use `vietlang build <file.vl> -o <binary_name>` to compile into a production standalone binary.
