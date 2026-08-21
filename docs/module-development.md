# VietLang Decentralized Community Module Guide

Learn how to develop, test, install, and distribute reusable modules in the VietLang ecosystem using decentralized Git repositories. No central server or account is required.

---

## 1. Decentralized Package Architecture

VietLang uses a **serverless, decentralized package model** (similar to Go and Deno):
- Any Git repository (GitHub, GitLab, self-hosted Gitea) is directly installable as a package.
- Packages are automatically cloned into your local `modules/` directory.
- `vpm.vl` manages dependencies directly in `vietlang.json`.

---

## 2. Package Directory Layout

Every VietLang package follows this standard layout:

```
my_awesome_module/
├── vietlang.json         # Package manifest
├── src/
│   ├── main.vl           # Primary public API entry point
│   └── helper.vl         # Internal implementation logic
├── tests/
│   └── main_test.vl      # Test suite
└── README.md             # Documentation & API reference
```

---

## 3. Creating a Community Module

Use the native CLI with the `lib` template:

```bash
vietlang init vietlang_redis lib
```

This creates `vietlang.json`:
```json
{
  "name": "vietlang_redis",
  "version": "1.0.0",
  "type": "lib",
  "description": "High-performance Redis client for VietLang",
  "main": "src/main.vl",
  "dependencies": {},
  "license": "MIT"
}
```

---

## 4. Writing Public APIs and Logic

In `src/main.vl`:

```rust
// vietlang_redis - Public API
import std.socket
import std.json

fn redis_connect(host: String = "127.0.0.1", port: Int = 6379) {
    let client = map_new()
    let client = map_set(client, "host", host)
    let client = map_set(client, "port", port)
    let client = map_set(client, "connected", true)
    return client
}

fn redis_set(client, key: String, val: String) -> Bool {
    let host = map_get(client, "host")
    let port = map_get(client, "port")
    let cmd = "SET " + key + " " + val + "\r\n"
    let resp = socket_tcp_send(host, port, cmd)
    return resp != ""
}

fn redis_get(client, key: String) -> String {
    let host = map_get(client, "host")
    let port = map_get(client, "port")
    let cmd = "GET " + key + "\r\n"
    return socket_tcp_send(host, port, cmd)
}
```

---

## 5. Writing Unit Tests

In `tests/main_test.vl`:

```rust
import std.test
import src.main

suite("Redis Client Test Suite")

test("Client connection initialization", fn() {
    let client = redis_connect("127.0.0.1", 6379)
    assert_eq(map_get(client, "host"), "127.0.0.1", "Host should match")
    assert_eq(map_get(client, "port"), 6379, "Port should match")
    assert_true(map_get(client, "connected"), "Should be connected")
})

test_summary()
```

Run tests:
```bash
vietlang tests/main_test.vl
```

---

## 6. Publishing to Central Community Registry

```bash
vietlang verify
vietlang publish
```

---

## 7. Installing in Other Projects

Any developer can install your module:

```bash
vietlang install redis
vietlang install redis@1.2.0
```

Import in user code:
```rust
import modules.redis.src.main

let r = redis_connect()
redis_set(r, "session:101", "active")
```

---

## 8. Updating & Removing Dependencies

```bash
# Update single package
vietlang update redis

# Update all packages
vietlang update

# Remove package
vietlang remove redis
```
