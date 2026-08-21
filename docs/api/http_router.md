# Module `std.http_router`

## Exported Functions

### `fn router_new()`

### `fn router_add_route(r, method: String, path: String, handler_name: String)`

### `fn router_use(r, middleware_name: String)`

### `fn http_serve_static(public_dir: String, req_path: String)`

### `fn response_json(status: Int, data, message: String = "OK") -> String`

### `fn response_error(status: Int, error_msg: String) -> String`

