# VietLang Security Policy

VietLang is an experimental prerelease. Version `0.3.0-alpha.1` adds automated
RustSec auditing, standards-based cryptography, bounded network runtimes, signed
package metadata enforcement, deterministic fuzzing, and supply-chain locks.
It has **not** received an independent security audit and must not be described
as zero-bug or certified for high-risk authentication/payment workloads.

## Supported versions

Only the newest tagged prerelease receives security fixes. Legacy JWT, MoMo,
and VNPay modules remain disabled. Unsigned registry records are rejected.

## Reporting a vulnerability

Do not open a public issue for an exploitable vulnerability. Use GitHub's
private vulnerability reporting feature for this repository and include the
affected version, reproducer, impact, preconditions, and proposed test/fix.

Maintainers should acknowledge within 72 hours, reproduce in isolation, add a
regression test and advisory, and coordinate a release before disclosure. Never
include production credentials or personal data.

## Independent audit gate

An external reviewer must assess cryptography, HTTP/TLS/QUIC, WebSockets,
SQL/migrations, parser/VM behavior, and package signing/lockfiles. Audit reports
and remediation commits must be linked from a release before the experimental
warning can be removed.
