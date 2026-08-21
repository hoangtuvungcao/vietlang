# Module `std.sql_builder`

VietLang Advanced SQL Query Builder (std.sql_builder)

## Quickstart

```vietlang
import std.sql_builder
```

## Exported Functions Reference

| Function Signature | Description |
| :--- | :--- |
| `fn sql_query_new(base_table: String)` | Function provided by module |
| `fn sql_select(q, columns)` | Function provided by module |
| `fn sql_join(q, join_type: String, target_table: String, on_clause: String)` | Function provided by module |
| `fn sql_where(q, column: String, operator: String, value)` | Function provided by module |
| `fn sql_group_by(q, columns)` | Function provided by module |
| `fn sql_build(q) -> String` | Function provided by module |

---

### Function Details

#### `fn sql_query_new(base_table: String)`

Function provided by module

#### `fn sql_select(q, columns)`

Function provided by module

#### `fn sql_join(q, join_type: String, target_table: String, on_clause: String)`

Function provided by module

#### `fn sql_where(q, column: String, operator: String, value)`

Function provided by module

#### `fn sql_group_by(q, columns)`

Function provided by module

#### `fn sql_build(q) -> String`

Function provided by module

