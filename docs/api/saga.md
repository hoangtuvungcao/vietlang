# Module `std.saga`

VietLang Distributed SAGA Transaction Coordinator (std.saga)

## Quickstart

```vietlang
import std.saga
```

## Exported Functions Reference

| Function Signature | Description |
| :--- | :--- |
| `fn saga_new(transaction_name: String)` | Function provided by module |
| `fn saga_add_step(saga, step_name: String, forward_action: String, compensate_action: String)` | Function provided by module |
| `fn saga_record_step_success(saga, step_name: String)` | Function provided by module |
| `fn saga_compensate_all(saga)` | When a step fails, trigger compensating rollback for all executed steps in reverse order |

---

### Function Details

#### `fn saga_new(transaction_name: String)`

Function provided by module

#### `fn saga_add_step(saga, step_name: String, forward_action: String, compensate_action: String)`

Function provided by module

#### `fn saga_record_step_success(saga, step_name: String)`

Function provided by module

#### `fn saga_compensate_all(saga)`

When a step fails, trigger compensating rollback for all executed steps in reverse order

