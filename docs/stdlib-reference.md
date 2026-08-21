# VietLang Standard Library Reference

## Core Functions

| Function | Args | Description |
|:---|:---|:---|
| `print(args...)` | variadic | Print without newline |
| `println(args...)` | variadic | Print with newline |
| `len(value)` | 1 | Length of string/array |
| `type_of(value)` | 1 | Get type name as string |
| `to_string(value)` | 1 | Convert to string |
| `to_int(value)` | 1 | Convert to integer |
| `to_float(value)` | 1 | Convert to float |
| `push(array, value)` | 2 | Return new array with value appended |
| `pop(array)` | 1 | Return last element |
| `input(prompt)` | 1 | Read user input |
| `abs(number)` | 1 | Absolute value |
| `min(a, b)` | 2 | Minimum of two numbers |
| `max(a, b)` | 2 | Maximum of two numbers |
| `assert(cond, [msg])` | 1-2 | Assert condition is true |
| `exit([code])` | 0-1 | Exit program |
| `format(template, args...)` | 1+ | String formatting |
| `range(start, end, [step])` | 2-3 | Generate array of integers |

---

## std.io — File I/O

| Function | Args | Description |
|:---|:---|:---|
| `file_read(path)` | 1 | Read file contents as string |
| `file_write(path, content)` | 2 | Write string to file |
| `file_append(path, content)` | 2 | Append to file |
| `file_exists(path)` | 1 | Check if file exists |
| `file_delete(path)` | 1 | Delete a file |
| `dir_list(path)` | 1 | List directory contents |
| `dir_create(path)` | 1 | Create directory (recursive) |

### Examples

```rust
// Write and read
file_write("data.txt", "Hello, VietLang!")
let content = file_read("data.txt")

// Append
file_append("log.txt", "[INFO] Server started\n")

// Check existence
if file_exists("config.json") {
    let config = json_parse(file_read("config.json"))
}

// Directory operations
dir_create("project/src")
let files = dir_list("project")
```

---

## std.json — JSON Processing

| Function | Args | Description |
|:---|:---|:---|
| `json_parse(string)` | 1 | Parse JSON string to value |
| `json_stringify(value, [pretty])` | 1-2 | Convert value to JSON string |

### Examples

```rust
// Parse
let data = json_parse("{\"name\": \"VietLang\", \"version\": \"0.1.0\"}")
let name = map_get(data, "name")

// Stringify
let obj = map_set(map_set(map_new(), "status", "ok"), "code", 200)
let json = json_stringify(obj, true)   // Pretty print
let compact = json_stringify(obj)       // Compact
```

---

## std.http — HTTP Server

| Function | Args | Description |
|:---|:---|:---|
| `http_listen(port, handler)` | 2 | Start HTTP server |

### Examples

```rust
// Start a JSON API server
http_listen(8080, handler)
```

The server automatically:
- Parses HTTP requests (method, path, headers, body)
- Returns JSON responses
- Logs all requests
- Handles concurrent connections

---

## std.db — Database

| Function | Args | Description |
|:---|:---|:---|
| `db_query(sql, [params...])` | 1+ | Execute raw SQL query |
| `db_table(name)` | 1 | Create query builder |

### Examples

```rust
// Raw SQL
db_query("SELECT * FROM users WHERE active = true")
db_query("INSERT INTO users (name, email) VALUES ('Trong', 'trong@vl.dev')")

// Query builder
let q = db_table("users")
```

---

## std.crypto — Security

| Function | Args | Description |
|:---|:---|:---|
| `sha256(input)` | 1 | SHA-256 hash |
| `uuid()` | 0 | Generate UUID v4 |
| `base64_encode(input)` | 1 | Base64 encode |
| `random_int(min, max)` | 2 | Random integer |

### Examples

```rust
let hash = sha256("password123")
let id = uuid()                      // "a1b2c3d4-..."
let token = base64_encode("user:pass")
let roll = random_int(1, 6)
```

---

## std.env — Environment

| Function | Args | Description |
|:---|:---|:---|
| `env_get(key, [default])` | 1-2 | Get environment variable |
| `env_set(key, value)` | 2 | Set environment variable |
| `env_all()` | 0 | Get all env vars as Map |

### Examples

```rust
let port = env_get("PORT", "8080")
env_set("APP_MODE", "production")
```

---

## std.time — Time & Date

| Function | Args | Description |
|:---|:---|:---|
| `time_now()` | 0 | Current Unix timestamp (seconds) |
| `time_now_ms()` | 0 | Current timestamp (milliseconds) |
| `sleep(ms)` | 1 | Sleep for milliseconds |
| `timer_start()` | 0 | Start a timer |

### Examples

```rust
let start = time_now_ms()
// ... do work ...
let elapsed = time_now_ms() - start
println("Took " + to_string(elapsed) + "ms")

sleep(1000)  // Sleep 1 second
```

---

## std.log — Structured Logging

| Function | Args | Description |
|:---|:---|:---|
| `log_debug(args...)` | variadic | Debug level log |
| `log_info(args...)` | variadic | Info level log |
| `log_warn(args...)` | variadic | Warning level log |
| `log_error(args...)` | variadic | Error level log |

Output format: `[LEVEL] [timestamp] message`

### Examples

```rust
log_info("Server started on port", 8080)
log_warn("High memory usage:", "85%")
log_error("Database connection failed")
log_debug("Request payload:", json_stringify(data))
```

---

## std.collections — Map (HashMap)

| Function | Args | Description |
|:---|:---|:---|
| `map_new()` | 0 | Create empty map |
| `map_set(map, key, value)` | 3 | Set key-value, returns new map |
| `map_get(map, key, [default])` | 2-3 | Get value by key |
| `map_has(map, key)` | 2 | Check if key exists |
| `map_keys(map)` | 1 | Get all keys as array |
| `map_values(map)` | 1 | Get all values as array |
| `map_remove(map, key)` | 2 | Remove key, returns new map |

### Examples

```rust
let config = map_new()
let config = map_set(config, "host", "localhost")
let config = map_set(config, "port", 8080)

let host = map_get(config, "host")           // "localhost"
let missing = map_get(config, "db", "none")  // "none"
let has_host = map_has(config, "host")       // true
let keys = map_keys(config)                   // ["host", "port"]
```

---

## Concurrency

| Function | Args | Description |
|:---|:---|:---|
| `spawn(task)` | 1 | Spawn a lightweight task |
| `channel([buffer])` | 0-1 | Create a channel |
| `mutex_new([value])` | 0-1 | Create a mutex |

### Examples

```rust
let ch = channel(100)
spawn(worker_function)
let lock = mutex_new(0)
```

---

## String Methods

Called on string values with dot notation:

| Method | Description | Example |
|:---|:---|:---|
| `.len()` | String length | `"hello".len()` → `5` |
| `.contains(sub)` | Check substring | `"hello".contains("ell")` → `true` |
| `.split(sep)` | Split by separator | `"a,b,c".split(",")` → `["a","b","c"]` |
| `.trim()` | Remove whitespace | `" hi ".trim()` → `"hi"` |
| `.to_upper()` | Uppercase | `"hi".to_upper()` → `"HI"` |
| `.to_lower()` | Lowercase | `"HI".to_lower()` → `"hi"` |
| `.starts_with(s)` | Check prefix | `"hello".starts_with("he")` → `true` |
| `.ends_with(s)` | Check suffix | `"hello".ends_with("lo")` → `true` |
| `.replace(a, b)` | Replace substring | `"hello".replace("l","r")` → `"herro"` |
| `.chars()` | To char array | `"hi".chars()` → `["h","i"]` |

---

## Array Methods

| Method | Description | Example |
|:---|:---|:---|
| `.len()` | Array length | `[1,2,3].len()` → `3` |
| `.is_empty()` | Check empty | `[].is_empty()` → `true` |
| `.first()` | First element | `[1,2,3].first()` → `1` |
| `.last()` | Last element | `[1,2,3].last()` → `3` |
| `.contains(x)` | Check membership | `[1,2,3].contains(2)` → `true` |
| `.join(sep)` | Join to string | `["a","b"].join(",")` → `"a,b"` |
| `.reversed()` | Reverse array | `[1,2,3].reversed()` → `[3,2,1]` |
| `.map(fn)` | Transform each | `[1,2].map(fn(x){return x*2})` → `[2,4]` |
| `.filter(fn)` | Filter elements | `[1,2,3].filter(fn(x){return x>1})` → `[2,3]` |
| `.reduce(fn, init)` | Reduce to value | `[1,2,3].reduce(fn(a,x){return a+x},0)` → `6` |
