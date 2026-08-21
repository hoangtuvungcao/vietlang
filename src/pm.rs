//! VietLang Central Community Registry Subsystem
//! High-throughput, sharded prefix-tree package indexing & decentralized distribution.

#![allow(dead_code)]

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use fs2::FileExt;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageVersionEntry {
    pub version: String,
    pub description: String,
    pub author: String,
    pub source: String,
    pub checksum: String,
    #[serde(default)]
    pub signature: String,
    #[serde(default)]
    pub public_key: String,
    #[serde(default)]
    pub keywords: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageIndexEntry {
    pub name: String,
    pub latest: String,
    pub versions: HashMap<String, PackageVersionEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct Lockfile {
    lockfile_version: u32,
    packages: std::collections::BTreeMap<String, LockedPackage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LockedPackage {
    version: String,
    source: String,
    revision: String,
    checksum: String,
}

pub fn handle_vpm_command(args: &[String]) {
    if args.is_empty() {
        print_pm_help();
        return;
    }

    match args[0].as_str() {
        "install" | "add" => {
            if args.len() < 2 {
                eprintln!("\x1b[31mError:\x1b[0m Package name required.\nExample: vietlang install redis@1.2.0");
                return;
            }
            install_package(&args[1]);
        }
        "update" => {
            let target = if args.len() >= 2 {
                Some(args[1].as_str())
            } else {
                None
            };
            update_package(target);
        }
        "remove" | "uninstall" => {
            if args.len() < 2 {
                eprintln!(
                    "\x1b[31mError:\x1b[0m Package name required.\nExample: vietlang remove redis"
                );
                return;
            }
            remove_package(&args[1]);
        }
        "search" => {
            let query = if args.len() >= 2 {
                args[1].as_str()
            } else {
                ""
            };
            search_registry(query);
        }
        "publish" => publish_package(),
        "sync" | "registry" => sync_registry(),
        "init" | "new" | "create" => {
            let name = if args.len() >= 2 {
                args[1].as_str()
            } else {
                "my_app"
            };
            let tmpl = if args.len() >= 3 {
                args[2].as_str()
            } else {
                "api"
            };
            init_project(name, tmpl);
        }
        "list" | "ls" => list_installed(),
        "verify" => verify_project(),
        "docs" => {
            if args.len() < 2 {
                eprintln!("\x1b[31mError:\x1b[0m Module name required.\nExample: vietlang docs std.db_sqlite");
                return;
            }
            show_docs(&args[1]);
        }
        "info" => show_info(),
        "help" | "--help" | "-h" => print_pm_help(),
        other => {
            eprintln!("\x1b[31mError:\x1b[0m Unknown package command '{}'", other);
            print_pm_help();
        }
    }
}

fn print_pm_help() {
    println!("\x1b[36mVietLang Central Package Manager & Community Registry\x1b[0m");
    println!("Official Sharded Community Package Catalog & Developer Toolchain");
    println!("================================================================");
    println!("USAGE:");
    println!("  vietlang install <pkg[@version]>    Install package from Central Registry (e.g. redis@1.2.0)");
    println!("  vietlang update [pkg[@version]]     Update installed package(s) to target/latest version");
    println!("  vietlang remove <pkg>               Remove an installed package");
    println!("  vietlang search <query>             Search Central Registry by name, keywords, or author");
    println!("  vietlang publish                    Publish your library from personal GitHub to Central Registry");
    println!("  vietlang sync                       Sync local registry index with remote community catalog");
    println!(
        "  vietlang init <name> [template]     Initialize new project (lib | api | microservice)"
    );
    println!("  vietlang list                       List installed dependencies");
    println!("  vietlang docs <module>              Inspect module exported function signatures");
    println!("  vietlang verify                     Verify project syntax & test suite");
    println!();
    println!("EXAMPLES:");
    println!("  vietlang install redis              Install latest Redis from Central Registry");
    println!("  vietlang install redis@1.2.0        Install exact version 1.2.0");
    println!(
        "  vietlang search postgres            Search for PostgreSQL modules in Central Registry"
    );
    println!(
        "  vietlang publish                    Publish current module to the Central Registry"
    );
    println!("  vietlang sync                       Sync sharded package index");
    println!();
}

pub fn get_shard_relpath(name: &str) -> String {
    let lower = name.to_lowercase();
    let chars: Vec<char> = lower.chars().collect();
    match chars.len() {
        0 => "shards/unknown.json".to_string(),
        1 => format!("shards/1/{}.json", lower),
        2 => format!("shards/2/{}.json", lower),
        3 => format!("shards/3/{}/{}.json", chars[0], lower),
        _ => {
            let p1: String = chars[0..2].iter().collect();
            let p2: String = chars[2..4].iter().collect();
            format!("shards/{}/{}/{}.json", p1, p2, lower)
        }
    }
}

fn get_registry_base() -> String {
    let candidates = ["registry", "../registry", "../../registry"];
    for path in &candidates {
        if Path::new(path).exists() {
            return path.to_string();
        }
    }
    if let Ok(home) = std::env::var("HOME") {
        let global = format!("{}/.vietlang/registry", home);
        let _ = fs::create_dir_all(&global);
        return global;
    }
    "registry".to_string()
}

fn sync_registry() {
    println!("\x1b[36mSyncing with VietLang Central Community Registry...\x1b[0m");
    let base = get_registry_base();
    let shards_dir = format!("{}/shards", base);
    let _ = fs::create_dir_all(&shards_dir);

    // Verify git connectivity
    let res = Command::new("git")
        .args([
            "ls-remote",
            "https://github.com/hoangtuvungcao/vietlang.git",
            "refs/heads/main",
        ])
        .output();

    if let Ok(out) = res {
        if out.status.success() {
            println!("\x1b[32mRegistry connection successful.\x1b[0m Synced with official repository index.");
        } else {
            println!("Operating in cached local registry mode.");
        }
    } else {
        println!("Operating in local registry mode.");
    }
    println!("Registry Index Path: {}", base);
}

fn load_registry() -> HashMap<String, PackageIndexEntry> {
    let mut map = HashMap::new();

    // Standard baseline catalog (30+ official modules & drivers)
    let default_entries = [
        (
            "redis",
            "1.2.0",
            "High-performance Redis client & Pub/Sub broker",
            "https://github.com/hoangtuvungcao/vietlang_redis.git",
            "hoangtuvungcao",
        ),
        (
            "postgres",
            "2.1.0",
            "Experimental PostgreSQL adapter with connection pool helpers",
            "https://github.com/hoangtuvungcao/vietlang_postgres.git",
            "hoangtuvungcao",
        ),
        (
            "mysql",
            "1.1.0",
            "MySQL protocol driver with connection pool & prepared queries",
            "https://github.com/hoangtuvungcao/vietlang_mysql.git",
            "hoangtuvungcao",
        ),
        (
            "sqlite",
            "1.0.0",
            "ACID compliant SQLite relational storage engine",
            "https://github.com/hoangtuvungcao/vietlang_sqlite.git",
            "hoangtuvungcao",
        ),
        (
            "auth",
            "3.0.0",
            "OAuth2, JWT RBAC security, session & password hashing",
            "https://github.com/hoangtuvungcao/vietlang_auth.git",
            "hoangtuvungcao",
        ),
        (
            "mailer",
            "1.0.1",
            "SMTP Email client with template rendering",
            "https://github.com/hoangtuvungcao/vietlang_mailer.git",
            "hoangtuvungcao",
        ),
        (
            "graphql",
            "0.9.0",
            "GraphQL schema parser, query executor & resolvers",
            "https://github.com/hoangtuvungcao/vietlang_graphql.git",
            "hoangtuvungcao",
        ),
        (
            "grpc",
            "0.8.5",
            "gRPC & Protobuf RPC microservice framework",
            "https://github.com/hoangtuvungcao/vietlang_grpc.git",
            "hoangtuvungcao",
        ),
        (
            "kafka",
            "2.0.1",
            "Distributed event streaming partitioned consumer & producer",
            "https://github.com/hoangtuvungcao/vietlang_kafka.git",
            "hoangtuvungcao",
        ),
        (
            "rabbitmq",
            "1.0.2",
            "AMQP RabbitMQ message broker client",
            "https://github.com/hoangtuvungcao/vietlang_rabbitmq.git",
            "hoangtuvungcao",
        ),
        (
            "saga",
            "1.0.0",
            "Distributed SAGA transaction coordinator with compensating rollbacks",
            "https://github.com/hoangtuvungcao/vietlang_saga.git",
            "hoangtuvungcao",
        ),
        (
            "retry",
            "1.0.0",
            "Exponential backoff retry policy with jitter",
            "https://github.com/hoangtuvungcao/vietlang_retry.git",
            "hoangtuvungcao",
        ),
        (
            "cron",
            "1.0.0",
            "Enterprise cron job scheduler engine",
            "https://github.com/hoangtuvungcao/vietlang_cron.git",
            "hoangtuvungcao",
        ),
        (
            "cache_lru",
            "1.0.0",
            "Fixed-capacity Least-Recently-Used cache with eviction",
            "https://github.com/hoangtuvungcao/vietlang_cache_lru.git",
            "hoangtuvungcao",
        ),
        (
            "sql_builder",
            "1.0.0",
            "Multi-table SQL Query Builder (INNER/LEFT JOIN, GROUP BY)",
            "https://github.com/hoangtuvungcao/vietlang_sql_builder.git",
            "hoangtuvungcao",
        ),
        (
            "metrics",
            "1.0.0",
            "Prometheus metrics exporter (Counters, Gauges)",
            "https://github.com/hoangtuvungcao/vietlang_metrics.git",
            "hoangtuvungcao",
        ),
        (
            "security",
            "1.0.0",
            "Password hashing, constant-time compare, CSRF, XSS filter",
            "https://github.com/hoangtuvungcao/vietlang_security.git",
            "hoangtuvungcao",
        ),
        (
            "crypto_advanced",
            "1.0.0",
            "Webhook HMAC-SHA256 signature verification & payload encryption",
            "https://github.com/hoangtuvungcao/vietlang_crypto_advanced.git",
            "hoangtuvungcao",
        ),
        (
            "kv_store",
            "1.0.0",
            "In-memory Redis engine (Atomic INCR, Hashes, TTL)",
            "https://github.com/hoangtuvungcao/vietlang_kv_store.git",
            "hoangtuvungcao",
        ),
        (
            "stream",
            "1.0.0",
            "Kafka-like partitioned stream broker with consumer offsets",
            "https://github.com/hoangtuvungcao/vietlang_stream.git",
            "hoangtuvungcao",
        ),
        (
            "http_pipeline",
            "1.0.0",
            "Onion-model middleware with automated Security Headers",
            "https://github.com/hoangtuvungcao/vietlang_http_pipeline.git",
            "hoangtuvungcao",
        ),
        (
            "websocket",
            "1.0.0",
            "WebSocket RFC 6455 framing & room broadcaster",
            "https://github.com/hoangtuvungcao/vietlang_websocket.git",
            "hoangtuvungcao",
        ),
        (
            "socket",
            "1.0.0",
            "Raw TCP/UDP low-level socket client",
            "https://github.com/hoangtuvungcao/vietlang_socket.git",
            "hoangtuvungcao",
        ),
        (
            "jwt",
            "1.0.0",
            "JWT authentication with Role-Based Access Control (RBAC)",
            "https://github.com/hoangtuvungcao/vietlang_jwt.git",
            "hoangtuvungcao",
        ),
        (
            "http_router",
            "1.0.0",
            "High-level web routing and JSON responses",
            "https://github.com/hoangtuvungcao/vietlang_http_router.git",
            "hoangtuvungcao",
        ),
        (
            "validator",
            "1.0.0",
            "Request payload validation rules",
            "https://github.com/hoangtuvungcao/vietlang_validator.git",
            "hoangtuvungcao",
        ),
        (
            "orm",
            "1.0.0",
            "SQL query builder and data layer",
            "https://github.com/hoangtuvungcao/vietlang_orm.git",
            "hoangtuvungcao",
        ),
        (
            "migration",
            "1.0.0",
            "Database schema migration versioning",
            "https://github.com/hoangtuvungcao/vietlang_migration.git",
            "hoangtuvungcao",
        ),
        (
            "rate_limiter",
            "1.0.0",
            "Token bucket DDoS protection",
            "https://github.com/hoangtuvungcao/vietlang_rate_limiter.git",
            "hoangtuvungcao",
        ),
        (
            "circuit_breaker",
            "1.0.0",
            "Fault-tolerance circuit breaker pattern",
            "https://github.com/hoangtuvungcao/vietlang_circuit_breaker.git",
            "hoangtuvungcao",
        ),
        (
            "telemetry",
            "1.0.0",
            "OpenTelemetry trace context and header propagation",
            "https://github.com/hoangtuvungcao/vietlang_telemetry.git",
            "hoangtuvungcao",
        ),
        (
            "health",
            "1.0.0",
            "Kubernetes /healthz and /readyz probe checkers",
            "https://github.com/hoangtuvungcao/vietlang_health.git",
            "hoangtuvungcao",
        ),
        (
            "event_bus",
            "1.0.0",
            "In-memory Pub/Sub event bus",
            "https://github.com/hoangtuvungcao/vietlang_event_bus.git",
            "hoangtuvungcao",
        ),
        (
            "queue",
            "1.0.0",
            "Asynchronous task queue with Dead-Letter Queue (DLQ)",
            "https://github.com/hoangtuvungcao/vietlang_queue.git",
            "hoangtuvungcao",
        ),
        (
            "config",
            "1.0.0",
            "Environment variables and .env loader",
            "https://github.com/hoangtuvungcao/vietlang_config.git",
            "hoangtuvungcao",
        ),
        (
            "multipart",
            "1.0.0",
            "Streaming multipart form-data and binary/text file upload decoder",
            "https://github.com/hoangtuvungcao/vietlang_multipart.git",
            "hoangtuvungcao",
        ),
    ];

    for (name, ver, desc, src, author) in default_entries {
        let mut versions = HashMap::new();
        versions.insert(
            ver.to_string(),
            PackageVersionEntry {
                version: ver.to_string(),
                description: desc.to_string(),
                author: author.to_string(),
                source: src.to_string(),
                checksum: "unverified".to_string(),
                signature: String::new(),
                public_key: String::new(),
                keywords: vec![name.to_string()],
            },
        );
        map.insert(
            name.to_string(),
            PackageIndexEntry {
                name: name.to_string(),
                latest: ver.to_string(),
                versions,
            },
        );
    }

    // The canonical index is parsed as JSON. Shards are discovery mirrors only;
    // they cannot override signed canonical metadata.
    let base = get_registry_base();
    let index_file = format!("{}/index.json", base);
    if let Ok(content) = fs::read_to_string(&index_file) {
        #[derive(Deserialize)]
        struct RegistryDocument {
            packages: HashMap<String, PackageIndexEntry>,
        }
        match serde_json::from_str::<RegistryDocument>(&content) {
            Ok(document) => map.extend(document.packages),
            Err(error) => eprintln!(
                "Ignoring malformed registry index '{}': {}",
                index_file, error
            ),
        }
    }

    map
}

fn scan_shards_recursive(dir: &Path, map: &mut HashMap<String, PackageIndexEntry>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                scan_shards_recursive(&path, map);
            } else if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    parse_shard_file(&content, map);
                }
            }
        }
    }
}

fn parse_shard_file(raw: &str, map: &mut HashMap<String, PackageIndexEntry>) {
    let name = extract_json_str(
        &raw.lines()
            .find(|l| l.trim().starts_with("\"name\":"))
            .unwrap_or(""),
    );
    let version = extract_json_str(
        &raw.lines()
            .find(|l| l.trim().starts_with("\"version\":"))
            .unwrap_or(""),
    );
    let desc = extract_json_str(&raw_manifest_desc(raw));
    let author = extract_json_str(
        &raw.lines()
            .find(|l| l.trim().starts_with("\"author\":"))
            .unwrap_or(""),
    );
    let source = extract_json_str(
        &raw.lines()
            .find(|l| l.trim().starts_with("\"source\":"))
            .unwrap_or(""),
    );

    if !name.is_empty() && !version.is_empty() {
        let author_str = if author.is_empty() {
            "community".to_string()
        } else {
            author
        };
        let src_str = if source.is_empty() {
            format!("https://github.com/hoangtuvungcao/vietlang_{}.git", name)
        } else {
            source
        };

        let mut versions = HashMap::new();
        versions.insert(
            version.clone(),
            PackageVersionEntry {
                version: version.clone(),
                description: desc,
                author: author_str,
                source: src_str,
                checksum: "unverified".to_string(),
                signature: String::new(),
                public_key: String::new(),
                keywords: vec![name.clone()],
            },
        );

        map.insert(
            name.clone(),
            PackageIndexEntry {
                name,
                latest: version,
                versions,
            },
        );
    }
}

fn raw_manifest_desc(raw: &str) -> &str {
    raw.lines()
        .find(|l| l.trim().starts_with("\"description\":"))
        .unwrap_or("")
}

fn parse_and_merge_registry(raw: &str, map: &mut HashMap<String, PackageIndexEntry>) {
    let lines: Vec<&str> = raw.lines().collect();
    let mut current_pkg: Option<String> = None;
    let mut current_latest = "1.0.0".to_string();
    let mut current_desc = "".to_string();
    let mut current_author = "community".to_string();
    let mut current_source = "".to_string();
    let mut current_ver = "1.0.0".to_string();

    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with("\"name\":") {
            let val = extract_json_str(trimmed);
            current_pkg = Some(val);
        } else if trimmed.starts_with("\"latest\":") {
            current_latest = extract_json_str(trimmed);
        } else if trimmed.starts_with("\"version\":") {
            current_ver = extract_json_str(trimmed);
        } else if trimmed.starts_with("\"description\":") {
            current_desc = extract_json_str(trimmed);
        } else if trimmed.starts_with("\"author\":") {
            current_author = extract_json_str(trimmed);
        } else if trimmed.starts_with("\"source\":") {
            current_source = extract_json_str(trimmed);
        }

        if (trimmed == "}," || trimmed == "}") && current_pkg.is_some() {
            let pkg_name = current_pkg.clone().unwrap();
            let src = if current_source.is_empty() {
                format!(
                    "https://github.com/hoangtuvungcao/vietlang_{}.git",
                    pkg_name
                )
            } else {
                current_source.clone()
            };

            let mut versions = HashMap::new();
            versions.insert(
                current_ver.clone(),
                PackageVersionEntry {
                    version: current_ver.clone(),
                    description: current_desc.clone(),
                    author: current_author.clone(),
                    source: src,
                    checksum: "unverified".to_string(),
                    signature: String::new(),
                    public_key: String::new(),
                    keywords: vec![pkg_name.clone()],
                },
            );

            map.insert(
                pkg_name.clone(),
                PackageIndexEntry {
                    name: pkg_name,
                    latest: current_latest.clone(),
                    versions,
                },
            );
        }
    }
}

fn extract_json_str(line: &str) -> String {
    let parts: Vec<&str> = line.splitn(2, ':').collect();
    if parts.len() >= 2 {
        let val = parts[1].trim().trim_matches(',').trim_matches('"');
        return val.to_string();
    }
    "".to_string()
}

fn search_registry(query: &str) {
    println!(
        "\x1b[36mSearching VietLang Central Community Registry for:\x1b[0m '\x1b[33m{}\x1b[0m'...",
        query
    );
    println!("=========================================================================");
    let registry = load_registry();
    let mut count = 0;

    let mut keys: Vec<&String> = registry.keys().collect();
    keys.sort();

    for name in keys {
        let entry = &registry[name];
        let latest_ver = entry.versions.get(&entry.latest);
        let desc = latest_ver
            .map(|v| v.description.as_str())
            .unwrap_or("No description");
        let author = latest_ver.map(|v| v.author.as_str()).unwrap_or("community");

        let matches = query.is_empty()
            || name.contains(query)
            || desc.to_lowercase().contains(&query.to_lowercase())
            || author.to_lowercase().contains(&query.to_lowercase());

        if matches {
            count += 1;
            println!(
                "* \x1b[36m{}\x1b[0m (v{}) by \x1b[35m@{}\x1b[0m",
                name, entry.latest, author
            );
            println!("    Description: {}", desc);
            println!(
                "    Install:     \x1b[33mvietlang install {}\x1b[0m (or vietlang install {}@{})",
                name, name, entry.latest
            );
            println!();
        }
    }

    println!("Found {} package(s) in Central Registry.", count);
}

fn install_package(spec: &str) {
    let _install_lock = match acquire_install_lock() {
        Ok(lock) => lock,
        Err(error) => {
            eprintln!(
                "\x1b[31mError:\x1b[0m Another package operation is active or the installer lock cannot be acquired: {}",
                error
            );
            return;
        }
    };
    let registry = load_registry();
    let (raw_name, requested) = spec
        .rsplit_once('@')
        .map_or((spec, None), |(name, version)| {
            if name.is_empty() || version.is_empty() {
                (spec, None)
            } else {
                (name, Some(version))
            }
        });
    let Some(entry) = registry.get(raw_name) else {
        eprintln!("\x1b[31mError:\x1b[0m Package '{}' is absent from the trusted registry. Direct Git installs are disabled because they have no signed metadata.", raw_name);
        return;
    };
    let Some(target_version) = resolve_version(entry, requested) else {
        eprintln!(
            "\x1b[31mError:\x1b[0m No version of '{}' satisfies '{}'.",
            raw_name,
            requested.unwrap_or("latest")
        );
        return;
    };
    let metadata = &entry.versions[&target_version];
    let pkg_name = raw_name.to_string();
    let source_url = metadata.source.clone();

    if !valid_package_name(&pkg_name) {
        eprintln!(
            "\x1b[31mError:\x1b[0m Invalid package name '{}'. Use ASCII letters, digits, '-' or '_' only.",
            pkg_name
        );
        return;
    }

    if let Err(error) = verify_registry_metadata(&pkg_name, metadata) {
        eprintln!("\x1b[31mSecurity error:\x1b[0m {}", error);
        return;
    }

    println!("\x1b[36m[Trusted Registry]\x1b[0m Resolving '\x1b[33m{}\x1b[0m' (version: \x1b[32m{}\x1b[0m)...", pkg_name, target_version);
    println!("Source Repository: {}", source_url);

    let _ = fs::create_dir_all("modules");
    let target_dir = format!("modules/{}", pkg_name);
    if Path::new(&target_dir).exists() {
        match package_sha256(Path::new(&target_dir)) {
            Ok(checksum) if checksum == metadata.checksum => {
                println!("Package '{}@{}' is already installed and verified.", pkg_name, target_version);
                if let Err(error) = write_lock_entry(&pkg_name, &target_version, &source_url, &target_dir, &checksum) {
                    eprintln!("Cannot update vietlang.lock: {}", error);
                }
            }
            _ => eprintln!("\x1b[31mError:\x1b[0m Existing module '{}' differs from trusted metadata. Remove it explicitly before reinstalling.", target_dir),
        }
        return;
    }
    let staging = format!(
        "modules/.vietlang-install-{}-{}",
        pkg_name,
        std::process::id()
    );
    if Path::new(&staging).exists() {
        eprintln!("\x1b[31mError:\x1b[0m Staging directory '{}' already exists; remove it after checking no installer is active.", staging);
        return;
    }
    let status = Command::new("git")
        .args(["clone", "--filter=blob:none", &source_url, &staging])
        .status();
    if !command_succeeded(status) {
        eprintln!("Cannot clone '{}'.", pkg_name);
        return;
    }
    let checkout = Command::new("git")
        .args(["-C", &staging, "checkout", &target_version])
        .status();
    if !command_succeeded(checkout) {
        let _ = fs::remove_dir_all(&staging);
        eprintln!("Cannot check out immutable version '{}'.", target_version);
        return;
    }
    let checksum = match package_sha256(Path::new(&staging)) {
        Ok(checksum) => checksum,
        Err(error) => {
            let _ = fs::remove_dir_all(&staging);
            eprintln!("Cannot hash package: {}", error);
            return;
        }
    };
    if !constant_time_hex_eq(&checksum, &metadata.checksum) {
        let _ = fs::remove_dir_all(&staging);
        eprintln!("\x1b[31mSecurity error:\x1b[0m checksum mismatch for '{}@{}' (expected {}, got {}). Package was not installed.", pkg_name, target_version, metadata.checksum, checksum);
        return;
    }
    if let Err(error) = fs::rename(&staging, &target_dir) {
        let _ = fs::remove_dir_all(&staging);
        eprintln!("Cannot activate verified package: {}", error);
        return;
    }
    ensure_manifest_dependency(&pkg_name, &target_version);
    if let Err(error) = write_lock_entry(
        &pkg_name,
        &target_version,
        &source_url,
        &target_dir,
        &checksum,
    ) {
        eprintln!("Package installed, but lockfile update failed: {}", error);
        return;
    }

    println!(
        "\x1b[32mSuccessfully installed\x1b[0m '{}@{}' into modules/{}",
        pkg_name, target_version, pkg_name
    );
    println!("Usage in your code:");
    println!("  \x1b[33mimport modules.{}.src.main\x1b[0m", pkg_name);
}

fn acquire_install_lock() -> std::io::Result<File> {
    let lock = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(".vietlang-packages.lock")?;
    lock.try_lock_exclusive()?;
    Ok(lock)
}

fn resolve_version(entry: &PackageIndexEntry, requested: Option<&str>) -> Option<String> {
    if requested.is_none() || requested == Some("latest") {
        return Some(entry.latest.clone());
    }
    let requested = requested?;
    if entry.versions.contains_key(requested) {
        return Some(requested.to_string());
    }
    let requirement = VersionReq::parse(requested).ok()?;
    let mut matching: Vec<_> = entry
        .versions
        .keys()
        .filter_map(|raw| {
            Version::parse(raw)
                .ok()
                .filter(|version| requirement.matches(version))
                .map(|version| (version, raw.clone()))
        })
        .collect();
    matching.sort_by(|left, right| left.0.cmp(&right.0));
    matching.pop().map(|(_, raw)| raw)
}

fn verify_registry_metadata(name: &str, metadata: &PackageVersionEntry) -> Result<(), String> {
    if metadata.checksum.len() != 64
        || !metadata
            .checksum
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
    {
        return Err(format!(
            "{}@{} has no valid SHA-256 checksum",
            name, metadata.version
        ));
    }
    if metadata.signature.is_empty() || metadata.public_key.is_empty() {
        return Err(format!(
            "{}@{} is unsigned; signed registry metadata is mandatory",
            name, metadata.version
        ));
    }
    let key_bytes = BASE64
        .decode(&metadata.public_key)
        .map_err(|_| "invalid registry public key encoding")?;
    let key_array: [u8; 32] = key_bytes
        .try_into()
        .map_err(|_| "registry public key must be 32 bytes")?;
    let key = VerifyingKey::from_bytes(&key_array).map_err(|_| "invalid registry public key")?;
    let signature_bytes = BASE64
        .decode(&metadata.signature)
        .map_err(|_| "invalid metadata signature encoding")?;
    let signature = Signature::from_slice(&signature_bytes)
        .map_err(|_| "metadata signature must be 64 bytes")?;
    let message = format!(
        "{}\n{}\n{}\n{}",
        name, metadata.version, metadata.source, metadata.checksum
    );
    key.verify(message.as_bytes(), &signature)
        .map_err(|_| "registry metadata signature verification failed".to_string())
}

fn constant_time_hex_eq(left: &str, right: &str) -> bool {
    if left.len() != right.len() {
        return false;
    }
    left.bytes()
        .zip(right.bytes())
        .fold(0u8, |difference, (a, b)| difference | (a ^ b))
        == 0
}

fn write_lock_entry(
    name: &str,
    version: &str,
    source: &str,
    target_dir: &str,
    checksum: &str,
) -> std::io::Result<()> {
    let mut lock = fs::read_to_string("vietlang.lock")
        .ok()
        .and_then(|raw| serde_json::from_str::<Lockfile>(&raw).ok())
        .unwrap_or_default();
    lock.lockfile_version = 1;
    let revision = Command::new("git")
        .args(["-C", target_dir, "rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_default();
    lock.packages.insert(
        name.to_string(),
        LockedPackage {
            version: version.to_string(),
            source: source.to_string(),
            revision,
            checksum: checksum.to_string(),
        },
    );
    let encoded = serde_json::to_vec_pretty(&lock).map_err(std::io::Error::other)?;
    let temporary = "vietlang.lock.tmp";
    fs::write(temporary, encoded)?;
    fs::rename(temporary, "vietlang.lock")
}

fn update_package(target: Option<&str>) {
    match target {
        Some(spec) => {
            let (pkg_name, requested) = spec
                .rsplit_once('@')
                .map_or((spec, None), |(name, version)| (name, Some(version)));
            if !valid_package_name(pkg_name) {
                eprintln!("\x1b[31mError:\x1b[0m Invalid package name '{}'.", pkg_name);
                return;
            }
            let target_dir = format!("modules/{}", pkg_name);
            if !Path::new(&target_dir).exists() {
                eprintln!(
                    "\x1b[31mError:\x1b[0m Module '{}' is not installed in modules/.",
                    pkg_name
                );
                return;
            }
            let registry = load_registry();
            let Some(entry) = registry.get(pkg_name) else {
                eprintln!(
                    "\x1b[31mError:\x1b[0m Package '{}' is absent from the trusted registry.",
                    pkg_name
                );
                return;
            };
            let Some(version) = resolve_version(entry, requested) else {
                eprintln!(
                    "\x1b[31mError:\x1b[0m No trusted version satisfies '{}'.",
                    requested.unwrap_or("latest")
                );
                return;
            };
            let metadata = &entry.versions[&version];
            if let Err(error) = verify_registry_metadata(pkg_name, metadata) {
                eprintln!("\x1b[31mSecurity error:\x1b[0m {}", error);
                return;
            }

            let backup = format!(
                "modules/.vietlang-backup-{}-{}",
                pkg_name,
                std::process::id()
            );
            if Path::new(&backup).exists() {
                eprintln!(
                    "\x1b[31mError:\x1b[0m Backup path '{}' already exists.",
                    backup
                );
                return;
            }
            if let Err(error) = fs::rename(&target_dir, &backup) {
                eprintln!("Cannot stage existing package for update: {}", error);
                return;
            }
            install_package(&format!("{}@={}", pkg_name, version));
            let verified = Path::new(&target_dir).is_dir()
                && package_sha256(Path::new(&target_dir))
                    .is_ok_and(|checksum| constant_time_hex_eq(&checksum, &metadata.checksum));
            if verified {
                if let Err(error) = fs::remove_dir_all(&backup) {
                    eprintln!(
                        "Package updated, but old backup '{}' could not be removed: {}",
                        backup, error
                    );
                }
                println!(
                    "\x1b[32mUpdated and verified '{}@{}'.\x1b[0m",
                    pkg_name, version
                );
            } else {
                if Path::new(&target_dir).exists() {
                    let _ = fs::remove_dir_all(&target_dir);
                }
                if let Err(error) = fs::rename(&backup, &target_dir) {
                    eprintln!(
                        "\x1b[31mCritical:\x1b[0m Update failed and backup restoration failed: {}. Backup remains at '{}'.",
                        error, backup
                    );
                } else {
                    eprintln!(
                        "\x1b[31mUpdate failed.\x1b[0m The previously installed package was restored."
                    );
                }
            }
        }
        None => {
            let packages: Vec<String> = fs::read_to_string("vietlang.lock")
                .ok()
                .and_then(|raw| serde_json::from_str::<Lockfile>(&raw).ok())
                .map(|lock| lock.packages.into_keys().collect())
                .unwrap_or_default();
            if packages.is_empty() {
                println!("No locked packages to update.");
                return;
            }
            for package in packages {
                update_package(Some(&package));
            }
        }
    }
}

fn remove_package(pkg_name: &str) {
    if !valid_package_name(pkg_name) {
        eprintln!("\x1b[31mError:\x1b[0m Invalid package name '{}'.", pkg_name);
        return;
    }
    let target_dir = format!("modules/{}", pkg_name);
    if Path::new(&target_dir).exists() {
        let _ = fs::remove_dir_all(&target_dir);
        println!("Removed directory {}", target_dir);
    }

    if Path::new("vietlang.json").exists() {
        if let Ok(content) = fs::read_to_string("vietlang.json") {
            if let Ok(mut manifest) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(dependencies) = manifest
                    .get_mut("dependencies")
                    .and_then(|value| value.as_object_mut())
                {
                    dependencies.remove(pkg_name);
                }
                if let Ok(encoded) = serde_json::to_vec_pretty(&manifest) {
                    let _ = fs::write("vietlang.json", encoded);
                }
            }
        }
    }
    if let Ok(content) = fs::read_to_string("vietlang.lock") {
        if let Ok(mut lock) = serde_json::from_str::<Lockfile>(&content) {
            lock.packages.remove(pkg_name);
            if let Ok(encoded) = serde_json::to_vec_pretty(&lock) {
                let _ = fs::write("vietlang.lock", encoded);
            }
        }
    }

    println!(
        "\x1b[32mRemoved package '{}' successfully.\x1b[0m",
        pkg_name
    );
}

fn valid_package_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 128
        && name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
}

fn command_succeeded(status: std::io::Result<std::process::ExitStatus>) -> bool {
    matches!(status, Ok(status) if status.success())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_names_cannot_escape_modules_directory() {
        assert!(valid_package_name("http-router_2"));
        assert!(!valid_package_name("../outside"));
        assert!(!valid_package_name("nested/package"));
        assert!(!valid_package_name(""));
    }

    #[test]
    fn semver_ranges_resolve_to_highest_matching_release() {
        let mut versions = HashMap::new();
        for version in ["1.2.0", "1.9.0", "2.0.0"] {
            versions.insert(
                version.into(),
                PackageVersionEntry {
                    version: version.into(),
                    description: String::new(),
                    author: String::new(),
                    source: String::new(),
                    checksum: "0".repeat(64),
                    signature: String::new(),
                    public_key: String::new(),
                    keywords: vec![],
                },
            );
        }
        let entry = PackageIndexEntry {
            name: "demo".into(),
            latest: "2.0.0".into(),
            versions,
        };
        assert_eq!(resolve_version(&entry, Some("^1.0")), Some("1.9.0".into()));
        assert_eq!(
            resolve_version(&entry, Some("=1.2.0")),
            Some("1.2.0".into())
        );
        assert_eq!(resolve_version(&entry, Some(">=3")), None);
    }

    #[test]
    fn registry_metadata_requires_valid_ed25519_signature() {
        let signing = SigningKey::from_bytes(&[7u8; 32]);
        let mut metadata = PackageVersionEntry {
            version: "1.0.0".into(),
            description: String::new(),
            author: "tester".into(),
            source: "https://example.invalid/demo.git".into(),
            checksum: "a".repeat(64),
            signature: String::new(),
            public_key: BASE64.encode(signing.verifying_key().as_bytes()),
            keywords: vec![],
        };
        let message = format!(
            "demo\n{}\n{}\n{}",
            metadata.version, metadata.source, metadata.checksum
        );
        metadata.signature = BASE64.encode(signing.sign(message.as_bytes()).to_bytes());
        verify_registry_metadata("demo", &metadata).unwrap();
        metadata.checksum = "b".repeat(64);
        assert!(verify_registry_metadata("demo", &metadata).is_err());
    }
}

fn publish_package() {
    if !Path::new("vietlang.json").exists() {
        eprintln!("\x1b[31mError:\x1b[0m No vietlang.json manifest found in current directory.");
        return;
    }
    if !Path::new("src/main.vl").exists() {
        eprintln!("\x1b[31mError:\x1b[0m Main entrypoint src/main.vl not found.");
        return;
    }

    let raw_manifest = fs::read_to_string("vietlang.json").unwrap_or_default();
    let manifest: serde_json::Value = match serde_json::from_str(&raw_manifest) {
        Ok(manifest) => manifest,
        Err(error) => {
            eprintln!("\x1b[31mError:\x1b[0m Invalid vietlang.json: {}", error);
            return;
        }
    };
    let manifest_string = |field: &str| {
        manifest
            .get(field)
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default()
            .to_string()
    };
    let pkg_name = manifest_string("name");
    let version = manifest_string("version");
    let desc = manifest_string("description");
    let author = manifest_string("author");
    let mut repo = manifest_string("repository");
    if !valid_package_name(&pkg_name) || Version::parse(&version).is_err() {
        eprintln!(
            "\x1b[31mError:\x1b[0m Publishing requires a safe package name and a valid semantic version."
        );
        return;
    }

    // Auto-detect git origin remote if not explicitly specified in manifest
    if repo.is_empty() {
        let git_remote_cmd = Command::new("git")
            .args(["remote", "get-url", "origin"])
            .output();
        if let Ok(out) = git_remote_cmd {
            if out.status.success() {
                repo = String::from_utf8_lossy(&out.stdout).trim().to_string();
            }
        }
    }

    let author_display = if author.is_empty() {
        "community_developer"
    } else {
        &author
    };
    let repo_display = if repo.is_empty() {
        format!("https://github.com/{}/{}.git", author_display, pkg_name)
    } else {
        repo.clone()
    };

    println!("\x1b[36m=== Publishing Module to Central Community Registry ===\x1b[0m");
    println!("  Package:     \x1b[33m{}\x1b[0m", pkg_name);
    println!("  Version:     \x1b[32m{}\x1b[0m", version);
    println!("  Author:      \x1b[35m@{}\x1b[0m", author_display);
    println!("  Repository:  {}", repo_display);
    println!("  Description: {}", desc);

    // Hash the manifest and complete source tree with stable relative paths.
    let checksum = match package_sha256(Path::new(".")) {
        Ok(digest) => digest,
        Err(error) => {
            eprintln!(
                "\x1b[31mError:\x1b[0m Cannot hash package contents: {}",
                error
            );
            return;
        }
    };

    let encoded_key = match std::env::var("VIETLANG_SIGNING_KEY") {
        Ok(value) => value,
        Err(_) => {
            eprintln!(
                "\x1b[31mSecurity error:\x1b[0m VIETLANG_SIGNING_KEY must contain a base64-encoded 32-byte Ed25519 private key."
            );
            return;
        }
    };
    let key_bytes = match BASE64.decode(encoded_key.trim()) {
        Ok(bytes) => bytes,
        Err(_) => {
            eprintln!("\x1b[31mSecurity error:\x1b[0m VIETLANG_SIGNING_KEY is not valid base64.");
            return;
        }
    };
    let key_array: [u8; 32] = match key_bytes.try_into() {
        Ok(bytes) => bytes,
        Err(_) => {
            eprintln!(
                "\x1b[31mSecurity error:\x1b[0m VIETLANG_SIGNING_KEY must decode to exactly 32 bytes."
            );
            return;
        }
    };
    let signing_key = SigningKey::from_bytes(&key_array);
    let signed_message = format!("{}\n{}\n{}\n{}", pkg_name, version, repo_display, checksum);
    let signature = BASE64.encode(signing_key.sign(signed_message.as_bytes()).to_bytes());
    let public_key = BASE64.encode(signing_key.verifying_key().as_bytes());

    println!("  Checksum:    {}", checksum);

    // Write to Sharded Index File
    let base = get_registry_base();
    let shard_rel = get_shard_relpath(&pkg_name);
    let shard_full = format!("{}/{}", base, shard_rel);

    if let Some(parent) = Path::new(&shard_full).parent() {
        let _ = fs::create_dir_all(parent);
    }

    let metadata = PackageVersionEntry {
        version: version.clone(),
        description: desc.clone(),
        author: author_display.to_string(),
        source: repo_display.clone(),
        checksum: checksum.clone(),
        signature,
        public_key,
        keywords: vec![pkg_name.clone()],
    };
    let shard_content = serde_json::to_vec_pretty(&serde_json::json!({
        "name": pkg_name,
        "latest": version,
        "versions": { version.clone(): metadata.clone() }
    }))
    .expect("serializable registry metadata");

    if let Err(error) = fs::write(&shard_full, shard_content) {
        eprintln!("Cannot write registry shard: {}", error);
        return;
    }
    println!("  Sharded Index: \x1b[32mCreated {}\x1b[0m", shard_full);

    // Update master index.json if present
    let master_index = format!("{}/index.json", base);
    if let Ok(reg_content) = fs::read_to_string(&master_index) {
        let mut registry: serde_json::Value = match serde_json::from_str(&reg_content) {
            Ok(registry) => registry,
            Err(error) => {
                eprintln!("Cannot update malformed registry index: {}", error);
                return;
            }
        };
        let Some(packages) = registry
            .get_mut("packages")
            .and_then(serde_json::Value::as_object_mut)
        else {
            eprintln!("Cannot update registry index without a packages object.");
            return;
        };
        let package = packages.entry(pkg_name.clone()).or_insert_with(
            || serde_json::json!({"name": pkg_name, "latest": version, "versions": {}}),
        );
        package["latest"] = serde_json::Value::String(version.clone());
        if !package["versions"].is_object() {
            package["versions"] = serde_json::json!({});
        }
        package["versions"][&version] =
            serde_json::to_value(&metadata).expect("serializable registry metadata");
        let encoded = serde_json::to_vec_pretty(&registry).expect("serializable registry");
        let temporary = format!("{}.tmp", master_index);
        if fs::write(&temporary, encoded)
            .and_then(|_| fs::rename(&temporary, &master_index))
            .is_err()
        {
            eprintln!("Cannot atomically update registry index.");
            return;
        }
        println!("  Master Index:  \x1b[32mUpdated {}\x1b[0m", master_index);
    }

    println!(
        "\x1b[32mSuccessfully published '{}@{}' to VietLang Central Registry!\x1b[0m",
        pkg_name, version
    );
    println!();
    println!("Global Discovery & Usage:");
    println!(
        "  Any developer can search:  \x1b[33mvietlang search {}\x1b[0m",
        pkg_name
    );
    println!("  Any developer can install: \x1b[33mvietlang install {}\x1b[0m (or vietlang install {}@{})", pkg_name, pkg_name, version);
    println!();
    println!("Community Contribution:");
    println!(
        "  Submit your shard file ({}) to https://github.com/hoangtuvungcao/vietlang/pulls",
        shard_rel
    );
}

fn package_sha256(root: &Path) -> std::io::Result<String> {
    let mut files = vec![root.join("vietlang.json")];
    collect_package_files(&root.join("src"), &mut files)?;
    files.retain(|path| path.is_file());
    files.sort();

    let mut hasher = Sha256::new();
    for path in files {
        let relative = path.strip_prefix(root).unwrap_or(&path);
        hasher.update(relative.to_string_lossy().as_bytes());
        hasher.update([0]);
        hasher.update(fs::read(&path)?);
        hasher.update([0]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn collect_package_files(dir: &Path, files: &mut Vec<std::path::PathBuf>) -> std::io::Result<()> {
    if !dir.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_package_files(&path, files)?;
        } else if path.is_file() {
            files.push(path);
        }
    }
    Ok(())
}

fn init_project(name: &str, _template_type: &str) {
    let _ = fs::create_dir_all(name);
    let _ = fs::create_dir_all(format!("{}/src/config", name));
    let _ = fs::create_dir_all(format!("{}/src/models", name));
    let _ = fs::create_dir_all(format!("{}/src/services", name));
    let _ = fs::create_dir_all(format!("{}/src/routes", name));
    let _ = fs::create_dir_all(format!("{}/data", name));
    let _ = fs::create_dir_all(format!("{}/tests", name));
    let _ = fs::create_dir_all(format!("{}/modules", name));

    // 1. vietlang.json manifest
    let manifest = format!(
        r#"{{
  "name": "{}",
  "version": "1.0.0",
  "author": "developer",
  "repository": "https://github.com/developer/{}.git",
  "type": "backend_api",
  "description": "Experimental Backend REST API Prototype in VietLang",
  "main": "src/main.vl",
  "scripts": {{
    "start": "vietlang run src/main.vl",
    "dev": "PORT=8080 vietlang run src/main.vl",
    "build": "vietlang build src/main.vl -o {}_service",
    "build:win": "vietlang build src/main.vl -o {}_service.exe --target windows",
    "test": "vietlang test tests"
  }},
  "dependencies": {{
    "sqlite": "1.0.0",
    "http_router": "1.0.0",
    "validator": "1.0.0"
  }},
  "license": "MIT"
}}
"#,
        name, name, name, name
    );
    let _ = fs::write(format!("{}/vietlang.json", name), manifest);

    // 2. config.json & .env.example
    let config_json = r#"{
  "port": 8080,
  "workers": 4,
  "db_path": "data/app.sqlite"
}
"#;
    let _ = fs::write(format!("{}/config.json", name), config_json);

    let env_example = r#"# VietLang Backend Service Environment Configuration
PORT=8080
WORKERS=4
DATABASE_PATH=data/app.sqlite
"#;
    let _ = fs::write(format!("{}/.env.example", name), env_example);

    // 3. src/config/database.vl
    let db_code = r#"import std.db_sqlite
import std.json

fn db_connect(db_path: String) {
    let db = sqlite_open(db_path)
    sqlite_exec(db, "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, email TEXT, role TEXT, created_at INTEGER)")
    return db
}

fn json_response(status_code: Int, data) {
    let mut payload = map_new()
    payload = map_set(payload, "status_code", status_code)
    payload = map_set(payload, "data", data)
    payload = map_set(payload, "timestamp", time_now())

    let mut res = map_new()
    res = map_set(res, "status_code", status_code)
    res = map_set(res, "content_type", "application/json; charset=utf-8")
    res = map_set(res, "body", json_stringify(payload, false))
    return res
}
"#;
    let _ = fs::write(format!("{}/src/config/database.vl", name), db_code);

    // 4. src/services/user_service.vl
    let service_code = r#"import std.json

fn service_list_users(db) {
    return sqlite_query(db, "SELECT id, name, email, role, created_at FROM users ORDER BY id DESC")
}

fn service_create_user(db, name: String, email: String, role: String) {
    sqlite_execute(
        db,
        "INSERT INTO users (name, email, role, created_at) VALUES (?, ?, ?, ?)",
        [name, email, role, time_now()]
    )
    
    let mut result = map_new()
    result = map_set(result, "name", name)
    result = map_set(result, "email", email)
    result = map_set(result, "role", role)
    return result
}
"#;
    let _ = fs::write(
        format!("{}/src/services/user_service.vl", name),
        service_code,
    );

    // 5. src/routes/router.vl
    let router_code = r#"import std.json
import src.services.user_service

fn dispatch_api(db, method: String, path: String, body_raw: String) {
    // GET /api/health - System Health Probe
    if method == "GET" && path == "/api/health" {
        let mut h = map_new()
        h = map_set(h, "status", "HEALTHY")
        h = map_set(h, "uptime_s", time_now())
        h = map_set(h, "engine", "VietLang Experimental Backend Runtime")
        return json_response(200, h)
    }

    // GET /api/users - List all users
    if method == "GET" && path == "/api/users" {
        let users = service_list_users(db)
        return json_response(200, users)
    }

    // POST /api/users - Create a new user
    if method == "POST" && path == "/api/users" {
        let parsed = json_parse(body_raw)
        if parsed == none {
            return json_response(400, "Invalid JSON payload")
        }
        let user_name = to_string(map_get(parsed, "name"))
        let user_email = to_string(map_get(parsed, "email"))
        let mut role = "USER"
        if map_has(parsed, "role") {
            role = to_string(map_get(parsed, "role"))
        }

        let created = service_create_user(db, user_name, user_email, role)
        return json_response(201, created)
    }

    return json_response(404, "Endpoint not found: " + path)
}
"#;
    let _ = fs::write(format!("{}/src/routes/router.vl", name), router_code);

    // 6. src/main.vl
    let main_code = r#"import std.http_router
import std.http2
import std.middleware
import src.config.database
import src.routes.router

// ------------------------------------------------------------------------
// Dynamic Runtime Configuration Loader
// ------------------------------------------------------------------------
let mut port = 8080
let mut db_path = "data/app.sqlite"

try {
    if file_exists("config.json") {
        let cfg = json_parse(file_read("config.json"))
        if map_has(cfg, "port") { port = to_int(map_get(cfg, "port")) }
        if map_has(cfg, "db_path") { db_path = to_string(map_get(cfg, "db_path")) }
    }
} catch err {}

let env_p = env_get("PORT")
if env_p != "" && env_p != none && env_p != "none" {
    port = to_int(env_p)
}

let db = db_connect(db_path)
println("========================================================================")
println("VietLang experimental backend service listening on http://0.0.0.0:" + to_string(port))
println("========================================================================")

let server_cfg = http2_server_config(port, 200)
http_listen(server_cfg, fn(req) {
    let method = to_string(map_get(req, "method"))
    let path = to_string(map_get(req, "path"))
    let body = to_string(map_get(req, "body"))

    return dispatch_api(db, method, path, body)
})
"#;
    let _ = fs::write(format!("{}/src/main.vl", name), main_code);

    // 7. tests/main_test.vl
    let test_code = format!(
        r#"import std.test

suite("{} Backend Prototype Test Suite")

test("Sanity test", fn() {{
    assert_eq(1 + 1, 2, "Sanity check failure")
}})

test_summary()
"#,
        name
    );
    let _ = fs::write(format!("{}/tests/main_test.vl", name), test_code);

    // 8. README.md
    let readme_content = format!(
        r#"# {}

Experimental backend REST API prototype built with **VietLang**. Complete the
security, load, failure-injection, and dependency-integrity review before any
public production deployment.

## 🚀 Commands

### 1. Start Development Server:
```bash
vietlang dev
```
Health Check Endpoint: `http://localhost:8080/api/health`

### 2. Bundle Source with the Runtime:
```bash
# Build for Linux
vietlang run build
./{}_service

# Build for Windows (.exe)
vietlang run build:win
```

### 3. Run Automated Tests:
```bash
vietlang test
```
"#,
        name, name
    );
    let _ = fs::write(format!("{}/README.md", name), readme_content);

    println!(
        "\x1b[32;1m[SUCCESS] Created new VietLang backend service '{}' successfully!\x1b[0m",
        name
    );
    println!("Project Structure Generated:");
    println!("  ├── \x1b[36mvietlang.json\x1b[0m       (Service manifest & scripts)");
    println!("  ├── \x1b[36mconfig.json\x1b[0m         (Dynamic runtime configuration)");
    println!("  ├── \x1b[36m.env.example\x1b[0m        (Environment variables template)");
    println!("  ├── \x1b[33msrc/\x1b[0m                (Layered Backend Prototype)");
    println!("  │   ├── \x1b[32mconfig/database.vl\x1b[0m (Database pool & JSON responses)");
    println!("  │   ├── \x1b[32mservices/user_service.vl\x1b[0m (Business logic & Queries)");
    println!("  │   ├── \x1b[32mroutes/router.vl\x1b[0m   (REST API Routing)");
    println!("  │   └── \x1b[32mmain.vl\x1b[0m            (Experimental HTTP/1.1 entrypoint)");
    println!("  ├── \x1b[33mdata/\x1b[0m               (SQLite Database directory)");
    println!("  └── \x1b[33mtests/\x1b[0m              (Automated test suites)");
    println!();
    println!("Next Steps:");
    println!("  1. \x1b[36mcd {}\x1b[0m", name);
    println!("  2. \x1b[36mvietlang dev\x1b[0m           (Start server on http://localhost:8080)");
    println!("  3. \x1b[36mvietlang run build\x1b[0m     (Bundle source with the runtime)");
}

fn list_installed() {
    if !Path::new("modules").exists() {
        println!("No modules/ directory found in current path.");
        return;
    }
    let lock = fs::read_to_string("vietlang.lock")
        .ok()
        .and_then(|raw| serde_json::from_str::<Lockfile>(&raw).ok());
    println!("Installed packages in modules/:");
    if let Ok(entries) = fs::read_dir("modules") {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let name = entry.file_name().into_string().unwrap_or_default();
                let status = lock
                    .as_ref()
                    .and_then(|lock| lock.packages.get(&name))
                    .map_or("UNLOCKED".to_string(), |package| {
                        match package_sha256(&entry.path()) {
                            Ok(checksum) if constant_time_hex_eq(&checksum, &package.checksum) => {
                                format!("{} VERIFIED", package.version)
                            }
                            _ => format!("{} TAMPERED", package.version),
                        }
                    });
                println!("  * \x1b[36m{}\x1b[0m [{}]", name, status);
            }
        }
    }
}

fn verify_project() {
    println!("\x1b[36mVerifying VietLang Project...\x1b[0m");
    if Path::new("vietlang.json").exists() {
        println!("  vietlang.json: \x1b[32mOK\x1b[0m");
    }
    if Path::new("src/main.vl").exists() {
        println!("  src/main.vl:   \x1b[32mOK\x1b[0m");
    }
    if Path::new("tests/main_test.vl").exists() {
        println!("  Running test suite...");
        let _ = Command::new("vietlang").arg("tests/main_test.vl").status();
    }
    println!("\x1b[32mVerification complete.\x1b[0m");
}

pub fn show_docs(module_name: &str) {
    if module_name.is_empty() || module_name == "--help" || module_name == "-h" {
        println!(
            "\x1b[36m╔════════════════════════════════════════════════════════════════════╗\x1b[0m"
        );
        println!(
            "\x1b[36m║             VietLang Standard Library & Module Explorer            ║\x1b[0m"
        );
        println!(
            "\x1b[36m╚════════════════════════════════════════════════════════════════════╝\x1b[0m"
        );
        println!("Usage: \x1b[33mvietlang doc <module_name>\x1b[0m (e.g. 'vietlang doc std.pagination', 'vietlang doc std.rate_limiter')");
        println!("       \x1b[33mvietlang doc --all\x1b[0m (generate full Markdown documentation in docs/api/)\n");
        println!("Available Standard Library Modules in std/:");

        if let Ok(entries) = fs::read_dir("std") {
            let mut mods = Vec::new();
            for entry in entries.flatten() {
                let name = entry.file_name().into_string().unwrap_or_default();
                if name.ends_with(".vl") {
                    mods.push(name.trim_end_matches(".vl").to_string());
                }
            }
            mods.sort();
            for m in mods {
                let file_path = format!("std/{}.vl", m);
                let first_desc = if let Ok(content) = fs::read_to_string(&file_path) {
                    content
                        .lines()
                        .find(|l| {
                            l.starts_with("// Module:")
                                || (l.starts_with("//") && !l.contains("==="))
                        })
                        .map(|l| l.trim_start_matches("//").trim().to_string())
                        .unwrap_or_else(|| "Standard utility module".to_string())
                } else {
                    "Standard utility module".to_string()
                };
                println!("  * \x1b[32mstd.{:<18}\x1b[0m — {}", m, first_desc);
            }
        }
        return;
    }

    if module_name == "--all" || module_name == "all" {
        println!("\x1b[36mGenerating comprehensive API Documentation into docs/api/...\x1b[0m");
        let _ = fs::create_dir_all("docs/api");
        if let Ok(entries) = fs::read_dir("std") {
            for entry in entries.flatten() {
                let name = entry.file_name().into_string().unwrap_or_default();
                if name.ends_with(".vl") {
                    let mod_name = name.trim_end_matches(".vl");
                    let file_path = format!("std/{}.vl", mod_name);
                    if let Ok(content) = fs::read_to_string(&file_path) {
                        let mut header_desc = Vec::new();
                        let mut functions = Vec::new();
                        let mut doc_buf = Vec::new();

                        for line in content.lines() {
                            let trimmed = line.trim();
                            if (trimmed.starts_with("// Module:")
                                || trimmed.starts_with("// VietLang"))
                                && !trimmed.contains("===")
                            {
                                header_desc
                                    .push(trimmed.trim_start_matches("//").trim().to_string());
                            } else if trimmed.starts_with("///")
                                || (trimmed.starts_with("//")
                                    && !trimmed.contains("===")
                                    && !trimmed.contains("Module:"))
                            {
                                doc_buf.push(
                                    trimmed
                                        .trim_start_matches("///")
                                        .trim_start_matches("//")
                                        .trim()
                                        .to_string(),
                                );
                            } else if trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") {
                                let sig = trimmed.trim_end_matches('{').trim().to_string();
                                let desc = if doc_buf.is_empty() {
                                    "Function provided by module".to_string()
                                } else {
                                    doc_buf.join(" ")
                                };
                                functions.push((sig, desc));
                                doc_buf.clear();
                            } else if trimmed.is_empty() {
                                doc_buf.clear();
                            }
                        }

                        let mut md = format!("# Module `std.{}`\n\n", mod_name);
                        if !header_desc.is_empty() {
                            for h in &header_desc {
                                md.push_str(&format!("{}\n\n", h));
                            }
                        } else {
                            md.push_str(&format!("Standard library module for high-throughput backend services in VietLang.\n\n"));
                        }

                        md.push_str("## Quickstart\n\n```vietlang\n");
                        md.push_str(&format!("import std.{}\n```\n\n", mod_name));

                        md.push_str("## Exported Functions Reference\n\n");
                        md.push_str("| Function Signature | Description |\n");
                        md.push_str("| :--- | :--- |\n");
                        for (sig, desc) in &functions {
                            md.push_str(&format!("| `{}` | {} |\n", sig, desc));
                        }
                        md.push_str("\n---\n\n### Function Details\n\n");
                        for (sig, desc) in &functions {
                            md.push_str(&format!("#### `{}`\n\n{}\n\n", sig, desc));
                        }

                        let _ = fs::write(format!("docs/api/{}.md", mod_name), md);
                    }
                }
            }
        }
        println!("\x1b[32mSuccessfully generated comprehensive documentation for all modules in docs/api/!\x1b[0m");
        return;
    }

    let clean = module_name.trim_start_matches("std.");
    let std_path = format!("std/{}.vl", clean);
    let mod_path = format!("modules/{}/src/main.vl", module_name);
    let pkg_src_path = format!("{}/src/main.vl", module_name);
    let pkg_readme_path = format!("{}/README.md", module_name);
    let mod_readme_path = format!("modules/{}/README.md", module_name);

    let target_path = if Path::new(&std_path).exists() {
        std_path
    } else if Path::new(&mod_path).exists() {
        mod_path
    } else if Path::new(&pkg_src_path).exists() {
        pkg_src_path
    } else if Path::new(module_name).exists() && !Path::new(module_name).is_dir() {
        module_name.to_string()
    } else {
        eprintln!(
            "\x1b[31mError:\x1b[0m Module '{}' not found in std/, modules/, or local path.",
            module_name
        );
        return;
    };

    println!(
        "\x1b[36m╔════════════════════════════════════════════════════════════════════╗\x1b[0m"
    );
    println!(
        "\x1b[36m║ Module: \x1b[32m{:<58}\x1b[36m ║\x1b[0m",
        module_name
    );
    println!(
        "\x1b[36m║ File:   \x1b[33m{:<58}\x1b[36m ║\x1b[0m",
        target_path
    );
    println!(
        "\x1b[36m╚════════════════════════════════════════════════════════════════════╝\x1b[0m\n"
    );

    let readme_file = if Path::new(&pkg_readme_path).exists() {
        Some(pkg_readme_path)
    } else if Path::new(&mod_readme_path).exists() {
        Some(mod_readme_path)
    } else {
        None
    };

    if let Some(r_path) = readme_file {
        if let Ok(readme_content) = fs::read_to_string(&r_path) {
            println!(
                "\x1b[35m=== Package Documentation ({}) ===\x1b[0m\n",
                r_path
            );
            println!("{}\n", readme_content.trim());
            println!("\x1b[36m=== Exported Function Signatures ===\x1b[0m\n");
        }
    }

    if let Ok(content) = fs::read_to_string(&target_path) {
        let mut doc_buf: Vec<String> = Vec::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("///")
                || (trimmed.starts_with("//")
                    && !trimmed.contains("===")
                    && !trimmed.contains("Module:"))
            {
                doc_buf.push(
                    trimmed
                        .trim_start_matches("///")
                        .trim_start_matches("//")
                        .trim()
                        .to_string(),
                );
            } else if trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") {
                let sig = trimmed.trim_end_matches('{').trim();
                println!(
                    "  \x1b[32mfn\x1b[0m \x1b[33m{}\x1b[0m",
                    sig.trim_start_matches("fn ").trim_start_matches("pub fn ")
                );
                if !doc_buf.is_empty() {
                    for d in &doc_buf {
                        println!("     \x1b[90m// {}\x1b[0m", d);
                    }
                    doc_buf.clear();
                }
                println!();
            } else if trimmed.is_empty() {
                doc_buf.clear();
            }
        }
    }
}

pub fn generate_docs(output: &Path) -> std::io::Result<()> {
    fs::create_dir_all(output)?;
    let mut modules: Vec<_> = fs::read_dir("std")?
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|extension| extension.to_str()) == Some("vl"))
        .collect();
    modules.sort();
    for path in modules {
        let module = path
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown");
        let source = fs::read_to_string(&path)?;
        let mut markdown = format!("# Module `std.{}`\n\nGenerated by VietLang {}. Do not edit manually.\n\n## Exports\n\n", module, env!("CARGO_PKG_VERSION"));
        let mut found = false;
        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") {
                found = true;
                let signature = trimmed.split('{').next().unwrap_or(trimmed).trim();
                markdown.push_str(&format!("- `{}`\n", signature.replace('`', "\\`")));
            }
        }
        if !found {
            markdown.push_str("_No public VietLang functions._\n");
        }
        let temporary = output.join(format!(".{}.md.tmp", module));
        let destination = output.join(format!("{}.md", module));
        fs::write(&temporary, markdown)?;
        fs::rename(temporary, destination)?;
    }
    println!(
        "Generated standard-library API documentation in {}",
        output.display()
    );
    Ok(())
}

fn show_info() {
    if let Ok(content) = fs::read_to_string("vietlang.json") {
        println!("Package Manifest (vietlang.json):");
        println!("{}", content);
    } else {
        println!("No vietlang.json manifest found in current directory.");
    }
}

fn ensure_manifest_dependency(pkg_name: &str, version: &str) {
    let mut manifest = fs::read_to_string("vietlang.json")
        .ok()
        .and_then(|raw| serde_json::from_str::<serde_json::Value>(&raw).ok())
        .unwrap_or_else(|| serde_json::json!({"name":"app","version":"1.0.0","dependencies":{}}));
    if !manifest
        .get("dependencies")
        .is_some_and(serde_json::Value::is_object)
    {
        manifest["dependencies"] = serde_json::json!({});
    }
    manifest["dependencies"][pkg_name] = serde_json::Value::String(format!("={}", version));
    if let Ok(encoded) = serde_json::to_vec_pretty(&manifest) {
        let _ = fs::write("vietlang.json", encoded);
    }
}

pub fn get_manifest_content() -> Option<String> {
    if Path::new("vietlang.json").exists() {
        fs::read_to_string("vietlang.json").ok()
    } else if Path::new("vpm.json").exists() {
        fs::read_to_string("vpm.json").ok()
    } else {
        None
    }
}

pub fn get_manifest_main() -> Option<String> {
    if let Some(content) = get_manifest_content() {
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("\"main\":") {
                return Some(extract_json_str(trimmed));
            }
        }
    }
    None
}

pub fn get_manifest_script(script_name: &str) -> Option<String> {
    if let Some(content) = get_manifest_content() {
        let mut in_scripts = false;
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("\"scripts\":") {
                in_scripts = true;
                continue;
            }
            if in_scripts {
                if trimmed.starts_with('}') {
                    break;
                }
                let target_key = format!("\"{}\":", script_name);
                if trimmed.starts_with(&target_key) {
                    return Some(extract_json_str(trimmed));
                }
            }
        }
    }
    None
}

pub fn run_script(script_name: &str, extra_args: &[String]) -> bool {
    if let Some(cmd_str) = get_manifest_script(script_name) {
        println!("\x1b[36m>\x1b[0m \x1b[32m{}\x1b[0m", cmd_str);
        let mut full_cmd = cmd_str;
        if !extra_args.is_empty() {
            full_cmd.push(' ');
            full_cmd.push_str(&extra_args.join(" "));
        }
        let status = if cfg!(windows) {
            Command::new("cmd").args(["/C", &full_cmd]).status()
        } else {
            Command::new("sh").args(["-c", &full_cmd]).status()
        };
        match status {
            Ok(s) => {
                if !s.success() {
                    std::process::exit(s.code().unwrap_or(1));
                }
                true
            }
            Err(e) => {
                eprintln!(
                    "\x1b[31mError running script '{}':\x1b[0m {}",
                    script_name, e
                );
                std::process::exit(1);
            }
        }
    } else {
        false
    }
}
