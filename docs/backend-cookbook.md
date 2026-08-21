# VietLang Production Backend Cookbook

Production-ready blueprints, architectural patterns, and practical recipes for building high-throughput microservices and distributed systems with VietLang.

Repository: [https://github.com/hoangtuvungcao/vietlang](https://github.com/hoangtuvungcao/vietlang)

---

## Table of Contents

1. [REST API with Validation & JWT RBAC Auth](#recipe-1-rest-api-with-validation--jwt-rbac-auth)
2. [Relational Databases (SQLite ACID, MySQL & PostgreSQL Pools)](#recipe-2-relational-databases-sqlite-mysql-postgres)
3. [In-Memory KV Store & LRU Caching](#recipe-3-in-memory-kv-store--lru-caching)
4. [Distributed SAGA Transactions & Compensating Rollback](#recipe-4-distributed-saga-transactions--compensating-rollback)
5. [Partitioned Event Streaming & Kafka Pub/Sub](#recipe-5-partitioned-event-streaming--kafka-pubsub)
6. [Real-Time WebSockets RFC 6455 & Room Broadcaster](#recipe-6-real-time-websockets-rfc-6455--room-broadcaster)
7. [Observability: OpenTelemetry Tracing & Prometheus Metrics](#recipe-7-observability-opentelemetry-tracing--prometheus-metrics)
8. [Onion Middleware Pipeline & Security Headers](#recipe-8-onion-middleware-pipeline--security-headers)
9. [Async Task Queue with Dead-Letter Queue (DLQ) & Exponential Retry](#recipe-9-async-task-queue-with-dlq--exponential-retry)
10. [Cron Job Scheduling & Kubernetes Health Probes](#recipe-10-cron-job-scheduling--kubernetes-health-probes)

---

## Recipe 1: REST API with Validation & JWT RBAC Auth

```rust
import std.http_router
import std.validator
import std.jwt

let JWT_SECRET = "production-super-secret-key-32-chars"

fn handle_login(request_body) {
    let v = validator_new()
    let v = validator_add_rule(v, "username", "required")
    let v = validator_add_rule(v, "password", "required")

    let val_res = validate(v, request_body)
    if !map_get(val_res, "is_valid") {
        return response_error(400, "Validation failed: username and password required")
    }

    let user_id = 1001
    let token_payload = map_set(map_set(map_new(), "uid", user_id), "role", "ADMIN")
    let token = jwt_sign(token_payload, JWT_SECRET)

    let resp_data = map_set(map_set(map_new(), "token", token), "user_id", user_id)
    return response_json(200, resp_data, "Login successful")
}

fn handle_protected_profile(auth_header: String) {
    if auth_header == "" {
        return response_error(401, "Missing Authorization header")
    }

    let parts = auth_header.split(" ")
    if parts.len() != 2 || parts[0] != "Bearer" {
        return response_error(401, "Invalid Bearer token format")
    }

    let auth_res = jwt_verify(parts[1], JWT_SECRET)
    if !map_get(auth_res, "valid") {
        return response_error(403, "Invalid or expired token")
    }

    let payload = map_get(auth_res, "payload")
    return response_json(200, payload, "Authorized")
}
```

---

## Recipe 2: Relational Databases (SQLite, MySQL, Postgres)

```rust
import std.db_sqlite
import std.db_mysql
import std.db_postgres

// SQLite in-memory with ACID transaction
fn process_balance_transfer(from_id: Int, to_id: Int, amount: Int) {
    let mut db = sqlite_open(":memory:")
    db = sqlite_exec(db, "CREATE TABLE accounts (id INT, balance INT)")
    db = sqlite_insert(db, "accounts", map_set(map_set(map_new(), "id", from_id), "balance", 1000))
    db = sqlite_insert(db, "accounts", map_set(map_set(map_new(), "id", to_id), "balance", 500))

    // Begin ACID transaction
    db = sqlite_begin_transaction(db)
    
    // Debit sender
    db = sqlite_insert(db, "accounts", map_set(map_set(map_new(), "id", from_id), "balance", 1000 - amount))
    
    // Commit transaction
    db = sqlite_commit(db)
    return sqlite_query(db, "accounts")
}

// MySQL connection pool
fn execute_mysql_order(user_id: Int, total_cents: Int) {
    let pool = mysql_connect_pool("root:secret@tcp(127.0.0.1:3306)/orders_db")
    if mysql_ping(pool) {
        return mysql_exec(pool, "INSERT INTO orders (user_id, total) VALUES (?, ?)", [user_id, total_cents])
    }
    return none
}
```

---

## Recipe 3: In-Memory KV Store & LRU Caching

```rust
import std.kv_store
import std.cache_lru

// Atomic rate limiting with Redis-like store
fn check_rate_limit(redis_store, client_ip: String, max_hits: Int = 60) -> Bool {
    let rate_key = "rate:" + client_ip
    let incr_res = kv_incr(redis_store, rate_key, 1)
    let current_hits = map_get(incr_res, "value")
    return current_hits <= max_hits
}

// Fast LRU Cache
let mut lru = lru_cache_new(500)
lru = lru_cache_put(lru, "session:1001", map_set(map_new(), "user", "Trong"))
let cached_user = lru_cache_get(lru, "session:1001")
```

---

## Recipe 4: Distributed SAGA Transactions & Compensating Rollback

```rust
import std.saga

fn execute_checkout_saga(order_id: String) {
    let mut s = saga_new("OrderCheckoutFlow")
    s = saga_add_step(s, "LockInventory", "StockReserved", "UnlockInventory")
    s = saga_add_step(s, "ChargePayment", "CardCharged", "RefundCard")
    s = saga_add_step(s, "DispatchCourier", "CourierAssigned", "CancelCourier")

    // Step 1: Lock stock -> OK
    s = saga_record_step_success(s, "LockInventory")

    // Step 2: Payment fails -> Trigger automatic rollback
    let rollback_report = saga_compensate_all(s)
    return rollback_report
}
```

---

## Recipe 5: Partitioned Event Streaming & Kafka Pub/Sub

```rust
import std.stream

let mut stream_broker = stream_engine_new()

// Produce event
let order_event = map_set(map_set(map_new(), "type", "ORDER_CREATED"), "id", "ord_777")
let pub_res = stream_publish(stream_broker, "orders_topic", order_event)
stream_broker = map_get(pub_res, "engine")

// Consume events from offset 0
let batch = stream_consume_from(stream_broker, "orders_topic", 0, 10)
```

---

## Recipe 6: Real-Time WebSockets RFC 6455 & Room Broadcaster

```rust
import std.websocket

// Calculate RFC 6455 handshake
let sec_key = "dGhlIHNhbXBsZSBub25jZQ=="
let handshake_header = ws_handshake_response(sec_key)

// Manage multi-room chat
let mut room_hub = ws_room_manager_new()
room_hub = ws_room_join(room_hub, "lobby", "user_101")
room_hub = ws_room_join(room_hub, "lobby", "user_102")

let msg = map_set(map_new(), "text", "Xin chào mọi người!")
let broadcast_res = ws_room_broadcast(room_hub, "lobby", "chat_message", msg)
```

---

## Recipe 7: Observability: OpenTelemetry Tracing & Prometheus Metrics

```rust
import std.telemetry
import std.metrics

// 1. OpenTelemetry Trace Context
let ctx = trace_context_new("payment_service")
let span = trace_create_child_span(ctx, "charge_stripe")
let ended_span = trace_end_span(span)
let headers = trace_inject_headers(ended_span) // Injects X-Trace-Id and X-Span-Id

// 2. Prometheus Metric Exporter
let mut metrics = metrics_registry_new("payment_service")
metrics = metrics_inc_counter(metrics, "http_requests_total", 1.0)
metrics = metrics_set_gauge(metrics, "active_db_connections", 12.0)
let prometheus_scrape_endpoint_body = metrics_to_prometheus(metrics)
```

---

## Recipe 8: Onion Middleware Pipeline & Security Headers

```rust
import std.http_pipeline

let pipeline = http_pipeline_new()

// CIDR IP Subnet filtering
let client_ip = "192.168.1.45"
let is_allowed = http_check_ip_allowed(pipeline, client_ip)

// Generate HTTP Response with automatic Security Headers (CSP, HSTS, X-Frame-Options: DENY, nosniff)
let resp = http_build_response(200, map_set(map_new(), "status", "secure_ok"))
```

---

## Recipe 9: Async Task Queue with DLQ & Exponential Retry

```rust
import std.queue
import std.retry

let mut q = queue_new()
q = queue_push(q, "SEND_EMAIL_NOTIFICATION", map_set(map_new(), "to", "user@example.com"), 3)

let policy = retry_policy_new(3, 100, 2.0)
let delay = retry_calculate_delay_ms(policy, 2) // 200ms backoff
```

---

## Recipe 10: Cron Job Scheduling & Kubernetes Health Probes

```rust
import std.cron
import std.health

// 1. Cron Scheduler
let mut sched = cron_scheduler_new()
sched = cron_add_interval_job(sched, "clean_old_tokens", 3600, "CLEAN_DB")
sched = cron_trigger_job(sched, "clean_old_tokens")

// 2. Kubernetes Probes (/healthz & /readyz)
let mut k8s = health_checker_new("order_service")
k8s = health_add_check(k8s, "database", true)
k8s = health_add_check(k8s, "redis", true)
let health_json = health_get_report(k8s)
```
