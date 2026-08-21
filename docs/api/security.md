# Module `std.security`

## Exported Functions

### `fn security_hash_password(password: String, salt: String = "") -> String`

### `fn security_verify_password(password: String, stored_hash: String) -> Bool`

### `fn security_constant_time_compare(a: String, b: String) -> Bool`

Constant-time string comparison to prevent timing attacks

### `fn security_generate_csrf_token() -> String`

### `fn security_sanitize_html(input: String) -> String`

