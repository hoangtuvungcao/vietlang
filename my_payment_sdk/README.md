# my_payment_sdk — VietLang Community Payment Gateway

A high-performance community payment library for VietLang backend microservices.

## Features
- **VietQR Generation**: Generate compact VietQR image links for all 50+ Vietnamese banks (VCB, TCB, MB, etc.).
- **Cryptographic Signatures**: HMAC SHA256 signatures for transaction integrity.

## Quickstart
```vietlang
import my_payment_sdk.src.main

let client = payment_client_new("secret_key", "MERCHANT_101")
let qr = payment_generate_vietqr_url("VCB", "0123456789", 500000, "DH101")
```
