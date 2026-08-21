# VietFintech Multi-Channel Payment Gateway Service

High-Performance Pure Backend Payment Gateway & Microservice written in **VietLang**.

## 💳 Supported Payment & Notification Channels

- **VietQR (Napas 247)**: Dynamic QR image generation for 50+ Vietnamese commercial banks.
- **VNPay 2.1.0**: Payment URL generation, HMAC-SHA512 checksum calculation.
- **MoMo E-Wallet**: HMAC-SHA256 signature generation and payment payloads.
- **Zalo OA & ZNS**: Customer notification templates and phone normalization (`84...`).
- **Parallel Batch Settlements**: Worker pool concurrency executing batch transactions in parallel.
- **SQLite ACID Persistence**: Transaction storage, audit logs, and status tracking.

## 🚀 Quick Start

### 1. Run Development Server:
```bash
vietlang dev
```
Service runs on: `http://localhost:8888`

### 2. Run Automated Unit Tests:
```bash
vietlang test tests/api_test.vl
```

### 3. Compile to Standalone Native Binary:
```bash
# Build standalone Linux ELF binary
vietlang run build
./fintech_gateway_service

# Build standalone Windows executable
vietlang run build:win
```

## 📡 API Endpoints

| Method | Endpoint | Description |
|:---|:---|:---|
| `GET` | `/api/v1/health` | Service health probe & channel status |
| `POST` | `/api/v1/payments/vietqr` | Create Napas 247 VietQR transfer |
| `POST` | `/api/v1/payments/vnpay` | Create VNPay 2.1.0 checkout URL |
| `POST` | `/api/v1/payments/momo` | Create MoMo E-Wallet payment payload |
| `POST` | `/api/v1/notifications/zns` | Queue Zalo ZNS customer notification |
| `GET` | `/api/v1/transactions` | Query recent transactions from SQLite |
| `POST` | `/api/v1/batch/process` | Process batch transactions concurrently |
