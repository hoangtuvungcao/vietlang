# Module `std.jwt`

Legacy compatibility module, disabled by default. Signature checking alone is
not sufficient authentication: claim decoding and validation (`exp`, `nbf`,
`iss`, `aud`), algorithm policy, and key rotation are incomplete. Do not use
this module for new or production authentication. Install a reviewed community
package or implement an application-specific identity boundary.

## Quickstart

```vietlang
import std.jwt
```

## Exported Functions Reference

| Function Signature | Description |
| :--- | :--- |
| `fn jwt_create_header()` | Function provided by module |
| `fn jwt_sign(payload_map, secret: String) -> String` | Function provided by module |
| `fn jwt_verify(token: String, secret: String)` | Function provided by module |

---

### Function Details

#### `fn jwt_create_header()`

Function provided by module

#### `fn jwt_sign(payload_map, secret: String) -> String`

Function provided by module

#### `fn jwt_verify(token: String, secret: String)`

Function provided by module
