# VietLang Central Package Registry Specification & Sharded Index Architecture

The formal specification of the VietLang Community Registry: A Git-backed, sharded prefix-tree package indexing system designed for infinite scalability, zero-server cost, and open community participation.

Repository: [https://github.com/hoangtuvungcao/vietlang](https://github.com/hoangtuvungcao/vietlang)

---

## 1. Problem Statement & Architecture Goals

Traditional package managers suffer from two common pitfalls:
1. **Centralized Proprietary Silos**: Requires high-maintenance backend server infrastructure with API rate limits and vendor lock-in.
2. **Monolithic Index Bloat**: Storing all package metadata in a single flat file causes severe performance degradation when scaling to 100,000+ packages.

### The VietLang Solution:
- **Decentralized Source Hosting**: Package code is hosted directly on the author's own GitHub, GitLab, Gitea, or self-hosted Git server.
- **Git-Backed Sharded Prefix Index**: Metadata is sharded into deterministic prefix paths (similar to Cargo and Crates.io), ensuring $O(1)$ package lookups with tiny network payloads (<1KB per package).
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
- **Zero Monolithic Bloat**: Installing `redis` only downloads a 300-byte file (`registry/shards/re/di/redis.json`), never the entire catalog.
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
  "description": "High-throughput payment gateway SDK for VietLang",
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
2. Computes the release checksum (`vlt_<sha256>`).
3. Generates the deterministic shard file (e.g. `registry/shards/my/pa/my_package.json`).
4. Updates local index and outputs automated 1-click registration to the official registry.

---

## 5. Installing and Discovering Community Modules

```bash
# Search across all sharded community modules
vietlang search payment

# Install from Central Registry (by short name)
vietlang install vietpay

# Install with explicit version lock
vietlang install vietpay@1.0.0

# Update to latest version
vietlang update vietpay
```
