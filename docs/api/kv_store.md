# Module `std.kv_store`

## Exported Functions

### `fn kv_store_new()`

### `fn kv_set(store, key: String, value, ttl_seconds: Int = 0)`

### `fn kv_get(store, key: String, default_val = none)`

### `fn kv_incr(store, key: String, delta: Int = 1)`

Atomic increment operation

### `fn kv_hset(store, hash_key: String, field: String, val)`

Hash operations (HSET / HGET)

### `fn kv_hget(store, hash_key: String, field: String, default_val = none)`

