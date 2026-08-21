# Module `std.zalo`

Module: std.zalo

## Quickstart

```vietlang
import std.zalo
```

## Exported Functions Reference

| Function Signature | Description |
| :--- | :--- |
| `fn zalo_client(app_id, secret_key, access_token)` | Create Zalo OA Client |
| `fn zalo_create_zns_payload(phone, template_id, template_data, tracking_id)` | Create ZNS (Zalo Notification Service) Template Payload |
| `fn zalo_normalize_phone(phone)` | Helper: Normalize Vietnamese Phone Number for ZNS (e.g. "0901234567" -> "84901234567") |

---

### Function Details

#### `fn zalo_client(app_id, secret_key, access_token)`

Create Zalo OA Client

#### `fn zalo_create_zns_payload(phone, template_id, template_data, tracking_id)`

Create ZNS (Zalo Notification Service) Template Payload

#### `fn zalo_normalize_phone(phone)`

Helper: Normalize Vietnamese Phone Number for ZNS (e.g. "0901234567" -> "84901234567")

