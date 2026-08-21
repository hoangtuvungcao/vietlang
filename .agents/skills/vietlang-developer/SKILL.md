---
name: vietlang-developer
description: Comprehensive expert guide for AI agents and developers to build, test, and maintain backend microservices and community libraries using VietLang.
---

# VietLang Developer Skill Guide

VietLang is a backend-first, statically-analyzed, garbage-collected programming language optimized for high-throughput network services, microservices, and database APIs.

---

## 1. Core Syntax & Language Rules

### 1.1 Variables & Mutation
- Immutable by default: `let name = "VietLang"`
- Mutable variables: `let mut counter = 0`
- Compound assignment: `counter += 1`, `x -= 2`, `y *= 3`, `z /= 4`, `mod %= 2`
- Null literal: `none`
- Boolean literals: `true`, `false`

### 1.2 Functions & Returns
```rust
fn add(a: Int, b: Int) -> Int {
    return a + b
}

// Anonymous functions / Closures
let numbers = [1, 2, 3, 4]
let doubled = numbers.map(fn(x) { return x * 2 })
let evens = numbers.filter(fn(x) { return x % 2 == 0 })
```

### 1.3 Control Flow & Pattern Matching
```rust
// If-else
if condition {
    // then block
} else {
    // else block
}

// Loops
while count < 10 {
    count += 1
}

for item in items {
    println(item)
}

// Match
let label = match status_code {
    200 => "OK",
    404 => "Not Found",
    500 => "Internal Server Error",
    _ => "Unknown"
}
```

### 1.4 Error Handling
```rust
try {
    let res = risky_io_operation()
    println("Success: " + res)
} catch err {
    log_error("Failed operation: " + err)
}

// Custom throw
throw("Invalid credentials")
```

---

## 2. Module System & Imports

VietLang resolves modules across multiple search paths:
1. `import path.to.file` -> loads `path/to/file.vl`
2. `import std.test` -> loads standard library `std/test.vl`
3. `import std.strings` -> loads `std/strings.vl`
4. `import std.jwt` -> loads `std/jwt.vl`
5. `import std.http_router` -> loads `std/http_router.vl`
6. `import std.validator` -> loads `std/validator.vl`
7. `import std.cache` -> loads `std/cache.vl`
8. `import std.orm` -> loads `std/orm.vl`

---

## 3. Backend Architecture Best Practices

### 3.1 REST API Controller Pattern
```rust
import std.test
import std.strings
import std.jwt
import std.http_router
import std.validator

fn handle_get_users(req) {
    let users = [
        map_set(map_set(map_new(), "id", 1), "username", "admin"),
        map_set(map_set(map_new(), "id", 2), "username", "developer")
    ]
    return response_json(200, users)
}

fn handle_create_user(req) {
    let v = validator_new()
    let v = validator_add_rule(v, "email", "required")
    let v = validator_add_rule(v, "email", "email")

    let body = map_get(req, "body")
    let validation_res = validate(v, body)
    if !map_get(validation_res, "is_valid") {
        return response_error(400, "Validation failed")
    }

    return response_json(201, map_set(map_new(), "created", true))
}
```

### 3.2 Testing Convention
Always write test suites with `std.test`:
```rust
import std.test

suite("User Service Tests")

test("User validation passes with valid email", fn() {
    assert_true(1 == 1)
})

test_summary()
```

---

## 4. Package Management with `vpm`

- Initialize project: `vietlang vpm.vl init <project-name>`
- List modules: `vietlang vpm.vl list`
- Inspect manifest: `vietlang vpm.vl info`

---

## 5. Coding Standards for Agents

- Strictly avoid all icons, emojis, or non-standard symbols in comments, string logs, and documentation.
- Use explicit return values for all query/data manipulation routines.
- Maintain test coverage for every newly introduced endpoint or module.
