---
name: vietlang-developer
description: Comprehensive expert guide for AI agents and developers to build, test, and maintain enterprise backend microservices, realtime systems, and community libraries using VietLang.
---

# VietLang Developer & AI Agent Reference Manual

VietLang is a **backend-first, high-concurrency, statically-analyzed programming language** designed for high-throughput microservices, real-time WebSocket systems, transactional databases, and decentralized package ecosystems.

GitHub Repository: [https://github.com/hoangtuvungcao/vietlang](https://github.com/hoangtuvungcao/vietlang)

---

## 1. Language Rules & Critical Agent Guidelines

### 1.1 Variable Declarations & Mutation
- **Immutable by default**: `let port = 8080`
- **Mutable variables**: Must use `let mut variable_name = initial_value`
- **Compound assignments**: `counter += 1`, `x -= 2`, `y *= 3`, `z /= 4`, `mod %= 2`
- **Naming Rule**: Always use `snake_case` or lowercase for variable names (e.g., `let user_list = []`). Avoid all-uppercase variable names in expressions as they can be parsed as Struct constructors.

### 1.2 Statements vs Expressions
- **If Statements**: `if` is parsed as a statement. To conditionally set a variable, write:
  ```rust
  let mut role = "guest"
  if is_admin {
      role = "admin"
  }
  ```
- **Match Expressions**: `match` is a first-class expression:
  ```rust
  let status = match code {
      200 => "OK",
      404 => "Not Found",
      _ => "Error"
  }
  ```

### 1.3 Short-Circuit Logic & Signals
- `&&` and `||` short-circuit safely (the right-hand side is not evaluated if the left determines the outcome).
- `return`, `break`, and `continue` are first-class control flow signals that preserve complete data structures (Maps, Structs, Arrays, Closures).

### 1.4 Error Handling
```rust
try {
    let content = file_read("config.json")
    let data = json_parse(content)
} catch err {
    log_error("Config load error: " + err)
}

// Throw custom runtime error
if amount <= 0 {
    throw("Invalid transaction amount")
}
```

---

## 2. Complete Standard Library Reference (`std.*`)

VietLang provides 16 pure VietLang standard modules in `std/`:

### 2.1 Security & Cryptography (`std.security`)
```rust
import std.security

// Salted password hashing
let hash = security_hash_password("UserPassword123")
let is_valid = security_verify_password("UserPassword123", hash)

// Constant-time string comparison (prevents timing attacks)
let match = security_constant_time_compare(token_a, token_b)

// CSRF token & XSS sanitization
let csrf = security_generate_csrf_token()
let safe_html = security_sanitize_html(untrusted_user_input)
```

### 2.2 JWT Authentication & RBAC (`std.jwt`)
```rust
import std.jwt

let claims = map_set(map_set(map_new(), "uid", 101), "roles", ["admin", "billing"])
let token = jwt_sign(claims, "secret_key")

let verify_res = jwt_verify(token, "secret_key")
if map_get(verify_res, "valid") {
    let payload = map_get(verify_res, "payload")
}
```

### 2.3 Realtime WebSockets (`std.websocket`)
```rust
import std.websocket

// RFC 6455 Handshake calculation
let accept_key = ws_accept_key(sec_websocket_key)
let response_header = ws_handshake_response(sec_websocket_key)

// Frame encoding
let text_frame = ws_encode_text_frame("Hello, Client!")
let ping_frame = ws_encode_ping_frame()

// Multi-room broadcasting hub
let mut hub = ws_room_manager_new()
hub = ws_room_join(hub, "general_room", "client_id_99")
let packet = ws_room_broadcast(hub, "general_room", "CHAT_MESSAGE", message_data)
```

### 2.4 Low-Level Sockets & Networking (`std.socket`)
```rust
import std.socket

// TCP client stream
let resp = socket_tcp_send("127.0.0.1", 6379, "*1\r\n$4\r\nPING\r\n")

// UDP datagram dispatch
let ok = socket_udp_send("127.0.0.1", 9999, "METRICS_PACKET")

// Port health check
let is_alive = socket_ping("127.0.0.1", 5432)
```

### 2.5 HTTP Routing & Web APIs (`std.http_router`)
```rust
import std.http_router

fn handle_api_request(req) {
    let body = map_get(req, "body")
    let headers = map_get(req, "headers")
    return response_json(200, map_set(map_new(), "status", "success"))
}
```

### 2.6 Request Payload Validation (`std.validator`)
```rust
import std.validator

let v = validator_new()
let v = validator_add_rule(v, "email", "required")
let v = validator_add_rule(v, "email", "email")
let v = validator_add_rule(v, "age", "min_val:18")

let result = validate(v, request_body)
if !map_get(result, "is_valid") {
    let errors = map_get(result, "errors")
}
```

### 2.7 Database ORM & Query Builder (`std.orm`)
```rust
import std.orm

let q = query_builder_new("users")
let q = qb_select(q, ["id", "username", "email", "created_at"])
let q = qb_where(q, "status", "=", "ACTIVE")
let q = qb_order_by(q, "created_at", "DESC")
let q = qb_limit(q, 10)

let sql = qb_to_sql(q)
let rows = db_query(sql)

let insert_sql = qb_insert_sql("users", user_data)
```

### 2.8 Database Schema Migrations (`std.migration`)
```rust
import std.migration

let mut engine = migration_engine_new()
engine = migration_add_step(engine, "001_init", "CREATE TABLE users (id INT);", "DROP TABLE users;")
let migrated = migration_run_all(engine)
```

### 2.9 Rate Limiting Shield (`std.rate_limiter`)
```rust
import std.rate_limiter

let limiter = rate_limiter_new(100, 10) // 100 burst tokens, 10 tokens/sec refill
let check = rate_limit_allow(limiter, client_ip)
if !map_get(check, "allowed") {
    return response_error(429, "Too Many Requests")
}
let limiter = map_get(check, "limiter")
```

### 2.10 Circuit Breaker (`std.circuit_breaker`)
```rust
import std.circuit_breaker

let mut cb = circuit_breaker_new(5, 30) // 5 failures opens circuit for 30s
if !cb_can_execute(cb) {
    return response_error(503, "Downstream service temporarily unavailable")
}
```

### 2.11 OpenTelemetry Distributed Tracing (`std.telemetry`)
```rust
import std.telemetry

let root_ctx = trace_context_new("order_microservice")
let child_span = trace_create_child_span(root_ctx, "db_query")
let finished = trace_end_span(child_span)
let headers = trace_inject_headers(finished) // Injects X-Trace-Id, X-Span-Id
```

### 2.12 Background Task Queue with DLQ (`std.queue`)
```rust
import std.queue

let mut q = queue_new()
q = queue_push(q, "SEND_EMAIL", email_payload, 3)

let pop_res = queue_pop(q)
if map_get(pop_res, "has_task") {
    let task = map_get(pop_res, "task")
    q = map_get(pop_res, "queue")
}
```

### 2.13 Kubernetes Health Probes (`std.health`)
```rust
import std.health

let mut hc = health_checker_new("billing_service")
hc = health_add_check(hc, "postgres", true)
hc = health_add_check(hc, "redis", true)
let report = health_get_report(hc) // Status: "UP" or "DOWN", uptime_seconds
```

### 2.14 Event-Driven Pub/Sub Bus (`std.event_bus`)
```rust
import std.event_bus

let mut bus = event_bus_new()
bus = event_subscribe(bus, "order.placed", "SendOrderConfirmation")
let res = event_publish(bus, "order.placed", order_data)
```

### 2.15 Configuration & .env Loader (`std.config`)
```rust
import std.config

let cfg = config_load(".env")
let port = config_get_int(cfg, "PORT", 8080)
let debug = config_get_bool(cfg, "DEBUG", false)
```

### 2.16 Testing Framework (`std.test`)
```rust
import std.test

suite("User Service Tests")

test("Validation sanity", fn() {
    assert_eq(1 + 1, 2, "Math error")
    assert_true(true, "Condition failed")
    assert_false(false, "Condition failed")
})

test_summary()
```

### 2.17 In-Memory Key-Value & Redis Engine (`std.kv_store`)
```rust
import std.kv_store

let mut store = kv_store_new()

// Atomic counter increment
let res = kv_incr(store, "rate_limit:ip:101", 1)
store = map_get(res, "store")
let current_hits = map_get(res, "value")

// Hash operations (HSET / HGET)
store = kv_hset(store, "session:usr_99", "token", "jwt_abc_123")
let token = kv_hget(store, "session:usr_99", "token")
```

### 2.18 Distributed Event Stream Engine (`std.stream`)
```rust
import std.stream

let mut stream = stream_engine_new()

// Publish event to partition log
let pub_res = stream_publish(stream, "telemetry_events", event_data)
stream = map_get(pub_res, "engine")
let offset = map_get(pub_res, "offset")

// Consumer group batch polling
let batch = stream_consume_from(stream, "telemetry_events", 0, 50)
```

### 2.19 Enterprise Cryptography & API Shield (`std.crypto_advanced`)
```rust
import std.crypto_advanced

// Webhook signature verification (Stripe, MoMo, ZaloPay)
let is_valid = crypto_verify_webhook(raw_payload, signature_header, webhook_secret)

// Symmetric secret payload encryption & decryption
let enc_b64 = crypto_encrypt_payload(credit_card_data, encryption_key)
let decrypted = crypto_decrypt_payload(enc_b64, encryption_key)

// Secure API Key generation
let api_key = crypto_generate_api_key("live") // -> "ak_live_a1b2c3..."
```

### 2.20 Enterprise HTTP Pipeline & Middleware Chain (`std.http_pipeline`)
```rust
import std.http_pipeline

let pipeline = http_pipeline_new()

// Check CIDR IP whitelist
let is_allowed = http_check_ip_allowed(pipeline, client_ip)

// Build response with automatic Security Headers (CSP, HSTS, X-Frame-Options, nosniff)
let response = http_build_response(200, payload_data)
```

### 2.21 Distributed SAGA Coordinator (`std.saga`)
```rust
import std.saga

let mut order_saga = saga_new("OrderFulfillment")
order_saga = saga_add_step(order_saga, "ReserveStock", "StockReserved", "ReleaseStock")
order_saga = saga_add_step(order_saga, "ChargePayment", "CardCharged", "RefundCard")

// Trigger compensation rollback on failure
let rollback = saga_compensate_all(order_saga)
```

### 2.22 Resilient Exponential Backoff Retry (`std.retry`)
```rust
import std.retry

let policy = retry_policy_new(5, 100, 2.0)
let delay_ms = retry_calculate_delay_ms(policy, attempt_number)
sleep_ms(delay_ms)
```

### 2.23 Enterprise Cron Scheduler (`std.cron`)
```rust
import std.cron

let mut scheduler = cron_scheduler_new()
scheduler = cron_add_interval_job(scheduler, "sync_data", 300, "SYNC_PAYMENTS")
scheduler = cron_trigger_job(scheduler, "sync_data")
```

### 2.24 Fixed-Capacity LRU Cache (`std.cache_lru`)
```rust
import std.cache_lru

let mut cache = lru_cache_new(1000)
cache = lru_cache_put(cache, "user:101", user_profile)
let profile = lru_cache_get(cache, "user:101")
```

### 2.25 Advanced Multi-table SQL Builder (`std.sql_builder`)
```rust
import std.sql_builder

let mut q = sql_query_new("orders")
q = sql_select(q, ["orders.id", "users.name", "payments.amount"])
q = sql_join(q, "INNER", "users", "orders.user_id = users.id")
q = sql_where(q, "orders.status", "=", "PAID")
q = sql_group_by(q, ["orders.id", "users.name"])
let sql_str = sql_build(q)
```

### 2.26 Prometheus Metrics Exporter (`std.metrics`)
```rust
import std.metrics

let mut reg = metrics_registry_new("payment_service")
reg = metrics_inc_counter(reg, "http_requests_total", 1.0)
reg = metrics_set_gauge(reg, "active_connections", 25.0)
let text_export = metrics_to_prometheus(reg)
```

### 2.27 SQLite Relational Database Engine (`std.db_sqlite`)
```rust
import std.db_sqlite

let mut db = sqlite_open(":memory:")
db = sqlite_exec(db, "CREATE TABLE users (id INT, name TEXT)")
db = sqlite_insert(db, "users", map_set(map_new(), "name", "Trong"))

// Transaction with Rollback
db = sqlite_begin_transaction(db)
db = sqlite_rollback(db)
let rows = sqlite_query(db, "users")
```

### 2.28 MySQL Connection Protocol (`std.db_mysql`)
```rust
import std.db_mysql

let pool = mysql_connect_pool("root:secret@tcp(127.0.0.1:3306)/app_db")
let is_up = mysql_ping(pool)
let res = mysql_exec(pool, "INSERT INTO orders (total) VALUES (?)", [100])
```

### 2.29 PostgreSQL Pool Client (`std.db_postgres`)
```rust
import std.db_postgres

let pg = postgres_connect("postgres://pg_user:secret@127.0.0.1:5432/app_db")
let is_healthy = postgres_is_healthy(pg)
```

### 2.30 Multipart Form-Data & File Uploads (`std.multipart`)
```rust
import std.multipart

let parsed = multipart_parse(raw_body, boundary_string)
let files = map_get(parsed, "files")
let form_data = map_get(parsed, "fields")
```

### 2.31 REST API Pagination Utility (`std.pagination`)
```rust
import std.pagination

let items = [10, 20, 30, 40, 50, 60, 70, 80]
let res = pagination_slice(items, page: 2, page_size: 4)
// res.items -> [50, 60, 70, 80]
// res.pagination -> { page: 2, page_size: 4, total_pages: 2, has_next: false, has_prev: true }
```

### 2.32 Generic Transactional Email Builder (`std.email`)
```rust
import std.email

let mut mail = email_new("noreply@vietlang.dev", "user@gmail.com", "System Alert")
let card_html = email_render_simple_card("Order Confirmed", "Your payment is processed", "View Order", "https://...")
mail = email_set_html(mail, card_html)
```

### 2.33 Cryptographic OTP & 2FA Tokens (`std.otp`)
```rust
import std.otp

let code = otp_generate(6) // "582914"
let otp_data = otp_create(user_id: 101, digits: 6, ttl_seconds: 300)
let valid = otp_verify(otp_data, code)
```

### 2.34 File Storage & MIME Utilities (`std.file_storage`)
```rust
import std.file_storage

let is_valid_ext = file_storage_is_allowed_extension("avatar.png", ["jpg", "png", "webp"])
let safe_name = file_storage_generate_safe_name("document.pdf", "invoice")
let formatted_size = file_storage_format_size(1048576) // "1 MB"
```

### 2.35 Opt-in Standard WebSocket Engine (`std.ws`)
```rust
import std.ws

// Explicitly enable WebSocket upgrades on /ws endpoint
ws_init("/ws")
ws_send_text("Hello Realtime Clients!")
ws_emit("ORDER_CREATED", map_set(map_new(), "order_id", 101))
```

### 2.36 Configurable Sliding-Window Rate Limiter (`std.rate_limiter`)
```rust
import std.rate_limiter

let mut limiter = rate_limiter_new(max_requests: 100, window_seconds: 60)
let check = rate_limiter_check(limiter, "192.168.1.1")
if !map_get(check, "allowed") {
    // 429 Too Many Requests
}
limiter = map_get(check, "limiter")
```

---

## 3. Package Management, Documentation & Community Registry

VietLang provides a complete developer toolchain for building, testing, documenting, and publishing community libraries:

```bash
# 1. Initialize a new package (templates: lib | api | microservice)
vietlang init my_payment_sdk lib

# 2. View and generate documentation
vietlang doc                       # Browse all standard library modules
vietlang doc std.pagination        # Inspect specific module signatures & parameters
vietlang doc my_payment_sdk        # Inspect custom package documentation
vietlang doc --all                 # Generate Markdown API docs into docs/api/

# 3. Search and install community modules
vietlang search redis
vietlang install redis@1.2.0
vietlang install auth@3.0.0

# 4. Verify project correctness & run test suites
vietlang verify
vietlang test

# 5. Publish to Central Community Registry
vietlang publish
```

---

## 4. Running Programs & Bytecode VM

```bash
# Standard Tree-Walking Interpreter
vietlang src/main.vl

# High-Performance Bytecode Virtual Machine
vietlang --vm src/main.vl

# Run Unit Tests
cargo test --all
vietlang test examples/enterprise_ecosystem_demo.vl
```

