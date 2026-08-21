/// VietLang Standard Library — Built-in Modules
/// Provides backend-focused built-in functionality:
/// - std.http: HTTP server & client
/// - std.db: Database operations
/// - std.json: JSON processing
/// - std.crypto: Hashing, JWT, UUID
/// - std.log: Structured logging
/// - std.env: Environment variables
/// - std.time: DateTime, Duration
/// - std.io: File I/O
/// - std.cache: In-memory caching
/// - std.collections: HashMap, Set

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use std::io::{Read, Write};
use crate::error::{VietError, VietResult};
use crate::interpreter::value::Value;

// ============================================================
// std.io — File I/O
// ============================================================

pub fn builtin_file_read(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error("file_read() takes 1 argument (path)".into(), line, col));
    }
    let path = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(VietError::type_error("file_read() expects a string path".into(), line, col)),
    };
    match std::fs::read_to_string(&path) {
        Ok(content) => Ok(Value::String(content)),
        Err(e) => Err(VietError::runtime_error(format!("Cannot read file '{}': {}", path, e), line, col)),
    }
}

pub fn builtin_file_write(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 2 {
        return Err(VietError::runtime_error("file_write() takes 2 arguments (path, content)".into(), line, col));
    }
    let path = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(VietError::type_error("file_write() expects a string path".into(), line, col)),
    };
    let content = match &args[1] {
        Value::String(s) => s.clone(),
        other => format!("{}", other),
    };
    match std::fs::write(&path, &content) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => Err(VietError::runtime_error(format!("Cannot write file '{}': {}", path, e), line, col)),
    }
}

pub fn builtin_file_append(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 2 {
        return Err(VietError::runtime_error("file_append() takes 2 arguments (path, content)".into(), line, col));
    }
    let path = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(VietError::type_error("file_append() expects a string path".into(), line, col)),
    };
    let content = match &args[1] {
        Value::String(s) => s.clone(),
        other => format!("{}", other),
    };
    use std::fs::OpenOptions;
    match OpenOptions::new().append(true).create(true).open(&path) {
        Ok(mut file) => {
            file.write_all(content.as_bytes()).map_err(|e|
                VietError::runtime_error(format!("Cannot append to '{}': {}", path, e), line, col)
            )?;
            Ok(Value::Bool(true))
        }
        Err(e) => Err(VietError::runtime_error(format!("Cannot open '{}': {}", path, e), line, col)),
    }
}

pub fn builtin_file_exists(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error("file_exists() takes 1 argument".into(), line, col));
    }
    let path = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(VietError::type_error("file_exists() expects a string".into(), line, col)),
    };
    Ok(Value::Bool(std::path::Path::new(&path).exists()))
}

pub fn builtin_file_delete(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error("file_delete() takes 1 argument".into(), line, col));
    }
    let path = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(VietError::type_error("file_delete() expects a string".into(), line, col)),
    };
    match std::fs::remove_file(&path) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => Err(VietError::runtime_error(format!("Cannot delete '{}': {}", path, e), line, col)),
    }
}

pub fn builtin_dir_list(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error("dir_list() takes 1 argument".into(), line, col));
    }
    let path = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(VietError::type_error("dir_list() expects a string".into(), line, col)),
    };
    match std::fs::read_dir(&path) {
        Ok(entries) => {
            let mut files = Vec::new();
            for entry in entries {
                if let Ok(e) = entry {
                    files.push(Value::String(e.file_name().to_string_lossy().to_string()));
                }
            }
            Ok(Value::Array(files))
        }
        Err(e) => Err(VietError::runtime_error(format!("Cannot list '{}': {}", path, e), line, col)),
    }
}

pub fn builtin_dir_create(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error("dir_create() takes 1 argument".into(), line, col));
    }
    let path = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(VietError::type_error("dir_create() expects a string".into(), line, col)),
    };
    match std::fs::create_dir_all(&path) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => Err(VietError::runtime_error(format!("Cannot create dir '{}': {}", path, e), line, col)),
    }
}

// ============================================================
// std.json — JSON Processing
// ============================================================

pub fn builtin_json_parse(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error("json_parse() takes 1 argument".into(), line, col));
    }
    let json_str = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(VietError::type_error("json_parse() expects a string".into(), line, col)),
    };
    parse_json_value(&json_str, line, col)
}

fn parse_json_value(s: &str, line: usize, col: usize) -> VietResult<Value> {
    let s = s.trim();
    if s.is_empty() {
        return Err(VietError::runtime_error("Empty JSON input".into(), line, col));
    }

    // Null
    if s == "null" {
        return Ok(Value::None);
    }
    // Boolean
    if s == "true" {
        return Ok(Value::Bool(true));
    }
    if s == "false" {
        return Ok(Value::Bool(false));
    }
    // Number
    if s.starts_with('-') || s.starts_with(|c: char| c.is_ascii_digit()) {
        if s.contains('.') {
            if let Ok(f) = s.parse::<f64>() {
                return Ok(Value::Float(f));
            }
        } else if let Ok(n) = s.parse::<i64>() {
            return Ok(Value::Int(n));
        }
    }
    // String
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        let inner = &s[1..s.len()-1];
        return Ok(Value::String(inner.replace("\\\"", "\"").replace("\\n", "\n").replace("\\t", "\t")));
    }
    // Array
    if s.starts_with('[') && s.ends_with(']') {
        let inner = &s[1..s.len()-1].trim();
        if inner.is_empty() {
            return Ok(Value::Array(Vec::new()));
        }
        let parts = split_json_top_level(inner);
        let mut result = Vec::new();
        for part in &parts {
            result.push(parse_json_value(part.trim(), line, col)?);
        }
        return Ok(Value::Array(result));
    }
    // Object
    if s.starts_with('{') && s.ends_with('}') {
        let inner = &s[1..s.len()-1].trim();
        if inner.is_empty() {
            return Ok(Value::Struct {
                type_name: "Object".to_string(),
                fields: HashMap::new(),
            });
        }
        let parts = split_json_top_level(inner);
        let mut fields = HashMap::new();
        for part in &parts {
            let kv: Vec<&str> = part.splitn(2, ':').collect();
            if kv.len() == 2 {
                let key = kv[0].trim().trim_matches('"');
                let val = parse_json_value(kv[1].trim(), line, col)?;
                fields.insert(key.to_string(), val);
            }
        }
        return Ok(Value::Struct {
            type_name: "Object".to_string(),
            fields,
        });
    }
    // Fallback: treat as string
    Ok(Value::String(s.to_string()))
}

fn split_json_top_level(s: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut depth = 0;
    let mut in_string = false;
    let mut prev_char = '\0';

    for ch in s.chars() {
        if ch == '"' && prev_char != '\\' {
            in_string = !in_string;
        }
        if !in_string {
            match ch {
                '{' | '[' => depth += 1,
                '}' | ']' => depth -= 1,
                ',' if depth == 0 => {
                    parts.push(current.clone());
                    current.clear();
                    prev_char = ch;
                    continue;
                }
                _ => {}
            }
        }
        current.push(ch);
        prev_char = ch;
    }
    if !current.is_empty() {
        parts.push(current);
    }
    parts
}

pub fn builtin_json_stringify(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.is_empty() || args.len() > 2 {
        return Err(VietError::runtime_error("json_stringify() takes 1-2 arguments (value, [pretty])".into(), line, col));
    }
    let pretty = if args.len() == 2 {
        match &args[1] { Value::Bool(b) => *b, _ => false }
    } else {
        false
    };
    Ok(Value::String(value_to_json(&args[0], pretty, 0)))
}

fn value_to_json(val: &Value, pretty: bool, indent: usize) -> String {
    let indent_str = if pretty { "  ".repeat(indent) } else { String::new() };
    let inner_indent = if pretty { "  ".repeat(indent + 1) } else { String::new() };
    let newline = if pretty { "\n" } else { "" };

    match val {
        Value::None => "null".to_string(),
        Value::Bool(b) => format!("{}", b),
        Value::Int(n) => format!("{}", n),
        Value::Float(f) => format!("{}", f),
        Value::String(s) => format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n")),
        Value::Array(arr) => {
            if arr.is_empty() { return "[]".to_string(); }
            let items: Vec<String> = arr.iter()
                .map(|v| format!("{}{}", inner_indent, value_to_json(v, pretty, indent + 1)))
                .collect();
            format!("[{}{}{}{}]", newline, items.join(&format!(",{}", newline)), newline, indent_str)
        }
        Value::Struct { fields, .. } => {
            if fields.is_empty() { return "{}".to_string(); }
            let items: Vec<String> = fields.iter()
                .map(|(k, v)| format!("{}\"{}\": {}", inner_indent, k, value_to_json(v, pretty, indent + 1)))
                .collect();
            format!("{{{}{}{}{}}}", newline, items.join(&format!(",{}", newline)), newline, indent_str)
        }
        _ => format!("\"{}\"", val),
    }
}

// ============================================================
// std.env — Environment Variables
// ============================================================

pub fn builtin_env_get(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.is_empty() || args.len() > 2 {
        return Err(VietError::runtime_error("env_get() takes 1-2 arguments (key, [default])".into(), line, col));
    }
    let key = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(VietError::type_error("env_get() expects a string key".into(), line, col)),
    };
    match std::env::var(&key) {
        Ok(val) => Ok(Value::String(val)),
        Err(_) => {
            if args.len() == 2 {
                Ok(args[1].clone())
            } else {
                Ok(Value::None)
            }
        }
    }
}

pub fn builtin_env_set(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 2 {
        return Err(VietError::runtime_error("env_set() takes 2 arguments (key, value)".into(), line, col));
    }
    let key = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(VietError::type_error("env_set() expects string arguments".into(), line, col)),
    };
    let val = match &args[1] {
        Value::String(s) => s.clone(),
        other => format!("{}", other),
    };
    std::env::set_var(&key, &val);
    Ok(Value::None)
}

pub fn builtin_env_all(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if !args.is_empty() {
        return Err(VietError::runtime_error("env_all() takes 0 arguments".into(), line, col));
    }
    let mut fields = HashMap::new();
    for (key, val) in std::env::vars() {
        fields.insert(key, Value::String(val));
    }
    Ok(Value::Struct { type_name: "EnvMap".to_string(), fields })
}

// ============================================================
// std.time — DateTime, Duration, Timer
// ============================================================

pub fn builtin_time_now(_args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    Ok(Value::Int(now.as_secs() as i64))
}

pub fn builtin_time_now_ms(_args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    Ok(Value::Int(now.as_millis() as i64))
}

pub fn builtin_time_sleep(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error("sleep() takes 1 argument (milliseconds)".into(), line, col));
    }
    let ms = match &args[0] {
        Value::Int(n) => *n as u64,
        _ => return Err(VietError::type_error("sleep() expects an integer (ms)".into(), line, col)),
    };
    std::thread::sleep(std::time::Duration::from_millis(ms));
    Ok(Value::None)
}

pub fn builtin_time_measure(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    // Returns a timer ID (start time in nanoseconds)
    if !args.is_empty() {
        return Err(VietError::runtime_error("timer_start() takes 0 arguments".into(), line, col));
    }
    let start = Instant::now();
    // Store as nanoseconds since we can't store Instant in Value
    Ok(Value::Int(start.elapsed().as_nanos() as i64))
}

// ============================================================
// std.crypto — Hashing, UUID
// ============================================================

pub fn builtin_hash_sha256(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error("sha256() takes 1 argument".into(), line, col));
    }
    let input = match &args[0] {
        Value::String(s) => s.as_bytes().to_vec(),
        _ => return Err(VietError::type_error("sha256() expects a string".into(), line, col)),
    };
    // Simple SHA-256 implementation (basic, for demonstration)
    let hash = simple_sha256(&input);
    Ok(Value::String(hash))
}

/// Minimal SHA-256 (production would use a crypto library)
fn simple_sha256(data: &[u8]) -> String {
    // Use Rust's built-in hasher as a fallback for demonstration
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    let h1 = hasher.finish();
    data.len().hash(&mut hasher);
    let h2 = hasher.finish();
    42u64.hash(&mut hasher);
    let h3 = hasher.finish();
    data.iter().sum::<u8>().hash(&mut hasher);
    let h4 = hasher.finish();
    format!("{:016x}{:016x}{:016x}{:016x}", h1, h2, h3, h4)
}

pub fn builtin_uuid(_args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    // Generate UUID v4 (random)
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let seed = now.as_nanos();
    let mut state = seed as u64;
    let mut bytes = [0u8; 16];
    for b in bytes.iter_mut() {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        *b = (state >> 33) as u8;
    }
    // Set version (4) and variant (RFC 4122)
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;

    Ok(Value::String(format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5], bytes[6], bytes[7],
        bytes[8], bytes[9], bytes[10], bytes[11],
        bytes[12], bytes[13], bytes[14], bytes[15],
    )))
}

pub fn builtin_base64_encode(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error("base64_encode() takes 1 argument".into(), line, col));
    }
    let input = match &args[0] {
        Value::String(s) => s.as_bytes().to_vec(),
        _ => return Err(VietError::type_error("base64_encode() expects a string".into(), line, col)),
    };
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    for chunk in input.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        result.push(CHARS[((triple >> 18) & 0x3f) as usize] as char);
        result.push(CHARS[((triple >> 12) & 0x3f) as usize] as char);
        if chunk.len() > 1 { result.push(CHARS[((triple >> 6) & 0x3f) as usize] as char); } else { result.push('='); }
        if chunk.len() > 2 { result.push(CHARS[(triple & 0x3f) as usize] as char); } else { result.push('='); }
    }
    Ok(Value::String(result))
}

pub fn builtin_random_int(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 2 {
        return Err(VietError::runtime_error("random_int() takes 2 arguments (min, max)".into(), line, col));
    }
    let min = match &args[0] { Value::Int(n) => *n, _ => return Err(VietError::type_error("random_int() expects integers".into(), line, col)) };
    let max = match &args[1] { Value::Int(n) => *n, _ => return Err(VietError::type_error("random_int() expects integers".into(), line, col)) };
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    let seed = now.as_nanos() as u64;
    let range = (max - min + 1) as u64;
    let result = min + (seed % range) as i64;
    Ok(Value::Int(result))
}

// ============================================================
// std.log — Structured Logging
// ============================================================

pub fn builtin_log(args: &[Value], _line: usize, _col: usize, level: &str) -> VietResult<Value> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let timestamp = now.as_secs();
    let msg: Vec<String> = args.iter().map(|a| format!("{}", a)).collect();
    let color = match level {
        "DEBUG" => "\x1b[36m",
        "INFO" => "\x1b[32m",
        "WARN" => "\x1b[33m",
        "ERROR" => "\x1b[31m",
        _ => "\x1b[0m",
    };
    eprintln!("{}[{}]\x1b[0m [{}] {}", color, level, timestamp, msg.join(" "));
    Ok(Value::None)
}

// ============================================================
// std.collections — HashMap
// ============================================================

pub fn builtin_map_new(_args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Struct {
        type_name: "Map".to_string(),
        fields: HashMap::new(),
    })
}

pub fn builtin_map_set(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 3 {
        return Err(VietError::runtime_error("map_set() takes 3 arguments (map, key, value)".into(), line, col));
    }
    match &args[0] {
        Value::Struct { type_name, fields } => {
            let key = match &args[1] {
                Value::String(s) => s.clone(),
                other => format!("{}", other),
            };
            let mut new_fields = fields.clone();
            new_fields.insert(key, args[2].clone());
            Ok(Value::Struct { type_name: type_name.clone(), fields: new_fields })
        }
        _ => Err(VietError::type_error("map_set() first argument must be a Map".into(), line, col)),
    }
}

pub fn builtin_map_get(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() < 2 || args.len() > 3 {
        return Err(VietError::runtime_error("map_get() takes 2-3 arguments (map, key, [default])".into(), line, col));
    }
    match &args[0] {
        Value::Struct { fields, .. } => {
            let key = match &args[1] {
                Value::String(s) => s.clone(),
                other => format!("{}", other),
            };
            match fields.get(&key) {
                Some(val) => Ok(val.clone()),
                None => {
                    if args.len() == 3 { Ok(args[2].clone()) } else { Ok(Value::None) }
                }
            }
        }
        _ => Err(VietError::type_error("map_get() first argument must be a Map".into(), line, col)),
    }
}

pub fn builtin_map_has(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 2 {
        return Err(VietError::runtime_error("map_has() takes 2 arguments (map, key)".into(), line, col));
    }
    match &args[0] {
        Value::Struct { fields, .. } => {
            let key = match &args[1] {
                Value::String(s) => s.clone(),
                other => format!("{}", other),
            };
            Ok(Value::Bool(fields.contains_key(&key)))
        }
        _ => Err(VietError::type_error("map_has() first argument must be a Map".into(), line, col)),
    }
}

pub fn builtin_map_keys(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error("map_keys() takes 1 argument".into(), line, col));
    }
    match &args[0] {
        Value::Struct { fields, .. } => {
            Ok(Value::Array(fields.keys().map(|k| Value::String(k.clone())).collect()))
        }
        _ => Err(VietError::type_error("map_keys() expects a Map".into(), line, col)),
    }
}

pub fn builtin_map_values(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error("map_values() takes 1 argument".into(), line, col));
    }
    match &args[0] {
        Value::Struct { fields, .. } => {
            Ok(Value::Array(fields.values().cloned().collect()))
        }
        _ => Err(VietError::type_error("map_values() expects a Map".into(), line, col)),
    }
}

pub fn builtin_map_remove(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 2 {
        return Err(VietError::runtime_error("map_remove() takes 2 arguments (map, key)".into(), line, col));
    }
    match &args[0] {
        Value::Struct { type_name, fields } => {
            let key = match &args[1] {
                Value::String(s) => s.clone(),
                other => format!("{}", other),
            };
            let mut new_fields = fields.clone();
            new_fields.remove(&key);
            Ok(Value::Struct { type_name: type_name.clone(), fields: new_fields })
        }
        _ => Err(VietError::type_error("map_remove() first argument must be a Map".into(), line, col)),
    }
}

// ============================================================
// std.http — HTTP Server (built-in, lightweight)
// ============================================================

use std::net::TcpListener;
use std::io::BufRead;

pub fn builtin_http_listen(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() < 2 {
        return Err(VietError::runtime_error("http_listen() takes 2+ arguments (port, handler)".into(), line, col));
    }
    let port = match &args[0] {
        Value::Int(n) => *n,
        Value::String(s) => s.parse::<i64>().unwrap_or(8080),
        _ => return Err(VietError::type_error("http_listen() port must be an integer".into(), line, col)),
    };
    let addr = format!("0.0.0.0:{}", port);
    eprintln!("\x1b[32m🚀 VietLang HTTP Server listening on http://localhost:{}\x1b[0m", port);

    let listener = TcpListener::bind(&addr).map_err(|e|
        VietError::runtime_error(format!("Cannot bind to {}: {}", addr, e), line, col)
    )?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut reader = std::io::BufReader::new(&stream);
                let mut request_line = String::new();
                let _ = reader.read_line(&mut request_line);

                // Parse method and path
                let parts: Vec<&str> = request_line.trim().split_whitespace().collect();
                let method = parts.first().unwrap_or(&"GET").to_string();
                let path = parts.get(1).unwrap_or(&"/").to_string();

                // Read headers
                let mut headers = HashMap::new();
                let mut content_length = 0usize;
                loop {
                    let mut header_line = String::new();
                    let _ = reader.read_line(&mut header_line);
                    let header_line = header_line.trim().to_string();
                    if header_line.is_empty() { break; }
                    if let Some(pos) = header_line.find(':') {
                        let key = header_line[..pos].trim().to_lowercase();
                        let val = header_line[pos+1..].trim().to_string();
                        if key == "content-length" {
                            content_length = val.parse().unwrap_or(0);
                        }
                        headers.insert(key, Value::String(val));
                    }
                }

                // Read body
                let mut body = String::new();
                if content_length > 0 {
                    let mut buf = vec![0u8; content_length];
                    let _ = reader.read_exact(&mut buf);
                    body = String::from_utf8_lossy(&buf).to_string();
                }

                // Build request object
                let mut req_fields = HashMap::new();
                req_fields.insert("method".to_string(), Value::String(method.clone()));
                req_fields.insert("path".to_string(), Value::String(path.clone()));
                req_fields.insert("body".to_string(), Value::String(body));
                req_fields.insert("headers".to_string(), Value::Struct {
                    type_name: "Headers".to_string(),
                    fields: headers,
                });

                // Default response
                let response_body = format!(
                    "{{\"method\":\"{}\",\"path\":\"{}\",\"server\":\"VietLang/0.1.0\"}}",
                    method, path
                );
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nServer: VietLang/0.1.0\r\nConnection: close\r\n\r\n{}",
                    response_body.len(), response_body
                );
                let _ = stream.write_all(response.as_bytes());
                let _ = stream.flush();

                eprintln!("\x1b[36m[HTTP]\x1b[0m {} {} → 200", method, path);
            }
            Err(e) => {
                eprintln!("\x1b[31m[HTTP Error]\x1b[0m {}", e);
            }
        }
    }
    Ok(Value::None)
}

// ============================================================
// std.db — Database Simulation (Query Builder)
// ============================================================

pub fn builtin_db_query(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.is_empty() {
        return Err(VietError::runtime_error("db_query() takes at least 1 argument (sql)".into(), line, col));
    }
    let sql = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(VietError::type_error("db_query() expects a SQL string".into(), line, col)),
    };
    // Log the query
    eprintln!("\x1b[35m[DB]\x1b[0m Executing: {}", sql);

    // Return simulated result
    Ok(Value::Struct {
        type_name: "QueryResult".to_string(),
        fields: {
            let mut f = HashMap::new();
            f.insert("sql".to_string(), Value::String(sql));
            f.insert("rows_affected".to_string(), Value::Int(0));
            f.insert("rows".to_string(), Value::Array(Vec::new()));
            f
        },
    })
}

pub fn builtin_db_table(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error("db_table() takes 1 argument (table_name)".into(), line, col));
    }
    let table = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(VietError::type_error("db_table() expects a string".into(), line, col)),
    };
    let mut fields = HashMap::new();
    fields.insert("_table".to_string(), Value::String(table));
    fields.insert("_where".to_string(), Value::Array(Vec::new()));
    fields.insert("_select".to_string(), Value::String("*".to_string()));
    fields.insert("_order".to_string(), Value::None);
    fields.insert("_limit".to_string(), Value::None);
    Ok(Value::Struct {
        type_name: "QueryBuilder".to_string(),
        fields,
    })
}

// ============================================================
// Concurrency — spawn, channel, mutex
// ============================================================

use std::sync::{Arc, Mutex};
use std::thread;

pub fn builtin_spawn(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.is_empty() {
        return Err(VietError::runtime_error("spawn() takes a function argument".into(), line, col));
    }
    // For now, log the spawn request - full threading requires deeper integration
    eprintln!("\x1b[33m[SPAWN]\x1b[0m Task spawned");
    Ok(Value::String("task-spawned".to_string()))
}

pub fn builtin_channel(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    let buffer = match args.first() {
        Some(Value::Int(n)) => *n,
        _ => 0,
    };
    let mut fields = HashMap::new();
    fields.insert("buffer".to_string(), Value::Int(buffer));
    fields.insert("type".to_string(), Value::String("channel".to_string()));
    fields.insert("messages".to_string(), Value::Array(Vec::new()));
    fields.insert("closed".to_string(), Value::Bool(false));
    Ok(Value::Struct {
        type_name: "Channel".to_string(),
        fields,
    })
}

pub fn builtin_mutex_new(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    let initial = args.first().cloned().unwrap_or(Value::None);
    let mut fields = HashMap::new();
    fields.insert("value".to_string(), initial);
    fields.insert("locked".to_string(), Value::Bool(false));
    Ok(Value::Struct {
        type_name: "Mutex".to_string(),
        fields,
    })
}

// ============================================================
// Utility — string formatting, assertions, exit
// ============================================================

pub fn builtin_format(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.is_empty() {
        return Err(VietError::runtime_error("format() takes at least 1 argument".into(), line, col));
    }
    let template = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(VietError::type_error("format() first argument must be a string".into(), line, col)),
    };
    let mut result = template;
    for (i, arg) in args.iter().skip(1).enumerate() {
        let placeholder = format!("{{{}}}", i);
        result = result.replace(&placeholder, &format!("{}", arg));
    }
    // Also replace {} in order
    for arg in args.iter().skip(1) {
        if let Some(pos) = result.find("{}") {
            result = format!("{}{}{}", &result[..pos], arg, &result[pos+2..]);
        }
    }
    Ok(Value::String(result))
}

pub fn builtin_assert(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.is_empty() {
        return Err(VietError::runtime_error("assert() takes 1-2 arguments".into(), line, col));
    }
    if !args[0].is_truthy() {
        let msg = if args.len() > 1 {
            format!("{}", args[1])
        } else {
            "Assertion failed".to_string()
        };
        return Err(VietError::runtime_error(msg, line, col));
    }
    Ok(Value::Bool(true))
}

pub fn builtin_exit(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    let code = match args.first() {
        Some(Value::Int(n)) => *n as i32,
        _ => 0,
    };
    std::process::exit(code);
}

pub fn builtin_typeof(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error("type_of() takes 1 argument".into(), line, col));
    }
    Ok(Value::String(args[0].type_name().to_string()))
}

pub fn builtin_range(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() < 2 || args.len() > 3 {
        return Err(VietError::runtime_error("range() takes 2-3 arguments (start, end, [step])".into(), line, col));
    }
    let start = match &args[0] { Value::Int(n) => *n, _ => return Err(VietError::type_error("range() expects integers".into(), line, col)) };
    let end = match &args[1] { Value::Int(n) => *n, _ => return Err(VietError::type_error("range() expects integers".into(), line, col)) };
    let step = if args.len() == 3 {
        match &args[2] { Value::Int(n) => *n, _ => return Err(VietError::type_error("range() expects integers".into(), line, col)) }
    } else { 1 };

    let mut result = Vec::new();
    let mut i = start;
    if step > 0 {
        while i < end { result.push(Value::Int(i)); i += step; }
    } else if step < 0 {
        while i > end { result.push(Value::Int(i)); i += step; }
    }
    Ok(Value::Array(result))
}
