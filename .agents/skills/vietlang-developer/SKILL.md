---
name: vietlang-developer
description: Comprehensive expert syntax guide, standard library manual, and full-stack backend programming reference for AI agents and developers building with VietLang.
---

# VietLang — Master Language Syntax, Standard Library & Backend Programming Manual

VietLang is a **backend-first, statically-analyzed, high-throughput programming language** designed specifically for production REST APIs, microservices, real-time WebSocket systems, and transactional ACID databases.

GitHub Repository: [https://github.com/hoangtuvungcao/vietlang](https://github.com/hoangtuvungcao/vietlang)

---

## 1. Multi-Platform Installation (1-Line Quick Install)

### 🐧 Linux & 🍎 macOS (Apple Silicon M1-M4 & Intel x64)
```bash
curl -fsSL https://raw.githubusercontent.com/hoangtuvungcao/vietlang/main/install.sh | bash
```
- **Auto-Configures PATH** in `~/.bashrc`, `~/.zshrc`, and `~/.profile`.
- **Pre-installs all 49 Standard Library Modules** into `~/.vietlang/std`.
- **Zero sudo required**: Binary installed directly to `~/.vietlang/bin/vietlang`.

### 🪟 Windows (PowerShell)
```powershell
iex (irm https://raw.githubusercontent.com/hoangtuvungcao/vietlang/main/install.ps1)
```
- Installs to `$HOME\.vietlang\bin\vietlang.exe` and sets user environment `PATH`.

---

## 2. Core Language Syntax Reference

### 2.1 Variables & Mutability
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

> **CRITICAL NAMING CONVENTION**: Always use `snake_case` or lowercase names for variables and functions (e.g. `let order_list = []`, `let user_data = map_new()`). Avoid all-uppercase variable names in expressions as they can be interpreted as Struct constructors.

---

### 2.2 Control Flow & Pattern Matching

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
`match` is a first-class expression that returns a value:
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

### 2.3 Functions, Closures & Higher-Order Lambdas

```rust
// 1. Standard Function with typed params and return type
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

### 2.4 Data Structures: Arrays, Maps, and JSON

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

### 2.5 String Manipulation Utilities
```rust
let text = "Nông Sản Sạch Việt Nam"
let length = text.len()                      // Length of string
let has_clean = text.contains("Sạch")        // true
let replaced = text.replace("Sạch", "Hữu Cơ")
let parts = text.split(" ")                  // ["Nông", "Sản", "Sạch", "Việt", "Nam"]
let sub = substring(text, 0, 8)              // "Nông Sản"
let str_val = to_string(12345)               // "12345"
let int_val = to_int("12345")                // 12345
```

---

### 2.6 Error Handling: `try / catch` & `throw`
```rust
try {
    let raw = file_read("config.json")
    let cfg = json_parse(raw)
} catch err {
    println("Config load error: " + to_string(err))
}

// Throw custom runtime exception
if stock_count <= 0 {
    throw("Sản phẩm đã hết hàng trong CSDL")
}
```

---

## 3. Built-In Functions Quick Reference

| Built-In Function | Signature | Description |
| :--- | :--- | :--- |
| `println(msg)` | `(Any) -> None` | Print line to stdout |
| `to_string(val)` | `(Any) -> String` | Convert value to string |
| `to_int(val)` | `(Any) -> Int` | Convert float/string to integer |
| `len(arr_or_str)` | `(Array/String) -> Int` | Return length |
| `push(arr, item)` | `(Array, Any) -> Array` | Append item and return new array |
| `map_new()` | `() -> Map` | Create new empty dictionary |
| `map_set(m, k, v)` | `(Map, String, Any) -> Map` | Set key-value pair |
| `map_get(m, k)` | `(Map, String) -> Any` | Get value by key |
| `map_has(m, k)` | `(Map, String) -> Bool` | Check key existence |
| `json_parse(str)` | `(String) -> Map/Array` | Parse JSON string |
| `json_stringify(v, pretty)` | `(Any, Bool) -> String` | Serialize to JSON |
| `time_now()` | `() -> Int` | Current UNIX timestamp (seconds) |
| `time_now_us()` | `() -> Int` | Current UNIX timestamp (microseconds) |
| `uuid()` | `() -> String` | Generate UUID v4 |
| `sha256(str)` | `(String) -> String` | SHA256 hex digest |
| `file_read(path)` | `(String) -> String` | Read file contents as string |
| `file_write(path, data)` | `(String, String) -> Bool` | Write string to file |
| `http_listen(cfg, handler)` | `(Map, Closure) -> None` | Start high-performance HTTP server |
| `http_serve_static(req, dir, fallback)` | `(Map, String, String) -> String` | High-speed static file server |

---

## 4. Standard Library Catalog (49 Pure Modules)

VietLang enforces **Zero-Cost Modularity (100% Opt-In)**:

| Category | Standard Modules | Usage Code Example |
| :--- | :--- | :--- |
| **Routing & REST** | `std.http_router`, `std.openapi`, `std.validator` | `import std.http_router` |
| **Databases (ACID)** | `std.db_sqlite`, `std.db_mysql`, `std.db_postgres` | `let db = db_sqlite_connect("app.sqlite")` |
| **Real-Time Stream** | `std.ws`, `std.sse` | `ws_init("/ws")`, `ws_emit("EVENT", data)` |
| **Caching & Redis** | `std.redis`, `std.cache_lru` | `let r = redis_connect("127.0.0.1", 6379)` |
| **Security & Auth** | `std.security`, `std.jwt`, `std.session`, `std.otp` | `let token = jwt_sign(claims, secret)` |
| **Traffic Control** | `std.rate_limiter`, `std.circuit_breaker` | `let rl = rate_limiter_new(100, 60)` |
| **Data & Files** | `std.pagination`, `std.csv`, `std.file_storage` | `let p = pagination_slice(items, 1, 10)` |
| **Notifications** | `std.email`, `std.queue` | `let mail = email_new("from", "to", "subject")` |
| **Observability** | `std.logger`, `std.metrics`, `std.telemetry` | `logger_info(log, "Order processed")` |

---

## 5. Standard Backend Architecture Blueprint

A production-grade VietLang backend project follows **Clean Architecture**:

```
my_backend_service/
├── src/
│   ├── main.vl                      # Server entrypoint & routing loop
│   ├── config/                      # Database & environment configuration
│   ├── controllers/                 # HTTP request handlers & validation
│   ├── services/                    # Business domain & ACID transactions
│   ├── repositories/                # Database SQL queries (SQLite, MySQL, Postgres)
│   └── routes/
│       └── api_router.vl            # Clean URL API router
├── tests/
│   └── full_system_test.vl          # End-to-End System Tests
├── vietlang.json                    # Package manifest & dependencies
└── README.md                        # Project documentation
```

### Complete Working Backend Server Example:
```rust
import std.http_router
import std.http2
import std.db_sqlite
import std.security
import std.jwt
import std.rate_limiter
import std.ws

// 1. Opt-in Rate Limiting & WebSocket
let mut limiter = rate_limiter_new(100, 60)
ws_init("/ws")

// 2. Connect to Relational Database
let db = db_sqlite_connect("data/app.sqlite")
db_sqlite_exec(db, "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT, email TEXT);")

// 3. Configure HTTP Server
let server_cfg = http2_server_config(port: 9090, max_connections: 500)
println("VietLang Backend listening on http://0.0.0.0:9090")

http_listen(server_cfg, fn(req) {
    let method = to_string(map_get(req, "method"))
    let path = to_string(map_get(req, "path"))

    // Static Asset Serving
    if method == "GET" && (path == "/" || path.contains(".")) {
        return http_serve_static(req, "public", "index.html")
    }

    // REST API Routes
    if method == "GET" && path == "/api/v1/health" {
        return "{\"status\":200,\"service\":\"active\"}"
    }

    if method == "GET" && path == "/api/v1/users" {
        let rows = db_sqlite_query(db, "SELECT * FROM users;")
        return api_response(200, rows, "Lay danh sach thanh cong")
    }

    return api_error(404, "Endpoint Not Found")
})
```

---

## 6. Developer Toolchain & Package Manager CLI

```bash
# 1. Initialize a new project or library
vietlang init my_service api         # templates: lib | api | microservice

# 2. Interactive documentation & API browser
vietlang doc                         # Browse all 49 standard modules
vietlang doc std.pagination          # View signatures, parameter docs, and types
vietlang doc my_custom_package       # View docs for community package
vietlang doc --all                   # Generate Markdown docs into docs/api/

# 3. Package installation & updates
vietlang search redis
vietlang install redis@1.2.0
vietlang update redis
vietlang remove redis

# 4. Testing & Verification
vietlang verify
vietlang test tests/full_system_test.vl

# 5. Publish to Central Community Registry
vietlang publish
```

---

## 7. Learning Documentation Index

- 📘 **[Getting Started 10-Minute Tutorial](file:///home/vantrong/Downloads/new_lang/docs/getting-started.md)**
- 📕 **[Complete Language Syntax Reference](file:///home/vantrong/Downloads/new_lang/docs/language-reference.md)**
- 📗 **[Standard Library Ecosystem (49 Modules)](file:///home/vantrong/Downloads/new_lang/docs/standard-library-ecosystem.md)**
- 📙 **[Backend Cookbook & Real-World Recipes](file:///home/vantrong/Downloads/new_lang/docs/backend-cookbook.md)**
- 🚀 **[Multi-Platform Installation & VS Code Guide](file:///home/vantrong/Downloads/new_lang/docs/installation-and-vscode-marketplace.md)**
