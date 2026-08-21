# VietLang Production Backend Cookbook

Production recipes and architectural design patterns for building scalable, high-throughput microservices and APIs with VietLang.

---

## Recipe 1: Production REST API with Validation & JWT Auth

```rust
import std.http_router
import std.validator
import std.jwt
import std.cache

let JWT_SECRET = "production-super-secret-key"
let auth_cache = cache_new()

fn handle_login(req) {
    let body = map_get(req, "body")
    let v = validator_new()
    let v = validator_add_rule(v, "username", "required")
    let v = validator_add_rule(v, "password", "required")

    let val_res = validate(v, body)
    if !map_get(val_res, "is_valid") {
        return response_error(400, "Username and password required")
    }

    let user_id = 1001
    let token_payload = map_set(map_set(map_new(), "uid", user_id), "role", "admin")
    let token = jwt_sign(token_payload, JWT_SECRET)

    let resp_data = map_set(map_set(map_new(), "token", token), "user_id", user_id)
    return response_json(200, resp_data, "Login successful")
}

fn handle_get_profile(req) {
    let headers = map_get(req, "headers")
    let auth_header = map_get(headers, "Authorization")

    if auth_header == none {
        return response_error(401, "Missing Authorization header")
    }

    let parts = auth_header.split(" ")
    if parts.len() != 2 || parts[0] != "Bearer" {
        return response_error(401, "Invalid Bearer format")
    }

    let token = parts[1]
    let auth_res = jwt_verify(token, JWT_SECRET)
    if !map_get(auth_res, "valid") {
        return response_error(403, "Invalid or expired token")
    }

    let profile = map_set(map_set(map_new(), "id", 1001), "email", "admin@vietlang.dev")
    return response_json(200, profile)
}
```

---

## Recipe 2: Database Operations with Query Builder

```rust
import std.orm

fn find_active_premium_users(limit_count: Int = 20) {
    let q = query_builder_new("users")
    let q = qb_select(q, ["id", "username", "email", "created_at"])
    let q = qb_where(q, "status", "=", "active")
    let q = qb_where(q, "tier", "=", "premium")
    let q = qb_order_by(q, "created_at", "DESC")
    let q = qb_limit(q, limit_count)

    let sql = qb_to_sql(q)
    return db_query(sql)
}

fn register_new_account(username: String, email: String) {
    let data = map_set(map_set(map_new(), "username", username), "email", email)
    let sql = qb_insert_sql("users", data)
    return db_query(sql)
}
```

---

## Recipe 3: Cache-Aside Architecture

```rust
import std.cache

let memory_cache = cache_new()

fn get_user_cached(user_id: Int) {
    let cache_key = "user:" + to_string(user_id)

    // 1. Check cache
    if cache_has(memory_cache, cache_key) {
        return cache_get(memory_cache, cache_key)
    }

    // 2. Fetch from database
    let user_data = db_query("SELECT * FROM users WHERE id = " + to_string(user_id))

    // 3. Store in cache with 10-minute TTL (600s)
    let memory_cache = cache_set(memory_cache, cache_key, user_data, 600)
    return user_data
}
```

---

## Recipe 4: Concurrent Background Queue Worker

```rust
let task_channel = channel(100)

fn start_background_workers(num_workers: Int = 4) {
    for i in 0..num_workers {
        spawn(fn() {
            println("Worker started")
        })
    }
}
```
