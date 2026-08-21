# VietLang Community Module Development & Distribution Guide

A comprehensive, step-by-step guide for developers to create, test, publish, discover, and install community packages in the decentralized VietLang ecosystem.

Repository: [https://github.com/hoangtuvungcao/vietlang](https://github.com/hoangtuvungcao/vietlang)

---

## 1. Overview: The Serverless Package Philosophy

VietLang uses a **decentralized, serverless package model**. Unlike npm or PyPI, you do not need an account on a centralized server to publish libraries. **Any public or private Git repository on GitHub, GitLab, Gitea, or self-hosted Git servers is an installable VietLang module.**

Benefits:
- Zero central point of failure.
- Instant publishing with `git push`.
- Native support for private microservice repositories.
- Deterministic semantic versioning and SHA256 checksum validation.

---

## 2. Creating a New Module

Use the VietLang Package Manager (`vpm.vl`) to scaffold your project:

```bash
# Template options: 'lib' (Community Library), 'api' (REST API), 'microservice' (Full backend)
vietlang vpm.vl init vietlang_email_sender lib
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

## 5. Publishing to the Community

1. **Verify project integrity before release**:
   ```bash
   vietlang vpm.vl verify
   vietlang vpm.vl publish
   ```

2. **Push to GitHub**:
   ```bash
   git init
   git add -A
   git commit -m "feat: initial release v0.1.0"
   git remote add origin https://github.com/your-username/vietlang_email_sender.git
   git push -u origin main
   ```

Your library is now live and installable worldwide!

---

## 6. Discovering & Installing Modules

### Searching for Modules
Developers can search the community catalog:
```bash
vietlang vpm.vl search redis
vietlang vpm.vl search postgres
vietlang vpm.vl search db
```

### Installing a Module
Install directly from any Git URL into your project:
```bash
vietlang vpm.vl install https://github.com/your-username/vietlang_email_sender.git
```

This will:
- Clone the repository into `modules/vietlang_email_sender`.
- Update your `vietlang.json` dependency map.

### Inspecting Module API Documentation
Inspect all exported functions without opening code:
```bash
vietlang vpm.vl docs vietlang_email_sender
# Output:
# Exported Functions in 'vietlang_email_sender' (modules/vietlang_email_sender/src/main.vl):
#   * fn email_client_new(smtp_host: String, smtp_port: Int = 587) {
#   * fn email_send(client, to_addr: String, subject: String, body: String) -> Bool {
```

### Importing in Your Backend Code
```rust
import modules.vietlang_email_sender.src.main

let mailer = email_client_new("smtp.sendgrid.net", 587)
email_send(mailer, "customer@example.com", "Order Confirmation", "Your order has shipped!")
```

### Updating & Removing Modules
```bash
# Pull latest updates from Git
vietlang vpm.vl update vietlang_email_sender

# Remove module
vietlang vpm.vl remove vietlang_email_sender
```
