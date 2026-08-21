# Module `std.cron`

VietLang Enterprise Job Scheduler & Cron Engine (std.cron)

## Quickstart

```vietlang
import std.cron
```

## Exported Functions Reference

| Function Signature | Description |
| :--- | :--- |
| `fn cron_scheduler_new()` | Function provided by module |
| `fn cron_add_interval_job(scheduler, job_id: String, interval_seconds: Int, task_action: String)` | Function provided by module |
| `fn cron_should_run_job(job) -> Bool` | Function provided by module |
| `fn cron_trigger_job(scheduler, job_id: String)` | Function provided by module |

---

### Function Details

#### `fn cron_scheduler_new()`

Function provided by module

#### `fn cron_add_interval_job(scheduler, job_id: String, interval_seconds: Int, task_action: String)`

Function provided by module

#### `fn cron_should_run_job(job) -> Bool`

Function provided by module

#### `fn cron_trigger_job(scheduler, job_id: String)`

Function provided by module

