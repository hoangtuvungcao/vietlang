# Module `std.db_postgres`

VietLang PostgreSQL Driver & Connection Pool (std.db_postgres)

## Quickstart

```vietlang
import std.db_postgres
```

## Exported Functions Reference

| Function Signature | Description |
| :--- | :--- |
| `fn postgres_parse_url(url: String)` | Function provided by module |
| `fn postgres_connect(url: String)` | Function provided by module |
| `fn postgres_exec(client, sql_query: String)` | Function provided by module |
| `fn postgres_execute(client, sql_query: String, params = [])` | Function provided by module |
| `fn postgres_query(client, query_sql: String, params = [])` | Function provided by module |
| `fn postgres_begin_transaction(client)` | Function provided by module |
| `fn postgres_commit(client)` | Function provided by module |
| `fn postgres_rollback(client)` | Function provided by module |
| `fn postgres_is_healthy(client) -> Bool` | Function provided by module |
| `fn postgres_close(client)` | Function provided by module |

---

### Function Details

#### `fn postgres_parse_url(url: String)`

Function provided by module

#### `fn postgres_connect(url: String)`

Function provided by module

#### `fn postgres_exec(client, sql_query: String)`

Function provided by module

#### `fn postgres_execute(client, sql_query: String, params = [])`

Function provided by module

#### `fn postgres_query(client, query_sql: String, params = [])`

Function provided by module

#### `fn postgres_begin_transaction(client)`

Function provided by module

#### `fn postgres_commit(client)`

Function provided by module

#### `fn postgres_rollback(client)`

Function provided by module

#### `fn postgres_is_healthy(client) -> Bool`

Function provided by module

#### `fn postgres_close(client)`

Function provided by module

