# Module `std.rpc`

VietLang Microservice RPC Framework (std.rpc)

## Quickstart

```vietlang
import std.rpc
```

## Exported Functions Reference

| Function Signature | Description |
| :--- | :--- |
| `fn rpc_server_new()` | Function provided by module |
| `fn rpc_register(server, procedure_name: String, handler_fn_name: String)` | Function provided by module |
| `fn rpc_create_request(procedure: String, params) -> String` | Function provided by module |
| `fn rpc_create_response(req_id: String, result) -> String` | Function provided by module |
| `fn rpc_create_error(req_id: String, code: Int, message: String) -> String` | Function provided by module |

---

### Function Details

#### `fn rpc_server_new()`

Function provided by module

#### `fn rpc_register(server, procedure_name: String, handler_fn_name: String)`

Function provided by module

#### `fn rpc_create_request(procedure: String, params) -> String`

Function provided by module

#### `fn rpc_create_response(req_id: String, result) -> String`

Function provided by module

#### `fn rpc_create_error(req_id: String, code: Int, message: String) -> String`

Function provided by module

