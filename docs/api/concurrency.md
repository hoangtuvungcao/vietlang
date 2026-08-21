# Module `std.concurrency`

Module: std.concurrency

## Quickstart

```vietlang
import std.concurrency
```

## Exported Functions Reference

| Function Signature | Description |
| :--- | :--- |
| `fn worker_pool_new(size, worker_fn)` | Create and spawn a new Worker Pool @param size - Number of concurrent worker threads to spawn @param worker_fn - Task handler function: fn(job) -> result @return WorkerPool instance |
| `fn worker_pool_submit(pool: WorkerPool, job)` | Submit a job to the Worker Pool |
| `fn worker_pool_recv(pool: WorkerPool)` | Receive a completed result from the Worker Pool (blocking) |
| `fn worker_pool_try_recv(pool: WorkerPool)` | Non-blocking check for completed result |
| `fn worker_pool_shutdown(pool: WorkerPool)` | Gracefully shutdown the Worker Pool |
| `fn parallel_map(items, task_fn)` | Run an array of items in parallel across Worker Threads and collect all results |

---

### Function Details

#### `fn worker_pool_new(size, worker_fn)`

Create and spawn a new Worker Pool @param size - Number of concurrent worker threads to spawn @param worker_fn - Task handler function: fn(job) -> result @return WorkerPool instance

#### `fn worker_pool_submit(pool: WorkerPool, job)`

Submit a job to the Worker Pool

#### `fn worker_pool_recv(pool: WorkerPool)`

Receive a completed result from the Worker Pool (blocking)

#### `fn worker_pool_try_recv(pool: WorkerPool)`

Non-blocking check for completed result

#### `fn worker_pool_shutdown(pool: WorkerPool)`

Gracefully shutdown the Worker Pool

#### `fn parallel_map(items, task_fn)`

Run an array of items in parallel across Worker Threads and collect all results

