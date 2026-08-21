# Module `std.db_mysql`

VietLang MySQL Enterprise Driver (std.db_mysql)

## Quickstart

```vietlang
import std.db_mysql
```

## Exported Functions Reference

| Function Signature | Description |
| :--- | :--- |
| `fn mysql_parse_dsn(dsn: String)` | Function provided by module |
| `fn mysql_connect_pool(dsn: String)` | Function provided by module |
| `fn mysql_exec(pool, sql_query: String)` | Function provided by module |
| `fn mysql_execute(pool, sql_query: String, params = [])` | Function provided by module |
| `fn mysql_query(pool, sql_query: String, params = [])` | Function provided by module |
| `fn mysql_begin_transaction(pool)` | Function provided by module |
| `fn mysql_commit(pool)` | Function provided by module |
| `fn mysql_rollback(pool)` | Function provided by module |
| `fn mysql_ping(pool) -> Bool` | Function provided by module |
| `fn mysql_close(pool)` | Function provided by module |

---

### Function Details

#### `fn mysql_parse_dsn(dsn: String)`

Function provided by module

#### `fn mysql_connect_pool(dsn: String)`

Function provided by module

#### `fn mysql_exec(pool, sql_query: String)`

Function provided by module

#### `fn mysql_execute(pool, sql_query: String, params = [])`

Function provided by module

#### `fn mysql_query(pool, sql_query: String, params = [])`

Function provided by module

#### `fn mysql_begin_transaction(pool)`

Function provided by module

#### `fn mysql_commit(pool)`

Function provided by module

#### `fn mysql_rollback(pool)`

Function provided by module

#### `fn mysql_ping(pool) -> Bool`

Function provided by module

#### `fn mysql_close(pool)`

Function provided by module

