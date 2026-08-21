# my_test_service

Ứng dụng Enterprise Full-Stack được xây dựng bằng **VietLang**.

## 🚀 Hướng Dẫn Chạy Dự Án

### 1. Khởi động Máy Chủ Phát Triển (Dev Server):
```bash
vietlang dev
```
Mở trình duyệt tại: [http://localhost:8080](http://localhost:8080)

### 2. Biên dịch Thành File Nhị Phân Độc Lập (Standalone Binary):
```bash
# Biên dịch cho Linux
vietlang run build
./my_test_service_app

# Biên dịch cho Windows (.exe)
vietlang run build:win
```

### 3. Chạy Kiểm Thử (Unit Tests):
```bash
vietlang test
```
