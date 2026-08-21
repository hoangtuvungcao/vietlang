# Module `std.rate_limiter`

Module: std.rate_limiter

## Quickstart

```vietlang
import std.rate_limiter
```

## Exported Functions Reference

| Function Signature | Description |
| :--- | :--- |
| `fn rate_limiter_new(max_requests: Int = 100, window_seconds: Int = 60)` | Khoi tao bo gioi han tan suat yeu cau (Rate Limiter) @param max_requests: Int - So luong request toi da cho phep trong 1 cua so thoi gian @param window_seconds: Int - Do dai cua so thoi gian (giay) @return Map - Rate limiter instance |
| `fn rate_limiter_whitelist(rl, ip: String)` | Them dia chi IP hoac API key vao danh sach trang bo qua rate limit @param rl: Map - Rate limiter instance @param ip: String - Dia chi IP hoac API key @return Map - Rate limiter instance cap nhat |
| `fn rate_limiter_is_whitelisted(rl, key: String) -> Bool` | Kiem tra xem key co nam trong danh sach trang khong @param rl: Map - Rate limiter instance @param key: String - IP can kiem tra @return Bool - True neu duoc whitelist |
| `fn rate_limiter_check(rl, key: String)` | Kiem tra va ghi nhan request theo thuat toan Sliding-Window @param rl: Map - Rate limiter instance @param key: String - IP hoac Token can kiem tra @return Map - Ket qua { allowed: Bool, remaining: Int, limiter: Map } |

---

### Function Details

#### `fn rate_limiter_new(max_requests: Int = 100, window_seconds: Int = 60)`

Khoi tao bo gioi han tan suat yeu cau (Rate Limiter) @param max_requests: Int - So luong request toi da cho phep trong 1 cua so thoi gian @param window_seconds: Int - Do dai cua so thoi gian (giay) @return Map - Rate limiter instance

#### `fn rate_limiter_whitelist(rl, ip: String)`

Them dia chi IP hoac API key vao danh sach trang bo qua rate limit @param rl: Map - Rate limiter instance @param ip: String - Dia chi IP hoac API key @return Map - Rate limiter instance cap nhat

#### `fn rate_limiter_is_whitelisted(rl, key: String) -> Bool`

Kiem tra xem key co nam trong danh sach trang khong @param rl: Map - Rate limiter instance @param key: String - IP can kiem tra @return Bool - True neu duoc whitelist

#### `fn rate_limiter_check(rl, key: String)`

Kiem tra va ghi nhan request theo thuat toan Sliding-Window @param rl: Map - Rate limiter instance @param key: String - IP hoac Token can kiem tra @return Map - Ket qua { allowed: Bool, remaining: Int, limiter: Map }

