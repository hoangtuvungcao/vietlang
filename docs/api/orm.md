# Module `std.orm`

VietLang Database Query Builder & ORM (std.orm)

## Quickstart

```vietlang
import std.orm
```

## Exported Functions Reference

| Function Signature | Description |
| :--- | :--- |
| `fn query_builder_new(table_name: String)` | Function provided by module |
| `fn qb_select(q, cols)` | Function provided by module |
| `fn qb_where(q, field: String, op: String, val)` | Function provided by module |
| `fn qb_order_by(q, field: String, dir: String = "ASC")` | Function provided by module |
| `fn qb_limit(q, n: Int)` | Function provided by module |
| `fn qb_to_sql(q) -> String` | Function provided by module |
| `fn qb_insert_sql(table_name: String, data_map) -> String` | Function provided by module |

---

### Function Details

#### `fn query_builder_new(table_name: String)`

Function provided by module

#### `fn qb_select(q, cols)`

Function provided by module

#### `fn qb_where(q, field: String, op: String, val)`

Function provided by module

#### `fn qb_order_by(q, field: String, dir: String = "ASC")`

Function provided by module

#### `fn qb_limit(q, n: Int)`

Function provided by module

#### `fn qb_to_sql(q) -> String`

Function provided by module

#### `fn qb_insert_sql(table_name: String, data_map) -> String`

Function provided by module

