# Module `std.rate_limiter`

## Exported Functions

### `fn rate_limiter_new(max_requests: Int = 100, window_seconds: Int = 60)`

### `fn rate_limiter_whitelist(rl, ip: String)`

### `fn rate_limiter_is_whitelisted(rl, key: String) -> Bool`

### `fn rate_limiter_check(rl, key: String)`

