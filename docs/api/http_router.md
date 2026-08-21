# Module `std.http_router`

VietLang HTTP Router & Web Framework (std.http_router)

## Quickstart

```vietlang
import std.http_router
```

## Exported Functions Reference

| Function Signature | Description |
| :--- | :--- |
| `fn router_new()` | Function provided by module |
| `fn router_add_route(r, method: String, path: String, handler_name: String)` | Function provided by module |
| `fn router_use(r, middleware_name: String)` | Function provided by module |
| `fn http_serve_static(public_dir: String, req_path: String)` | Function provided by module |
| `fn response_json(status: Int, data, message: String = "OK") -> String` | Function provided by module |
| `fn response_error(status: Int, error_msg: String) -> String` | Function provided by module |

---

### Function Details

#### `fn router_new()`

Function provided by module

#### `fn router_add_route(r, method: String, path: String, handler_name: String)`

Function provided by module

#### `fn router_use(r, middleware_name: String)`

Function provided by module

#### `fn http_serve_static(public_dir: String, req_path: String)`

Function provided by module

#### `fn response_json(status: Int, data, message: String = "OK") -> String`

Function provided by module

#### `fn response_error(status: Int, error_msg: String) -> String`

Function provided by module

