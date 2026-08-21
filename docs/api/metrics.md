# Module `std.metrics`

## Exported Functions

### `fn metrics_registry_new(service_name: String = "vietlang_app")`

### `fn metrics_inc_counter(registry, name: String, delta: Float = 1.0)`

### `fn metrics_set_gauge(registry, name: String, value: Float)`

### `fn metrics_to_prometheus(registry) -> String`

Render metrics in standard Prometheus text format

