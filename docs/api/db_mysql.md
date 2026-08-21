# Module `std.db_mysql` — disabled compatibility API

Native MySQL operations are disabled in VietLang 0.2.0-alpha.2. The previous
synchronous driver depended on a RustSec-unsound cache release. Every connection,
query, transaction, and close operation now fails explicitly; no fake pool or
successful result is returned. Use SQLite or a separately reviewed adapter until
the async core migration is complete.

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
