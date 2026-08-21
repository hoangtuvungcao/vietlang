# VietLang Standard Library — Zero-Cost Modular Architecture Reference

VietLang follows an explicit-import modularity model. The native server uses
Tokio, Hyper, Axum, and Rustls for bounded HTTP/1.1, HTTP/2, HTTP, and HTTPS.
HTTP/3 over QUIC is available as an opt-in experimental transport. Background
services, databases, caching, and real-time modules remain opt-in:

---

## 1. Modular Catalog by Backend Domain

| Domain / Use Case | Standard Modules to Import | Primary Functions |
| :--- | :--- | :--- |
| **REST & API Gateway** | `std.http_router`, `std.openapi`, `std.validator` | `http_serve_static`, `openapi_spec_new`, `validator_validate` |
| **Relational Database & ACID** | `std.db_sqlite` (enabled); `std.db_mysql` (disabled); `std.db_postgres` (experimental) | `sqlite_open`, `sqlite_execute`, `sqlite_query` |
| **Real-Time WebSocket** | `std.ws` | `ws_init("/ws")`, `ws_emit("event", data)`, `ws_send_text()` |
| **Distributed Caching & Redis** | `std.redis`, `std.cache_lru` | `redis_connect()`, `redis_get()`, `redis_set()` |
| **Security primitives** | `std.security`, `std.session` | `security_hash_password()`, `session_create()` |
| **API Traffic Management** | `std.rate_limiter`, `std.circuit_breaker` | `rate_limiter_new(100, 60)`, `rate_limiter_check()` |
| **Async Tasks & Queues** | `std.queue`, `std.cron` | `queue_new()`, `queue_push()`, `queue_pop()` |
| **Observability & Logging** | `std.logger`, `std.metrics`, `std.telemetry` | `logger_new()`, `logger_info()`, `metrics_inc_counter()` |
| **Data Export & Reporting** | `std.csv`, `std.stream` | `csv_decode()`, `csv_encode()` |

---

## 2. Example: Building a Lightweight Microservice (Zero Extra Services)

```vietlang
import std.http_router
import std.http2

let server_config = http2_server_config(8080, 200)

http_listen(server_config, fn(req) {
    let path = to_string(map_get(req, "path"))
    if path == "/health" {
        return "{\"status\":\"UP\"}"
    }
    return "{\"service\":\"inventory_service\",\"version\":\"1.0.0\"}"
})
```

---

## 3. Example: Building a Real-Time Fintech & Banking Ledger

```vietlang
import std.db_sqlite
import std.security
import std.rate_limiter
import std.ws

// 1. Opt-in Rate Limiter: 10 requests / 60 seconds for money transfer
let transfer_limiter = rate_limiter_new(10, 60)

// 2. Opt-in Real-Time Push on /ws/transactions
ws_init("/ws/transactions")

// 3. Connect to ACID SQLite / MySQL Database
let db = db_sqlite_connect("ledger.sqlite")
```
