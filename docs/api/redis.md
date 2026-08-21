# Module `std.redis`

VietLang Standard Library — Native Redis Client (std.redis)

## Quickstart

```vietlang
import std.redis
```

## Exported Functions Reference

| Function Signature | Description |
| :--- | :--- |
| `fn redis_connect(host: String = "127.0.0.1", port: Int = 6379, auth: String = "")` | Function provided by module |
| `fn redis_command_format(args: Array) -> String` | Function provided by module |
| `fn redis_set(client, key: String, val, ttl_sec: Int = 0)` | Function provided by module |
| `fn redis_get(client, key: String)` | Function provided by module |
| `fn redis_del(client, key: String)` | Function provided by module |

---

### Function Details

#### `fn redis_connect(host: String = "127.0.0.1", port: Int = 6379, auth: String = "")`

Function provided by module

#### `fn redis_command_format(args: Array) -> String`

Function provided by module

#### `fn redis_set(client, key: String, val, ttl_sec: Int = 0)`

Function provided by module

#### `fn redis_get(client, key: String)`

Function provided by module

#### `fn redis_del(client, key: String)`

Function provided by module

