# VietLang Language Reference

## Table of Contents

1. [Types](#types)
2. [Variables](#variables)
3. [Operators](#operators)
4. [Functions](#functions)
5. [Control Flow](#control-flow)
6. [Data Structures](#data-structures)
7. [Pattern Matching](#pattern-matching)
8. [Error Handling](#error-handling)
9. [Modules & Imports](#modules--imports)
10. [Concurrency](#concurrency)
11. [Comments](#comments)

---

## Types

### Primitive Types

| Type | Description | Example |
|:---|:---|:---|
| `Int` | 64-bit integer | `42`, `-7`, `1_000_000` |
| `Float` | 64-bit float | `3.14`, `-0.5` |
| `String` | UTF-8 string | `"hello"`, `"line\n"` |
| `Bool` | Boolean | `true`, `false` |
| `None` | Null value | `none` |

### Composite Types

| Type | Description | Example |
|:---|:---|:---|
| `Array` | Dynamic array | `[1, 2, 3]` |
| `Struct` | Named fields | `User { name: "Trong" }` |
| `Enum` | Sum type | `Ok(42)`, `Err("fail")` |
| `Map` | Key-value map | `map_new()` |
| `Range` | Integer range | `1..10` |
| `Function` | First-class func | `fn(x) { x * 2 }` |

### Type Annotations

```rust
let x: Int = 42
let name: String = "hello"
let items: [Int] = [1, 2, 3]       // Array type
let maybe: ?String = none           // Nullable
let f: fn(Int, Int) -> Int = add    // Function type
```

---

## Variables

```rust
let name = "VietLang"       // Immutable (default)
let mut counter = 0         // Mutable
let pi: Float = 3.14159    // With type annotation

counter = counter + 1       // OK (mutable)
// name = "other"           // ERROR: cannot assign to immutable
```

---

## Operators

### Arithmetic
| Op | Description | Example |
|:---|:---|:---|
| `+` | Add / String concat | `1 + 2`, `"a" + "b"` |
| `-` | Subtract | `5 - 3` |
| `*` | Multiply / String repeat | `3 * 4`, `"ab" * 3` |
| `/` | Divide | `10 / 3` |
| `%` | Modulo | `10 % 3` |

### Comparison
| Op | Description |
|:---|:---|
| `==` | Equal |
| `!=` | Not equal |
| `<` | Less than |
| `>` | Greater than |
| `<=` | Less or equal |
| `>=` | Greater or equal |

### Logical
| Op | Description |
|:---|:---|
| `&&` | Logical AND |
| `\|\|` | Logical OR |
| `!` | Logical NOT |

### Other
| Op | Description | Example |
|:---|:---|:---|
| `..` | Range | `1..10` |
| `.` | Field access | `user.name` |
| `[]` | Index | `arr[0]` |
| `->` | Return type | `fn() -> Int` |
| `=>` | Match arm | `1 => "one"` |

---

## Functions

```rust
// Basic function
fn add(a: Int, b: Int) -> Int {
    return a + b
}

// No return type
fn greet(name: String) {
    println("Hello, " + name)
}

// Default parameters
fn connect(host: String, port: Int = 8080) {
    println("Connecting to " + host + ":" + to_string(port))
}

// Lambda / Anonymous function
let double = fn(x) { return x * 2 }

// Higher-order functions
let nums = [1, 2, 3, 4, 5]
let doubled = nums.map(fn(x) { return x * 2 })
let evens = nums.filter(fn(x) { return x % 2 == 0 })
let sum = nums.reduce(fn(acc, x) { return acc + x }, 0)

// Recursion
fn factorial(n: Int) -> Int {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}
```

---

## Control Flow

### If / Else

```rust
if condition {
    // ...
} else if other_condition {
    // ...
} else {
    // ...
}
```

### While Loop

```rust
let mut i = 0
while i < 10 {
    println(to_string(i))
    i = i + 1
}
```

### For Loop

```rust
// Range-based
for i in 0..10 {
    println(to_string(i))
}

// Array iteration
for item in items {
    process(item)
}

// String iteration
for ch in "hello".chars() {
    print(ch)
}
```

### Break & Continue

```rust
for i in 0..100 {
    if i % 2 == 0 { continue }
    if i > 20 { break }
    println(to_string(i))
}
```

---

## Data Structures

### Struct

```rust
struct User {
    name: String,
    email: String,
    age: Int
}

// Instantiation
let user = User {
    name: "Trong",
    email: "trong@vietlang.dev",
    age: 25
}

// Field access
println(user.name)
```

### Enum

```rust
enum Color {
    Red,
    Green,
    Blue,
    Custom(Int, Int, Int)
}

enum Result {
    Ok(value),
    Err(message)
}
```

### Impl (Methods)

```rust
impl User {
    pub fn display(self) {
        println(self.name + " <" + self.email + ">")
    }

    pub fn is_adult(self) -> Bool {
        return self.age >= 18
    }
}
```

### Map (HashMap)

```rust
let config = map_new()
let config = map_set(config, "host", "localhost")
let config = map_set(config, "port", 8080)

let host = map_get(config, "host")
let has_port = map_has(config, "port")
let keys = map_keys(config)
```

---

## Pattern Matching

```rust
let result = match value {
    0 => "zero",
    1 => "one",
    n => "number: " + to_string(n),    // variable binding
    _ => "unknown"                      // wildcard
}

// Enum matching
match result {
    Ok(value) => println("Success: " + to_string(value)),
    Err(msg) => println("Error: " + msg)
}
```

---

## Error Handling

```rust
// Result type pattern
enum Result {
    Ok(value),
    Err(message)
}

fn divide(a: Float, b: Float) -> Result {
    if b == 0.0 {
        return Err("Division by zero")
    }
    return Ok(a / b)
}

// Assertions
assert(1 + 1 == 2)
assert(name != "", "Name cannot be empty")
```

---

## Modules & Imports

```rust
import models.user
import services.auth
import std.http
```

---

## Concurrency

```rust
// Channels
let ch = channel(100)

// Spawn lightweight tasks
spawn(process_task)

// Thread-safe shared state
let counter = mutex_new(0)
```

---

## Comments

```rust
// Single-line comment

/* 
   Multi-line
   block comment
*/

/* Nested /* comments */ are supported */
```

---

## Built-in Functions

See [Standard Library Reference](stdlib-reference.md) for complete API documentation.
