# Hướng Dẫn Toàn Diện Tiện Ích VietLang Cho VS Code (VS Code Extension Guide)

Extension chính thức **VietLang** dành cho Visual Studio Code mang đến trải nghiệm lập trình Backend đỉnh cao: Tô màu cú pháp (Syntax Highlighting), Icon file `.vl` đặc trưng, nút chạy code 1-Click (1-Click Run Button), Kiểm tra lỗi cú pháp thời gian thực (Real-Time Diagnostics & Linter), và kho mẫu code thông minh (Fintech & Backend Snippets).

---

## 📥 1. Hướng Dẫn Cài Đặt (Installation)

### 🔹 Cách 1: Cài đặt trực tiếp từ file `.vsix` (Khuyên dùng ngay lập tức)

#### Bằng dòng lệnh (Terminal):
```bash
code --install-extension editors/vscode/vietlang-0.1.1.vsix
```

#### Bằng giao diện VS Code:
1. Mở VS Code, nhấn tổ hợp phím `Ctrl + Shift + X` (hoặc `Cmd + Shift + X` trên macOS) để mở bảng **Extensions**.
2. Nhấn vào biểu tượng **`...`** (Views and More Actions) ở góc trên bên phải của bảng Extensions.
3. Chọn **"Install from VSIX..."**.
4. Trỏ tới file [`editors/vscode/vietlang-0.1.1.vsix`](file:///home/vantrong/Downloads/new_lang/editors/vscode/vietlang-0.1.1.vsix) và nhấn **Install**.

---

### 🔹 Cách 2: Tìm kiếm và cài đặt trên VS Code Marketplace

1. Mở VS Code -> Nhấn `Ctrl + Shift + X`.
2. Trên thanh tìm kiếm, gõ: `VietLang` hoặc `hoangtuvungcao`.
3. Chọn extension **VietLang - Backend Programming Language** và nhấn **Install**.

---

## ✨ 2. Các Tính Năng Nổi Bật (Key Features)

### 🎨 1. Custom File Icon Theme (`.vl`)
- Tự động gán biểu tượng logo **VietLang màu xanh lục lục bảo** đặc trưng cho mọi file đuôi `.vl` trong cây thư mục dự án (File Explorer), giúp nhận diện file code chuyên nghiệp và đồng bộ.

### ▶ 2. Nút Chạy Code 1-Click (1-Click Run & Commands)
- **Nút Run (▶)**: Xuất hiện trực tiếp ở thanh công cụ góc trên bên phải của trình soạn thảo mỗi khi mở file `.vl`. Chỉ cần bấm 1 click để chạy ngay file hiện tại trong Terminal tích hợp!
- **Phím tắt**: Nhấn `Ctrl + Alt + V` (hoặc `Cmd + Alt + V` trên macOS) để chạy file lập tức.
- **Command Palette (`Ctrl + Shift + P`)**:
  - `VietLang: Run Active File` — Chạy file `.vl` hiện hành
  - `VietLang: Check Syntax (Linter)` — Kiểm tra lỗi cú pháp và AST
  - `VietLang: Build Standalone Binary` — Biên dịch ra file thực thi độc lập (AOT Binary)
  - `VietLang: Start Interactive REPL` — Mở REPL tương tác
  - `VietLang: Browse Standard Library Docs` — Tra cứu tài liệu 55 thư viện chuẩn

### 🔍 3. Kiểm Tra Lỗi Cú Pháp Thời Gian Thực (Real-Time Diagnostics)
- Tự động kích hoạt bộ phân tích cú pháp của VietLang khi bạn lưu file hoặc gõ code.
- Gạch chân đỏ cảnh báo lỗi chính xác theo dòng và cột (`Line:Column`), hiển thị chi tiết trong tab **Problems** (`Ctrl + Shift + M`) của VS Code.

### ⚡ 4. Kho Snippets Lập Trình Backend & Fintech Việt Nam
Chỉ cần gõ từ khóa và nhấn <kbd>Tab</kbd> hoặc <kbd>Enter</kbd>:

| Prefix Snippet | Mô Tả Chức Năng |
| :--- | :--- |
| **`vietqr`** | Tạo mã thanh toán VietQR Napas 247 cho 50+ ngân hàng |
| **`vnpay`** | Khởi tạo phiên thanh toán VNPay 2.1.0 kèm chữ ký HMAC-SHA512 |
| **`momo`** | Ký số giao dịch Ví MoMo chuẩn HMAC-SHA256 |
| **`zalo`** | Gửi tin nhắn chăm sóc khách hàng Zalo ZNS / OA |
| **`http-server`** | Khởi tạo máy chủ REST API HTTP/2 hiệu năng cao |
| **`ws-server`** | Tạo máy chủ WebSocket Real-time 2 chiều |
| **`sqlite-crud`** | Khởi tạo kết nối và truy vấn CSDL SQLite ACID |
| **`concurrency`** | Mẫu chạy đa luồng Green Threads `spawn` và CSP `channel` |
| **`jwt-auth`** | Tạo chữ ký và xác thực JWT Bearer Token |
| **`struct-def`** | Khai báo Struct dữ liệu với các trường kiểu |
| **`try-catch`** | Xử lý ngoại lệ có cấu trúc |

---

## 🛠️ 3. Hướng Dẫn Phát Hành Lên Marketplace (Publishing Guide)

Dành cho tác giả dự án muốn cập nhật bản phát hành mới lên VS Code Marketplace:

1. Cài đặt công cụ đóng gói VSCE:
   ```bash
   npm install -g @vscode/vsce
   ```
2. Đóng gói file `.vsix`:
   ```bash
   cd editors/vscode
   vsce package
   ```
3. Đăng tải lên Marketplace:
   ```bash
   vsce publish -p <YOUR_PERSONAL_ACCESS_TOKEN>
   ```
