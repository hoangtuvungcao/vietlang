# Module `std.sql_builder`

## Exported Functions

### `fn sql_query_new(base_table: String)`

### `fn sql_select(q, columns)`

### `fn sql_join(q, join_type: String, target_table: String, on_clause: String)`

### `fn sql_where(q, column: String, operator: String, value)`

### `fn sql_group_by(q, columns)`

### `fn sql_build(q) -> String`

