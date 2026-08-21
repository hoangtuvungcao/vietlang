# Module `std.http2`

VietLang HTTP/2 & High-Performance Transport Layer (std.http2)

## Quickstart

```vietlang
import std.http2
```

## Exported Functions Reference

| Function Signature | Description |
| :--- | :--- |
| `fn http2_server_config(port: Int = 9090, max_concurrent_streams: Int = 250)` | Function provided by module |
| `fn http2_create_stream(stream_id: Int, path: String)` | Function provided by module |
| `fn http2_push_promise(stream, resource_path: String, content_type: String)` | Function provided by module |

---

### Function Details

#### `fn http2_server_config(port: Int = 9090, max_concurrent_streams: Int = 250)`

Function provided by module

#### `fn http2_create_stream(stream_id: Int, path: String)`

Function provided by module

#### `fn http2_push_promise(stream, resource_path: String, content_type: String)`

Function provided by module

