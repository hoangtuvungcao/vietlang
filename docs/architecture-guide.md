# VietLang Enterprise Architecture & Systems Design Guide

Comprehensive blueprint for architecting mission-critical, high-concurrency backend services using VietLang.

---

## 1. Architectural Principles

VietLang was designed to solve the common pain points of backend engineering:
1. **Low Memory Footprint**: Minimal runtime overhead without heavy JVM or Node.js runtime layers.
2. **Zero-Dependency Core**: HTTP, Database, JSON, Hashing, Concurrency, and File I/O are built directly into the language runtime.
3. **Structured Concurrency**: Green threads and channels allow 100,000+ concurrent connections per machine.
4. **Predictable Performance**: Stack-based Bytecode VM execution and deterministic resource release.

---

## 2. Microservice Layering Pattern

A production VietLang backend service is partitioned into 4 distinct layers:

```
[ HTTP / RPC Transport Layer ] (std.http_router, std.rpc)
               |
               v
[ Application & Security Layer ] (std.jwt, std.validator, std.rate_limiter)
               |
               v
[ Domain & Business Logic Layer ] (Services, Models, Workflow Engine)
               |
               v
[ Infrastructure & Persistence Layer ] (std.orm, std.cache, std.queue)
```

---

## 3. High-Concurrency Traffic Management

### Rate Limiting & DDoS Shield
Deploy `std.rate_limiter` at the entry-point router to protect internal services:

```rust
import std.rate_limiter

let api_limiter = rate_limiter_new(100, 10) // 100 max burst, 10 req/sec refill

fn protect_route(client_ip: String) -> Bool {
    let check = rate_limit_allow(api_limiter, client_ip)
    return map_get(check, "allowed")
}
```

### Circuit Breaking for Fault Isolation
Wrap third-party payment gateways, search clusters, and external microservices with `std.circuit_breaker` to prevent cascading failures:

```rust
import std.circuit_breaker

let payment_breaker = circuit_breaker_new(5, 60) // Open after 5 failures for 60s
```

---

## 4. Scalable Data Persistence & Caching Strategy

### Query Builder with Connection Pooling
```rust
import std.orm

fn query_customer_orders(customer_id: Int) {
    let q = query_builder_new("orders")
    let q = qb_select(q, ["id", "amount", "status", "created_at"])
    let q = qb_where(q, "customer_id", "=", customer_id)
    let q = qb_order_by(q, "created_at", "DESC")
    return db_query(qb_to_sql(q))
}
```

### Dual-Layer Cache Strategy (L1 Memory + L2 Distributed)
```rust
import std.cache

let l1_cache = cache_new()

fn get_cached_entity(id: String) {
    if cache_has(l1_cache, id) {
        return cache_get(l1_cache, id)
    }
    let data = db_query("SELECT * FROM entities WHERE id = '" + id + "'")
    let l1_cache = cache_set(l1_cache, id, data, 300)
    return data
}
```

---

## 5. Deployment & Observability

- **Single Binary Artifact**: The compiler outputs a standalone executable with no runtime dependencies.
- **Docker Multi-Stage Build**:
```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/vietlang /usr/local/bin/
COPY src/ /app/src/
WORKDIR /app
CMD ["vietlang", "src/main.vl"]
```
