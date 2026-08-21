# Module `std.momo`

Module: std.momo

## Quickstart

```vietlang
import std.momo
```

## Exported Functions Reference

| Function Signature | Description |
| :--- | :--- |
| `fn momo_client(partner_code, access_key, secret_key, endpoint)` | Create a new MoMo Client Configuration |
| `fn momo_create_payment_payload(client, order_id, amount, order_info, redirect_url, ipn_url, extra_data)` | Create MoMo Payment Payload with HMAC-SHA256 Signature |
| `fn momo_verify_ipn(client, raw_sig_string, incoming_signature)` | Verify MoMo Webhook IPN Signature |

---

### Function Details

#### `fn momo_client(partner_code, access_key, secret_key, endpoint)`

Create a new MoMo Client Configuration

#### `fn momo_create_payment_payload(client, order_id, amount, order_info, redirect_url, ipn_url, extra_data)`

Create MoMo Payment Payload with HMAC-SHA256 Signature

#### `fn momo_verify_ipn(client, raw_sig_string, incoming_signature)`

Verify MoMo Webhook IPN Signature

