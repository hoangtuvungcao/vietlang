# Module `std.logger`

Module: std.logger

## Quickstart

```vietlang
import std.logger
```

## Exported Functions Reference

| Function Signature | Description |
| :--- | :--- |
| `fn logger_new(service_name: String = "vietlang_service", min_level: String = "INFO")` | Function provided by module |
| `fn logger_format_entry(l, level: String, message: String, extra_map) -> String` | Function provided by module |
| `fn logger_info(l, message: String, extra_map = none) -> String` | Function provided by module |
| `fn logger_warn(l, message: String, extra_map = none) -> String` | Function provided by module |
| `fn logger_error(l, message: String, error_msg: String = "", extra_map = none) -> String` | Function provided by module |
| `fn logger_debug(l, message: String, extra_map = none) -> String` | Function provided by module |

---

### Function Details

#### `fn logger_new(service_name: String = "vietlang_service", min_level: String = "INFO")`

Function provided by module

#### `fn logger_format_entry(l, level: String, message: String, extra_map) -> String`

Function provided by module

#### `fn logger_info(l, message: String, extra_map = none) -> String`

Function provided by module

#### `fn logger_warn(l, message: String, extra_map = none) -> String`

Function provided by module

#### `fn logger_error(l, message: String, error_msg: String = "", extra_map = none) -> String`

Function provided by module

#### `fn logger_debug(l, message: String, extra_map = none) -> String`

Function provided by module

