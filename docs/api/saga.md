# Module `std.saga`

## Exported Functions

### `fn saga_new(transaction_name: String)`

### `fn saga_add_step(saga, step_name: String, forward_action: String, compensate_action: String)`

### `fn saga_record_step_success(saga, step_name: String)`

### `fn saga_compensate_all(saga)`

When a step fails, trigger compensating rollback for all executed steps in reverse order

