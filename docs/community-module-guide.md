# VietLang Community Module Development & Central Registry Guide

A comprehensive, step-by-step guide for developers to create, test, publish, discover, and install community packages in the VietLang ecosystem.

Repository: [https://github.com/hoangtuvungcao/vietlang](https://github.com/hoangtuvungcao/vietlang)

---

## 1. Overview: The Central Community Registry & Native Package Toolchain

VietLang integrates a **native package manager directly into the `vietlang` runtime binary**. No external scripts or node/python runtimes needed.

Highlights:
- **Central Community Registry (`registry/index.json`)**: One centralized place to search and discover modules.
- **Short Name & Version Tagging**: Install instantly via `vietlang install redis` or `vietlang install redis@1.2.0`.
- **Zero Configuration**: Built-in dependency tracking and version locking in `vietlang.json`.
- **Deterministic Checksum Validation**: Every published package is verified by source checksum.

---

## 2. Creating a New Module

Use the native CLI to scaffold your project:

```bash
# Template options: 'lib' (Community Library), 'api' (REST API), 'microservice' (Full backend)
vietlang init vietlang_email_sender lib
```

This creates the standard project structure:
```
vietlang_email_sender/
├── vietlang.json        # Package manifest
├── src/
│   └── main.vl          # Library entrypoint
├── tests/
│   └── main_test.vl     # Automated unit test suite
└── modules/             # Downstream dependencies
```

---

## 3. Writing Clean Library Code

In `src/main.vl`, write your library logic in pure VietLang:

```rust
// ===================================================
// VietLang Email Sender Plugin
// ===================================================

import std.socket
import std.strings

fn email_client_new(smtp_host: String, smtp_port: Int = 587) {
    let client = map_new()
    let client = map_set(client, "host", smtp_host)
    let client = map_set(client, "port", smtp_port)
    let client = map_set(client, "connected", true)
    return client
}

fn email_send(client, to_addr: String, subject: String, body: String) -> Bool {
    let host = map_get(client, "host")
    let port = map_get(client, "port")
    
    let payload = "TO: " + to_addr + "\nSUBJECT: " + subject + "\n\n" + body
    let raw_resp = socket_tcp_send(host, port, payload)
    return raw_resp != ""
}
```

---

## 4. Writing Unit Tests

In `tests/main_test.vl`, verify your library:

```rust
import std.test

// Import your library entrypoint
import src.main

suite("Email Sender Test Suite")

test("Client configuration initialization", fn() {
    let client = email_client_new("smtp.mailtrap.io", 2525)
    assert_eq(map_get(client, "host"), "smtp.mailtrap.io", "Host mismatch")
    assert_eq(map_get(client, "port"), 2525, "Port mismatch")
})

test_summary()
```

Run your tests locally:
```bash
vietlang tests/main_test.vl
```

---

## 5. Publishing to the Central Community Registry

1. **Verify project integrity before release**:
   ```bash
   vietlang verify
   ```

2. **Publish to the Central Registry**:
   ```bash
   vietlang publish
   ```

This validates `vietlang.json`, executes test suites, computes the release checksum, and registers the module entry into the Central Community Registry index.

---

## 6. Discovering & Installing Modules

### Searching the Central Registry
Developers can search the community catalog by keywords, package name, or author:
```bash
vietlang search redis
vietlang search postgres
vietlang search mailer
```

### Installing a Module
Install directly using the module name or with a specific version lock:
```bash
# Install latest
vietlang install redis

# Install specific version
vietlang install redis@1.2.0
vietlang install auth@3.0.0
vietlang install user/custom_module@v1.0.0
```

This will:
- Download the module into `modules/<pkg_name>`.
- Checkout the exact version tag.
- Update your `vietlang.json` dependency manifest.

### Inspecting Module API Signatures
Inspect exported functions without opening code:
```bash
vietlang docs redis
vietlang docs std.db_sqlite
```

### Importing in Your Backend Code
```rust
import modules.redis.src.main

let client = redis_client_new("127.0.0.1", 6379)
```

### Updating & Removing Modules
```bash
# Update single module to target or latest version
vietlang update redis
vietlang update redis@2.0.0

# Update all dependencies
vietlang update

# Remove module
vietlang remove redis
```
