# Module `std.rpc`

## Exported Functions

### `fn rpc_server_new()`

### `fn rpc_register(server, procedure_name: String, handler_fn_name: String)`

### `fn rpc_create_request(procedure: String, params) -> String`

### `fn rpc_create_response(req_id: String, result) -> String`

### `fn rpc_create_error(req_id: String, code: Int, message: String) -> String`

