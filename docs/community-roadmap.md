# VietLang Community Expansion & Ecosystem Scaling Strategy

A strategic blueprint for growing VietLang into a global, community-driven backend programming language.

---

## 1. Pillars of Community Growth

To scale VietLang and outpace legacy languages, we focus on four core pillars:

1. **Zero Friction Onboarding**: 1-line installation, rich VS Code extension, full self-hosting, and beginner-friendly documentation.
2. **First-Class Package Ecosystem (`vpm`)**: Centralized package registry, automated dependency resolution, and semantic versioning.
3. **High-Performance Core**: Continuous optimization of the Bytecode VM and LLVM/Native backend compiler.
4. **Community RFC Process**: Open decision-making process for language features and standard library proposals.

---

## 2. Package Ecosystem Expansion (VPM Registry)

### How Developers Publish Modules
```bash
# 1. Initialize a community module
vietlang vpm.vl init my_awesome_logger lib

# 2. Develop and test with std.test
vietlang tests/main_test.vl

# 3. Publish to registry
vietlang vpm.vl publish
```

### Official Target Libraries for Community Bounties
- `vietlang-redis`: Redis client with cluster mode and pipeline support.
- `vietlang-postgres`: High-throughput PostgreSQL binary protocol driver.
- `vietlang-grpc`: gRPC client and protobuf code generator.
- `vietlang-graphql`: GraphQL schema parser and resolver engine.
- `vietlang-oauth2`: OAuth2 / OpenID Connect provider and client.
- `vietlang-openapi`: Automatic Swagger / OpenAPI 3.0 documentation generator.

---

## 3. The VietLang RFC (Request for Comments) Process

All major language changes, syntax additions, and core library proposals follow the RFC lifecycle:

1. **Idea & Discussion**: Open an issue on GitHub discussions.
2. **Drafting RFC**: Submit a Markdown PR to `rfcs/0000-feature-name.md`.
3. **Review Period**: Community feedback and core maintainer review.
4. **Implementation & Acceptance**: Merged into development branch with reference test suite.

---

## 4. Developer Outreach & Community Channels

- **Official GitHub**: [https://github.com/hoangtuvungcao/vietlang](https://github.com/hoangtuvungcao/vietlang)
- **Documentation & Issues**: [https://github.com/hoangtuvungcao/vietlang#readme](https://github.com/hoangtuvungcao/vietlang#readme)
- **VietLang Conference & Hackathons**: Annual backend engineering hackathons and community showcases.
