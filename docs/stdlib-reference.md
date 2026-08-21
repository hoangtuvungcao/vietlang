# VietLang Standard Library (Stdlib) Master Reference

Complete, exhaustive reference documentation for all built-in functions and 30 standard library modules in VietLang.

Repository: [https://github.com/hoangtuvungcao/vietlang](https://github.com/hoangtuvungcao/vietlang)

---

## Table of Contents

1. [Core Built-in Functions](#1-core-built-in-functions)
2. [Security & Cryptography (`std.security`, `std.crypto_advanced`)](#2-security--cryptography)
3. [In-Memory Caching & Storage (`std.kv_store`, `std.cache_lru`, `std.cache`)](#3-in-memory-caching--storage)
4. [Distributed Streaming & Events (`std.stream`, `std.event_bus`)](#4-distributed-streaming--events)
5. [Distributed Transactions & Resilience (`std.saga`, `std.retry`, `std.circuit_breaker`)](#5-distributed-transactions--resilience)
6. [HTTP & Realtime Networking (`std.http_pipeline`, `std.http_router`, `std.websocket`, `std.socket`)](#6-http--realtime-networking)
7. [Database & Migrations (`std.orm`, `std.sql_builder`, `std.migration`)](#7-database--migrations)
8. [Telemetry, Observability & Health (`std.telemetry`, `std.metrics`, `std.health`)](#8-telemetry-observability--health)
9. [Rate Limiting & Traffic Protection (`std.rate_limiter`)](#9-rate-limiting--traffic-protection)
10. [Identity, JWT & Validation (`std.jwt`, `std.validator`)](#10-identity-jwt--validation)
11. [Configuration & Scheduling (`std.config`, `std.cron`, `std.queue`)](#11-configuration--scheduling)
12. [Utilities & Testing (`std.strings`, `std.test`)](#12-utilities--testing)

---

## 1. Core Built-in Functions

| Function | Signature | Description |
|---|---|---|
| `print(args...)` | `print(...) -> None` | Prints values without a trailing newline |
| `println(args...)` | `println(...) -> None` | Prints values followed by a newline |
| `len(val)` | `len(Array \| String \| Map) -> Int` | Returns element count or character length |
| `type_of(val)` / `typeof(val)` | `type_of(Any) -> String` | Returns the runtime type name as a String |
| `to_string(val)` | `to_string(Any) -> String` | Converts any value to its String representation |
| `to_int(val)` | `to_int(Any) -> Int` | Converts a numeric or string value to 64-bit Int |
| `to_float(val)` | `to_float(Any) -> Float` | Converts a numeric or string value to Float |
| `push(arr, item)` | `push(Array, Any) -> Array` | Returns a new array with the item appended |
| `pop(arr)` | `pop(Array) -> Any` | Returns the last element from the array |
| `slice(arr, start, [end])` | `slice(Array, Int, [Int]) -> Array` | Slices an array by index |
| `sort(arr)` | `sort(Array) -> Array` | Returns a sorted copy of the array |
| `char_at(str, idx)` | `char_at(String, Int) -> String` | Returns single character at index |
| `char_code(char)` | `char_code(String) -> Int` | Returns ASCII/Unicode code point |
| `from_char_code(code)` | `from_char_code(Int) -> String` | Constructs character from code point |
| `substring(str, start, [end])`| `substring(String, Int, [Int]) -> String` | Extracts substring |
| `url_encode(str)` | `url_encode(String) -> String` | RFC-compliant URL percent encoder |
| `url_decode(str)` | `url_decode(String) -> String` | URL percent decoder |
| `hex_encode(str)` | `hex_encode(String) -> String` | Encodes bytes/string to hex string |
| `hex_decode(str)` | `hex_decode(String) -> String` | Decodes hex string back to string |
| `crypto_random_hex([len])` | `crypto_random_hex([Int]) -> String` | Generates cryptographically secure random hex |
| `time_now()` | `time_now() -> Int` | Current UNIX timestamp in seconds |
| `time_now_ms()` | `time_now_ms() -> Int` | Current UNIX timestamp in milliseconds |
| `time_now_us()` | `time_now_us() -> Int` | Current UNIX timestamp in microseconds |
| `sleep_ms(ms)` | `sleep_ms(Int) -> None` | Pauses current thread execution |
| `system_cmd(cmd)` | `system_cmd(String) -> CommandResult` | Executes system shell command |

---

## 2. Security & Cryptography

### `std.security`
- `security_hash_password(password: String, salt: String = "") -> String`: Generates salted SHA-256 hash.
- `security_verify_password(password: String, stored_hash: String) -> Bool`: Verifies password against hash.
- `security_constant_time_compare(a: String, b: String) -> Bool`: Constant-time string comparison.
- `security_generate_csrf_token() -> String`: Generates unique CSRF token.
- `security_sanitize_html(input: String) -> String`: Escapes HTML entities (`<`, `>`, `&`, `"`, `'`).

### `std.crypto_advanced`
- `crypto_verify_webhook(payload: String, signature: String, secret_key: String) -> Bool`: Validates HMAC webhook signature.
- `crypto_generate_api_key(environment: String = "live") -> String`: Generates API key (`ak_live_...`).
- `crypto_encrypt_payload(data, secret_key: String) -> String`: Encrypts structured data to Base64 ciphertext.
- `crypto_decrypt_payload(encrypted_b64: String, secret_key: String) -> Any`: Decrypts Base64 ciphertext back to data.

---

## 3. In-Memory Caching & Storage

### `std.kv_store` (Redis Engine)
- `kv_store_new() -> Map`: Initializes in-memory key-value store.
- `kv_set(store, key: String, value, ttl_seconds: Int = 0) -> Map`: Sets key with optional TTL.
- `kv_get(store, key: String, default_val = none) -> Any`: Retrieves value if not expired.
- `kv_incr(store, key: String, delta: Int = 1) -> Map`: Atomically increments a counter.
- `kv_hset(store, hash_key: String, field: String, val) -> Map`: Sets field in a Hash map.
- `kv_hget(store, hash_key: String, field: String, default_val = none) -> Any`: Gets field from Hash map.

### `std.cache_lru`
- `lru_cache_new(capacity: Int = 100) -> Map`: Initializes fixed-capacity LRU cache.
- `lru_cache_put(cache, key: String, val) -> Map`: Inserts value, evicting oldest item when full.
- `lru_cache_get(cache, key: String, default_val = none) -> Any`: Gets value by key.

---

## 4. Distributed Streaming & Events

### `std.stream` (Kafka Engine)
- `stream_engine_new() -> Map`: Initializes event stream engine.
- `stream_publish(engine, topic: String, message) -> Map`: Appends event to topic log with sequential offset.
- `stream_consume_from(engine, topic: String, start_offset: Int, batch_size: Int = 10) -> Array`: Batch reads events.

### `std.event_bus`
- `event_bus_new() -> Map`: Initializes in-process event bus.
- `event_subscribe(bus, topic: String, subscriber_name: String) -> Map`: Subscribes to topic.
- `event_publish(bus, topic: String, payload) -> Map`: Broadcasts event to all active subscribers.

---

## 5. Distributed Transactions & Resilience

### `std.saga`
- `saga_new(transaction_name: String) -> Map`: Initializes SAGA coordinator.
- `saga_add_step(saga, name: String, forward_action: String, compensate_action: String) -> Map`: Registers step.
- `saga_record_step_success(saga, step_name: String) -> Map`: Marks step as completed.
- `saga_compensate_all(saga) -> Map`: Executes compensating rollback in reverse order.

### `std.retry`
- `retry_policy_new(max_attempts: Int = 3, initial_delay_ms: Int = 100, backoff_factor: Float = 2.0) -> Map`: Configures retry strategy.
- `retry_calculate_delay_ms(policy, attempt: Int) -> Int`: Computes exponential backoff delay.

### `std.circuit_breaker`
- `circuit_breaker_new(max_failures: Int = 5, timeout_seconds: Int = 30) -> Map`: Initializes breaker.
- `cb_can_execute(cb) -> Bool`: Checks if downstream request can proceed.
- `cb_record_success(cb) -> Map`: Resets failure counter on success.
- `cb_record_failure(cb) -> Map`: Records failure and transitions state to OPEN if threshold reached.

---

## 6. HTTP & Realtime Networking

### `std.http_pipeline`
- `http_pipeline_new() -> Map`: Initializes enterprise HTTP middleware pipeline.
- `http_apply_security_headers(headers: Map) -> Map`: Applies CSP, HSTS, X-Frame-Options, nosniff.
- `http_check_ip_allowed(pipeline, client_ip: String) -> Bool`: Evaluates IP against CIDR whitelists.
- `http_build_response(status_code: Int, body_data, headers = none) -> Map`: Constructs secure HTTP response.

### `std.websocket`
- `ws_accept_key(sec_websocket_key: String) -> String`: Computes RFC 6455 SHA-1 + Base64 handshake key.
- `ws_handshake_response(sec_websocket_key: String) -> String`: Generates complete 101 Switching Protocols header.
- `ws_encode_text_frame(message: String) -> Map`: Encodes WebSocket text frame.
- `ws_encode_ping_frame(payload: String = "") -> Map`: Encodes WebSocket ping frame.
- `ws_room_manager_new() -> Map`: Initializes WebSocket room broadcaster hub.
- `ws_room_join(manager, room_name: String, client_id: String) -> Map`: Registers client to room.
- `ws_room_broadcast(manager, room_name: String, event: String, data) -> Map`: Broadcasts message to room.

### `std.multipart`
- `multipart_parse(raw_body: String, boundary: String) -> Map`: Streaming multipart form-data and binary/text file upload decoder.

### `std.socket`
- `socket_tcp_send(host: String, port: Int, message: String, timeout_ms: Int = 3000) -> String`: Raw TCP stream client.
- `socket_udp_send(host: String, port: Int, payload: String) -> Bool`: Raw UDP packet sender.
- `socket_ping(host: String, port: Int, timeout_ms: Int = 1000) -> Bool`: Network port probe.

---

## 7. Database & Migrations

### `std.db_sqlite` (SQLite Relational Storage Engine)
- `sqlite_open(db_path: String = ":memory:") -> Map`: Opens an in-memory or file-backed SQLite database.
- `sqlite_exec(db, sql: String) -> Map`: Executes DDL SQL (such as `CREATE TABLE`).
- `sqlite_insert(db, table_name: String, row_data: Map) -> Map`: Inserts a row into a table.
- `sqlite_query(db, table_name: String, filter_fn = none) -> Array`: Queries rows with optional predicate.
- `sqlite_begin_transaction(db) -> Map`: Takes an ACID snapshot for transactions.
- `sqlite_commit(db) -> Map`: Commits pending changes.
- `sqlite_rollback(db) -> Map`: Rolls back to pre-transaction snapshot on failure.

### `std.db_mysql` (async SQLx pool)
- `mysql_connect_pool` creates a bounded lazy pool on Tokio/Rustls.
- `mysql_execute` and `mysql_query` bind parameter arrays.
- `mysql_ping` checks a live connection and `mysql_close` drains the pool.

### `std.db_postgres` (PostgreSQL Client & Schema Manager)
- `postgres_parse_url(url: String) -> Map`: Parses PostgreSQL URL (`postgres://user:pass@host:5432/db`).
- `postgres_connect(url: String) -> Map`: Connects to PostgreSQL server.
- `postgres_query(client, query_sql: String, params = none) -> Map`: Executes SQL query.
- `postgres_is_healthy(client) -> Bool`: Probes PostgreSQL connection health.

### `std.sql_builder`
- `sql_query_new(table: String) -> Map`: Initializes SQL builder.
- `sql_select(q, columns: Array) -> Map`: Sets SELECT columns.
- `sql_join(q, join_type: String, target_table: String, on_clause: String) -> Map`: Adds INNER/LEFT/RIGHT JOIN.
- `sql_where(q, column: String, operator: String, value) -> Map`: Adds WHERE condition.
- `sql_group_by(q, columns: Array) -> Map`: Adds GROUP BY clause.
- `sql_build(q) -> String`: Compiles query to SQL string.

### `std.orm`
- `query_builder_new(table: String) -> Map`: Initializes ORM builder.
- `qb_select`, `qb_where`, `qb_order_by`, `qb_limit`, `qb_to_sql`: ActiveRecord query methods.
- `qb_insert_sql(table: String, data: Map) -> String`: Generates parameterized INSERT SQL.

### `std.migration`
- `migration_engine_new() -> Map`: Initializes schema migration runner.
- `migration_add_step(engine, name: String, up_sql: String, down_sql: String) -> Map`: Registers migration step.
- `migration_run_all(engine) -> Map`: Applies all pending schema migrations.

---

## 8. Telemetry, Observability & Health

### `std.telemetry` (OpenTelemetry)
- `trace_context_new(service_name: String) -> Map`: Creates root tracing context.
- `trace_create_child_span(parent_ctx, span_name: String) -> Map`: Spawns child span.
- `trace_end_span(span: Map) -> Map`: Calculates duration with microsecond accuracy.
- `trace_inject_headers(span: Map) -> Map`: Injects `X-Trace-Id` and `X-Span-Id` headers.

### `std.metrics` (Prometheus)
- `metrics_registry_new(service_name: String) -> Map`: Initializes metrics store.
- `metrics_inc_counter(registry, name: String, delta: Float = 1.0) -> Map`: Increments counter metric.
- `metrics_set_gauge(registry, name: String, value: Float) -> Map`: Sets gauge value.
- `metrics_to_prometheus(registry) -> String`: Formats all metrics in standard Prometheus text format.

### `std.health`
- `health_checker_new(service_name: String) -> Map`: Initializes health probe.
- `health_add_check(hc, component_name: String, is_healthy: Bool) -> Map`: Adds component status.
- `health_get_report(hc) -> Map`: Returns overall status (`UP` / `DOWN`) and uptime in seconds.

---

## 9. Rate Limiting (`std.rate_limiter`)

- `rate_limiter_new(max_tokens: Int, refill_rate_per_sec: Int) -> Map`: Configures Token Bucket limiter.
- `rate_limit_allow(rl, client_key: String) -> Map`: Evaluates request allowance (`allowed: true/false`, remaining tokens).

---

## 10. Identity, JWT & Validation

### `std.jwt`
- Disabled legacy compatibility module. It is not a complete JWT validator and
  must not be used for new or production authentication.

### `std.validator`
- `validator_new() -> Map`: Initializes validator.
- `validator_add_rule(v, field: String, rule: String) -> Map`: Adds validation rule (`required`, `email`, `min_len:N`, `min_val:N`).
- `validate(v, data: Map) -> Map`: Validates input (`is_valid: true/false`, `errors: Array`).

---

## 11. Configuration & Scheduling

### `std.config`
- `config_load(env_file: String = ".env") -> Map`: Loads environment variables from `.env`.
- `config_get(cfg, key: String, default_val = none) -> Any`: Retrieves value with fallback.
- `config_get_int(cfg, key: String, default_val: Int = 0) -> Int`: Parses integer configuration.
- `config_get_bool(cfg, key: String, default_val: Bool = false) -> Bool`: Parses boolean configuration.

### `std.cron`
- `cron_scheduler_new() -> Map`: Initializes cron scheduler.
- `cron_add_interval_job(scheduler, job_id: String, interval_seconds: Int, action: String) -> Map`: Registers recurring job.
- `cron_trigger_job(scheduler, job_id: String) -> Map`: Executes job and records timestamp.

### `std.queue`
- `queue_new() -> Map`: Initializes task queue.
- `queue_push(q, task_type: String, payload, max_retries: Int = 3) -> Map`: Schedules background task.
- `queue_pop(q) -> Map`: Dequeues next task.
- `queue_retry_or_fail(q, task: Map) -> Map`: Retries task or moves to Dead Letter Queue (DLQ).

---

## 12. Utilities & Testing

### `std.strings`
- `pad_left(str, len, pad_char)`: Pads string on the left.
- `pad_right(str, len, pad_char)`: Pads string on the right.
- `snake_case(str)`: Converts camelCase/TitleCase to snake_case.
- `camel_case(str)`: Converts snake_case to camelCase.
- `slugify(str)`: Converts text to URL-safe slug.
- `capitalize(str)`: Capitalizes first letter.

### `std.test`
- `suite(name: String)`: Declares a test suite header.
- `test(description: String, test_fn)`: Runs a unit test block.
- `assert_eq(actual, expected, [message])`: Asserts equality.
- `assert_true(condition, [message])`: Asserts condition is true.
- `assert_false(condition, [message])`: Asserts condition is false.
- `test_summary()`: Prints summary report with pass/fail statistics.
