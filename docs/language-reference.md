# VietLang Language Reference

Exhaustive language syntax, semantic specification, and type system reference for VietLang.

Repository: [https://github.com/hoangtuvungcao/vietlang](https://github.com/hoangtuvungcao/vietlang)

---

## Table of Contents

1. [Type System](#1-type-system)
2. [Variables & Mutability](#2-variables--mutability)
3. [Operators & Compound Expressions](#3-operators--compound-expressions)
4. [Functions & Closures](#4-functions--closures)
5. [Control Flow & Jump Signals](#5-control-flow--jump-signals)
6. [Error Handling (`try / catch`, `throw`)](#6-error-handling)
7. [Data Structures (Struct, Map, Array)](#7-data-structures)
8. [Pattern Matching (`match`)](#8-pattern-matching)
9. [Module System & Git Packages](#9-module-system--git-packages)
10. [Virtual Machine Execution (`--vm`)](#10-virtual-machine-execution)

---

## 1. Type System

### Primitive Types
- `Int`: 64-bit signed integer (`let count: Int = 42`)
- `Float`: 64-bit IEEE-754 floating-point (`let price: Float = 99.95`)
- `String`: UTF-8 immutable string (`let greeting: String = "Xin chào!"`)
- `Bool`: Boolean truth values (`true`, `false`)
- `None`: Representation of no value (`none`)

### Composite & Collection Types
- `Array`: Homogeneous/heterogeneous dynamic array (`let items = [1, 2, 3]`)
- `Map`: Key-value hash map (`let user = map_set(map_new(), "id", 1)`)
- `Struct`: User-defined compound structure (`struct User { name: String, age: Int }`)
- `Function`: First-class callable closure (`let fn_val = fn(x) { x * 2 }`)

---

## 2. Variables & Mutability

Variables in VietLang are immutable by default:
```rust
let server_name = "AuthService" // Immutable
// server_name = "Other"        // Compile / Runtime Error

let mut request_count = 0        // Mutable
request_count += 1               // OK
```

Variable naming convention:
- Always use `snake_case` for variables and function names (`user_id`, `process_order`).
- `PascalCase` is reserved for Struct and Type declarations (`UserSession`, `HttpPipeline`).

---

## 3. Operators & Compound Expressions

### Arithmetic & Compound Assignment
- Addition / Concatenation: `+`, `+=`
- Subtraction: `-`, `-=`
- Multiplication: `*`, `*=`
- Division: `/`, `/=`
- Modulo: `%`, `%=`

```rust
let mut total = 100
total += 25  // total is now 125
total *= 2   // total is now 250
```

### Logical & Short-Circuit Operators
- `&&`: Short-circuit logical AND (if left is false, right is not evaluated)
- `||`: Short-circuit logical OR (if left is true, right is not evaluated)
- `!`: Logical NOT

```rust
if client_ip != "" && ip_in_cidr(client_ip, "10.0.0.0/8") {
    // Right operand only evaluated if client_ip is non-empty
}
```

### Comparison Operators
- `==`, `!=`, `<`, `<=`, `>`, `>=`

---

## 4. Functions & Closures

Functions support default arguments, explicit type annotations, and first-class higher-order passing:

```rust
// Standard function with default parameter
fn build_endpoint(base: String, port: Int = 8080) -> String {
    return base + ":" + to_string(port)
}

// Higher-order function / Closure
fn apply_twice(val: Int, operation) -> Int {
    return operation(operation(val))
}

let result = apply_twice(5, fn(x) { x + 10 }) // Returns 25
```

---

## 5. Control Flow & Jump Signals

### Conditional Statements
In VietLang, `if` is a statement:
```rust
let mut role = "guest"
if is_admin {
    role = "administrator"
} else if is_manager {
    role = "manager"
}
```

### While & For Loops
```rust
let mut i = 0
while i < 10 {
    i += 1
    if i == 5 { continue }
    if i == 8 { break }
}

for item in ["alpha", "beta", "gamma"] {
    println(item)
}
```

---

## 6. Error Handling

VietLang provides native `try / catch` blocks and `throw`:

```rust
try {
    if payload.len() == 0 {
        throw("Empty payload received")
    }
} catch err {
    println("Caught error: " + to_string(err))
}
```

---

## 7. Data Structures

### Structs
```rust
struct User {
    id: Int,
    name: String,
    email: String
}

let u = User {
    id: 101,
    name: "Trong",
    email: "trong@example.com"
}

let user_name = u.name
```

### Map Collections
```rust
let mut session = map_new()
session = map_set(session, "token", "jwt_xyz_789")
session = map_set(session, "user_id", 101)

if map_has(session, "token") {
    let token = map_get(session, "token")
}
```

---

## 8. Pattern Matching

Match expressions provide exhaustive branch dispatch:

```rust
let status_code = 200

let message = match status_code {
    200 => "OK",
    201 => "Created",
    400 => "Bad Request",
    401 => "Unauthorized",
    404 => "Not Found",
    500 => "Internal Server Error",
    _ => "Unknown Status"
}
```

---

## 9. Module System & Git Packages

Modules are resolved across 4 hierarchic paths:
1. Relative local directory (`./my_submodule.vl`)
2. Standard library directory (`std/*.vl`)
3. Community modules directory (`modules/*`)
4. Package entrypoint (`modules/*/src/main.vl`)

```rust
import std.security
import std.http_pipeline
import std.kv_store
```

---

## 10. Virtual Machine Execution (`--vm`)

VietLang includes a stack-based Bytecode Virtual Machine for high-throughput execution:

```bash
# Execute via Bytecode VM
vietlang --vm my_script.vl
```
