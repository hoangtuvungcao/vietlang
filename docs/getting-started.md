# Getting Started with VietLang

## Installation

### Option 1: Download Binary (Recommended)

Download the latest release for your platform from [GitHub Releases](https://github.com/hoangtuvungcao/vietlang/releases).

| Platform | Download |
|:---|:---|
| Linux x86_64 | `vietlang-linux-x64` |
| Linux ARM64 | `vietlang-linux-arm64` |
| macOS x86_64 | `vietlang-macos-x64` |
| macOS ARM64 (M1/M2) | `vietlang-macos-arm64` |
| Windows x86_64 | `vietlang-windows-x64.exe` |

```bash
# Linux / macOS
chmod +x vietlang-linux-x64
sudo mv vietlang-linux-x64 /usr/local/bin/vietlang
```

### Option 2: Build from Source

Requirements: **Rust 1.70+**

```bash
git clone https://github.com/hoangtuvungcao/vietlang.git
cd vietlang
cargo build --release
sudo cp target/release/vietlang /usr/local/bin/
```

### Verify Installation

```bash
vietlang --version
# VietLang v0.1.0
```

---

## Your First Program

Create a file called `hello.vl`:

```rust
println("Hello, VietLang! ")
```

Run it:

```bash
vietlang hello.vl
# Output: Hello, VietLang! 
```

---

## Tutorial

### Step 1: Variables

```rust
// Immutable by default
let name = "VietLang"
let version = 0.1
let is_awesome = true

// Mutable with 'mut'
let mut counter = 0
counter = counter + 1

println(name + " v" + to_string(version))
```

### Step 2: Functions

```rust
fn greet(name: String) -> String {
    return "Hello, " + name + "! "
}

fn add(a: Int, b: Int) -> Int {
    return a + b
}

println(greet("World"))
println("2 + 3 = " + to_string(add(2, 3)))
```

### Step 3: Control Flow

```rust
// If/Else
let score = 85
if score >= 90 {
    println("Grade: A")
} else if score >= 80 {
    println("Grade: B")
} else {
    println("Grade: C")
}

// For loop
for i in 1..6 {
    println("Count: " + to_string(i))
}

// While loop
let mut n = 5
while n > 0 {
    println("Countdown: " + to_string(n))
    n = n - 1
}
```

### Step 4: Arrays & Higher-Order Functions

```rust
let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

let doubled = numbers.map(fn(x) { return x * 2 })
let evens = numbers.filter(fn(x) { return x % 2 == 0 })
let sum = numbers.reduce(fn(acc, x) { return acc + x }, 0)

println("Doubled: " + to_string(doubled))
println("Evens: " + to_string(evens))
println("Sum: " + to_string(sum))
```

### Step 5: Structs

```rust
struct User {
    name: String,
    email: String,
    age: Int
}

let user = User { name: "Trong", email: "trong@example.com", age: 25 }
println("User: " + user.name + " (" + to_string(user.age) + ")")
```

### Step 6: Pattern Matching

```rust
fn describe(n: Int) -> String {
    return match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => "many"
    }
}

for i in 0..5 {
    println(to_string(i) + " is " + describe(i))
}
```

### Step 7: Backend Features

```rust
// JSON
let data = map_new()
let data = map_set(data, "message", "Hello from VietLang!")
let data = map_set(data, "status", 200)
println(json_stringify(data, true))

// File I/O
file_write("output.json", json_stringify(data, true))
let content = file_read("output.json")
println("Read back: " + content)

// Crypto
let hash = sha256("my-password")
let id = uuid()
println("Hash: " + hash)
println("UUID: " + id)

// Logging
log_info("Application started")
log_warn("High memory usage")
```

---

## Using the REPL

Start the interactive REPL:

```bash
$ vietlang

vl:1 > let x = 42
vl:2 > x * 2
  = 84
vl:3 > let arr = [1, 2, 3, 4, 5]
vl:4 > arr.map(fn(x) { return x * x })
  = [1, 4, 9, 16, 25]
vl:5 > exit
Goodbye! 
```

**REPL Commands:**
- `help` — Show help
- `clear` — Clear screen
- `exit` — Exit REPL

---

## Debug Tools

```bash
# Show tokens
vietlang --tokens myfile.vl

# Show AST
vietlang --ast myfile.vl
```

---

## Next Steps

-  Read the [Language Reference](language-reference.md)
-  Browse the [Standard Library](stdlib-reference.md)
-  Check out [examples/](../examples/) for more demos
-  [Contribute](../CONTRIBUTING.md) to VietLang
