---
name: vietlang-developer
description: Comprehensive expert syntax guide, project architecture manual, standard library catalog, and backend programming reference for AI agents and developers building with VietLang.
---

# VietLang — Master Language Syntax, Project Architecture & Full-Stack Backend Manual

VietLang 0.3.0-alpha.1 is an **experimental, backend-oriented language/runtime prerelease**.
Its frontend resolves a deterministic module graph, rejects cycles, runs
semantic analysis, and lowers checked modules to typed IR. The analyzer covers
annotations, arity and returns, struct fields, methods, mutability, built-in
generic `Option<T>` / `Result<T,E>`, and Bool/ADT match exhaustiveness. Dynamic
native values can still be `Unknown`, so do not claim sound whole-program type
safety, zero bugs, or independent security certification.

Security boundary for generated code: do not use the disabled legacy
`std.jwt`, `std.momo`, or `std.vnpay` modules in new applications. Use core
cryptographic/HTTP primitives only where their documented contract fits, and
select reviewed, versioned community packages for provider protocols. The
package installer requires exact semver resolution, SHA-256 content verification,
Ed25519-signed registry metadata, an immutable Git revision, and `vietlang.lock`.
Unsigned legacy registry entries are intentionally rejected.

MySQL and PostgreSQL use bounded SQLx/Tokio/Rustls pools with parameter binding,
health checks, acquisition timeouts, and explicit close. SQLite migrations use
an immediate transaction and immutable migration table; server databases expose
advisory migration locking. Keep multi-statement business transactions inside a
reviewed repository/service adapter until callback transaction handles stabilize.

For semantic edge cases, consult `docs/language-specification.md`. It is a
descriptive 0.3.0-alpha.1 draft; do not infer sound static type safety from annotations or
claim whole-program type safety or interpreter/VM equivalence beyond covered
conformance tests.

GitHub Repository: [https://github.com/hoangtuvungcao/vietlang](https://github.com/hoangtuvungcao/vietlang)

---

## 1. Quick Installation & Multi-Platform Setup

### 🐧 Linux & 🍎 macOS (Apple Silicon M1-M4 & Intel x64) & 📱 Android (Termux)
```bash
curl -fsSL https://raw.githubusercontent.com/hoangtuvungcao/vietlang/main/install.sh | bash
```
- **Auto-Configures PATH** in `~/.bashrc`, `~/.zshrc`, and `~/.profile`.
- **Pre-installs the 60+ Standard Library Modules** into `~/.vietlang/std`.
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

## 2. Master Project Architecture & 1-Command Backend Scaffolding

To generate an experimental **backend REST API prototype** with a layered layout,
SQLite adapter, router helpers, and automation scripts, run:

```bash
# 🚀 1-Command Backend Project Scaffolding (aliases: 'new', 'init', 'create')
vietlang new my_backend_service

cd my_backend_service
vietlang dev                 # Start development server on http://localhost:8080
vietlang run build           # Bundle source with the interpreter runtime
```

### 2.1 Layered Backend Project Directory Layout

```text
my_backend_service/
├── vietlang.json                 # Service manifest (scripts, main, dependencies, metadata)
├── config.json (or .env)         # Runtime dynamic configuration (port, workers, db_path)
├── README.md                     # API Documentation & Endpoints
├── data/                         # Persistent database files (SQLite)
│   └── app.sqlite
└── src/
    ├── config/
    │   └── database.vl           # DB Connection pool & standardized JSON response helpers
    ├── models/
    │   └── schema.vl             # Struct models, DTOs & Table auto-migrations
    ├── services/
    │   ├── user_service.vl       # Business logic & Database queries
    │   ├── auth_service.vl       # Application authentication boundary
    │   └── integration_service.vl # Application-owned, reviewed external integrations
    ├── routes/
    │   └── router.vl             # REST API routing & Middleware dispatch
    └── main.vl                   # Bounded HTTP/1.1 + HTTP/2 entrypoint
```

### 2.1 Project Manifest & npm-style Scripts (`vietlang.json`)

Just like `package.json` in Node.js, `vietlang.json` defines your project's entrypoint, dependencies, and executable automation scripts:

```json
{
  "name": "my_enterprise_app",
  "version": "1.0.0",
  "main": "src/main.vl",
  "scripts": {
    "start": "vietlang run src/main.vl",
    "dev": "PORT=8080 vietlang run src/main.vl",
    "build": "vietlang build src/main.vl -o app",
    "build:win": "vietlang build src/main.vl -o app.exe --target windows",
    "test": "vietlang check src/main.vl"
  },
  "dependencies": {
    "sqlite": "1.0.0",
    "http_router": "1.0.0",
    "validator": "1.0.0"
  },
  "license": "MIT"
}
```

#### Running Scripts:
```bash
# Run the 'start' script (or 'main' entrypoint)
vietlang start

# Run the 'dev' script
vietlang dev

# Run any custom defined script (e.g. build, test, lint)
vietlang run build
vietlang run build:win
vietlang run test

# Official compiler/tooling commands
vietlang check src/main.vl
vietlang fmt .
vietlang lsp
vietlang debug src/main.vl
vietlang docs --generate docs/api
vietlang fuzz --iterations 10000
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

// 2. Spawn Background OS Thread
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

Closures capture lexical bindings, remain usable after the declaring scope
returns, and share captured mutable cells with sibling closures. Use explicit
channels or mutex-backed services for multi-step state transitions across
`spawn`; synchronization of an individual binding does not make `x += 1` an
atomic cross-thread transaction.

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

Provider-specific payment integrations such as MoMo and VNPay are application
or community-package responsibilities. Agents must not generate imports of the
disabled legacy `std.momo` or `std.vnpay` modules for new code.

#### B. Zalo OA & ZNS Notifications (`std.zalo`)
```rust
import std.zalo

let normalized_phone = zalo_normalize_phone("0901234567") // "84901234567"
let zns_data = zalo_create_zns_payload(normalized_phone, "TEMPLATE_ID_101", map_new(), "TRACK_99")
```

---

## 4. Standalone Source Bundling (`vietlang build`)

Bundle VietLang source with its interpreter runtime into a **self-contained
standalone executable**. The embedded source is parsed at startup; this is not
native AOT compilation of VietLang code.

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

## 5. Experimental REST API Blueprint

Here is a reference for a local experimental REST API service in VietLang:

```rust
import std.http_router
import std.db_sqlite
import std.json

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

// 3. Bounded HTTP/1.1 + HTTP/2 Server Config
let mut server_config = map_new()
server_config = map_set(server_config, "port", 8080)
server_config = map_set(server_config, "workers", 4)
server_config = map_set(server_config, "max_concurrency", 256)
server_config = map_set(server_config, "max_body_bytes", 1048576)
server_config = map_set(server_config, "max_header_bytes", 65536)
server_config = map_set(server_config, "max_response_bytes", 8388608)
server_config = map_set(server_config, "request_timeout_ms", 30000)

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
        
        sqlite_execute(
            db,
            "INSERT INTO users (name, email, created_at) VALUES (?, ?, ?)",
            [name, email, time_now()]
        )
        
        let mut created = map_new()
        created = map_set(created, "message", "User created successfully")
        return json_response(201, created)
    }

    // 404 Not Found
    return json_response(404, "Endpoint not found: " + path)
})
```

### HTTP transport contract

- `http_listen(config, handler)` serves HTTP/1.1 and HTTP/2 using the native
  Hyper/Axum runtime. Set both `tls_cert_file` and `tls_key_file` for HTTPS and
  HTTP/2 ALPN; setting only one is an error.
- Set `http3` to `true` (and optionally `http3_port`) only with TLS configured.
  This starts an HTTP/3 QUIC listener and adds `Alt-Svc`. The h3 transport is
  experimental and must not be represented as production-certified.
- Every request has `method`, `path`, `query`, `protocol`, `client_ip`,
  `request_id`, `headers`, UTF-8-lossy `body`, and lossless `body_base64`.
- Default limits are 256 concurrent handlers, 1 MiB request body, 64 KiB
  headers, 8 MiB response, 30-second request timeout, and 10-second shutdown.
- `cors_allow_origin` is absent by default. Never add wildcard CORS unless the
  application explicitly requires a public cross-origin API.
- `http_fetch` accepts HTTP and HTTPS, negotiates HTTP/1.1 or HTTP/2, does not
  follow redirects, and supports an options map with `timeout_ms` and
  `max_response_bytes`. It does not currently implement an HTTP/3 client.

---

## 5.1 Database Ecosystem & Storage Adapters

VietLang provides native connection pooling and asynchronous drivers for 8 database ecosystems:

### Relational & Embedded Databases
- **`std.db_sqlite`**: File-backed ACID relational database in WAL mode with immediate transactions.
  ```rust
  import std.db_sqlite
  let db = sqlite_open("data/app.sqlite")
  sqlite_execute(db, "INSERT INTO users (name) VALUES (?)", ["Lan"])
  let rows = sqlite_query(db, "SELECT * FROM users")
  sqlite_close(db)
  ```
- **`std.db_postgres`**: Async SQLx PostgreSQL pool with parameter binding and advisory migration locks.
  ```rust
  import std.db_postgres
  let pool = postgres_connect("postgres://user:pass@localhost/db")
  let rows = postgres_query(pool, "SELECT * FROM users WHERE active = $1", [true])
  postgres_close(pool)
  ```
- **`std.db_mysql`**: Async SQLx MySQL / MariaDB connection pool with bounded workers.
  ```rust
  import std.db_mysql
  let pool = mysql_connect("mysql://user:pass@localhost/db")
  let count = mysql_execute(pool, "UPDATE users SET status = ? WHERE id = ?", ["active", 1])
  mysql_close(pool)
  ```

### NoSQL, Cache, Search & Analytics Adapters
- **`std.db_mongodb`**: Native MongoDB async client with BSON conversion, CRUD, and Aggregation pipeline.
  ```rust
  import std.db_mongodb
  let conn = mongo_connect("mongodb://localhost:27017", "my_db")
  let id = mongo_insert(conn, "users", doc)
  let user = mongo_find_one(conn, "users", filter)
  mongo_close(conn)
  ```
- **`std.db_redis`**: Native async Redis connection pool with string, list, hash, and Pub/Sub primitives.
  ```rust
  import std.db_redis
  let r = redis_connect("redis://localhost:6379")
  redis_set(r, "session:123", "user_data", 3600)
  let val = redis_get(r, "session:123")
  redis_close(r)
  ```
- **`std.db_clickhouse`**: Native ClickHouse HTTP analytics engine for high-throughput OLAP and timeseries.
  ```rust
  import std.db_clickhouse
  let ch = clickhouse_connect("localhost", 8123, "default", "default", "")
  let stats = clickhouse_query(ch, "SELECT count(), avg(amount) FROM transactions", [])
  clickhouse_close(ch)
  ```
- **`std.db_cassandra`**: Native ScyllaDB/Cassandra CQL cluster session for distributed wide-column storage.
  ```rust
  import std.db_cassandra
  let session = cassandra_connect(["127.0.0.1:9042"], "my_keyspace")
  let rows = cassandra_query(session, "SELECT * FROM timeline WHERE user_id = ?", ["u100"])
  cassandra_close(session)
  ```
- **`std.db_elasticsearch`**: Native Elasticsearch / OpenSearch REST client for full-text search DSL and indexing.
  ```rust
  import std.db_elasticsearch
  let es = elastic_connect("http://localhost:9200", "")
  let results = elastic_search(es, "products", "smart phone", 10, 0)
  elastic_close(es)
  ```

### Fail-Explicit Contract
Unimplemented driver paths, disabled legacy administrative operations, or unbacked dynamic conversions **fail explicitly with a descriptive `VietError`** rather than returning dummy data or masking errors.

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
10. **Standalone Bundling**: Use `vietlang build <file.vl> -o <binary_name>` to bundle source with the interpreter for experimental deployment.
