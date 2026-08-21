# Module `std.socket`

VietLang Low-Level Socket Library (std.socket)

## Quickstart

```vietlang
import std.socket
```

## Exported Functions Reference

| Function Signature | Description |
| :--- | :--- |
| `fn socket_tcp_send(host: String, port: Int, message: String, timeout_ms: Int = 3000) -> String` | Function provided by module |
| `fn socket_udp_send(host: String, port: Int, payload: String) -> Bool` | Function provided by module |
| `fn socket_ping(host: String, port: Int, timeout_ms: Int = 1000) -> Bool` | Function provided by module |

---

### Function Details

#### `fn socket_tcp_send(host: String, port: Int, message: String, timeout_ms: Int = 3000) -> String`

Function provided by module

#### `fn socket_udp_send(host: String, port: Int, payload: String) -> Bool`

Function provided by module

#### `fn socket_ping(host: String, port: Int, timeout_ms: Int = 1000) -> Bool`

Function provided by module

