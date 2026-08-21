# Changelog

## 0.3.0-alpha.1 — 2026-08-22

- Added canonical module graph resolution, cycle rejection, cross-module
  semantic checks, and a stable typed declaration IR.
- Added built-in generic `Option<T>` / `Result<T,E>` semantics and exhaustive patterns.
- Added bounded async SQLx/Tokio/Rustls PostgreSQL and MySQL pools.
- Added SQLite transactional and server advisory migration locks.
- Replaced the production WebSocket path with bounded Axum/Tokio/Tungstenite.
- Added semver, SHA-256, Ed25519 metadata verification and `vietlang.lock`.
- Added formatter, LSP, doc generator, debugger, fuzzing, and load/soak tools.

This is an experimental prerelease, not an independent production certification.
