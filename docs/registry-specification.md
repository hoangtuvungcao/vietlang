# VietLang Central Package Registry Specification & Sharded Index Architecture

Specification for the experimental VietLang community registry. The 0.3 CLI
requires semver resolution to an exact release, Ed25519-signed metadata,
SHA-256 verification, an immutable Git revision, atomic activation, and a
versioned lockfile. Legacy unsigned records are rejected rather than trusted.

Repository: [https://github.com/hoangtuvungcao/vietlang](https://github.com/hoangtuvungcao/vietlang)

---

## 1. Problem Statement & Architecture Goals

Traditional package managers suffer from two common pitfalls:
1. **Centralized Proprietary Silos**: Requires high-maintenance backend server infrastructure with API rate limits and vendor lock-in.
2. **Monolithic Index Bloat**: Storing all package metadata in a single flat file causes severe performance degradation when scaling to 100,000+ packages.

### Intended design:
- **Decentralized Source Hosting**: Package code is hosted directly on the author's own GitHub, GitLab, Gitea, or self-hosted Git server.
- **Git-Backed Sharded Prefix Index**: Metadata uses deterministic prefix paths.
- **Open Contribution for Any Developer**: Any developer with a GitHub account can publish without requiring admin permissions through automated Pull Request workflows or personal access tokens.

---

## 2. Sharded Prefix Tree Layout

Package metadata files are stored in a deterministic shard hierarchy based on the package name length:

| Name Length | Shard Path Format | Example Package Name | Shard Location |
|:---:|---|---|---|
| **1 char** | `registry/shards/1/{name}.json` | `a` | `registry/shards/1/a.json` |
| **2 chars** | `registry/shards/2/{name}.json` | `db` | `registry/shards/2/db.json` |
| **3 chars** | `registry/shards/3/{c1}/{name}.json` | `orm` | `registry/shards/3/o/orm.json` |
| **4+ chars** | `registry/shards/{c1c2}/{c3c4}/{name}.json` | `redis` | `registry/shards/re/di/redis.json` |
| **4+ chars** | `registry/shards/{c1c2}/{c3c4}/{name}.json` | `postgres` | `registry/shards/po/st/postgres.json` |
| **4+ chars** | `registry/shards/{c1c2}/{c3c4}/{name}.json` | `vietpay` | `registry/shards/vi/et/vietpay.json` |

### Benefits:
- **Small index lookups**: A client can fetch a package shard rather than a full catalog once remote shard fetching is implemented.
- **Filesystem Scalability**: Prevents directories from exceeding filesystem inode thresholds even with 500,000+ packages.
- **Fast In-Memory Search**: Search indexing scans sharded leaves in parallel with microsecond latency.

---

## 3. Package Manifest (`vietlang.json`) Specification

When authoring a package on your own GitHub account:

```json
{
  "name": "vietpay",
  "version": "1.0.0",
  "author": "your_github_username",
  "repository": "https://github.com/your_github_username/vietpay.git",
  "type": "lib",
  "description": "Example community library for VietLang",
  "keywords": ["payment", "fintech", "vietpay", "billing"],
  "license": "MIT",
  "dependencies": {
    "redis": "1.2.0"
  }
}
```

---

## 4. How Any Developer Publishes Their Module

### Step 1: Initialize and Develop
```bash
vietlang init my_package lib
cd my_package
# Write logic in src/main.vl and tests in tests/main_test.vl
```

### Step 2: Push Code to Your Personal GitHub Account
```bash
git init
git add -A
git commit -m "feat: initial v1.0.0 release"
git remote add origin https://github.com/your_username/my_package.git
git push -u origin main
```

### Step 3: Publish to Central Registry
```bash
vietlang publish
```

*What `vietlang publish` does automatically:*
1. Auto-detects your repository URL from `git remote get-url origin`.
2. Computes `sha256:<hex>` over `vietlang.json` and the complete `src/` tree.
3. Generates the deterministic shard file (e.g. `registry/shards/my/pa/my_package.json`).
4. Updates local index and outputs automated 1-click registration to the official registry.

---

## 5. Installing and Discovering Community Modules

```bash
# Search across all sharded community modules
vietlang search payment

# Install from Central Registry (by short name)
vietlang install vietpay

# Request an explicit version (recorded as an immutable lock)
vietlang install vietpay@1.0.0

# Update to latest version
vietlang update vietpay
```

The installer clones into an isolated staging directory, verifies metadata and
content before activation, then records the exact revision and digest in
`vietlang.lock`. Registry transparency/provenance and yanking policy remain
future ecosystem controls.
