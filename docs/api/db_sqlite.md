# Module `std.db_sqlite`

## Exported Functions

### `fn sqlite_open(db_path: String = "data/database.sqlite")`

### `fn sqlite_exec(conn, sql: String)`

### `fn sqlite_execute(conn, sql: String, params = [])`

### `fn sqlite_query(conn, sql_or_table: String, params_or_filter = [])`

### `fn sqlite_close(conn)`

### `fn sqlite_begin_transaction(conn)`

### `fn sqlite_commit(conn)`

### `fn sqlite_rollback(conn)`

### `fn sqlite_insert(conn, table_name: String, row_data)`

### `fn sqlite_update(conn, table_name: String, predicate_fn_or_id, update_fn_or_map)`

### `fn sqlite_delete(conn, table_name: String, predicate_fn_or_id)`

### `fn sqlite_find_by_id(conn, table_name: String, id)`

### `fn sqlite_count(conn, table_name: String, where_clause: String = "")`

### `fn sqlite_aggregate(conn, table_name: String, initial_val, acc_fn)`

