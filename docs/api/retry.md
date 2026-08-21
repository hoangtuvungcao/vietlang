# Module `std.retry`

VietLang Retry Policy & Exponential Backoff (std.retry)

## Quickstart

```vietlang
import std.retry
```

## Exported Functions Reference

| Function Signature | Description |
| :--- | :--- |
| `fn retry_policy_new(max_attempts: Int = 3, initial_delay_ms: Int = 100, backoff_factor: Float = 2.0)` | Function provided by module |
| `fn retry_calculate_delay_ms(policy, attempt: Int) -> Int` | Function provided by module |

---

### Function Details

#### `fn retry_policy_new(max_attempts: Int = 3, initial_delay_ms: Int = 100, backoff_factor: Float = 2.0)`

Function provided by module

#### `fn retry_calculate_delay_ms(policy, attempt: Int) -> Int`

Function provided by module

