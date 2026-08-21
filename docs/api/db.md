# Module `std.db`

VietLang Unified Database Driver & Connection Manager (std.db)

## Quickstart

```vietlang
import std.db
```

## Exported Functions Reference

| Function Signature | Description |
| :--- | :--- |
| `fn db_connect(connection_url: String = "data/database.sqlite")` | Function provided by module |
| `fn db_exec(handle, sql: String)` | Function provided by module |
| `fn db_execute(handle, sql: String, params = [])` | Function provided by module |
| `fn db_query(handle, sql: String, params = [])` | Function provided by module |
| `fn db_begin(handle)` | Function provided by module |
| `fn db_commit(handle)` | Function provided by module |
| `fn db_rollback(handle)` | Function provided by module |
| `fn db_close(handle)` | Function provided by module |

---

### Function Details

#### `fn db_connect(connection_url: String = "data/database.sqlite")`

Function provided by module

#### `fn db_exec(handle, sql: String)`

Function provided by module

#### `fn db_execute(handle, sql: String, params = [])`

Function provided by module

#### `fn db_query(handle, sql: String, params = [])`

Function provided by module

#### `fn db_begin(handle)`

Function provided by module

#### `fn db_commit(handle)`

Function provided by module

#### `fn db_rollback(handle)`

Function provided by module

#### `fn db_close(handle)`

Function provided by module

