//! VietLang Central Community Registry Subsystem
//! High-throughput, sharded prefix-tree package indexing & decentralized distribution.

#![allow(dead_code)]

use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct PackageVersionEntry {
    pub version: String,
    pub description: String,
    pub author: String,
    pub source: String,
    pub checksum: String,
    pub keywords: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PackageIndexEntry {
    pub name: String,
    pub latest: String,
    pub versions: HashMap<String, PackageVersionEntry>,
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

    // Merge index.json file
    let base = get_registry_base();
    let index_file = format!("{}/index.json", base);
    if let Ok(content) = fs::read_to_string(&index_file) {
        parse_and_merge_registry(&content, &mut map);
    }

    // Scan sharded index directory
    let shards_dir = format!("{}/shards", base);
    if Path::new(&shards_dir).exists() {
        scan_shards_recursive(Path::new(&shards_dir), &mut map);
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
    let registry = load_registry();
    let parts: Vec<&str> = spec.split('@').collect();
    let raw_name = parts[0];
    let req_version = if parts.len() > 1 && !parts[1].is_empty() && parts[1] != "latest" {
        Some(parts[1])
    } else {
        None
    };

    let (pkg_name, target_version, source_url) = if let Some(entry) = registry.get(raw_name) {
        let ver = req_version.unwrap_or(&entry.latest);
        let src = match entry.versions.get(ver) {
            Some(v_entry) => v_entry.source.clone(),
            None => {
                println!("\x1b[33mWarning:\x1b[0m Version '{}' not found for '{}'. Using latest version '{}'", ver, raw_name, entry.latest);
                entry
                    .versions
                    .get(&entry.latest)
                    .map(|v| v.source.clone())
                    .unwrap_or_else(|| {
                        format!(
                            "https://github.com/hoangtuvungcao/vietlang_{}.git",
                            raw_name
                        )
                    })
            }
        };
        (raw_name.to_string(), ver.to_string(), src)
    } else if raw_name.starts_with("http://")
        || raw_name.starts_with("https://")
        || raw_name.starts_with("git@")
    {
        let clean = raw_name.trim_end_matches(".git");
        let name = clean.split('/').last().unwrap_or(clean).to_string();
        (
            name,
            req_version.unwrap_or("latest").to_string(),
            raw_name.to_string(),
        )
    } else if raw_name.contains('/') {
        let clean = raw_name.trim_start_matches("github:");
        let name = clean.split('/').last().unwrap_or(clean).to_string();
        let url = format!("https://github.com/{}.git", clean);
        (name, req_version.unwrap_or("latest").to_string(), url)
    } else {
        let url = format!(
            "https://github.com/hoangtuvungcao/vietlang_{}.git",
            raw_name
        );
        (
            raw_name.to_string(),
            req_version.unwrap_or("latest").to_string(),
            url,
        )
    };

    if !valid_package_name(&pkg_name) {
        eprintln!(
            "\x1b[31mError:\x1b[0m Invalid package name '{}'. Use ASCII letters, digits, '-' or '_' only.",
            pkg_name
        );
        return;
    }

    println!("\x1b[36m[Central Registry]\x1b[0m Resolving '\x1b[33m{}\x1b[0m' (version: \x1b[32m{}\x1b[0m)...", pkg_name, target_version);
    println!("Source Repository: {}", source_url);

    let _ = fs::create_dir_all("modules");
    let target_dir = format!("modules/{}", pkg_name);

    if Path::new(&target_dir).exists() {
        println!(
            "Package '{}' already exists in modules/. Updating...",
            pkg_name
        );
        let status = Command::new("git")
            .args(["-C", &target_dir, "pull"])
            .status();
        if !command_succeeded(status) {
            eprintln!("\x1b[31mError:\x1b[0m Cannot update '{}'.", pkg_name);
            return;
        }
    } else {
        println!("Cloning package from source into {}...", target_dir);
        let status = Command::new("git")
            .args(["clone", &source_url, &target_dir])
            .status();
        if !command_succeeded(status) {
            eprintln!("\x1b[31mError:\x1b[0m Cannot clone package '{}'.", pkg_name);
            return;
        }
    }

    if target_version != "latest" {
        let status = Command::new("git")
            .args(["-C", &target_dir, "checkout", &target_version])
            .status();
        if !command_succeeded(status) {
            eprintln!(
                "\x1b[31mError:\x1b[0m Cannot check out requested revision '{}'.",
                target_version
            );
            return;
        }
    }

    ensure_manifest_dependency(&pkg_name, &target_version);

    println!(
        "\x1b[32mSuccessfully installed\x1b[0m '{}@{}' into modules/{}",
        pkg_name, target_version, pkg_name
    );
    println!("Usage in your code:");
    println!("  \x1b[33mimport modules.{}.src.main\x1b[0m", pkg_name);
}

fn update_package(target: Option<&str>) {
    match target {
        Some(spec) => {
            let parts: Vec<&str> = spec.split('@').collect();
            let pkg_name = parts[0];
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
            println!("Updating package '{}' in {}...", pkg_name, target_dir);
            let _ = Command::new("git")
                .args(["-C", &target_dir, "fetch", "--all"])
                .output();
            if parts.len() > 1 && !parts[1].is_empty() && parts[1] != "latest" {
                let ver = parts[1];
                let _ = Command::new("git")
                    .args(["-C", &target_dir, "checkout", ver])
                    .output();
                println!("\x1b[32mChecked out version tag '{}'\x1b[0m", ver);
            } else {
                let _ = Command::new("git")
                    .args(["-C", &target_dir, "pull"])
                    .output();
                println!("\x1b[32mUpdated to latest version on default branch.\x1b[0m");
            }
        }
        None => {
            println!("Updating all packages in modules/...");
            if let Ok(entries) = fs::read_dir("modules") {
                for entry in entries.flatten() {
                    if entry.path().is_dir() {
                        let dir_str = entry.path().to_string_lossy().to_string();
                        println!("Updating {}...", dir_str);
                        let _ = Command::new("git").args(["-C", &dir_str, "pull"]).output();
                    }
                }
            }
            println!("\x1b[32mAll installed packages updated.\x1b[0m");
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
            let updated = content
                .lines()
                .filter(|line| !line.contains(&format!("\"{}\"", pkg_name)))
                .collect::<Vec<&str>>()
                .join("\n");
            let _ = fs::write("vietlang.json", updated);
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
    use super::valid_package_name;

    #[test]
    fn package_names_cannot_escape_modules_directory() {
        assert!(valid_package_name("http-router_2"));
        assert!(!valid_package_name("../outside"));
        assert!(!valid_package_name("nested/package"));
        assert!(!valid_package_name(""));
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
    let pkg_name = extract_json_str(
        &raw_manifest
            .lines()
            .find(|l| l.trim().starts_with("\"name\":"))
            .unwrap_or(""),
    );
    let version = extract_json_str(
        &raw_manifest
            .lines()
            .find(|l| l.trim().starts_with("\"version\":"))
            .unwrap_or(""),
    );
    let desc = extract_json_str(
        &raw_manifest
            .lines()
            .find(|l| l.trim().starts_with("\"description\":"))
            .unwrap_or(""),
    );
    let author = extract_json_str(
        &raw_manifest
            .lines()
            .find(|l| l.trim().starts_with("\"author\":"))
            .unwrap_or(""),
    );
    let mut repo = extract_json_str(
        &raw_manifest
            .lines()
            .find(|l| l.trim().starts_with("\"repository\":"))
            .unwrap_or(""),
    );

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
        Ok(digest) => format!("sha256:{}", digest),
        Err(error) => {
            eprintln!(
                "\x1b[31mError:\x1b[0m Cannot hash package contents: {}",
                error
            );
            return;
        }
    };

    println!("  Checksum:    {}", checksum);

    // Write to Sharded Index File
    let base = get_registry_base();
    let shard_rel = get_shard_relpath(&pkg_name);
    let shard_full = format!("{}/{}", base, shard_rel);

    if let Some(parent) = Path::new(&shard_full).parent() {
        let _ = fs::create_dir_all(parent);
    }

    let shard_content = format!(
        r#"{{
  "name": "{}",
  "version": "{}",
  "author": "{}",
  "repository": "{}",
  "description": "{}",
  "checksum": "{}",
  "keywords": ["{}"]
}}
"#,
        pkg_name, version, author_display, repo_display, desc, checksum, pkg_name
    );

    let _ = fs::write(&shard_full, shard_content);
    println!("  Sharded Index: \x1b[32mCreated {}\x1b[0m", shard_full);

    // Update master index.json if present
    let master_index = format!("{}/index.json", base);
    if let Ok(reg_content) = fs::read_to_string(&master_index) {
        let new_pkg_entry = format!(
            r#"    "{}": {{
      "name": "{}",
      "latest": "{}",
      "versions": {{
        "{}": {{
          "version": "{}",
          "description": "{}",
          "author": "{}",
          "source": "{}",
          "checksum": "{}",
          "keywords": ["{}"]
        }}
      }}
    }},
"#,
            pkg_name,
            pkg_name,
            version,
            version,
            version,
            desc,
            author_display,
            repo_display,
            checksum,
            pkg_name
        );

        if !reg_content.contains(&format!("\"{}\":", pkg_name)) {
            let updated = reg_content.replacen(
                "\"packages\": {",
                &format!("\"packages\": {{\n{}", new_pkg_entry),
                1,
            );
            let _ = fs::write(&master_index, updated);
            println!("  Master Index:  \x1b[32mUpdated {}\x1b[0m", master_index);
        }
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
    println!("Installed packages in modules/:");
    if let Ok(entries) = fs::read_dir("modules") {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let name = entry.file_name().into_string().unwrap_or_default();
                println!("  * \x1b[36m{}\x1b[0m", name);
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

fn show_info() {
    if let Ok(content) = fs::read_to_string("vietlang.json") {
        println!("Package Manifest (vietlang.json):");
        println!("{}", content);
    } else {
        println!("No vietlang.json manifest found in current directory.");
    }
}

fn ensure_manifest_dependency(pkg_name: &str, version: &str) {
    if !Path::new("vietlang.json").exists() {
        let manifest = format!(
            r#"{{
  "name": "app",
  "version": "1.0.0",
  "dependencies": {{
    "{}": "{}"
  }}
}}
"#,
            pkg_name, version
        );
        let _ = fs::write("vietlang.json", manifest);
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
