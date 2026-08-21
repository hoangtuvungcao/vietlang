# Module `std.db_sqlite`

VietLang Production-Grade SQLite Driver & Storage Engine (std.db_sqlite)

## Quickstart

```vietlang
import std.db_sqlite
```

## Exported Functions Reference

| Function Signature | Description |
| :--- | :--- |
| `fn sqlite_open(db_path: String = "data/database.sqlite")` | Function provided by module |
| `fn sqlite_exec(conn, sql: String)` | Function provided by module |
| `fn sqlite_execute(conn, sql: String, params = [])` | Function provided by module |
| `fn sqlite_query(conn, sql_or_table: String, params_or_filter = [])` | Function provided by module |
| `fn sqlite_close(conn)` | Function provided by module |
| `fn sqlite_begin_transaction(conn)` | Function provided by module |
| `fn sqlite_commit(conn)` | Function provided by module |
| `fn sqlite_rollback(conn)` | Function provided by module |
| `fn sqlite_insert(conn, table_name: String, row_data)` | Function provided by module |
| `fn sqlite_update(conn, table_name: String, predicate_fn_or_id, update_fn_or_map)` | Function provided by module |
| `fn sqlite_delete(conn, table_name: String, predicate_fn_or_id)` | Function provided by module |
| `fn sqlite_find_by_id(conn, table_name: String, id)` | Function provided by module |
| `fn sqlite_count(conn, table_name: String, where_clause: String = "")` | Function provided by module |
| `fn sqlite_aggregate(conn, table_name: String, initial_val, acc_fn)` | Function provided by module |

---

### Function Details

#### `fn sqlite_open(db_path: String = "data/database.sqlite")`

Function provided by module

#### `fn sqlite_exec(conn, sql: String)`

Function provided by module

#### `fn sqlite_execute(conn, sql: String, params = [])`

Function provided by module

#### `fn sqlite_query(conn, sql_or_table: String, params_or_filter = [])`

Function provided by module

#### `fn sqlite_close(conn)`

Function provided by module

#### `fn sqlite_begin_transaction(conn)`

Function provided by module

#### `fn sqlite_commit(conn)`

Function provided by module

#### `fn sqlite_rollback(conn)`

Function provided by module

#### `fn sqlite_insert(conn, table_name: String, row_data)`

Function provided by module

#### `fn sqlite_update(conn, table_name: String, predicate_fn_or_id, update_fn_or_map)`

Function provided by module

#### `fn sqlite_delete(conn, table_name: String, predicate_fn_or_id)`

Function provided by module

#### `fn sqlite_find_by_id(conn, table_name: String, id)`

Function provided by module

#### `fn sqlite_count(conn, table_name: String, where_clause: String = "")`

Function provided by module

#### `fn sqlite_aggregate(conn, table_name: String, initial_val, acc_fn)`

Function provided by module

