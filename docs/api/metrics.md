# Module `std.metrics`

VietLang Prometheus Metrics Exporter (std.metrics)

## Quickstart

```vietlang
import std.metrics
```

## Exported Functions Reference

| Function Signature | Description |
| :--- | :--- |
| `fn metrics_registry_new(service_name: String = "vietlang_app")` | Function provided by module |
| `fn metrics_inc_counter(registry, name: String, delta: Float = 1.0)` | Function provided by module |
| `fn metrics_set_gauge(registry, name: String, value: Float)` | Function provided by module |
| `fn metrics_to_prometheus(registry) -> String` | Render metrics in standard Prometheus text format |

---

### Function Details

#### `fn metrics_registry_new(service_name: String = "vietlang_app")`

Function provided by module

#### `fn metrics_inc_counter(registry, name: String, delta: Float = 1.0)`

Function provided by module

#### `fn metrics_set_gauge(registry, name: String, value: Float)`

Function provided by module

#### `fn metrics_to_prometheus(registry) -> String`

Render metrics in standard Prometheus text format

