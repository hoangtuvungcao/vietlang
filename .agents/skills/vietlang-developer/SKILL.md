---
name: vietlang-developer
description: Comprehensive expert syntax guide and programming reference for AI agents and developers to build, test, and maintain complete enterprise backend projects using VietLang.
---

# VietLang Language Syntax & Complete Programming Manual

VietLang is a **backend-first, statically-analyzed, high-throughput programming language** designed specifically for REST APIs, microservices, real-time WebSocket servers, and transactional ACID databases.

GitHub Repository: [https://github.com/hoangtuvungcao/vietlang](https://github.com/hoangtuvungcao/vietlang)

---

## 1. Core Language Syntax & Variable Rules

### 1.1 Variable Declarations
```rust
// Immutable by default
let port = 8080
let host = "0.0.0.0"
let is_production = false

// Mutable variables MUST use 'let mut'
let mut total_requests = 0
let mut status_message = "Pending"
status_message = "Ready"

// Compound Assignments (+=, -=, *=, /=, %=)
total_requests += 1
total_requests -= 5
total_requests *= 2
total_requests /= 2
total_requests %= 10

// Type Annotations (Optional)
let user_id: Int = 101
let price: Float = 45000.0
let title: String = "Gạo ST25 Sóc Trăng"
let is_verified: Bool = true
```

> **CRITICAL NAMING RULE**: Always use `snake_case` or lowercase for variable names (e.g. `let order_list = []`, `let user_data = map_new()`). Avoid all-uppercase variable names in expressions as they can be interpreted as Struct constructors.

---

## 2. Control Flow & Pattern Matching

### 2.1 Conditional `if / else`
`if` is a statement block. When conditionally mutating a variable, declare it with `let mut` before the block:
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

### 2.2 Short-Circuit Logical Operators
`&&` and `||` safely short-circuit:
```rust
if user != none && map_get(user, "is_active") == true {
    println("User is authorized")
}
```

### 2.3 Pattern Matching (`match`)
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
    _ => "Unknown Status"
}
```

### 2.4 Loops: `while` and `for..in`
```rust
// 1. While Loop
let mut i = 0
while i < 5 {
    println("Counter: " + to_string(i))
    i += 1
}

// 2. For..in Loop
let fruits = ["Cam Sành", "Xoài Cát", "Sầu Riêng"]
for item in fruits {
    println("Nông sản: " + item)
}

// Loop Control: break and continue
let mut count = 0
while count < 10 {
    count += 1
    if count == 3 {
        continue
    }
    if count == 8 {
        break
    }
}
```

---

## 3. Functions, Closures & Higher-Order Lambdas

### 3.1 Function Declarations
```rust
// Standard function with typed parameters and return type
fn calculate_vat(amount: Int, rate: Float = 0.08) -> Int {
    let vat = to_int(amount * rate)
    return vat
}

// Function with default arguments
fn format_log(message: String, level: String = "INFO") -> String {
    return "[" + level + "] " + message
}
```

### 3.2 Closures & Lambdas
```rust
// Anonymous Function assigned to variable
let double_fn = fn(x: Int) -> Int {
    return x * 2
}

// Higher-Order Function usage
let numbers = [1, 2, 3, 4, 5]
let doubled = numbers.map(fn(x) { return x * 2 })
let evens = numbers.filter(fn(x) { return x % 2 == 0 })
```

---

## 4. Data Structures: Arrays, Maps, and JSON

### 4.1 Arrays
```rust
let mut list = [10, 20, 30]
let length = len(list)            // 3
let first = list[0]               // 10
list = push(list, 40)             // [10, 20, 30, 40]
```

### 4.2 Maps (Key-Value Dictionaries)
Maps are the primary flexible data structure in VietLang:
```rust
let mut user = map_new()
user = map_set(user, "id", 101)
user = map_set(user, "name", "Nguyễn Văn A")
user = map_set(user, "role", "CUSTOMER")

let user_name = map_get(user, "name")       // "Nguyễn Văn A"
let has_phone = map_has(user, "phone")      // false
```

### 4.3 JSON Parsing & Serialization
```rust
// Parse JSON string into VietLang Map/Array
let json_str = "{\"status\":200,\"message\":\"Thành công\"}"
let parsed_map = json_parse(json_str)
let msg = map_get(parsed_map, "message")

// Serialize Map/Array into JSON string
let mut order = map_new()
order = map_set(order, "order_id", "DH_101")
order = map_set(order, "total", 350000)

let json_compact = json_stringify(order, false)
let json_pretty = json_stringify(order, true)
```

---

## 5. String Manipulation Utilities
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

## 6. Error Handling: `try / catch` and `throw`
```rust
try {
    let raw = file_read("config.json")
    let cfg = json_parse(raw)
} catch err {
    println("Lỗi nạp cấu hình: " + to_string(err))
}

// Throw custom exception
if inventory <= 0 {
    throw("Sản phẩm đã hết hàng tồn kho trong CSDL")
}
```

---

## 7. Standard Backend Architecture Blueprint

A production-grade VietLang backend project follows **Clean Architecture**:

```
my_backend_service/
├── src/
│   ├── main.vl                      # Entrypoint & Server Startup
│   ├── config/                      # App Environment & DB Config
│   ├── controllers/                 # HTTP Request Handlers
│   ├── services/                    # Business Domain & Transactions
│   ├── repositories/                # Database Queries (SQLite, MySQL, Postgres)
│   └── routes/
│       └── api_router.vl            # API Routing & Middleware Pipeline
├── tests/
│   └── full_system_test.vl          # End-to-End System Tests
├── vietlang.json                    # Package Manifest & Dependencies
└── README.md                        # Documentation
```

### 7.1 Example: Enterprise HTTP Server with Routing
```rust
import std.http_router
import std.http2
import std.db_sqlite
import std.security
import std.jwt

// 1. Connect to ACID Database
let db = db_sqlite_connect("data/app.sqlite")

// 2. Configure HTTP Server
let server_cfg = http2_server_config(port: 9090, max_connections: 500)

println("VietLang Backend listening on http://0.0.0.0:9090")

http_listen(server_cfg, fn(req) {
    let method = to_string(map_get(req, "method"))
    let path = to_string(map_get(req, "path"))
    let body_str = to_string(map_get(req, "body"))

    // Static Asset Serving (Frontend SPA)
    if method == "GET" && (path == "/" || path.contains(".")) {
        return http_serve_static(req, "public", "index.html")
    }

    // Health Check Endpoint
    if method == "GET" && path == "/api/v1/health" {
        return "{\"status\":200,\"service\":\"active\"}"
    }

    // Default 404
    return "{\"status_code\":404,\"error\":\"Endpoint Not Found\"}"
})
```

---

## 8. Package Management & Documentation CLI

```bash
# 1. Initialize a new project or library
vietlang init my_project api         # templates: lib | api | microservice

# 2. Interactive documentation explorer
vietlang doc                         # List all 49 standard library modules
vietlang doc std.pagination          # View signatures and parameter docs for a module
vietlang doc my_custom_module        # View docs for custom/community package
vietlang doc --all                   # Generate Markdown docs into docs/api/

# 3. Package installation from Central Community Registry
vietlang install redis@1.2.0
vietlang install auth@3.0.0

# 4. Run tests & verification
vietlang test tests/full_system_test.vl
vietlang verify

# 5. Publish package to Central Community Registry
vietlang publish
```
