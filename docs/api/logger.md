# Module `std.logger`

## Exported Functions

### `fn logger_new(service_name: String = "vietlang_service", min_level: String = "INFO")`

### `fn logger_format_entry(l, level: String, message: String, extra_map) -> String`

### `fn logger_info(l, message: String, extra_map = none) -> String`

### `fn logger_warn(l, message: String, extra_map = none) -> String`

### `fn logger_error(l, message: String, error_msg: String = "", extra_map = none) -> String`

### `fn logger_debug(l, message: String, extra_map = none) -> String`

