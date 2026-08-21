# Module `std.vnpay`

Module: std.vnpay

## Quickstart

```vietlang
import std.vnpay
```

## Exported Functions Reference

| Function Signature | Description |
| :--- | :--- |
| `fn vnpay_client(tmn_code, hash_secret, payment_url)` | Create a new VNPay Client Configuration |
| `fn vnpay_create_payment_url(client, order_id, amount, order_info, return_url, client_ip)` | Create VNPay Payment URL @param client - VNPay client struct @param order_id - Merchant order ID @param amount - Amount in VND (auto-multiplied by 100 as per VNPay spec) @param order_info - Order description @param return_url - Merchant Return URL @param client_ip - Customer IP address @return Full VNPay Payment Redirection URL |
| `fn vnpay_verify_checksum(client, raw_query_without_hash, incoming_hash)` | Verify VNPay IPN or ReturnURL Query Checksum @param client - VNPay client struct @param raw_query_without_hash - Raw sorted query string without vnp_SecureHash @param incoming_hash - The vnp_SecureHash received from VNPay @return Boolean true if valid |

---

### Function Details

#### `fn vnpay_client(tmn_code, hash_secret, payment_url)`

Create a new VNPay Client Configuration

#### `fn vnpay_create_payment_url(client, order_id, amount, order_info, return_url, client_ip)`

Create VNPay Payment URL @param client - VNPay client struct @param order_id - Merchant order ID @param amount - Amount in VND (auto-multiplied by 100 as per VNPay spec) @param order_info - Order description @param return_url - Merchant Return URL @param client_ip - Customer IP address @return Full VNPay Payment Redirection URL

#### `fn vnpay_verify_checksum(client, raw_query_without_hash, incoming_hash)`

Verify VNPay IPN or ReturnURL Query Checksum @param client - VNPay client struct @param raw_query_without_hash - Raw sorted query string without vnp_SecureHash @param incoming_hash - The vnp_SecureHash received from VNPay @return Boolean true if valid

