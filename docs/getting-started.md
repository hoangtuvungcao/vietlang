# Getting Started with VietLang

Welcome to **VietLang** — The Backend-First Programming Language designed for high-performance microservices, REST APIs, real-time WebSockets, and ACID transactional databases.

---

## 1. Fast 1-Line Installation (Auto-Set PATH)

### 🐧 Linux & 🍎 macOS (Apple Silicon M1-M4 & Intel x64)

```bash
curl -fsSL https://raw.githubusercontent.com/hoangtuvungcao/vietlang/main/install.sh | bash
```

- **Auto-Configures PATH** in `~/.bashrc`, `~/.zshrc`, and `~/.profile`.
- **Pre-syncs all 49 Standard Library Modules** into `~/.vietlang/std`.
- **Zero sudo required**: Binary installed directly to `~/.vietlang/bin/vietlang`.

### 🪟 Windows (PowerShell)

```powershell
iex (irm https://raw.githubusercontent.com/hoangtuvungcao/vietlang/main/install.ps1)
```

- Installs to `$HOME\.vietlang\bin\vietlang.exe` and sets user environment `PATH`.

---

## 2. Verify Installation

```bash
vietlang --version
# VietLang v0.1.0
```

Start the interactive REPL:
```bash
vietlang
vl> let greeting = "Xin chào Việt Nam!"
vl> println(greeting)
Xin chào Việt Nam!
vl> :quit
```

---

## 3. Your First VietLang Program (5 Minutes)

Create a file `hello.vl`:
```rust
let name = "VietLang Backend"
let port = 8080

println("Khởi động dịch vụ: " + name + " trên cổng " + to_string(port))

let mut counter = 0
while counter < 3 {
    println("Request id: " + to_string(counter + 1))
    counter += 1
}
```

Run it with:
```bash
vietlang hello.vl
```

---

## 4. Building a Real REST API Microservice (10 Minutes)

Create `server.vl`:
```rust
import std.http_router
import std.http2
import std.db_sqlite

// 1. Connect to SQLite database
let db = db_sqlite_connect("app.sqlite")
db_sqlite_exec(db, "CREATE TABLE IF NOT EXISTS products (id INTEGER PRIMARY KEY, name TEXT, price REAL);")
db_sqlite_exec(db, "INSERT OR IGNORE INTO products VALUES (1, 'Gạo ST25 Sóc Trăng', 195000);")

// 2. Start HTTP Server
let server_cfg = http2_server_config(port: 9090)
println("REST API Server running on http://0.0.0.0:9090")

http_listen(server_cfg, fn(req) {
    let method = to_string(map_get(req, "method"))
    let path = to_string(map_get(req, "path"))

    if method == "GET" && path == "/api/v1/health" {
        return "{\"status\":200,\"message\":\"Service Healthy\"}"
    }

    if method == "GET" && path == "/api/v1/products" {
        let items = db_sqlite_query(db, "SELECT * FROM products;")
        return api_response(200, items, "Lấy danh sách sản phẩm thành công")
    }

    return api_error(404, "Endpoint Not Found")
})
```

Run your server:
```bash
vietlang server.vl
```

Test with curl:
```bash
curl http://localhost:9090/api/v1/health
curl http://localhost:9090/api/v1/products
```

---

## 5. Next Steps & Learning Roadmap

- 📖 **[Complete Language Reference](language-reference.md)**: Full syntax, types, and error handling.
- 📦 **[Standard Library Ecosystem (49 Modules)](standard-library-ecosystem.md)**: Explore all built-in modules.
- 🍳 **[Backend Cookbook](backend-cookbook.md)**: JWT auth, WebSocket events, and database transactions.
- 💻 **[VS Code Setup](installation-and-vscode-marketplace.md)**: Install the official VS Code extension with snippets and syntax highlighting.
