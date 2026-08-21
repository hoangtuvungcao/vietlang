# Module `std.security`

VietLang Security & Cryptography Shield (std.security)

## Quickstart

```vietlang
import std.security
```

## Exported Functions Reference

| Function Signature | Description |
| :--- | :--- |
| `fn security_hash_password(password: String, salt: String = "") -> String` | Function provided by module |
| `fn security_verify_password(password: String, stored_hash: String) -> Bool` | Function provided by module |
| `fn security_constant_time_compare(a: String, b: String) -> Bool` | Constant-time string comparison to prevent timing attacks |
| `fn security_generate_csrf_token() -> String` | Function provided by module |
| `fn security_sanitize_html(input: String) -> String` | Function provided by module |
| `fn security_hmac_sha256(message: String, secret_key: String) -> String` | Function provided by module |

---

### Function Details

#### `fn security_hash_password(password: String, salt: String = "") -> String`

Function provided by module

#### `fn security_verify_password(password: String, stored_hash: String) -> Bool`

Function provided by module

#### `fn security_constant_time_compare(a: String, b: String) -> Bool`

Constant-time string comparison to prevent timing attacks

#### `fn security_generate_csrf_token() -> String`

Function provided by module

#### `fn security_sanitize_html(input: String) -> String`

Function provided by module

#### `fn security_hmac_sha256(message: String, secret_key: String) -> String`

Function provided by module

