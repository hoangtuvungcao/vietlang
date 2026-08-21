# VietLang 0.3.0-alpha.1

This prerelease implements typed module graph/IR, generic `Option`/`Result`,
real async SQL pools, async WebSockets, migration locks, verified/reproducible
packages, formatter, LSP, documentation generator, debugger, fuzzing, and
load/soak tooling.

## Verification gates

- Rust tests, Clippy with warnings denied, and RustSec audit;
- P0 cryptographic vectors and HTTP/1.1 load smoke;
- HTTP/2/TLS, HTTP/3/QUIC, and async RFC 6455 tests;
- deterministic malformed-input mutation campaign;
- Linux, Windows and macOS artifacts plus VSIX packaging.

Unsigned legacy packages now fail closed. Registry signatures cover exactly
`name\nversion\nsource\nchecksum`, where checksum is lowercase SHA-256 hex.

The release remains experimental. An external audit and independently operated
extended failure-injection/soak campaign cannot be self-certified by the
implementer and remain gates before a stable production declaration.
