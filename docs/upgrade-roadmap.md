# VietLang Upgrade Roadmap

VietLang 0.3.0-alpha.1 is an experimental language/runtime. This roadmap separates
correctness and safety work from feature expansion. A phase is complete only
when all of its acceptance criteria are automated in CI.

## P0 — Safety and truthful capabilities

Status: implemented locally; pending independent review.

### Cryptography and authentication

- [x] Replace the function named `sha256` with SHA-256 from a maintained Rust crate.
- [x] Replace HMAC helpers with RFC-compliant HMAC-SHA256 and HMAC-SHA512.
- [x] Replace time-seeded UUID/random helpers with the operating system CSPRNG.
- [x] Replace reversible XOR secret storage with Argon2id key derivation and
      authenticated AES-256-GCM envelopes.
- [x] Replace fast password hashing with Argon2id PHC strings.
- [x] Correct legacy JWT encoding/signature primitives and disable `std.jwt` by
      default until claim validation and conformance are complete.
- [x] Keep provider-specific payment behavior outside core security conformance;
      payment integrations belong in application/community packages.
- [x] Add NIST/RFC and interoperability vectors plus an end-to-end VietLang smoke test.
- [x] Replace package publishing's `DefaultHasher` checksum with deterministic SHA-256 over the manifest and complete `src/` tree.
- [ ] Obtain an independent cryptographic review.

Acceptance criteria: Rust tests and `tests/p0_security.vl` pass; altered
ciphertext and wrong passwords are rejected. Production claims
remain disabled until independent review finishes.

### P0 migration notes

This security correction intentionally breaks compatibility with unsafe legacy
formats:

- Existing XOR `encrypt_secret` values cannot be decrypted by the new version.
  Decrypt them once with the old isolated runtime, then re-encrypt with the new
  AES-256-GCM envelope; never expose the old runtime to untrusted input.
- Existing `salt$sha256` password records do not verify as Argon2id. On a
  successful legacy login, verify in an isolated migration path and immediately
  replace the record with `password_hash()` output. Force-reset all remaining accounts.
- Tokens produced by the old non-standard JWT code must be revoked and reissued.
- Old `vlt_...`/placeholder package checksums are untrusted metadata. Republish
  packages to produce `sha256:...`; install-time enforcement remains P3 work.

### Network and VM correctness

- [x] Replace the plaintext client with pooled Reqwest/Rustls HTTP/HTTPS and HTTP/2 support.
- [x] Make unsupported VM statements and expressions fail compilation with source positions.
- [x] Replace the HTTP client/server path with maintained Reqwest, Hyper, Axum,
      Tokio, Rustls, Quinn, and h3 libraries.
- [x] Add bounded concurrency, body/header/response limits, request timeout,
      graceful shutdown, secure response headers, TLS, and request isolation.
- [x] Add CI protocol smoke tests, async WebSocket handshake/frame coverage,
      bounded HTTP load, deterministic mutation fuzzing, and a soak harness.
- [ ] Complete an externally operated multi-hour failure-injection campaign.
- [x] Add interpreter-versus-VM differential tests for the currently shared,
      documented compiler subset; every newly shared construct must extend it.

Acceptance criteria: no TLS URL can leave through plaintext TCP, and no AST node
can be silently discarded by the VM compiler.

### Documentation

- [x] Mark prerelease versions experimental.
- [x] Describe `vietlang build` as a standalone source bundle, not native AOT compilation.
- [x] Describe `spawn` as OS-thread-based rather than a green-thread scheduler.
- [x] Keep production claims disabled and document only implemented protocol capabilities.

## P1 — Language correctness

Target outcome: a written language specification backed by a semantic analyzer.

1. Expand the initial descriptive draft into a stable specification for scopes,
   mutability, evaluation order, equality, numeric conversion,
   errors, modules, closures, structs, enums, `match`, `Option`, and `Result`.
2. Introduce a resolved/typed IR between AST and execution backends.
3. Check variable declarations, assignments, function arity, argument types,
   return types, struct fields, nullable values, and match exhaustiveness.
4. Extend completed lexical capture tests with cancellation and concurrent
   state-protocol tests as the concurrency model evolves.
5. Make `vietlang check` run semantic and type checking, not only parsing.
6. Build a shared conformance suite that must produce equivalent interpreter
   and VM results or the same explicit unsupported-feature diagnostic.

Exit criteria: no advertised type-safety feature is enforced only at runtime;
language semantics have executable tests and versioned documentation.

Implemented initial analyzer: `vietlang check`, interpreter execution, VM entry,
standalone bundling, imported modules, and the REPL now run semantic checks.
The analyzer covers lexical scopes, mutability, local annotations, function
arity/defaults/returns, exact local struct fields, method signatures and `self`,
enum constructors/pattern payloads, and Bool/enum match exhaustiveness. Runtime
arity checks remain defense in depth. The 0.3 frontend adds a canonical module
graph, cycle rejection, typed IR, and generic substitution for built-in
`Option`/`Result`. Lexical
closure capture, recursion, mutable sibling captures, typed/inferred lambda
returns, and a differential suite for the current VM subset are implemented.

## P2 — Backend runtime

Target outcome: a bounded, observable, protocol-conformant service runtime.

1. [x] Replace the production WebSocket path with Axum/Tokio/Tungstenite and
   bounded broadcast backpressure.
2. Complete cancellation propagation for timed-out handlers and database calls.
3. Add connection/read/write timeouts plus protocol conformance tests;
   request timeout, body/header/response limits, graceful shutdown, and
   overload rejection are implemented.
4. [x] Add bounded SQLx PostgreSQL/MySQL pools, parameter binding, acquisition
   timeouts, health checks, explicit close, and SQLite/server migration locks.
   Multi-statement callback transaction ergonomics remain future language API work.
5. [x] Fuzz lexer, parser, JSON, HTTP configuration, and manifests with a
   deterministic CI mutation harness; malformed bytecode has panic-free tests.
6. Publish reproducible latency, throughput, memory, and concurrency benchmarks.

Exit criteria: protocol suites, fuzzing, soak tests, and failure-injection tests
run in CI; performance statements link to reproducible benchmark artifacts.

## P3 — Package ecosystem and developer tooling

Target outcome: reproducible builds and a usable daily development workflow.

1. [x] Add a lockfile with immutable source revisions and cryptographic content hashes.
2. Add dependency resolution, signed registry metadata, provenance, yanking, and
   a vulnerability/advisory process.
3. [x] Ship a formatter, LSP, documentation generator, top-level source debugger, and stable
   machine-readable diagnostics.
4. Continue self-hosting through semantic analysis and bytecode generation only
   after the Rust reference implementation has conformance coverage.

Exit criteria: identical inputs and lockfiles resolve to identical verified
dependency trees; editor tools consume the same compiler diagnostics as CI.

The installer rejects unsigned/placeholder metadata, resolves semver ranges to
an exact release, verifies Ed25519 metadata and package SHA-256 before atomic
activation, and records the Git revision in `vietlang.lock`. Registry
yanking/provenance transparency and an independent ecosystem audit remain open.
