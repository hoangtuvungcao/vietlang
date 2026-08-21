# Module `std.http2`

Configuration helpers for the native `http_listen` runtime. The server uses
Hyper/Axum for HTTP/1.1 and HTTP/2. Configure `tls_cert_file` and
`tls_key_file` on the returned map to serve HTTPS and negotiate HTTP/2 with
ALPN. HTTP/3 is a separate opt-in QUIC listener and remains experimental.

## Quickstart

```vietlang
import std.http2
```

## Exported Functions Reference

| Function Signature | Description |
| :--- | :--- |
| `fn http2_server_config(port: Int = 9090, max_concurrent_streams: Int = 250)` | Creates a bounded HTTP/1.1 + HTTP/2 server config |
| `fn http2_create_stream(stream_id: Int, path: String)` | Function provided by module |
| `fn http2_push_promise(stream, resource_path: String, content_type: String)` | Function provided by module |

---

### Function Details

#### `fn http2_server_config(port: Int = 9090, max_concurrent_streams: Int = 250)`

Returns a map containing `port`, `max_concurrency`, body/header/response
limits, and a request timeout. Pass the map to `http_listen(config, handler)`.

#### `fn http2_create_stream(stream_id: Int, path: String)`

Function provided by module

#### `fn http2_push_promise(stream, resource_path: String, content_type: String)`

Function provided by module
