# VietFinance Desktop Pro (Ứng Dụng Đa File / Standalone Desktop App)

Ứng dụng Desktop & Terminal tài chính tương tác cao, được xây dựng theo **Kiến trúc Đa File (Modular Clean Architecture)** bằng 100% ngôn ngữ **VietLang**.

## 🏗️ Cấu Trúc Mã Nguồn Đa File (Multi-File Architecture)

```text
apps/viet_finance_desktop/
├── vietlang.json                 # Manifest dự án & khối scripts
├── README.md                     # Tài liệu ứng dụng
├── data/
│   └── app.sqlite                # CSDL SQLite lưu trữ hóa đơn & lịch sử
└── src/
    ├── config/
    │   ├── theme.vl              # ANSI Colors, Styling Tokens, Screen formatting
    │   └── database.vl           # SQLite ACID Connection & Tables Auto-Migration
    ├── services/
    │   ├── vietqr_service.vl     # Xử lý tạo mã VietQR Napas 247 & ghi CSDL
    │   ├── crypto_service.vl     # Chữ ký HMAC-SHA256, HMAC-SHA512 & mã hóa đối xứng
    │   ├── benchmark_service.vl  # Đo lường hiệu năng 100 tác vụ song song trên Green Threads
    │   └── calculator_service.vl # Công cụ tính toán lãi suất kép đa chu kỳ
    ├── views/
    │   ├── banner_view.vl        # Giao diện Header & System Status
    │   ├── menu_view.vl          # Giao diện Menu điều hướng tương tác
    │   ├── qr_view.vl            # Khung hiển thị Hóa đơn ASCII VietQR
    │   └── ledger_view.vl        # Bảng ASCII hiển thị Sổ cái CSDL SQLite
    └── main.vl                   # Entrypoint điều phối trạng thái ứng dụng
```

## 🎮 Các Tính Năng Ứng Dụng

1. **💳 Tạo & Quản Lý VietQR Napas 247**: Nhập ngân hàng (MB, VCB, TCB, VPB, ACB,...), số tài khoản, số tiền và tự động lưu vào SQLite.
2. **⚡ Benchmark Đa Luồng CSP**: Phân bổ 100 tác vụ song song trên Worker Pool và tính toán TPS.
3. **📊 Sổ Cái SQLite ACID**: Hiển thị bảng ASCII các giao dịch đã lưu trên đĩa cứng.
4. **🔐 Trình Ký Số & Mã Hóa**: Tính toán HMAC-SHA256 / HMAC-SHA512 tức thì.
5. **📈 Máy Tính Lãi Suất Kép**: Bảng tiến trình tăng trưởng vốn đầu tư theo năm.

## 🚀 Hướng Dẫn Chạy & Đóng Gói Ứng Dụng

### 1. Chạy trong Terminal:
```bash
vietlang apps/viet_finance_desktop/src/main.vl
```

### 2. Mở trong Cửa Sổ Riêng (Gnome-Terminal / XTerm Window):
```bash
gnome-terminal --title="VietFinance Desktop Pro" -- bash -c "vietlang apps/viet_finance_desktop/src/main.vl; read -p 'Nhan Enter de thoat...'"
# Hoặc:
xterm -T "VietFinance Desktop Pro" -geometry 95x35 -e "vietlang apps/viet_finance_desktop/src/main.vl"
```

### 3. Đóng Gói Thành File Nhị Phân Độc Lập (.exe / Linux ELF Binary):
```bash
# Biên dịch file thực thi cho Linux:
vietlang build apps/viet_finance_desktop/src/main.vl -o viet_finance_app
./viet_finance_app

# Biên dịch file .exe cho Windows:
vietlang build apps/viet_finance_desktop/src/main.vl -o viet_finance_app.exe --target windows
```
