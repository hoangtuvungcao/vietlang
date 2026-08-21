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

Use `vpm.vl` with the `lib` template:

```bash
vietlang vpm.vl init vietlang_redis lib
```

This creates `vietlang.json`:
```json
{
  "name": "vietlang_redis",
  "version": "0.1.0",
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
import std.socket

fn redis_connect(host: String = "127.0.0.1", port: Int = 6379) {
    let client = map_new()
    let client = map_set(client, "host", host)
    let client = map_set(client, "port", port)
    return client
}

fn redis_set(client, key: String, val: String) -> String {
    let host = map_get(client, "host")
    let port = map_get(client, "port")
    let cmd = "*3\r\n$3\r\nSET\r\n$" + to_string(key.len()) + "\r\n" + key + "\r\n$" + to_string(val.len()) + "\r\n" + val + "\r\n"
    return socket_tcp_send(host, port, cmd)
}

fn redis_get(client, key: String) -> String {
    let host = map_get(client, "host")
    let port = map_get(client, "port")
    let cmd = "*2\r\n$3\r\nGET\r\n$" + to_string(key.len()) + "\r\n" + key + "\r\n"
    return socket_tcp_send(host, port, cmd)
}
```

---

## 5. Testing with `std.test`

In `tests/main_test.vl`:

```rust
import std.test
import src.main

suite("Redis Client Test Suite")

test("Redis Client Config", fn() {
    let client = redis_connect("127.0.0.1", 6379)
    assert_eq(map_get(client, "port"), 6379, "Port mismatch")
})

test_summary()
```

Run tests:
```bash
vietlang tests/main_test.vl
```

---

## 6. Publishing Your Module to the Community

1. Initialize Git and commit your files:
```bash
git init
git add -A
git commit -m "feat: initial release v0.1.0"
```

2. Validate package with `vpm`:
```bash
vietlang vpm.vl publish
```

3. Push to GitHub:
```bash
git remote add origin https://github.com/yourusername/vietlang_redis.git
git push -u origin main
```

---

## 7. Installing and Using Community Modules in Projects

In any other VietLang project:

### Install via Git URL:
```bash
vietlang vpm.vl install https://github.com/yourusername/vietlang_redis.git
```

### Import into your backend service:
```rust
import modules.vietlang_redis.src.main

let r = redis_connect("127.0.0.1", 6379)
println("Redis client ready")
```

### Update or Remove:
```bash
# Update module to latest git commit
vietlang vpm.vl update vietlang_redis

# Remove module
vietlang vpm.vl remove vietlang_redis
```
