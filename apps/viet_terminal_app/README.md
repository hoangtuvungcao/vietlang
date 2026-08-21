# VietFintech Terminal Pro (Ứng Dụng Console / Desktop)

Ứng dụng Terminal TUI tương tác trực tiếp bằng bàn phím, được viết 100% bằng ngôn ngữ **VietLang** (Zero Web, Zero Browser, Pure Standalone App).

## 🎮 Tính Năng Ứng Dụng

1. **💳 Máy Tạo & Quét VietQR 24/7 (Napas 247)**:
   - Tương tác nhập Ngân Hàng (MB, VCB, TCB, VPB, ACB,...), Số tài khoản, Số tiền, Nội dung.
   - Tự động lưu lịch sử vào CSDL SQLite.
   - Hiển thị hóa đơn ASCII Box chuẩn EMVCo.
2. **⚡ Bộ Đo Hiệu Năng Đa Luồng CSP (Benchmark Concurrency)**:
   - Phân bổ hàng trăm tác vụ song song trên Green Threads.
   - Đo lường tổng thời gian (microseconds) và tốc độ throughput (Tasks/sec).
3. **📊 Sổ Cái Giao Dịch SQLite ACID**:
   - Truy vấn và hiển thị bảng ASCII dữ liệu giao dịch lưu trữ trực tiếp trên đĩa cứng.
4. **🔐 Công Cụ Ký Số & Mã Hóa Bảo Mật**:
   - Tính toán nhanh chữ ký HMAC-SHA256, HMAC-SHA512, SHA-256 Digest và mã hóa đối xứng.
5. **📈 Máy Tính Lãi Suất Kép & Dự Phóng Đầu Tư**:
   - Tính toán bảng tăng trưởng tài sản theo năm với lãi suất kép.

## 🚀 Hướng Dẫn Chạy Ứng Dụng

### 1. Chạy trực tiếp từ mã nguồn:
```bash
vietlang apps/viet_terminal_app/main.vl
```

### 2. Biên dịch thành File Thực Thi Độc Lập (.exe / Linux ELF Binary):
```bash
# Biên dịch cho Linux:
vietlang build apps/viet_terminal_app/main.vl -o viet_fintech_terminal
./viet_fintech_terminal

# Biên dịch cho Windows (.exe):
vietlang build apps/viet_terminal_app/main.vl -o viet_fintech_terminal.exe --target windows
```
