# Module `std.db_mysql`

## Exported Functions

### `fn mysql_parse_dsn(dsn: String)`

### `fn mysql_connect_pool(dsn: String)`

### `fn mysql_exec(pool, sql_query: String)`

### `fn mysql_execute(pool, sql_query: String, params = [])`

### `fn mysql_query(pool, sql_query: String, params = [])`

### `fn mysql_begin_transaction(pool)`

### `fn mysql_commit(pool)`

### `fn mysql_rollback(pool)`

### `fn mysql_ping(pool) -> Bool`

### `fn mysql_close(pool)`

