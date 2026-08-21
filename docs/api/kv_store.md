# Module `std.kv_store`

VietLang In-Memory Key-Value & Redis Engine (std.kv_store)

## Quickstart

```vietlang
import std.kv_store
```

## Exported Functions Reference

| Function Signature | Description |
| :--- | :--- |
| `fn kv_store_new()` | Function provided by module |
| `fn kv_set(store, key: String, value, ttl_seconds: Int = 0)` | Function provided by module |
| `fn kv_get(store, key: String, default_val = none)` | Function provided by module |
| `fn kv_incr(store, key: String, delta: Int = 1)` | Atomic increment operation |
| `fn kv_hset(store, hash_key: String, field: String, val)` | Hash operations (HSET / HGET) |
| `fn kv_hget(store, hash_key: String, field: String, default_val = none)` | Function provided by module |

---

### Function Details

#### `fn kv_store_new()`

Function provided by module

#### `fn kv_set(store, key: String, value, ttl_seconds: Int = 0)`

Function provided by module

#### `fn kv_get(store, key: String, default_val = none)`

Function provided by module

#### `fn kv_incr(store, key: String, delta: Int = 1)`

Atomic increment operation

#### `fn kv_hset(store, hash_key: String, field: String, val)`

Hash operations (HSET / HGET)

#### `fn kv_hget(store, hash_key: String, field: String, default_val = none)`

Function provided by module

