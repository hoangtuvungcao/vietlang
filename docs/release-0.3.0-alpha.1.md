# VietLang v0.3.0-alpha.1 — Full Database Ecosystem & Production Engine

### 🚀 Điểm mới nổi bật trong VietLang v0.3.0-alpha.1

#### 1. 🗄️ Hệ sinh thái Database Drivers toàn diện (8 Hệ CSDL)
- **MongoDB** (`std.db_mongodb`): Async BSON CRUD, Aggregation pipeline, collection management.
- **Redis** (`std.db_redis`): Async connection pool, Strings, Lists, Sets, Hashes, Pub/Sub.
- **ClickHouse** (`std.db_clickhouse`): OLAP Analytics engine, timeseries aggregation, DDL & stats.
- **Cassandra / ScyllaDB** (`std.db_cassandra`): CQL cluster sessions, prepared queries, wide-column clustering.
- **Elasticsearch / OpenSearch** (`std.db_elasticsearch`): Full-text search DSL, index lifecycle & document indexing.
- **SQLite** (`std.db_sqlite`): Zero-config ACID relational database với immediate transaction migrations.
- **PostgreSQL** (`std.db_postgres`): Async SQLx connection pool & advisory migration lock.
- **MySQL / MariaDB** (`std.db_mysql`): Async SQLx connection pool với worker bounded management.

#### 2. ⚡ Core Engine & Runtime nâng cấp
- Typed Module Graph & IR Lowering.
- Generic `Option<T>` & `Result<T, E>` pattern matching.
- Asynchronous WebSocket RFC 6455 hub & streaming.
- Bounded HTTP/1.1 + HTTP/2 (ALPN TLS) & experimental HTTP/3 QUIC engine.
- Language Server Protocol (LSP) over stdio & Interactive Step Debugger.
- Deterministic Mutation Fuzzing engine.
- Ed25519-signed reproducible package management.

---

### 📦 Tải về & Cài đặt tự động

#### 🐧 Linux, 🍎 macOS & 📱 Termux
```bash
curl -fsSL https://raw.githubusercontent.com/hoangtuvungcao/vietlang/main/install.sh | bash
```

#### 🪟 Windows (PowerShell)
```powershell
iex (irm https://raw.githubusercontent.com/hoangtuvungcao/vietlang/main/install.ps1)
```
