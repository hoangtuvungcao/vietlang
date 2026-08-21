# VietLang Community Module Development & Distribution Guide

Learn how to build, test, package, and distribute reusable libraries and modules in the VietLang ecosystem.

---

## 1. Package Structure

Every standard VietLang package follows this directory layout:

```
my_module/
├── vietlang.json         # Package manifest
├── src/
│   ├── main.vl           # Primary entry point
│   └── lib.vl            # Internal library logic
├── tests/
│   └── main_test.vl      # Test suite
└── README.md             # Documentation
```

---

## 2. Package Manifest (`vietlang.json`)

```json
{
  "name": "my_module",
  "version": "0.1.0",
  "description": "Custom backend utility module for VietLang",
  "main": "src/main.vl",
  "dependencies": {},
  "license": "MIT"
}
```

---

## 3. Creating a Module

Use the VietLang package manager (`vpm.vl`):

```bash
vietlang vpm.vl init my_module
```

---

## 4. Writing Reusable Functions

In `src/main.vl`:

```rust
fn format_currency(amount: Float, symbol: String = "VND") -> String {
    return to_string(amount) + " " + symbol
}

fn calculate_discount(price: Float, percent: Float) -> Float {
    return price * (1.0 - percent / 100.0)
}
```

---

## 5. Testing Your Module

In `tests/main_test.vl`:

```rust
import std.test
import my_module

suite("Currency Module Tests")

test("Format currency with default symbol", fn() {
    let result = format_currency(50000.0)
    assert_eq(result, "50000 VND")
})

test_summary()
```

Run tests:
```bash
vietlang tests/main_test.vl
```

---

## 6. Using Your Module in Other Projects

Place the module folder into the `modules/` directory of any VietLang project:

```rust
import modules.my_module.src.main
```
Or import directly if installed via `vpm`.
