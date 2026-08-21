# Module `std.vietqr`

Module: std.vietqr

## Quickstart

```vietlang
import std.vietqr
```

## Exported Functions Reference

| Function Signature | Description |
| :--- | :--- |
| `fn vietqr_generate_url(bank_bin, account_no, amount, memo, template)` | Generate standard VietQR quick link URL for banking apps @param bank_bin - Bank BIN code (e.g., "970422" for MBBank, "970436" for Vietcombank) @param account_no - Beneficiary bank account number @param amount - Amount in VND (0 for open amount) @param memo - Transfer description @param template - QR template ("compact", "compact2", "qr_only", "print") @return Complete VietQR image URL |
| `fn vietqr_get_bank_bin(bank_code)` | Helper: Get Bank BIN code by common Bank abbreviation @param bank_code - Bank short name (e.g. "MB", "VCB", "TCB", "VPB", "ACB", "BIDV") @return 6-digit Napas Bank BIN code |
| `fn vietqr_create_payment(bank_code, account_no, amount, memo)` | Create a VietQR Transaction Map |

---

### Function Details

#### `fn vietqr_generate_url(bank_bin, account_no, amount, memo, template)`

Generate standard VietQR quick link URL for banking apps @param bank_bin - Bank BIN code (e.g., "970422" for MBBank, "970436" for Vietcombank) @param account_no - Beneficiary bank account number @param amount - Amount in VND (0 for open amount) @param memo - Transfer description @param template - QR template ("compact", "compact2", "qr_only", "print") @return Complete VietQR image URL

#### `fn vietqr_get_bank_bin(bank_code)`

Helper: Get Bank BIN code by common Bank abbreviation @param bank_code - Bank short name (e.g. "MB", "VCB", "TCB", "VPB", "ACB", "BIDV") @return 6-digit Napas Bank BIN code

#### `fn vietqr_create_payment(bank_code, account_no, amount, memo)`

Create a VietQR Transaction Map

