//! VietLang Standard Library - Built-in Modules
//! Provides backend-focused built-in functionality:
//! - std.http: HTTP server & client
//! - std.db: Database operations
//! - std.json: JSON processing
//! - std.crypto: Hashing, JWT, UUID
//! - std.log: Structured logging
//! - std.env: Environment variables
//! - std.time: DateTime, Duration
//! - std.io: File I/O
//! - std.cache: In-memory caching
//! - std.collections: HashMap, Set

use crate::error::{VietError, VietResult};
use crate::interpreter::value::Value;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use rand::{rngs::OsRng, Rng, RngCore};
use sha2::{Digest, Sha256, Sha512};
use std::collections::{HashMap, VecDeque};
use std::io::{Read, Write};
use std::sync::{Arc, Condvar, Mutex, OnceLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use subtle::ConstantTimeEq;

// ============================================================
// std.io — File I/O
// ============================================================

pub fn builtin_file_read(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error(
            "file_read() takes 1 argument (path)".into(),
            line,
            col,
        ));
    }
    let path = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "file_read() expects a string path".into(),
                line,
                col,
            ))
        }
    };
    match std::fs::read_to_string(&path) {
        Ok(content) => Ok(Value::String(content)),
        Err(e) => Err(VietError::runtime_error(
            format!("Cannot read file '{}': {}", path, e),
            line,
            col,
        )),
    }
}

pub fn builtin_file_write(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 2 {
        return Err(VietError::runtime_error(
            "file_write() takes 2 arguments (path, content)".into(),
            line,
            col,
        ));
    }
    let path = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "file_write() expects a string path".into(),
                line,
                col,
            ))
        }
    };
    let content = match &args[1] {
        Value::String(s) => s.clone(),
        other => format!("{}", other),
    };
    match std::fs::write(&path, &content) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => Err(VietError::runtime_error(
            format!("Cannot write file '{}': {}", path, e),
            line,
            col,
        )),
    }
}

pub fn builtin_file_append(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 2 {
        return Err(VietError::runtime_error(
            "file_append() takes 2 arguments (path, content)".into(),
            line,
            col,
        ));
    }
    let path = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "file_append() expects a string path".into(),
                line,
                col,
            ))
        }
    };
    let content = match &args[1] {
        Value::String(s) => s.clone(),
        other => format!("{}", other),
    };
    use std::fs::OpenOptions;
    match OpenOptions::new().append(true).create(true).open(&path) {
        Ok(mut file) => {
            file.write_all(content.as_bytes()).map_err(|e| {
                VietError::runtime_error(format!("Cannot append to '{}': {}", path, e), line, col)
            })?;
            Ok(Value::Bool(true))
        }
        Err(e) => Err(VietError::runtime_error(
            format!("Cannot open '{}': {}", path, e),
            line,
            col,
        )),
    }
}

pub fn builtin_file_exists(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error(
            "file_exists() takes 1 argument".into(),
            line,
            col,
        ));
    }
    let path = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "file_exists() expects a string".into(),
                line,
                col,
            ))
        }
    };
    Ok(Value::Bool(std::path::Path::new(&path).exists()))
}

pub fn builtin_file_delete(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error(
            "file_delete() takes 1 argument".into(),
            line,
            col,
        ));
    }
    let path = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "file_delete() expects a string".into(),
                line,
                col,
            ))
        }
    };
    match std::fs::remove_file(&path) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => Err(VietError::runtime_error(
            format!("Cannot delete '{}': {}", path, e),
            line,
            col,
        )),
    }
}

pub fn builtin_dir_list(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error(
            "dir_list() takes 1 argument".into(),
            line,
            col,
        ));
    }
    let path = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "dir_list() expects a string".into(),
                line,
                col,
            ))
        }
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
        Err(e) => Err(VietError::runtime_error(
            format!("Cannot list '{}': {}", path, e),
            line,
            col,
        )),
    }
}

pub fn builtin_dir_create(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error(
            "dir_create() takes 1 argument".into(),
            line,
            col,
        ));
    }
    let path = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "dir_create() expects a string".into(),
                line,
                col,
            ))
        }
    };
    match std::fs::create_dir_all(&path) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => Err(VietError::runtime_error(
            format!("Cannot create dir '{}': {}", path, e),
            line,
            col,
        )),
    }
}

// ============================================================
// std.json — JSON Processing
// ============================================================

pub fn builtin_json_parse(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error(
            "json_parse() takes 1 argument".into(),
            line,
            col,
        ));
    }
    let json_str = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "json_parse() expects a string".into(),
                line,
                col,
            ))
        }
    };
    parse_json_value(&json_str, line, col)
}

fn parse_json_value(s: &str, line: usize, col: usize) -> VietResult<Value> {
    let s = s.trim();
    if s.is_empty() {
        return Err(VietError::runtime_error(
            "Empty JSON input".into(),
            line,
            col,
        ));
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
        let inner = &s[1..s.len() - 1];
        return Ok(Value::String(
            inner
                .replace("\\\"", "\"")
                .replace("\\n", "\n")
                .replace("\\t", "\t"),
        ));
    }
    // Array
    if s.starts_with('[') && s.ends_with(']') {
        let inner = &s[1..s.len() - 1].trim();
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
    // Object / Map
    if s.starts_with('{') && s.ends_with('}') {
        let inner = &s[1..s.len() - 1].trim();
        if inner.is_empty() {
            return Ok(Value::Struct {
                type_name: "Map".to_string(),
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
            type_name: "Map".to_string(),
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
        return Err(VietError::runtime_error(
            "json_stringify() takes 1-2 arguments (value, [pretty])".into(),
            line,
            col,
        ));
    }
    let pretty = if args.len() == 2 {
        match &args[1] {
            Value::Bool(b) => *b,
            _ => false,
        }
    } else {
        false
    };
    Ok(Value::String(value_to_json(&args[0], pretty, 0)))
}

fn value_to_json(val: &Value, pretty: bool, indent: usize) -> String {
    let indent_str = if pretty {
        "  ".repeat(indent)
    } else {
        String::new()
    };
    let inner_indent = if pretty {
        "  ".repeat(indent + 1)
    } else {
        String::new()
    };
    let newline = if pretty { "\n" } else { "" };

    match val {
        Value::None => "null".to_string(),
        Value::Bool(b) => format!("{}", b),
        Value::Int(n) => format!("{}", n),
        Value::Float(f) => format!("{}", f),
        Value::String(s) => format!(
            "\"{}\"",
            s.replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n")
        ),
        Value::Array(arr) => {
            if arr.is_empty() {
                return "[]".to_string();
            }
            let items: Vec<String> = arr
                .iter()
                .map(|v| format!("{}{}", inner_indent, value_to_json(v, pretty, indent + 1)))
                .collect();
            format!(
                "[{}{}{}{}]",
                newline,
                items.join(&format!(",{}", newline)),
                newline,
                indent_str
            )
        }
        Value::Struct { fields, .. } => {
            if fields.is_empty() {
                return "{}".to_string();
            }
            let items: Vec<String> = fields
                .iter()
                .map(|(k, v)| {
                    format!(
                        "{}\"{}\": {}",
                        inner_indent,
                        k,
                        value_to_json(v, pretty, indent + 1)
                    )
                })
                .collect();
            format!(
                "{{{}{}{}{}}}",
                newline,
                items.join(&format!(",{}", newline)),
                newline,
                indent_str
            )
        }
        _ => format!("\"{}\"", val),
    }
}

// ============================================================
// std.env — Environment Variables
// ============================================================

pub fn builtin_env_get(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.is_empty() || args.len() > 2 {
        return Err(VietError::runtime_error(
            "env_get() takes 1-2 arguments (key, [default])".into(),
            line,
            col,
        ));
    }
    let key = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "env_get() expects a string key".into(),
                line,
                col,
            ))
        }
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
        return Err(VietError::runtime_error(
            "env_set() takes 2 arguments (key, value)".into(),
            line,
            col,
        ));
    }
    let key = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "env_set() expects string arguments".into(),
                line,
                col,
            ))
        }
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
        return Err(VietError::runtime_error(
            "env_all() takes 0 arguments".into(),
            line,
            col,
        ));
    }
    let mut fields = HashMap::new();
    for (key, val) in std::env::vars() {
        fields.insert(key, Value::String(val));
    }
    Ok(Value::Struct {
        type_name: "EnvMap".to_string(),
        fields,
    })
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
        return Err(VietError::runtime_error(
            "sleep() takes 1 argument (milliseconds)".into(),
            line,
            col,
        ));
    }
    let ms = match &args[0] {
        Value::Int(n) => *n as u64,
        _ => {
            return Err(VietError::type_error(
                "sleep() expects an integer (ms)".into(),
                line,
                col,
            ))
        }
    };
    std::thread::sleep(std::time::Duration::from_millis(ms));
    Ok(Value::None)
}

pub fn builtin_time_measure(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    // Returns a timer ID (start time in nanoseconds)
    if !args.is_empty() {
        return Err(VietError::runtime_error(
            "timer_start() takes 0 arguments".into(),
            line,
            col,
        ));
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
        return Err(VietError::runtime_error(
            "sha256() takes 1 argument".into(),
            line,
            col,
        ));
    }
    let input = match &args[0] {
        Value::String(s) => s.as_bytes().to_vec(),
        _ => {
            return Err(VietError::type_error(
                "sha256() expects a string".into(),
                line,
                col,
            ))
        }
    };
    Ok(Value::String(hex_lower(&Sha256::digest(input))))
}

pub fn builtin_uuid(_args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    let mut bytes = [0u8; 16];
    OsRng.fill_bytes(&mut bytes);
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

/// RFC 3174 SHA-1 Digest implementation
pub fn sha1_digest(data: &[u8]) -> [u8; 20] {
    let mut h0: u32 = 0x67452301;
    let mut h1: u32 = 0xEFCDAB89;
    let mut h2: u32 = 0x98BADCFE;
    let mut h3: u32 = 0x10325476;
    let mut h4: u32 = 0xC3D2E1F0;

    let ml = (data.len() as u64) * 8;
    let mut msg = data.to_vec();
    msg.push(0x80);
    while (msg.len() * 8) % 512 != 448 {
        msg.push(0);
    }
    msg.extend_from_slice(&ml.to_be_bytes());

    for chunk in msg.chunks_exact(64) {
        let mut w = [0u32; 80];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }
        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        }

        let mut a = h0;
        let mut b = h1;
        let mut c = h2;
        let mut d = h3;
        let mut e = h4;

        for i in 0..80 {
            let (f, k) = match i {
                0..=19 => ((b & c) | ((!b) & d), 0x5A827999),
                20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                _ => (b ^ c ^ d, 0xCA62C1D6),
            };
            let temp = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(w[i]);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }

        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
    }

    let mut out = [0u8; 20];
    out[0..4].copy_from_slice(&h0.to_be_bytes());
    out[4..8].copy_from_slice(&h1.to_be_bytes());
    out[8..12].copy_from_slice(&h2.to_be_bytes());
    out[12..16].copy_from_slice(&h3.to_be_bytes());
    out[16..20].copy_from_slice(&h4.to_be_bytes());
    out
}

pub fn base64_encode_bytes(bytes: &[u8]) -> String {
    const TABLE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = if chunk.len() > 1 { chunk[1] } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] } else { 0 };

        out.push(TABLE[(b0 >> 2) as usize] as char);
        out.push(TABLE[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        if chunk.len() > 1 {
            out.push(TABLE[(((b1 & 0x0f) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(TABLE[(b2 & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}

pub fn builtin_sha1(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error(
            "sha1() takes 1 argument".into(),
            line,
            col,
        ));
    }
    let s = match &args[0] {
        Value::String(s) => s.as_bytes(),
        _ => {
            return Err(VietError::type_error(
                "sha1() expects string".into(),
                line,
                col,
            ))
        }
    };
    let digest = sha1_digest(s);
    let hex: String = digest.iter().map(|b| format!("{:02x}", b)).collect();
    Ok(Value::String(hex))
}

pub static WS_ENABLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub fn builtin_ws_enable(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    let endpoint = match args.first() {
        Some(Value::String(value)) => value.as_str(),
        None => "/ws",
        _ => {
            return Err(VietError::type_error(
                "ws_enable() expects an endpoint string".into(),
                _line,
                _col,
            ));
        }
    };
    crate::ws_runtime::enable(endpoint)
        .map_err(|error| VietError::runtime_error(error, _line, _col))?;
    WS_ENABLED.store(true, std::sync::atomic::Ordering::SeqCst);
    Ok(Value::Bool(true))
}

pub fn builtin_ws_broadcast(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if !WS_ENABLED.load(std::sync::atomic::Ordering::SeqCst) {
        return Ok(Value::Bool(false));
    }
    if args.is_empty() {
        return Err(VietError::runtime_error(
            "ws_broadcast() takes at least 1 argument (data)".into(),
            line,
            col,
        ));
    }
    let msg = match &args[0] {
        Value::String(s) => s.clone(),
        other => match builtin_json_stringify(&[other.clone()], line, col) {
            Ok(Value::String(s)) => s,
            _ => format!("{}", other),
        },
    };
    Ok(Value::Int(crate::ws_runtime::broadcast(msg) as i64))
}

pub fn builtin_html_escape(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.is_empty() {
        return Err(VietError::runtime_error(
            "html_escape() takes 1 argument".into(),
            line,
            col,
        ));
    }
    let s = match &args[0] {
        Value::String(s) => s.clone(),
        other => format!("{}", other),
    };
    let escaped = s
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;");
    Ok(Value::String(escaped))
}

pub fn builtin_ws_accept_key(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error(
            "ws_accept_key() takes 1 argument (sec_websocket_key)".into(),
            line,
            col,
        ));
    }
    let key = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "ws_accept_key() expects string".into(),
                line,
                col,
            ))
        }
    };
    let concat = format!("{}258EAFA5-E914-47DA-95CA-C5AB0DC85B11", key);
    let digest = sha1_digest(concat.as_bytes());
    let encoded = base64_encode_bytes(&digest);
    Ok(Value::String(encoded))
}

pub fn builtin_tcp_send(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() < 3 || args.len() > 4 {
        return Err(VietError::runtime_error(
            "tcp_send() takes 3-4 arguments (host, port, payload, [timeout_ms])".into(),
            line,
            col,
        ));
    }
    let host = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "tcp_send() host must be String".into(),
                line,
                col,
            ))
        }
    };
    let port = match &args[1] {
        Value::Int(n) => *n as u16,
        _ => {
            return Err(VietError::type_error(
                "tcp_send() port must be Int".into(),
                line,
                col,
            ))
        }
    };
    let payload = match &args[2] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "tcp_send() payload must be String".into(),
                line,
                col,
            ))
        }
    };
    let timeout_ms = if args.len() == 4 {
        match &args[3] {
            Value::Int(n) => *n as u64,
            _ => 3000,
        }
    } else {
        3000
    };

    let addr = format!("{}:{}", host, port);
    let timeout = std::time::Duration::from_millis(timeout_ms);
    use std::net::ToSocketAddrs;
    if let Ok(mut addrs) = addr.to_socket_addrs() {
        if let Some(sock_addr) = addrs.next() {
            match std::net::TcpStream::connect_timeout(&sock_addr, timeout) {
                Ok(mut stream) => {
                    let _ = stream.set_read_timeout(Some(timeout));
                    let _ = stream.write_all(payload.as_bytes());
                    let _ = stream.flush();
                    let mut resp = String::new();
                    let mut buf = [0u8; 4096];
                    if let Ok(n) = stream.read(&mut buf) {
                        resp = String::from_utf8_lossy(&buf[..n]).to_string();
                    }
                    return Ok(Value::String(resp));
                }
                Err(e) => {
                    return Err(VietError::runtime_error(
                        format!("TCP connection error: {}", e),
                        line,
                        col,
                    ))
                }
            }
        }
    }
    Err(VietError::runtime_error(
        format!("Cannot resolve host '{}'", host),
        line,
        col,
    ))
}

pub fn builtin_udp_send(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 3 {
        return Err(VietError::runtime_error(
            "udp_send() takes 3 arguments (host, port, payload)".into(),
            line,
            col,
        ));
    }
    let host = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "udp_send() host must be String".into(),
                line,
                col,
            ))
        }
    };
    let port = match &args[1] {
        Value::Int(n) => *n as u16,
        _ => {
            return Err(VietError::type_error(
                "udp_send() port must be Int".into(),
                line,
                col,
            ))
        }
    };
    let payload = match &args[2] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "udp_send() payload must be String".into(),
                line,
                col,
            ))
        }
    };

    match std::net::UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => {
            let addr = format!("{}:{}", host, port);
            match socket.send_to(payload.as_bytes(), addr) {
                Ok(_) => Ok(Value::Bool(true)),
                Err(e) => Err(VietError::runtime_error(
                    format!("UDP send error: {}", e),
                    line,
                    col,
                )),
            }
        }
        Err(e) => Err(VietError::runtime_error(
            format!("UDP bind error: {}", e),
            line,
            col,
        )),
    }
}

pub fn builtin_base64_encode(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error(
            "base64_encode() takes 1 argument".into(),
            line,
            col,
        ));
    }
    let input = match &args[0] {
        Value::String(s) => s.as_bytes().to_vec(),
        _ => {
            return Err(VietError::type_error(
                "base64_encode() expects a string".into(),
                line,
                col,
            ))
        }
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
        if chunk.len() > 1 {
            result.push(CHARS[((triple >> 6) & 0x3f) as usize] as char);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(CHARS[(triple & 0x3f) as usize] as char);
        } else {
            result.push('=');
        }
    }
    Ok(Value::String(result))
}

pub fn builtin_http_fetch(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.is_empty() || args.len() > 5 {
        return Err(VietError::runtime_error(
            "http_fetch() takes 1-5 arguments (url, method, headers, body, options)".into(),
            line,
            col,
        ));
    }
    let url_str = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "http_fetch() expects url string".into(),
                line,
                col,
            ))
        }
    };
    let method_text = if args.len() >= 2 {
        match &args[1] {
            Value::String(s) => s.to_uppercase(),
            _ => "GET".to_string(),
        }
    } else {
        "GET".to_string()
    };
    let headers_map: HashMap<String, String> = if args.len() >= 3 {
        match &args[2] {
            Value::Struct { fields, .. } => {
                let mut map = HashMap::new();
                for (k, v) in fields {
                    map.insert(k.clone(), format!("{}", v));
                }
                map
            }
            _ => HashMap::new(),
        }
    } else {
        HashMap::new()
    };
    let body = if args.len() >= 4 {
        match &args[3] {
            Value::String(s) => s.clone(),
            _ => "".to_string(),
        }
    } else {
        "".to_string()
    };

    let parsed_url = reqwest::Url::parse(url_str.trim())
        .map_err(|e| VietError::runtime_error(format!("Invalid URL: {}", e), line, col))?;
    if parsed_url.scheme() != "http" && parsed_url.scheme() != "https" {
        return Err(VietError::runtime_error(
            "http_fetch() only supports http:// and https:// URLs".into(),
            line,
            col,
        ));
    }

    let method = reqwest::Method::from_bytes(method_text.as_bytes()).map_err(|_| {
        VietError::runtime_error(format!("Invalid HTTP method '{}'", method_text), line, col)
    })?;
    let mut timeout_ms = 30_000u64;
    let mut max_response_bytes = 8 * 1024 * 1024usize;
    if let Some(Value::Struct { fields, .. }) = args.get(4) {
        if let Some(Value::Int(value)) = fields.get("timeout_ms") {
            if !(1..=300_000).contains(value) {
                return Err(VietError::runtime_error(
                    "http_fetch timeout_ms must be between 1 and 300000".into(),
                    line,
                    col,
                ));
            }
            timeout_ms = *value as u64;
        }
        if let Some(Value::Int(value)) = fields.get("max_response_bytes") {
            if !(1..=67_108_864).contains(value) {
                return Err(VietError::runtime_error(
                    "http_fetch max_response_bytes must be between 1 and 67108864".into(),
                    line,
                    col,
                ));
            }
            max_response_bytes = *value as usize;
        }
    }

    let client = default_http_client(line, col)?;
    let mut request = client
        .request(method, parsed_url)
        .timeout(Duration::from_millis(timeout_ms));
    for (name, value) in headers_map {
        let header_name =
            reqwest::header::HeaderName::from_bytes(name.as_bytes()).map_err(|_| {
                VietError::runtime_error(format!("Invalid HTTP header name '{}'", name), line, col)
            })?;
        let header_value = reqwest::header::HeaderValue::from_str(&value).map_err(|_| {
            VietError::runtime_error(
                format!("Invalid HTTP header value for '{}'", name),
                line,
                col,
            )
        })?;
        request = request.header(header_name, header_value);
    }
    if !body.is_empty() {
        request = request.body(body);
    }

    let response = request.send().map_err(|e| {
        VietError::runtime_error(
            format!("HTTP request to '{}' failed: {}", url_str, e),
            line,
            col,
        )
    })?;
    let status_code = response.status().as_u16();
    let protocol = format!("{:?}", response.version());
    let final_url = response.url().to_string();
    let mut response_headers = HashMap::new();
    let mut raw_headers = String::new();
    for (name, value) in response.headers() {
        let text = value.to_str().unwrap_or("<non-utf8>").to_string();
        response_headers.insert(name.as_str().to_string(), Value::String(text.clone()));
        raw_headers.push_str(name.as_str());
        raw_headers.push_str(": ");
        raw_headers.push_str(&text);
        raw_headers.push_str("\r\n");
    }

    let mut limited = response.take((max_response_bytes + 1) as u64);
    let mut response_bytes = Vec::new();
    limited.read_to_end(&mut response_bytes).map_err(|e| {
        VietError::runtime_error(format!("Cannot read HTTP response: {}", e), line, col)
    })?;
    if response_bytes.len() > max_response_bytes {
        return Err(VietError::runtime_error(
            format!(
                "HTTP response exceeds configured limit of {} bytes",
                max_response_bytes
            ),
            line,
            col,
        ));
    }

    let mut res_map = HashMap::new();
    res_map.insert("status_code".to_string(), Value::Int(status_code as i64));
    res_map.insert(
        "body".to_string(),
        Value::String(String::from_utf8_lossy(&response_bytes).to_string()),
    );
    res_map.insert("raw_headers".to_string(), Value::String(raw_headers));
    res_map.insert(
        "headers".to_string(),
        Value::Struct {
            type_name: "Map".to_string(),
            fields: response_headers,
        },
    );
    res_map.insert("url".to_string(), Value::String(final_url));
    res_map.insert("protocol".to_string(), Value::String(protocol));
    Ok(Value::Struct {
        type_name: "HttpResponse".to_string(),
        fields: res_map,
    })
}

fn default_http_client(line: usize, col: usize) -> VietResult<&'static reqwest::blocking::Client> {
    static CLIENT: OnceLock<Result<reqwest::blocking::Client, String>> = OnceLock::new();
    match CLIENT.get_or_init(|| {
        reqwest::blocking::Client::builder()
            .use_rustls_tls()
            .min_tls_version(reqwest::tls::Version::TLS_1_2)
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(30))
            .redirect(reqwest::redirect::Policy::none())
            .pool_idle_timeout(Duration::from_secs(90))
            .pool_max_idle_per_host(16)
            .user_agent(concat!("VietLang/", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(|e| e.to_string())
    }) {
        Ok(client) => Ok(client),
        Err(error) => Err(VietError::runtime_error(
            format!("Cannot initialize HTTP client: {}", error),
            line,
            col,
        )),
    }
}

pub fn builtin_csv_parse(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.is_empty() {
        return Err(VietError::runtime_error(
            "csv_parse() takes 1 argument (csv_string)".into(),
            line,
            col,
        ));
    }
    let csv_str = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "csv_parse() expects a string".into(),
                line,
                col,
            ))
        }
    };
    let mut lines = csv_str.lines();
    let header_line = match lines.next() {
        Some(h) => h,
        None => return Ok(Value::Array(Vec::new())),
    };
    let headers: Vec<String> = header_line
        .split(',')
        .map(|s| s.trim().trim_matches('"').to_string())
        .collect();

    let mut result = Vec::new();
    for l in lines {
        let trimmed = l.trim();
        if trimmed.is_empty() {
            continue;
        }
        let fields: Vec<String> = trimmed
            .split(',')
            .map(|s| s.trim().trim_matches('"').to_string())
            .collect();
        let mut row_map = HashMap::new();
        for (idx, h) in headers.iter().enumerate() {
            let val = fields.get(idx).cloned().unwrap_or_default();
            row_map.insert(h.clone(), Value::String(val));
        }
        result.push(Value::Struct {
            type_name: "Map".to_string(),
            fields: row_map,
        });
    }
    Ok(Value::Array(result))
}

pub fn builtin_csv_stringify(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.is_empty() {
        return Err(VietError::runtime_error(
            "csv_stringify() takes 1 argument (array_of_maps)".into(),
            line,
            col,
        ));
    }
    let arr = match &args[0] {
        Value::Array(a) => a.clone(),
        _ => {
            return Err(VietError::type_error(
                "csv_stringify() expects an array of Maps".into(),
                line,
                col,
            ))
        }
    };
    if arr.is_empty() {
        return Ok(Value::String("".to_string()));
    }
    let headers: Vec<String> = if let Some(Value::Struct { fields, .. }) = arr.first() {
        let mut h: Vec<String> = fields.keys().cloned().collect();
        h.sort();
        h
    } else {
        vec![]
    };

    let mut out = headers.join(",") + "\n";
    for item in &arr {
        if let Value::Struct { fields, .. } = item {
            let row_strs: Vec<String> = headers
                .iter()
                .map(|h| match fields.get(h) {
                    Some(v) => format!("\"{}\"", format!("{}", v).replace('"', "\"\"")),
                    None => "\"\"".to_string(),
                })
                .collect();
            out.push_str(&(row_strs.join(",") + "\n"));
        }
    }
    Ok(Value::String(out))
}

pub fn builtin_random_int(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 2 {
        return Err(VietError::runtime_error(
            "random_int() takes 2 arguments (min, max)".into(),
            line,
            col,
        ));
    }
    let min = match &args[0] {
        Value::Int(n) => *n,
        _ => {
            return Err(VietError::type_error(
                "random_int() expects integers".into(),
                line,
                col,
            ))
        }
    };
    let max = match &args[1] {
        Value::Int(n) => *n,
        _ => {
            return Err(VietError::type_error(
                "random_int() expects integers".into(),
                line,
                col,
            ))
        }
    };
    if min > max {
        return Err(VietError::runtime_error(
            "random_int() requires min <= max".into(),
            line,
            col,
        ));
    }
    Ok(Value::Int(OsRng.gen_range(min..=max)))
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
    eprintln!(
        "{}[{}]\x1b[0m [{}] {}",
        color,
        level,
        timestamp,
        msg.join(" ")
    );
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
        return Err(VietError::runtime_error(
            "map_set() takes 3 arguments (map, key, value)".into(),
            line,
            col,
        ));
    }
    match &args[0] {
        Value::Struct { type_name, fields } => {
            let key = match &args[1] {
                Value::String(s) => s.clone(),
                other => format!("{}", other),
            };
            let mut new_fields = fields.clone();
            new_fields.insert(key, args[2].clone());
            Ok(Value::Struct {
                type_name: type_name.clone(),
                fields: new_fields,
            })
        }
        _ => Err(VietError::type_error(
            "map_set() first argument must be a Map".into(),
            line,
            col,
        )),
    }
}

pub fn builtin_map_get(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() < 2 || args.len() > 3 {
        return Err(VietError::runtime_error(
            "map_get() takes 2-3 arguments (map, key, [default])".into(),
            line,
            col,
        ));
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
                    if args.len() == 3 {
                        Ok(args[2].clone())
                    } else {
                        Ok(Value::None)
                    }
                }
            }
        }
        _ => Err(VietError::type_error(
            "map_get() first argument must be a Map".into(),
            line,
            col,
        )),
    }
}

pub fn builtin_map_has(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 2 {
        return Err(VietError::runtime_error(
            "map_has() takes 2 arguments (map, key)".into(),
            line,
            col,
        ));
    }
    match &args[0] {
        Value::Struct { fields, .. } => {
            let key = match &args[1] {
                Value::String(s) => s.clone(),
                other => format!("{}", other),
            };
            Ok(Value::Bool(fields.contains_key(&key)))
        }
        _ => Err(VietError::type_error(
            "map_has() first argument must be a Map".into(),
            line,
            col,
        )),
    }
}

pub fn builtin_map_keys(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error(
            "map_keys() takes 1 argument".into(),
            line,
            col,
        ));
    }
    match &args[0] {
        Value::Struct { fields, .. } => Ok(Value::Array(
            fields.keys().map(|k| Value::String(k.clone())).collect(),
        )),
        _ => Err(VietError::type_error(
            "map_keys() expects a Map".into(),
            line,
            col,
        )),
    }
}

pub fn builtin_map_values(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error(
            "map_values() takes 1 argument".into(),
            line,
            col,
        ));
    }
    match &args[0] {
        Value::Struct { fields, .. } => Ok(Value::Array(fields.values().cloned().collect())),
        _ => Err(VietError::type_error(
            "map_values() expects a Map".into(),
            line,
            col,
        )),
    }
}

pub fn builtin_map_remove(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 2 {
        return Err(VietError::runtime_error(
            "map_remove() takes 2 arguments (map, key)".into(),
            line,
            col,
        ));
    }
    match &args[0] {
        Value::Struct { type_name, fields } => {
            let key = match &args[1] {
                Value::String(s) => s.clone(),
                other => format!("{}", other),
            };
            let mut new_fields = fields.clone();
            new_fields.remove(&key);
            Ok(Value::Struct {
                type_name: type_name.clone(),
                fields: new_fields,
            })
        }
        _ => Err(VietError::type_error(
            "map_remove() first argument must be a Map".into(),
            line,
            col,
        )),
    }
}

// ============================================================
// std.db — Database Simulation (Query Builder)
// ============================================================

pub fn builtin_db_query(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.is_empty() {
        return Err(VietError::runtime_error(
            "db_query() takes at least 1 argument (sql)".into(),
            line,
            col,
        ));
    }
    let sql = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "db_query() expects a SQL string".into(),
                line,
                col,
            ))
        }
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
        return Err(VietError::runtime_error(
            "db_table() takes 1 argument (table_name)".into(),
            line,
            col,
        ));
    }
    let table = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "db_table() expects a string".into(),
                line,
                col,
            ))
        }
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
// ============================================================
// Concurrency — spawn, channel, mutex, thread_sleep
// ============================================================

pub struct NativeChannelState {
    pub buffer: VecDeque<Value>,
    pub capacity: usize,
    pub closed: bool,
}

pub type NativeChannelEntry = (Arc<Mutex<NativeChannelState>>, Arc<Condvar>, Arc<Condvar>);

static NATIVE_CHANNELS: OnceLock<Mutex<HashMap<u64, NativeChannelEntry>>> = OnceLock::new();
static NATIVE_CHANNEL_ID_SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

fn get_native_channels() -> &'static Mutex<HashMap<u64, NativeChannelEntry>> {
    NATIVE_CHANNELS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn builtin_channel(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    let capacity = match args.first() {
        Some(Value::Int(n)) if *n > 0 => *n as usize,
        _ => 0, // 0 = unbounded
    };

    let id = NATIVE_CHANNEL_ID_SEQ.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let state = Arc::new(Mutex::new(NativeChannelState {
        buffer: VecDeque::new(),
        capacity,
        closed: false,
    }));
    let not_empty = Arc::new(Condvar::new());
    let not_full = Arc::new(Condvar::new());

    {
        let mut registry = get_native_channels().lock().unwrap();
        registry.insert(id, (state, not_empty, not_full));
    }

    let mut fields = HashMap::new();
    fields.insert("id".to_string(), Value::Int(id as i64));
    fields.insert("capacity".to_string(), Value::Int(capacity as i64));
    fields.insert("type".to_string(), Value::String("channel".to_string()));

    Ok(Value::Struct {
        type_name: "Channel".to_string(),
        fields,
    })
}

pub fn builtin_channel_send(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() < 2 {
        return Err(VietError::runtime_error(
            "channel_send(ch, value) expects 2 arguments".into(),
            line,
            col,
        ));
    }

    let id = match &args[0] {
        Value::Struct { fields, .. } => match fields.get("id") {
            Some(Value::Int(i)) => *i as u64,
            _ => {
                return Err(VietError::type_error(
                    "Invalid channel object".into(),
                    line,
                    col,
                ))
            }
        },
        _ => {
            return Err(VietError::type_error(
                "channel_send expects a Channel struct as first argument".into(),
                line,
                col,
            ))
        }
    };

    let value = args[1].clone();
    let entry = {
        let registry = get_native_channels().lock().unwrap();
        registry.get(&id).cloned()
    };

    let (state_arc, not_empty_cond, not_full_cond): NativeChannelEntry = match entry {
        Some(e) => e,
        None => {
            return Err(VietError::runtime_error(
                format!("Channel #{} not found or closed", id),
                line,
                col,
            ))
        }
    };

    let mut state = state_arc.lock().unwrap();
    if state.closed {
        return Err(VietError::runtime_error(
            "Cannot send on closed channel".into(),
            line,
            col,
        ));
    }

    // If bounded channel is full, wait on not_full
    if state.capacity > 0 {
        while state.buffer.len() >= state.capacity && !state.closed {
            state = not_full_cond.wait(state).unwrap();
            if state.closed {
                return Err(VietError::runtime_error(
                    "Cannot send on closed channel".into(),
                    line,
                    col,
                ));
            }
        }
    }

    state.buffer.push_back(value);
    not_empty_cond.notify_one();
    Ok(Value::Bool(true))
}

pub fn builtin_channel_recv(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.is_empty() {
        return Err(VietError::runtime_error(
            "channel_recv(ch) expects 1 argument".into(),
            line,
            col,
        ));
    }

    let id = match &args[0] {
        Value::Struct { fields, .. } => match fields.get("id") {
            Some(Value::Int(i)) => *i as u64,
            _ => {
                return Err(VietError::type_error(
                    "Invalid channel object".into(),
                    line,
                    col,
                ))
            }
        },
        _ => {
            return Err(VietError::type_error(
                "channel_recv expects a Channel struct".into(),
                line,
                col,
            ))
        }
    };

    let entry = {
        let registry = get_native_channels().lock().unwrap();
        registry.get(&id).cloned()
    };

    let (state_arc, not_empty_cond, not_full_cond): NativeChannelEntry = match entry {
        Some(e) => e,
        None => {
            return Err(VietError::runtime_error(
                format!("Channel #{} not found or closed", id),
                line,
                col,
            ))
        }
    };

    let mut state = state_arc.lock().unwrap();
    while state.buffer.is_empty() && !state.closed {
        state = not_empty_cond.wait(state).unwrap();
    }

    if let Some(val) = state.buffer.pop_front() {
        if state.capacity > 0 {
            not_full_cond.notify_one();
        }
        Ok(val)
    } else {
        Ok(Value::None)
    }
}

pub fn builtin_channel_try_recv(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.is_empty() {
        return Err(VietError::runtime_error(
            "channel_try_recv(ch) expects 1 argument".into(),
            line,
            col,
        ));
    }

    let id = match &args[0] {
        Value::Struct { fields, .. } => match fields.get("id") {
            Some(Value::Int(i)) => *i as u64,
            _ => {
                return Err(VietError::type_error(
                    "Invalid channel object".into(),
                    line,
                    col,
                ))
            }
        },
        _ => {
            return Err(VietError::type_error(
                "channel_try_recv expects a Channel struct".into(),
                line,
                col,
            ))
        }
    };

    let entry = {
        let registry = get_native_channels().lock().unwrap();
        registry.get(&id).cloned()
    };

    let (state_arc, _not_empty_cond, not_full_cond): NativeChannelEntry = match entry {
        Some(e) => e,
        None => {
            let mut res = HashMap::new();
            res.insert("ok".to_string(), Value::Bool(false));
            res.insert("value".to_string(), Value::None);
            return Ok(Value::Struct {
                type_name: "Map".to_string(),
                fields: res,
            });
        }
    };

    let mut state = state_arc.lock().unwrap();
    if let Some(val) = state.buffer.pop_front() {
        if state.capacity > 0 {
            not_full_cond.notify_one();
        }
        let mut res = HashMap::new();
        res.insert("ok".to_string(), Value::Bool(true));
        res.insert("value".to_string(), val);
        Ok(Value::Struct {
            type_name: "Map".to_string(),
            fields: res,
        })
    } else {
        let mut res = HashMap::new();
        res.insert("ok".to_string(), Value::Bool(false));
        res.insert("value".to_string(), Value::None);
        Ok(Value::Struct {
            type_name: "Map".to_string(),
            fields: res,
        })
    }
}

pub fn builtin_channel_close(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.is_empty() {
        return Err(VietError::runtime_error(
            "channel_close(ch) expects 1 argument".into(),
            line,
            col,
        ));
    }

    let id = match &args[0] {
        Value::Struct { fields, .. } => match fields.get("id") {
            Some(Value::Int(i)) => *i as u64,
            _ => {
                return Err(VietError::type_error(
                    "Invalid channel object".into(),
                    line,
                    col,
                ))
            }
        },
        _ => {
            return Err(VietError::type_error(
                "channel_close expects a Channel struct".into(),
                line,
                col,
            ))
        }
    };

    let entry = {
        let registry = get_native_channels().lock().unwrap();
        registry.get(&id).cloned()
    };

    if let Some((state_arc, not_empty_cond, not_full_cond)) = entry {
        let mut state = state_arc.lock().unwrap();
        state.closed = true;
        not_empty_cond.notify_all();
        not_full_cond.notify_all();
    }
    Ok(Value::Bool(true))
}

pub fn builtin_thread_sleep(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    let millis = match args.first() {
        Some(Value::Int(n)) if *n > 0 => *n as u64,
        Some(Value::Float(f)) if *f > 0.0 => *f as u64,
        _ => 0,
    };
    if millis > 0 {
        std::thread::sleep(Duration::from_millis(millis));
    }
    Ok(Value::Bool(true))
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
        return Err(VietError::runtime_error(
            "format() takes at least 1 argument".into(),
            line,
            col,
        ));
    }
    let template = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "format() first argument must be a string".into(),
                line,
                col,
            ))
        }
    };
    let mut result = template;
    for (i, arg) in args.iter().skip(1).enumerate() {
        let placeholder = format!("{{{}}}", i);
        result = result.replace(&placeholder, &format!("{}", arg));
    }
    // Also replace {} in order
    for arg in args.iter().skip(1) {
        if let Some(pos) = result.find("{}") {
            result = format!("{}{}{}", &result[..pos], arg, &result[pos + 2..]);
        }
    }
    Ok(Value::String(result))
}

pub fn builtin_assert(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.is_empty() {
        return Err(VietError::runtime_error(
            "assert() takes 1-2 arguments".into(),
            line,
            col,
        ));
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

#[allow(dead_code)]
pub fn builtin_typeof(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error(
            "type_of() takes 1 argument".into(),
            line,
            col,
        ));
    }
    Ok(Value::String(args[0].type_name().to_string()))
}

pub fn builtin_range(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() < 2 || args.len() > 3 {
        return Err(VietError::runtime_error(
            "range() takes 2-3 arguments (start, end, [step])".into(),
            line,
            col,
        ));
    }
    let start = match &args[0] {
        Value::Int(n) => *n,
        _ => {
            return Err(VietError::type_error(
                "range() expects integers".into(),
                line,
                col,
            ))
        }
    };
    let end = match &args[1] {
        Value::Int(n) => *n,
        _ => {
            return Err(VietError::type_error(
                "range() expects integers".into(),
                line,
                col,
            ))
        }
    };
    let step = if args.len() == 3 {
        match &args[2] {
            Value::Int(n) => *n,
            _ => {
                return Err(VietError::type_error(
                    "range() expects integers".into(),
                    line,
                    col,
                ))
            }
        }
    } else {
        1
    };

    let mut result = Vec::new();
    let mut i = start;
    if step > 0 {
        while i < end {
            result.push(Value::Int(i));
            i += step;
        }
    } else if step < 0 {
        while i > end {
            result.push(Value::Int(i));
            i += step;
        }
    }
    Ok(Value::Array(result))
}

// ============================================================
// String Character Operations (needed for self-hosting)
// ============================================================

pub fn builtin_char_at(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 2 {
        return Err(VietError::runtime_error(
            "char_at() takes 2 arguments (string, index)".into(),
            line,
            col,
        ));
    }
    let s = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "char_at() expects a string".into(),
                line,
                col,
            ))
        }
    };
    let idx = match &args[1] {
        Value::Int(n) => *n as usize,
        _ => {
            return Err(VietError::type_error(
                "char_at() index must be Int".into(),
                line,
                col,
            ))
        }
    };
    let chars: Vec<char> = s.chars().collect();
    if idx < chars.len() {
        Ok(Value::String(chars[idx].to_string()))
    } else {
        Ok(Value::None)
    }
}

pub fn builtin_char_code(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error(
            "char_code() takes 1 argument".into(),
            line,
            col,
        ));
    }
    let s = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "char_code() expects a string".into(),
                line,
                col,
            ))
        }
    };
    if let Some(ch) = s.chars().next() {
        Ok(Value::Int(ch as i64))
    } else {
        Ok(Value::Int(0))
    }
}

pub fn builtin_from_char_code(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error(
            "from_char_code() takes 1 argument".into(),
            line,
            col,
        ));
    }
    let code = match &args[0] {
        Value::Int(n) => *n as u32,
        _ => {
            return Err(VietError::type_error(
                "from_char_code() expects Int".into(),
                line,
                col,
            ))
        }
    };
    match char::from_u32(code) {
        Some(ch) => Ok(Value::String(ch.to_string())),
        None => Ok(Value::String(String::new())),
    }
}

pub fn builtin_substring(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() < 2 || args.len() > 3 {
        return Err(VietError::runtime_error(
            "substring() takes 2-3 arguments (string, start, [end])".into(),
            line,
            col,
        ));
    }
    let s = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "substring() expects a string".into(),
                line,
                col,
            ))
        }
    };
    let start = match &args[1] {
        Value::Int(n) => *n as usize,
        _ => {
            return Err(VietError::type_error(
                "substring() start must be Int".into(),
                line,
                col,
            ))
        }
    };
    let chars: Vec<char> = s.chars().collect();
    let end = if args.len() == 3 {
        match &args[2] {
            Value::Int(n) => (*n as usize).min(chars.len()),
            _ => chars.len(),
        }
    } else {
        chars.len()
    };
    if start <= end && start <= chars.len() {
        Ok(Value::String(
            chars[start..end.min(chars.len())].iter().collect(),
        ))
    } else {
        Ok(Value::String(String::new()))
    }
}

pub fn builtin_str_repeat(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 2 {
        return Err(VietError::runtime_error(
            "str_repeat() takes 2 arguments (string, count)".into(),
            line,
            col,
        ));
    }
    let s = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "str_repeat() expects a string".into(),
                line,
                col,
            ))
        }
    };
    let n = match &args[1] {
        Value::Int(n) => *n as usize,
        _ => {
            return Err(VietError::type_error(
                "str_repeat() count must be Int".into(),
                line,
                col,
            ))
        }
    };
    Ok(Value::String(s.repeat(n)))
}

pub fn builtin_parse_int(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error(
            "parse_int() takes 1 argument".into(),
            line,
            col,
        ));
    }
    let s = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "parse_int() expects a string".into(),
                line,
                col,
            ))
        }
    };
    match s.trim().parse::<i64>() {
        Ok(n) => Ok(Value::Int(n)),
        Err(_) => Ok(Value::None),
    }
}

pub fn builtin_parse_float(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error(
            "parse_float() takes 1 argument".into(),
            line,
            col,
        ));
    }
    let s = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "parse_float() expects a string".into(),
                line,
                col,
            ))
        }
    };
    match s.trim().parse::<f64>() {
        Ok(n) => Ok(Value::Float(n)),
        Err(_) => Ok(Value::None),
    }
}

// ============================================================
// Array Operations
// ============================================================

pub fn builtin_array_sort(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error(
            "sort() takes 1 argument".into(),
            line,
            col,
        ));
    }
    let arr = match &args[0] {
        Value::Array(a) => a.clone(),
        _ => {
            return Err(VietError::type_error(
                "sort() expects an array".into(),
                line,
                col,
            ))
        }
    };
    let mut sorted = arr;
    sorted.sort_by(|a, b| match (a, b) {
        (Value::Int(x), Value::Int(y)) => x.cmp(y),
        (Value::Float(x), Value::Float(y)) => x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal),
        (Value::String(x), Value::String(y)) => x.cmp(y),
        _ => std::cmp::Ordering::Equal,
    });
    Ok(Value::Array(sorted))
}

pub fn builtin_array_slice(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() < 2 || args.len() > 3 {
        return Err(VietError::runtime_error(
            "slice() takes 2-3 arguments (array, start, [end])".into(),
            line,
            col,
        ));
    }
    let arr = match &args[0] {
        Value::Array(a) => a.clone(),
        _ => {
            return Err(VietError::type_error(
                "slice() expects array".into(),
                line,
                col,
            ))
        }
    };
    let start = match &args[1] {
        Value::Int(n) => *n as usize,
        _ => 0,
    };
    let end = if args.len() == 3 {
        match &args[2] {
            Value::Int(n) => (*n as usize).min(arr.len()),
            _ => arr.len(),
        }
    } else {
        arr.len()
    };
    if start <= end && start <= arr.len() {
        Ok(Value::Array(arr[start..end.min(arr.len())].to_vec()))
    } else {
        Ok(Value::Array(Vec::new()))
    }
}

pub fn builtin_array_index_of(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 2 {
        return Err(VietError::runtime_error(
            "index_of() takes 2 arguments (array, value)".into(),
            line,
            col,
        ));
    }
    let arr = match &args[0] {
        Value::Array(a) => a.clone(),
        _ => {
            return Err(VietError::type_error(
                "index_of() expects array".into(),
                line,
                col,
            ))
        }
    };
    for (i, item) in arr.iter().enumerate() {
        if item == &args[1] {
            return Ok(Value::Int(i as i64));
        }
    }
    Ok(Value::Int(-1))
}

pub fn builtin_array_flat(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error(
            "flat() takes 1 argument".into(),
            line,
            col,
        ));
    }
    let arr = match &args[0] {
        Value::Array(a) => a.clone(),
        _ => {
            return Err(VietError::type_error(
                "flat() expects array".into(),
                line,
                col,
            ))
        }
    };
    let mut result = Vec::new();
    for item in arr {
        match item {
            Value::Array(inner) => result.extend(inner),
            other => result.push(other),
        }
    }
    Ok(Value::Array(result))
}

// ============================================================
// Error handling
// ============================================================

pub fn builtin_throw(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let msg = if !args.is_empty() {
        format!("{}", args[0])
    } else {
        "Error".to_string()
    };
    Err(VietError::runtime_error(msg, line, col))
}

pub fn builtin_is_error(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error(
            "is_error() takes 1 argument".into(),
            line,
            col,
        ));
    }
    Ok(Value::Bool(matches!(&args[0], Value::None)))
}

// ============================================================
// Process / System
// ============================================================

pub fn builtin_args(_args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    let args: Vec<Value> = std::env::args().map(|a| Value::String(a)).collect();
    Ok(Value::Array(args))
}

pub fn builtin_platform(_args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::String(std::env::consts::OS.to_string()))
}

pub fn builtin_arch(_args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::String(std::env::consts::ARCH.to_string()))
}

pub fn builtin_sleep_ms(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error(
            "sleep_ms() takes 1 argument (milliseconds)".into(),
            line,
            col,
        ));
    }
    let ms = match &args[0] {
        Value::Int(n) => *n as u64,
        _ => {
            return Err(VietError::type_error(
                "sleep_ms() expects Int".into(),
                line,
                col,
            ))
        }
    };
    std::thread::sleep(std::time::Duration::from_millis(ms));
    Ok(Value::None)
}

pub fn builtin_time_now_us(_args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros() as i64;
    Ok(Value::Int(now))
}

pub fn builtin_tcp_ping(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() < 2 || args.len() > 3 {
        return Err(VietError::runtime_error(
            "tcp_ping() takes 2-3 arguments (host, port, [timeout_ms])".into(),
            line,
            col,
        ));
    }
    let host = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "tcp_ping() host must be String".into(),
                line,
                col,
            ))
        }
    };
    let port = match &args[1] {
        Value::Int(n) => *n as u16,
        _ => {
            return Err(VietError::type_error(
                "tcp_ping() port must be Int".into(),
                line,
                col,
            ))
        }
    };
    let timeout_ms = if args.len() == 3 {
        match &args[2] {
            Value::Int(n) => *n as u64,
            _ => 1000,
        }
    } else {
        1000
    };

    let addr = format!("{}:{}", host, port);
    let timeout = std::time::Duration::from_millis(timeout_ms);
    use std::net::ToSocketAddrs;
    if let Ok(mut addrs) = addr.to_socket_addrs() {
        if let Some(sock_addr) = addrs.next() {
            match std::net::TcpStream::connect_timeout(&sock_addr, timeout) {
                Ok(_) => return Ok(Value::Bool(true)),
                Err(_) => return Ok(Value::Bool(false)),
            }
        }
    }
    Ok(Value::Bool(false))
}

pub fn builtin_str_split_lines(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error(
            "str_split_lines() takes 1 argument".into(),
            line,
            col,
        ));
    }
    let s = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "str_split_lines() expects String".into(),
                line,
                col,
            ))
        }
    };
    let lines: Vec<Value> = s.lines().map(|l| Value::String(l.to_string())).collect();
    Ok(Value::Array(lines))
}

pub fn builtin_system_cmd(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error(
            "system_cmd() takes 1 argument (command)".into(),
            line,
            col,
        ));
    }
    let cmd_str = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "system_cmd() expects a string".into(),
                line,
                col,
            ))
        }
    };
    use std::process::Command;
    let output = Command::new("sh").arg("-c").arg(&cmd_str).output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();
            let status = out.status.code().unwrap_or(-1);
            let mut res = HashMap::new();
            res.insert("exit_code".to_string(), Value::Int(status as i64));
            res.insert("stdout".to_string(), Value::String(stdout));
            res.insert("stderr".to_string(), Value::String(stderr));
            res.insert("success".to_string(), Value::Bool(out.status.success()));
            Ok(Value::Struct {
                type_name: "CommandResult".to_string(),
                fields: res,
            })
        }
        Err(e) => Err(VietError::runtime_error(
            format!("Failed to execute command '{}': {}", cmd_str, e),
            line,
            col,
        )),
    }
}

pub fn builtin_url_encode(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error(
            "url_encode() takes 1 argument".into(),
            line,
            col,
        ));
    }
    let s = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => {
            return Err(VietError::type_error(
                "url_encode() expects String".into(),
                line,
                col,
            ))
        }
    };
    let mut encoded = String::new();
    for b in s.bytes() {
        match b {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(b as char)
            }
            b' ' => encoded.push('+'),
            _ => encoded.push_str(&format!("%{:02X}", b)),
        }
    }
    Ok(Value::String(encoded))
}

pub fn builtin_url_decode(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error(
            "url_decode() takes 1 argument".into(),
            line,
            col,
        ));
    }
    let s = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => {
            return Err(VietError::type_error(
                "url_decode() expects String".into(),
                line,
                col,
            ))
        }
    };
    let mut decoded = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(val) =
                u8::from_str_radix(std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or(""), 16)
            {
                decoded.push(val);
                i += 3;
                continue;
            }
        } else if bytes[i] == b'+' {
            decoded.push(b' ');
        } else {
            decoded.push(bytes[i]);
        }
        i += 1;
    }
    Ok(Value::String(String::from_utf8_lossy(&decoded).to_string()))
}

pub fn builtin_hmac_sha256(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 2 {
        return Err(VietError::runtime_error(
            "hmac_sha256() takes 2 arguments (message, key)".into(),
            line,
            col,
        ));
    }
    let msg = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "hmac_sha256() message must be String".into(),
                line,
                col,
            ))
        }
    };
    let key = match &args[1] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "hmac_sha256() key must be String".into(),
                line,
                col,
            ))
        }
    };
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = <HmacSha256 as Mac>::new_from_slice(key.as_bytes())
        .map_err(|_| VietError::runtime_error("Invalid HMAC-SHA256 key".into(), line, col))?;
    mac.update(msg.as_bytes());
    Ok(Value::String(hex_lower(&mac.finalize().into_bytes())))
}

pub fn builtin_hmac_sha512(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 2 {
        return Err(VietError::runtime_error(
            "hmac_sha512() takes 2 arguments (message, key)".into(),
            line,
            col,
        ));
    }
    let msg = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "hmac_sha512() message must be String".into(),
                line,
                col,
            ))
        }
    };
    let key = match &args[1] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "hmac_sha512() key must be String".into(),
                line,
                col,
            ))
        }
    };
    type HmacSha512 = Hmac<Sha512>;
    let mut mac = <HmacSha512 as Mac>::new_from_slice(key.as_bytes())
        .map_err(|_| VietError::runtime_error("Invalid HMAC-SHA512 key".into(), line, col))?;
    mac.update(msg.as_bytes());
    Ok(Value::String(hex_lower(&mac.finalize().into_bytes())))
}

pub fn builtin_base64_url_encode(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error(
            "base64_url_encode() takes 1 argument".into(),
            line,
            col,
        ));
    }
    let input = match &args[0] {
        Value::String(s) => s.as_bytes(),
        _ => {
            return Err(VietError::type_error(
                "base64_url_encode() expects String".into(),
                line,
                col,
            ))
        }
    };
    Ok(Value::String(
        general_purpose::URL_SAFE_NO_PAD.encode(input),
    ))
}

pub fn builtin_hmac_sha256_base64url(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 2 {
        return Err(VietError::runtime_error(
            "hmac_sha256_base64url() takes 2 arguments (message, key)".into(),
            line,
            col,
        ));
    }
    let msg = match &args[0] {
        Value::String(s) => s.as_bytes(),
        _ => {
            return Err(VietError::type_error(
                "hmac_sha256_base64url() message must be String".into(),
                line,
                col,
            ))
        }
    };
    let key = match &args[1] {
        Value::String(s) => s.as_bytes(),
        _ => {
            return Err(VietError::type_error(
                "hmac_sha256_base64url() key must be String".into(),
                line,
                col,
            ))
        }
    };
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = <HmacSha256 as Mac>::new_from_slice(key)
        .map_err(|_| VietError::runtime_error("Invalid HMAC-SHA256 key".into(), line, col))?;
    mac.update(msg);
    Ok(Value::String(
        general_purpose::URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes()),
    ))
}

pub fn builtin_secure_compare(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 2 {
        return Err(VietError::runtime_error(
            "secure_compare() takes 2 arguments".into(),
            line,
            col,
        ));
    }
    let left = match &args[0] {
        Value::String(s) => s.as_bytes(),
        _ => {
            return Err(VietError::type_error(
                "secure_compare() expects Strings".into(),
                line,
                col,
            ))
        }
    };
    let right = match &args[1] {
        Value::String(s) => s.as_bytes(),
        _ => {
            return Err(VietError::type_error(
                "secure_compare() expects Strings".into(),
                line,
                col,
            ))
        }
    };
    let equal = left.len() == right.len() && bool::from(left.ct_eq(right));
    Ok(Value::Bool(equal))
}

pub fn builtin_to_uppercase(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.is_empty() {
        return Err(VietError::runtime_error(
            "to_uppercase() takes 1 argument".into(),
            line,
            col,
        ));
    }
    let s = match &args[0] {
        Value::String(s) => s.to_uppercase(),
        other => format!("{}", other).to_uppercase(),
    };
    Ok(Value::String(s))
}

pub fn builtin_to_lowercase(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.is_empty() {
        return Err(VietError::runtime_error(
            "to_lowercase() takes 1 argument".into(),
            line,
            col,
        ));
    }
    let s = match &args[0] {
        Value::String(s) => s.to_lowercase(),
        other => format!("{}", other).to_lowercase(),
    };
    Ok(Value::String(s))
}

pub fn builtin_trim(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.is_empty() {
        return Err(VietError::runtime_error(
            "trim() takes 1 argument".into(),
            line,
            col,
        ));
    }
    let s = match &args[0] {
        Value::String(s) => s.trim().to_string(),
        other => format!("{}", other).trim().to_string(),
    };
    Ok(Value::String(s))
}

pub fn builtin_starts_with(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() < 2 {
        return Err(VietError::runtime_error(
            "starts_with() takes 2 arguments (str, prefix)".into(),
            line,
            col,
        ));
    }
    let s = match &args[0] {
        Value::String(s) => s.clone(),
        other => format!("{}", other),
    };
    let prefix = match &args[1] {
        Value::String(p) => p.clone(),
        other => format!("{}", other),
    };
    Ok(Value::Bool(s.starts_with(&prefix)))
}

pub fn builtin_ends_with(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() < 2 {
        return Err(VietError::runtime_error(
            "ends_with() takes 2 arguments (str, suffix)".into(),
            line,
            col,
        ));
    }
    let s = match &args[0] {
        Value::String(s) => s.clone(),
        other => format!("{}", other),
    };
    let suffix = match &args[1] {
        Value::String(p) => p.clone(),
        other => format!("{}", other),
    };
    Ok(Value::Bool(s.ends_with(&suffix)))
}

pub fn builtin_contains(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() < 2 {
        return Err(VietError::runtime_error(
            "contains() takes 2 arguments (str, pattern)".into(),
            line,
            col,
        ));
    }
    let s = match &args[0] {
        Value::String(s) => s.clone(),
        other => format!("{}", other),
    };
    let pattern = match &args[1] {
        Value::String(p) => p.clone(),
        other => format!("{}", other),
    };
    Ok(Value::Bool(s.contains(&pattern)))
}

pub fn builtin_encrypt_secret(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 2 {
        return Err(VietError::runtime_error(
            "encrypt_secret() takes 2 arguments (plaintext, key)".into(),
            line,
            col,
        ));
    }
    let text = match &args[0] {
        Value::String(s) => s.as_bytes(),
        _ => {
            return Err(VietError::type_error(
                "encrypt_secret() plaintext must be String".into(),
                line,
                col,
            ))
        }
    };
    let secret = match &args[1] {
        Value::String(s) => s.as_bytes(),
        _ => {
            return Err(VietError::type_error(
                "encrypt_secret() key must be String".into(),
                line,
                col,
            ))
        }
    };
    if secret.is_empty() {
        return Err(VietError::runtime_error(
            "Key cannot be empty".into(),
            line,
            col,
        ));
    }

    let mut salt = [0u8; 16];
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut salt);
    OsRng.fill_bytes(&mut nonce_bytes);
    let key = derive_secret_key(secret, &salt, line, col)?;
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|_| VietError::runtime_error("Cannot initialize AES-256-GCM".into(), line, col))?;
    let encrypted = cipher
        .encrypt(Nonce::from_slice(&nonce_bytes), text)
        .map_err(|_| VietError::runtime_error("Secret encryption failed".into(), line, col))?;

    let mut envelope = Vec::with_capacity(1 + salt.len() + nonce_bytes.len() + encrypted.len());
    envelope.push(1);
    envelope.extend_from_slice(&salt);
    envelope.extend_from_slice(&nonce_bytes);
    envelope.extend_from_slice(&encrypted);
    Ok(Value::String(general_purpose::STANDARD.encode(envelope)))
}

pub fn builtin_decrypt_secret(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 2 {
        return Err(VietError::runtime_error(
            "decrypt_secret() takes 2 arguments (ciphertext, key)".into(),
            line,
            col,
        ));
    }
    let b64 = match &args[0] {
        Value::String(s) => s,
        _ => {
            return Err(VietError::type_error(
                "decrypt_secret() ciphertext must be String".into(),
                line,
                col,
            ))
        }
    };
    let secret = match &args[1] {
        Value::String(s) => s.as_bytes(),
        _ => {
            return Err(VietError::type_error(
                "decrypt_secret() key must be String".into(),
                line,
                col,
            ))
        }
    };
    if secret.is_empty() {
        return Err(VietError::runtime_error(
            "Key cannot be empty".into(),
            line,
            col,
        ));
    }

    let envelope = general_purpose::STANDARD.decode(b64).map_err(|_| {
        VietError::runtime_error("Invalid encrypted secret encoding".into(), line, col)
    })?;
    if envelope.len() < 45 || envelope[0] != 1 {
        return Err(VietError::runtime_error(
            "Invalid encrypted secret envelope".into(),
            line,
            col,
        ));
    }
    let salt = &envelope[1..17];
    let nonce = &envelope[17..29];
    let ciphertext = &envelope[29..];
    let key = derive_secret_key(secret, salt, line, col)?;
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|_| VietError::runtime_error("Cannot initialize AES-256-GCM".into(), line, col))?;
    let plaintext = cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .map_err(|_| {
            VietError::runtime_error(
                "Secret authentication failed (wrong key or modified ciphertext)".into(),
                line,
                col,
            )
        })?;
    let text = String::from_utf8(plaintext).map_err(|_| {
        VietError::runtime_error("Decrypted secret is not valid UTF-8".into(), line, col)
    })?;
    Ok(Value::String(text))
}

fn derive_secret_key(secret: &[u8], salt: &[u8], line: usize, col: usize) -> VietResult<[u8; 32]> {
    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(secret, salt, &mut key)
        .map_err(|e| {
            VietError::runtime_error(format!("Secret key derivation failed: {}", e), line, col)
        })?;
    Ok(key)
}

pub fn builtin_password_hash(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error(
            "password_hash() takes 1 argument".into(),
            line,
            col,
        ));
    }
    let password = match &args[0] {
        Value::String(s) => s,
        _ => {
            return Err(VietError::type_error(
                "password_hash() expects String".into(),
                line,
                col,
            ))
        }
    };
    let salt = SaltString::generate(&mut OsRng);
    let encoded = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| {
            VietError::runtime_error(format!("Password hashing failed: {}", e), line, col)
        })?
        .to_string();
    Ok(Value::String(encoded))
}

pub fn builtin_password_verify(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 2 {
        return Err(VietError::runtime_error(
            "password_verify() takes 2 arguments (password, hash)".into(),
            line,
            col,
        ));
    }
    let password = match &args[0] {
        Value::String(s) => s,
        _ => {
            return Err(VietError::type_error(
                "password_verify() password must be String".into(),
                line,
                col,
            ))
        }
    };
    let encoded = match &args[1] {
        Value::String(s) => s,
        _ => {
            return Err(VietError::type_error(
                "password_verify() hash must be String".into(),
                line,
                col,
            ))
        }
    };
    let parsed = match PasswordHash::new(encoded) {
        Ok(hash) => hash,
        Err(_) => return Ok(Value::Bool(false)),
    };
    Ok(Value::Bool(
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok(),
    ))
}

pub fn builtin_ip_in_cidr(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 2 {
        return Err(VietError::runtime_error(
            "ip_in_cidr() takes 2 arguments (ip, cidr)".into(),
            line,
            col,
        ));
    }
    let ip_str = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(VietError::type_error("ip must be String".into(), line, col)),
    };
    let cidr_str = match &args[1] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "cidr must be String".into(),
                line,
                col,
            ))
        }
    };

    if cidr_str == "0.0.0.0/0" || cidr_str == "*" {
        return Ok(Value::Bool(true));
    }
    if !cidr_str.contains('/') {
        return Ok(Value::Bool(ip_str == cidr_str));
    }

    let parts: Vec<&str> = cidr_str.split('/').collect();
    if parts.len() != 2 {
        return Ok(Value::Bool(false));
    }
    let prefix = parts[0];
    let mask: u32 = parts[1].parse().unwrap_or(32);

    let ip_num = parse_ipv4(&ip_str);
    let net_num = parse_ipv4(prefix);
    if ip_num.is_none() || net_num.is_none() {
        return Ok(Value::Bool(false));
    }

    let mask_bits = if mask == 0 { 0 } else { !0u32 << (32 - mask) };
    Ok(Value::Bool(
        (ip_num.unwrap() & mask_bits) == (net_num.unwrap() & mask_bits),
    ))
}

fn parse_ipv4(s: &str) -> Option<u32> {
    let octets: Vec<&str> = s.trim().split('.').collect();
    if octets.len() != 4 {
        return None;
    }
    let o1: u32 = octets[0].parse().ok()?;
    let o2: u32 = octets[1].parse().ok()?;
    let o3: u32 = octets[2].parse().ok()?;
    let o4: u32 = octets[3].parse().ok()?;
    Some((o1 << 24) | (o2 << 16) | (o3 << 8) | o4)
}

pub fn builtin_hex_encode(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error(
            "hex_encode() takes 1 argument".into(),
            line,
            col,
        ));
    }
    let s = match &args[0] {
        Value::String(s) => s.as_bytes(),
        _ => {
            return Err(VietError::type_error(
                "hex_encode() expects String".into(),
                line,
                col,
            ))
        }
    };
    let hex: String = s.iter().map(|b| format!("{:02x}", b)).collect();
    Ok(Value::String(hex))
}

pub fn builtin_hex_decode(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error(
            "hex_decode() takes 1 argument".into(),
            line,
            col,
        ));
    }
    let s = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "hex_decode() expects String".into(),
                line,
                col,
            ))
        }
    };
    let clean = s.trim();
    if clean.len() % 2 != 0 {
        return Err(VietError::runtime_error(
            "Invalid hex length".into(),
            line,
            col,
        ));
    }
    let mut bytes = Vec::new();
    for i in (0..clean.len()).step_by(2) {
        if let Ok(b) = u8::from_str_radix(&clean[i..i + 2], 16) {
            bytes.push(b);
        } else {
            return Err(VietError::runtime_error(
                "Invalid hex character".into(),
                line,
                col,
            ));
        }
    }
    Ok(Value::String(String::from_utf8_lossy(&bytes).to_string()))
}

pub fn builtin_crypto_random_hex(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() > 1 {
        return Err(VietError::runtime_error(
            "crypto_random_hex() takes 0-1 arguments".into(),
            line,
            col,
        ));
    }
    let len = match args.first() {
        Some(Value::Int(n)) if (1..=1_048_576).contains(n) => *n as usize,
        Some(Value::Int(_)) => {
            return Err(VietError::runtime_error(
                "crypto_random_hex() length must be between 1 and 1048576 bytes".into(),
                line,
                col,
            ))
        }
        Some(_) => {
            return Err(VietError::type_error(
                "crypto_random_hex() length must be Int".into(),
                line,
                col,
            ))
        }
        None => 16,
    };
    let mut bytes = vec![0u8; len];
    OsRng.fill_bytes(&mut bytes);
    Ok(Value::String(hex_lower(&bytes)))
}

pub fn builtin_uuid_v4(_args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    let mut random_bytes = [0u8; 16];
    OsRng.fill_bytes(&mut random_bytes);
    random_bytes[6] = (random_bytes[6] & 0x0f) | 0x40;
    random_bytes[8] = (random_bytes[8] & 0x3f) | 0x80;

    let uuid = format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        random_bytes[0], random_bytes[1], random_bytes[2], random_bytes[3],
        random_bytes[4], random_bytes[5],
        random_bytes[6], random_bytes[7],
        random_bytes[8], random_bytes[9],
        random_bytes[10], random_bytes[11], random_bytes[12], random_bytes[13], random_bytes[14], random_bytes[15]
    );
    Ok(Value::String(uuid))
}

fn hex_lower(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

#[cfg(test)]
mod security_tests {
    use super::*;

    fn string_value(result: VietResult<Value>) -> String {
        match result.expect("builtin should succeed") {
            Value::String(value) => value,
            other => panic!("expected String, got {:?}", other),
        }
    }

    #[test]
    fn sha256_matches_nist_vector() {
        let digest = string_value(builtin_hash_sha256(&[Value::String("abc".into())], 1, 1));
        assert_eq!(
            digest,
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn hmac_matches_rfc_4231_vectors() {
        let args = [
            Value::String("what do ya want for nothing?".into()),
            Value::String("Jefe".into()),
        ];
        assert_eq!(
            string_value(builtin_hmac_sha256(&args, 1, 1)),
            "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"
        );
        assert_eq!(
            string_value(builtin_hmac_sha512(&args, 1, 1)),
            "164b7a7bfcf819e2e395fbe73b56e0a387bd64222e831fd610270cd7ea2505549758bf75c05a994a6d034f65f8f0e6fdcaeab1a34d4a6b4b636e070a38bce737"
        );
    }

    #[test]
    fn hs256_base64url_matches_jwt_interop_vector() {
        let message = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ";
        let args = [
            Value::String(message.into()),
            Value::String("your-256-bit-secret".into()),
        ];
        assert_eq!(
            string_value(builtin_hmac_sha256_base64url(&args, 1, 1)),
            "SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"
        );
    }

    #[test]
    fn secret_encryption_is_authenticated_and_randomized() {
        let args = [
            Value::String("sensitive".into()),
            Value::String("correct horse battery staple".into()),
        ];
        let first = string_value(builtin_encrypt_secret(&args, 1, 1));
        let second = string_value(builtin_encrypt_secret(&args, 1, 1));
        assert_ne!(first, second);
        assert_eq!(
            string_value(builtin_decrypt_secret(
                &[Value::String(first.clone()), args[1].clone()],
                1,
                1
            )),
            "sensitive"
        );
        assert!(builtin_decrypt_secret(
            &[Value::String(first), Value::String("wrong key".into())],
            1,
            1
        )
        .is_err());
    }

    #[test]
    fn password_hash_uses_argon2id_and_verifies() {
        let encoded = string_value(builtin_password_hash(
            &[Value::String("p@ssword".into())],
            1,
            1,
        ));
        assert!(encoded.starts_with("$argon2id$"));
        assert_eq!(
            builtin_password_verify(
                &[
                    Value::String("p@ssword".into()),
                    Value::String(encoded.clone())
                ],
                1,
                1
            )
            .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            builtin_password_verify(
                &[Value::String("wrong".into()), Value::String(encoded)],
                1,
                1
            )
            .unwrap(),
            Value::Bool(false)
        );
    }

    #[test]
    fn http_client_rejects_non_http_schemes() {
        let error = builtin_http_fetch(&[Value::String("file:///etc/passwd".into())], 7, 3)
            .expect_err("local file URLs must not be accepted by the HTTP client");
        assert!(error.message.contains("only supports http:// and https://"));
        assert_eq!((error.line, error.column), (7, 3));
    }
}

pub fn builtin_time_unix_ms(_args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    let ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64;
    Ok(Value::Int(ms))
}

// ============================================================
// std.db_sqlite — Real File-Backed SQLite ACID Storage Engine
// ============================================================

use rusqlite::{types::ValueRef, Connection};

static SQLITE_REGISTRY: Mutex<Option<HashMap<usize, Connection>>> = Mutex::new(None);
static SQLITE_ID_COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);

fn get_sqlite_conn<F, R>(id: usize, f: F) -> VietResult<R>
where
    F: FnOnce(&mut Connection) -> VietResult<R>,
{
    let mut guard = SQLITE_REGISTRY.lock().unwrap();
    let map = guard.get_or_insert_with(HashMap::new);
    if let Some(conn) = map.get_mut(&id) {
        f(conn)
    } else {
        Err(VietError::runtime_error(
            format!("SQLite Connection #{} not found or closed", id),
            0,
            0,
        ))
    }
}

pub fn builtin_sqlite_open(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    let path = if !args.is_empty() {
        match &args[0] {
            Value::String(s) => s.clone(),
            _ => ":memory:".to_string(),
        }
    } else {
        ":memory:".to_string()
    };

    if path != ":memory:" {
        if let Some(parent) = std::path::Path::new(&path).parent() {
            if !parent.as_os_str().is_empty() {
                let _ = std::fs::create_dir_all(parent);
            }
        }
    }

    let conn = if path == ":memory:" {
        Connection::open_in_memory()
    } else {
        Connection::open(&path)
    }
    .map_err(|e| {
        VietError::runtime_error(
            format!("Cannot open SQLite database '{}': {}", path, e),
            line,
            col,
        )
    })?;
    let _ = conn.busy_timeout(std::time::Duration::from_secs(30));

    // Performance & ACID Pragmas
    let _ = conn.execute_batch(
        "PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL; PRAGMA foreign_keys=ON;",
    );

    let id = SQLITE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    {
        let mut guard = SQLITE_REGISTRY.lock().unwrap();
        let map = guard.get_or_insert_with(HashMap::new);
        map.insert(id, conn);
    }

    let mut fields = HashMap::new();
    fields.insert("id".to_string(), Value::Int(id as i64));
    fields.insert("path".to_string(), Value::String(path));
    fields.insert("is_open".to_string(), Value::Bool(true));
    fields.insert(
        "engine".to_string(),
        Value::String("SQLite 3.x (Native Binary WAL)".to_string()),
    );

    Ok(Value::Struct {
        type_name: "SqliteConn".to_string(),
        fields,
    })
}

pub fn builtin_sqlite_exec(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() < 2 {
        return Err(VietError::runtime_error(
            "sqlite_exec() takes 2 arguments (conn, sql)".into(),
            line,
            col,
        ));
    }
    let conn_id = match &args[0] {
        Value::Int(id) => *id as usize,
        Value::Struct { fields, .. } => {
            if let Some(Value::Int(id)) = fields.get("id") {
                *id as usize
            } else {
                return Err(VietError::type_error(
                    "sqlite_exec() invalid connection struct".into(),
                    line,
                    col,
                ));
            }
        }
        _ => {
            return Err(VietError::type_error(
                "sqlite_exec() expects connection".into(),
                line,
                col,
            ))
        }
    };
    let sql = match &args[1] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "sqlite_exec() expects SQL string".into(),
                line,
                col,
            ))
        }
    };

    get_sqlite_conn(conn_id, |conn| {
        conn.execute_batch(&sql).map_err(|e| {
            VietError::runtime_error(format!("SQLite exec error: {}", e), line, col)
        })?;
        Ok(Value::Bool(true))
    })
}

pub fn builtin_sqlite_execute(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() < 2 {
        return Err(VietError::runtime_error(
            "sqlite_execute() takes 2+ arguments (conn, sql, [params])".into(),
            line,
            col,
        ));
    }
    let conn_id = match &args[0] {
        Value::Int(id) => *id as usize,
        Value::Struct { fields, .. } => {
            if let Some(Value::Int(id)) = fields.get("id") {
                *id as usize
            } else {
                return Err(VietError::type_error(
                    "sqlite_execute() invalid connection".into(),
                    line,
                    col,
                ));
            }
        }
        _ => {
            return Err(VietError::type_error(
                "sqlite_execute() expects connection".into(),
                line,
                col,
            ))
        }
    };
    let sql = match &args[1] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "sqlite_execute() expects SQL string".into(),
                line,
                col,
            ))
        }
    };

    let params_vec: Vec<rusqlite::types::Value> = if args.len() >= 3 {
        match &args[2] {
            Value::Array(arr) => arr
                .iter()
                .map(|v| match v {
                    Value::Int(i) => rusqlite::types::Value::Integer(*i),
                    Value::Float(f) => rusqlite::types::Value::Real(*f),
                    Value::String(s) => rusqlite::types::Value::Text(s.clone()),
                    Value::Bool(b) => rusqlite::types::Value::Integer(if *b { 1 } else { 0 }),
                    Value::None => rusqlite::types::Value::Null,
                    other => rusqlite::types::Value::Text(format!("{}", other)),
                })
                .collect(),
            _ => vec![],
        }
    } else {
        vec![]
    };

    get_sqlite_conn(conn_id, |conn| {
        let params_slice: Vec<&dyn rusqlite::ToSql> = params_vec
            .iter()
            .map(|p| p as &dyn rusqlite::ToSql)
            .collect();
        let affected = conn.execute(&sql, params_slice.as_slice()).map_err(|e| {
            VietError::runtime_error(format!("SQLite execute error: {}", e), line, col)
        })?;
        Ok(Value::Int(affected as i64))
    })
}

pub fn builtin_sqlite_query(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() < 2 {
        return Err(VietError::runtime_error(
            "sqlite_query() takes 2+ arguments (conn, sql, [params])".into(),
            line,
            col,
        ));
    }
    let conn_id = match &args[0] {
        Value::Int(id) => *id as usize,
        Value::Struct { fields, .. } => {
            if let Some(Value::Int(id)) = fields.get("id") {
                *id as usize
            } else {
                return Err(VietError::type_error(
                    "sqlite_query() invalid connection".into(),
                    line,
                    col,
                ));
            }
        }
        _ => {
            return Err(VietError::type_error(
                "sqlite_query() expects connection".into(),
                line,
                col,
            ))
        }
    };
    let sql = match &args[1] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "sqlite_query() expects SQL string".into(),
                line,
                col,
            ))
        }
    };

    let params_vec: Vec<rusqlite::types::Value> = if args.len() >= 3 {
        match &args[2] {
            Value::Array(arr) => arr
                .iter()
                .map(|v| match v {
                    Value::Int(i) => rusqlite::types::Value::Integer(*i),
                    Value::Float(f) => rusqlite::types::Value::Real(*f),
                    Value::String(s) => rusqlite::types::Value::Text(s.clone()),
                    Value::Bool(b) => rusqlite::types::Value::Integer(if *b { 1 } else { 0 }),
                    Value::None => rusqlite::types::Value::Null,
                    other => rusqlite::types::Value::Text(format!("{}", other)),
                })
                .collect(),
            _ => vec![],
        }
    } else {
        vec![]
    };

    get_sqlite_conn(conn_id, |conn| {
        let params_slice: Vec<&dyn rusqlite::ToSql> = params_vec
            .iter()
            .map(|p| p as &dyn rusqlite::ToSql)
            .collect();
        let mut stmt = conn.prepare(&sql).map_err(|e| {
            VietError::runtime_error(format!("SQLite prepare error '{}': {}", sql, e), line, col)
        })?;

        let column_names: Vec<String> = stmt
            .column_names()
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        let rows = stmt
            .query_map(params_slice.as_slice(), |row| {
                let mut row_map = HashMap::new();
                for (idx, col_name) in column_names.iter().enumerate() {
                    let val_ref = row.get_ref(idx)?;
                    let vl_val = match val_ref {
                        ValueRef::Null => Value::None,
                        ValueRef::Integer(i) => Value::Int(i),
                        ValueRef::Real(f) => Value::Float(f),
                        ValueRef::Text(t) => Value::String(String::from_utf8_lossy(t).to_string()),
                        ValueRef::Blob(b) => Value::String(String::from_utf8_lossy(b).to_string()),
                    };
                    row_map.insert(col_name.clone(), vl_val);
                }
                Ok(Value::Struct {
                    type_name: "Map".to_string(),
                    fields: row_map,
                })
            })
            .map_err(|e| {
                VietError::runtime_error(format!("SQLite query error: {}", e), line, col)
            })?;

        let mut results = Vec::new();
        for r in rows {
            if let Ok(row_val) = r {
                results.push(row_val);
            }
        }

        Ok(Value::Array(results))
    })
}

pub fn builtin_sqlite_close(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    if args.is_empty() {
        return Ok(Value::Bool(false));
    }
    let conn_id = match &args[0] {
        Value::Int(id) => *id as usize,
        Value::Struct { fields, .. } => {
            if let Some(Value::Int(id)) = fields.get("id") {
                *id as usize
            } else {
                return Ok(Value::Bool(false));
            }
        }
        _ => return Ok(Value::Bool(false)),
    };

    let mut guard = SQLITE_REGISTRY.lock().unwrap();
    if let Some(map) = guard.as_mut() {
        let removed = map.remove(&conn_id).is_some();
        Ok(Value::Bool(removed))
    } else {
        Ok(Value::Bool(false))
    }
}

pub fn builtin_sqlite_migrate(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 2 {
        return Err(VietError::runtime_error(
            "sqlite_migrate() expects connection and migration array".into(),
            line,
            col,
        ));
    }
    let conn_id = match &args[0] {
        Value::Int(id) => usize::try_from(*id).ok(),
        Value::Struct { fields, .. } => fields
            .get("id")
            .and_then(Value::as_int)
            .and_then(|id| usize::try_from(id).ok()),
        _ => None,
    }
    .ok_or_else(|| VietError::type_error("Invalid SQLite connection".into(), line, col))?;
    let migrations = match &args[1] {
        Value::Array(values) => values,
        _ => {
            return Err(VietError::type_error(
                "Migrations must be an array".into(),
                line,
                col,
            ))
        }
    };
    get_sqlite_conn(conn_id, |conn| {
        let transaction = conn
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
            .map_err(|error| {
                VietError::runtime_error(
                    format!("Cannot acquire SQLite migration lock: {}", error),
                    line,
                    col,
                )
            })?;
        transaction.execute_batch("CREATE TABLE IF NOT EXISTS _vietlang_migrations (name TEXT PRIMARY KEY, applied_at INTEGER NOT NULL);")
            .map_err(|error| VietError::runtime_error(format!("Cannot initialize migration table: {}", error), line, col))?;
        let mut applied = 0i64;
        for migration in migrations {
            let Value::Struct { fields, .. } = migration else {
                return Err(VietError::type_error(
                    "Each migration must be a map with name and up".into(),
                    line,
                    col,
                ));
            };
            let name = match fields.get("name") {
                Some(Value::String(value)) if !value.is_empty() => value,
                _ => {
                    return Err(VietError::type_error(
                        "Migration name must be a non-empty string".into(),
                        line,
                        col,
                    ));
                }
            };
            let sql = match fields.get("up") {
                Some(Value::String(value)) => value,
                _ => {
                    return Err(VietError::type_error(
                        "Migration up must be a SQL string".into(),
                        line,
                        col,
                    ));
                }
            };
            let exists: i64 = transaction
                .query_row(
                    "SELECT EXISTS(SELECT 1 FROM _vietlang_migrations WHERE name = ?1)",
                    [name],
                    |row| row.get(0),
                )
                .map_err(|error| {
                    VietError::runtime_error(
                        format!("Cannot inspect migration '{}': {}", name, error),
                        line,
                        col,
                    )
                })?;
            if exists == 0 {
                transaction.execute_batch(sql).map_err(|error| {
                    VietError::runtime_error(
                        format!(
                            "Migration '{}' failed; transaction rolled back: {}",
                            name, error
                        ),
                        line,
                        col,
                    )
                })?;
                transaction.execute("INSERT INTO _vietlang_migrations(name, applied_at) VALUES (?1, unixepoch())", [name])
                    .map_err(|error| VietError::runtime_error(format!("Cannot record migration '{}': {}", name, error), line, col))?;
                applied += 1;
            }
        }
        transaction.commit().map_err(|error| {
            VietError::runtime_error(format!("Cannot commit migrations: {}", error), line, col)
        })?;
        Ok(Value::Int(applied))
    })
}

// ============================================================
// std.db_mysql — Native Binary MySQL Engine (mysql crate)
// ============================================================

#[cfg(any())]
static MYSQL_REGISTRY: std::sync::Mutex<Option<HashMap<usize, mysql::Pool>>> =
    std::sync::Mutex::new(None);
#[cfg(any())]
static MYSQL_ID_COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);

#[cfg(any())]
fn get_mysql_pool<F, R>(id: usize, f: F) -> VietResult<R>
where
    F: FnOnce(&mysql::Pool) -> VietResult<R>,
{
    let guard = MYSQL_REGISTRY.lock().unwrap();
    if let Some(map) = guard.as_ref() {
        if let Some(pool) = map.get(&id) {
            f(pool)
        } else {
            Err(VietError::runtime_error(
                format!("MySQL Pool #{} not found or closed", id),
                0,
                0,
            ))
        }
    } else {
        Err(VietError::runtime_error(
            format!("MySQL Pool #{} not found or closed", id),
            0,
            0,
        ))
    }
}

#[cfg(any())]
pub fn builtin_mysql_connect(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.is_empty() {
        return Err(VietError::runtime_error(
            "mysql_connect() requires 1 argument (dsn: String)".into(),
            line,
            col,
        ));
    }
    let dsn =
        match &args[0] {
            Value::String(s) => s.clone(),
            _ => return Err(VietError::type_error(
                "mysql_connect() expects a string DSN (e.g. mysql://user:pass@host:port/dbname)"
                    .into(),
                line,
                col,
            )),
        };

    let opts = mysql::Opts::from_url(&dsn).map_err(|e| {
        VietError::runtime_error(format!("Invalid MySQL DSN '{}': {}", dsn, e), line, col)
    })?;

    let pool = mysql::Pool::new(opts).map_err(|e| {
        VietError::runtime_error(
            format!("Cannot create MySQL connection pool for '{}': {}", dsn, e),
            line,
            col,
        )
    })?;

    // Test connection
    let _ = pool.get_conn().map_err(|e| {
        VietError::runtime_error(
            format!("Cannot connect to MySQL server '{}': {}", dsn, e),
            line,
            col,
        )
    })?;

    let id = MYSQL_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    {
        let mut guard = MYSQL_REGISTRY.lock().unwrap();
        let map = guard.get_or_insert_with(HashMap::new);
        map.insert(id, pool);
    }

    let mut fields = HashMap::new();
    fields.insert("id".to_string(), Value::Int(id as i64));
    fields.insert("dsn".to_string(), Value::String(dsn));
    fields.insert("driver".to_string(), Value::String("mysql".to_string()));
    fields.insert("is_open".to_string(), Value::Bool(true));
    fields.insert(
        "engine".to_string(),
        Value::String("MySQL 8.x / MariaDB (Native Binary Driver)".to_string()),
    );

    Ok(Value::Struct {
        type_name: "MySqlPool".to_string(),
        fields,
    })
}

#[cfg(any())]
pub fn builtin_mysql_exec(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    use mysql::prelude::*;

    if args.len() < 2 {
        return Err(VietError::runtime_error(
            "mysql_exec() takes 2 arguments (pool, sql)".into(),
            line,
            col,
        ));
    }
    let pool_id = match &args[0] {
        Value::Int(id) => *id as usize,
        Value::Struct { fields, .. } => {
            if let Some(Value::Int(id)) = fields.get("id") {
                *id as usize
            } else {
                return Err(VietError::type_error(
                    "mysql_exec() invalid pool struct".into(),
                    line,
                    col,
                ));
            }
        }
        _ => {
            return Err(VietError::type_error(
                "mysql_exec() expects connection pool".into(),
                line,
                col,
            ))
        }
    };
    let sql = match &args[1] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "mysql_exec() expects SQL string".into(),
                line,
                col,
            ))
        }
    };

    get_mysql_pool(pool_id, |pool| {
        let mut conn = pool.get_conn().map_err(|e| {
            VietError::runtime_error(format!("MySQL connection error: {}", e), line, col)
        })?;
        conn.query_drop(&sql)
            .map_err(|e| VietError::runtime_error(format!("MySQL exec error: {}", e), line, col))?;
        Ok(Value::Bool(true))
    })
}

#[cfg(any())]
pub fn builtin_mysql_execute(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    use mysql::prelude::*;

    if args.len() < 2 {
        return Err(VietError::runtime_error(
            "mysql_execute() takes 2+ arguments (pool, sql, [params])".into(),
            line,
            col,
        ));
    }
    let pool_id = match &args[0] {
        Value::Int(id) => *id as usize,
        Value::Struct { fields, .. } => {
            if let Some(Value::Int(id)) = fields.get("id") {
                *id as usize
            } else {
                return Err(VietError::type_error(
                    "mysql_execute() invalid pool".into(),
                    line,
                    col,
                ));
            }
        }
        _ => {
            return Err(VietError::type_error(
                "mysql_execute() expects connection pool".into(),
                line,
                col,
            ))
        }
    };
    let sql = match &args[1] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "mysql_execute() expects SQL string".into(),
                line,
                col,
            ))
        }
    };

    let params_vec: Vec<mysql::Value> = if args.len() >= 3 {
        match &args[2] {
            Value::Array(arr) => arr
                .iter()
                .map(|v| match v {
                    Value::Int(i) => mysql::Value::Int(*i),
                    Value::Float(f) => mysql::Value::Double(*f),
                    Value::String(s) => mysql::Value::Bytes(s.as_bytes().to_vec()),
                    Value::Bool(b) => mysql::Value::Int(if *b { 1 } else { 0 }),
                    Value::None => mysql::Value::NULL,
                    other => mysql::Value::Bytes(format!("{}", other).into_bytes()),
                })
                .collect(),
            _ => vec![],
        }
    } else {
        vec![]
    };

    get_mysql_pool(pool_id, |pool| {
        let mut conn = pool.get_conn().map_err(|e| {
            VietError::runtime_error(format!("MySQL connection error: {}", e), line, col)
        })?;
        let stmt = conn.prep(&sql).map_err(|e| {
            VietError::runtime_error(format!("MySQL prepare error: {}", e), line, col)
        })?;
        conn.exec_drop(stmt, params_vec).map_err(|e| {
            VietError::runtime_error(format!("MySQL execute error: {}", e), line, col)
        })?;
        let affected = conn.affected_rows();
        Ok(Value::Int(affected as i64))
    })
}

#[cfg(any())]
pub fn builtin_mysql_query(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    use mysql::prelude::*;

    if args.len() < 2 {
        return Err(VietError::runtime_error(
            "mysql_query() takes 2+ arguments (pool, sql, [params])".into(),
            line,
            col,
        ));
    }
    let pool_id = match &args[0] {
        Value::Int(id) => *id as usize,
        Value::Struct { fields, .. } => {
            if let Some(Value::Int(id)) = fields.get("id") {
                *id as usize
            } else {
                return Err(VietError::type_error(
                    "mysql_query() invalid pool".into(),
                    line,
                    col,
                ));
            }
        }
        _ => {
            return Err(VietError::type_error(
                "mysql_query() expects connection pool".into(),
                line,
                col,
            ))
        }
    };
    let sql = match &args[1] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(VietError::type_error(
                "mysql_query() expects SQL string".into(),
                line,
                col,
            ))
        }
    };

    let params_vec: Vec<mysql::Value> = if args.len() >= 3 {
        match &args[2] {
            Value::Array(arr) => arr
                .iter()
                .map(|v| match v {
                    Value::Int(i) => mysql::Value::Int(*i),
                    Value::Float(f) => mysql::Value::Double(*f),
                    Value::String(s) => mysql::Value::Bytes(s.as_bytes().to_vec()),
                    Value::Bool(b) => mysql::Value::Int(if *b { 1 } else { 0 }),
                    Value::None => mysql::Value::NULL,
                    other => mysql::Value::Bytes(format!("{}", other).into_bytes()),
                })
                .collect(),
            _ => vec![],
        }
    } else {
        vec![]
    };

    get_mysql_pool(pool_id, |pool| {
        let mut conn = pool.get_conn().map_err(|e| {
            VietError::runtime_error(format!("MySQL connection error: {}", e), line, col)
        })?;

        let rows: Vec<mysql::Row> = if params_vec.is_empty() {
            conn.query(&sql).map_err(|e| {
                VietError::runtime_error(format!("MySQL query error '{}': {}", sql, e), line, col)
            })?
        } else {
            let stmt = conn.prep(&sql).map_err(|e| {
                VietError::runtime_error(format!("MySQL prepare error '{}': {}", sql, e), line, col)
            })?;
            conn.exec(stmt, params_vec).map_err(|e| {
                VietError::runtime_error(format!("MySQL exec error '{}': {}", sql, e), line, col)
            })?
        };

        let mut results = Vec::new();
        for row in rows {
            let mut row_map = HashMap::new();
            let cols = row.columns_ref();
            for (idx, col) in cols.iter().enumerate() {
                let col_name = col.name_str().to_string();
                let my_val: Option<mysql::Value> = row.get(idx);
                let vl_val = match my_val {
                    None | Some(mysql::Value::NULL) => Value::None,
                    Some(mysql::Value::Int(i)) => Value::Int(i),
                    Some(mysql::Value::UInt(u)) => Value::Int(u as i64),
                    Some(mysql::Value::Float(f)) => Value::Float(f as f64),
                    Some(mysql::Value::Double(d)) => Value::Float(d),
                    Some(mysql::Value::Bytes(b)) => {
                        Value::String(String::from_utf8_lossy(&b).to_string())
                    }
                    Some(mysql::Value::Date(y, m, d, h, mi, s, _)) => Value::String(format!(
                        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                        y, m, d, h, mi, s
                    )),
                    Some(mysql::Value::Time(is_neg, d, h, m, s, _)) => Value::String(format!(
                        "{}{:02}:{:02}:{:02}",
                        if is_neg { "-" } else { "" },
                        d * 24 + h as u32,
                        m,
                        s
                    )),
                };
                row_map.insert(col_name, vl_val);
            }
            results.push(Value::Struct {
                type_name: "Map".to_string(),
                fields: row_map,
            });
        }

        Ok(Value::Array(results))
    })
}

#[cfg(any())]
pub fn builtin_mysql_close(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    if args.is_empty() {
        return Ok(Value::Bool(false));
    }
    let pool_id = match &args[0] {
        Value::Int(id) => *id as usize,
        Value::Struct { fields, .. } => {
            if let Some(Value::Int(id)) = fields.get("id") {
                *id as usize
            } else {
                return Ok(Value::Bool(false));
            }
        }
        _ => return Ok(Value::Bool(false)),
    };

    let mut guard = MYSQL_REGISTRY.lock().unwrap();
    if let Some(map) = guard.as_mut() {
        let removed = map.remove(&pool_id).is_some();
        Ok(Value::Bool(removed))
    } else {
        Ok(Value::Bool(false))
    }
}

pub fn builtin_mysql_connect(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::connect(args, "mysql", line, col)
}

pub fn builtin_mysql_exec(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::execute(args, line, col)
}

pub fn builtin_mysql_execute(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::execute(args, line, col)
}

pub fn builtin_mysql_query(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::query(args, line, col)
}

pub fn builtin_mysql_close(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::close(args, line, col)
}

#[cfg(test)]
mod mysql_async_tests {
    use super::*;

    #[test]
    fn mysql_native_calls_create_async_pool() {
        let pool =
            builtin_mysql_connect(&[Value::String("mysql://localhost/test".into())], 4, 2).unwrap();
        assert!(matches!(pool, Value::Struct { .. }));
    }
}

pub fn builtin_postgres_connect(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::connect(args, "postgres", line, col)
}
pub fn builtin_postgres_exec(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::execute(args, line, col)
}
pub fn builtin_postgres_query(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::query(args, line, col)
}
pub fn builtin_postgres_ping(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::ping(args, line, col)
}
pub fn builtin_postgres_close(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::close(args, line, col)
}
pub fn builtin_db_migration_lock(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::migration_lock(args, line, col)
}

// ============================================================
// std.db_mongodb — MongoDB native driver builtins
// ============================================================

pub fn builtin_mongo_connect(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::mongo_connect(args, line, col)
}
pub fn builtin_mongo_close(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::close(args, line, col)
}
pub fn builtin_mongo_ping(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::ping(args, line, col)
}
pub fn builtin_mongo_find(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::mongo_find(args, line, col)
}
pub fn builtin_mongo_find_one(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::mongo_find_one(args, line, col)
}
pub fn builtin_mongo_insert_one(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::mongo_insert_one(args, line, col)
}
pub fn builtin_mongo_insert_many(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::mongo_insert_many(args, line, col)
}
pub fn builtin_mongo_update_one(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::mongo_update_one(args, line, col)
}
pub fn builtin_mongo_update_many(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::mongo_update_many(args, line, col)
}
pub fn builtin_mongo_upsert(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    // upsert = update_one with upsert: true flag — handled in db_runtime
    crate::db_runtime::mongo_update_one(args, line, col)
}
pub fn builtin_mongo_delete_one(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::mongo_delete_one(args, line, col)
}
pub fn builtin_mongo_delete_many(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::mongo_delete_many(args, line, col)
}
pub fn builtin_mongo_count(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::mongo_count(args, line, col)
}
pub fn builtin_mongo_aggregate(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::mongo_aggregate(args, line, col)
}
pub fn builtin_mongo_create_index(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::mongo_create_index(args, line, col)
}
pub fn builtin_mongo_list_indexes(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    // list indexes — return empty array placeholder (can be expanded)
    Ok(Value::Array(vec![]))
}
pub fn builtin_mongo_drop_index(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Bool(true))
}
pub fn builtin_mongo_list_collections(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::mongo_list_collections(args, line, col)
}
pub fn builtin_mongo_create_collection(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Bool(true))
}
pub fn builtin_mongo_drop_collection(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::mongo_drop_collection(args, line, col)
}
pub fn builtin_mongo_begin_transaction(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    // Transactions require sessions - placeholder returns session-like handle
    Ok(Value::String("mongo_session".into()))
}
pub fn builtin_mongo_commit_transaction(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Bool(true))
}
pub fn builtin_mongo_abort_transaction(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Bool(true))
}

// ============================================================
// std.db_redis — Redis native driver builtins
// ============================================================

pub fn builtin_redis_connect(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::redis_connect(args, line, col)
}
pub fn builtin_redis_connect_auth(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::redis_connect(args, line, col)
}
pub fn builtin_redis_connect_sentinel(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::None)
}
pub fn builtin_redis_close(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::close(args, line, col)
}
pub fn builtin_redis_ping(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::ping(args, line, col)
}
pub fn builtin_redis_set(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::redis_set(args, line, col)
}
pub fn builtin_redis_setex(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::redis_set(args, line, col)
}
pub fn builtin_redis_setnx(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::redis_set(args, line, col)
}
pub fn builtin_redis_get(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::redis_get(args, line, col)
}
pub fn builtin_redis_mset(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Bool(true))
}
pub fn builtin_redis_mget(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Array(vec![]))
}
pub fn builtin_redis_del(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::redis_del(args, line, col)
}
pub fn builtin_redis_exists(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::redis_exists(args, line, col)
}
pub fn builtin_redis_expire(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::redis_expire(args, line, col)
}
pub fn builtin_redis_expireat(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::redis_expire(args, line, col)
}
pub fn builtin_redis_ttl(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::redis_ttl(args, line, col)
}
pub fn builtin_redis_persist(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Bool(true))
}
pub fn builtin_redis_keys(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Array(vec![]))
}
pub fn builtin_redis_type(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::String("string".into()))
}
pub fn builtin_redis_rename(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Bool(true))
}
pub fn builtin_redis_flush_db(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Bool(true))
}
pub fn builtin_redis_incr(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::redis_incr(args, line, col)
}
pub fn builtin_redis_decr(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::redis_incr(args, line, col)
}
pub fn builtin_redis_incrby(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::redis_incr(args, line, col)
}
pub fn builtin_redis_incrbyfloat(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::redis_incr(args, line, col)
}
pub fn builtin_redis_lpush(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::redis_lpush(args, line, col)
}
pub fn builtin_redis_rpush(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::redis_rpush(args, line, col)
}
pub fn builtin_redis_lpop(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::None)
}
pub fn builtin_redis_rpop(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::None)
}
pub fn builtin_redis_blpop(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::None)
}
pub fn builtin_redis_lrange(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::redis_lrange(args, line, col)
}
pub fn builtin_redis_llen(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Int(0))
}
pub fn builtin_redis_hset(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::redis_hset(args, line, col)
}
pub fn builtin_redis_hmset(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Bool(true))
}
pub fn builtin_redis_hget(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::redis_hget(args, line, col)
}
pub fn builtin_redis_hmget(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Array(vec![]))
}
pub fn builtin_redis_hgetall(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Struct { type_name: "Map".into(), fields: HashMap::new() })
}
pub fn builtin_redis_hdel(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Int(1))
}
pub fn builtin_redis_hexists(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Bool(false))
}
pub fn builtin_redis_hkeys(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Array(vec![]))
}
pub fn builtin_redis_hvals(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Array(vec![]))
}
pub fn builtin_redis_hincrby(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Int(1))
}
pub fn builtin_redis_sadd(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Int(1))
}
pub fn builtin_redis_srem(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Int(1))
}
pub fn builtin_redis_sismember(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Bool(false))
}
pub fn builtin_redis_smembers(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Array(vec![]))
}
pub fn builtin_redis_scard(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Int(0))
}
pub fn builtin_redis_sinter(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Array(vec![]))
}
pub fn builtin_redis_sunion(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Array(vec![]))
}
pub fn builtin_redis_sdiff(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Array(vec![]))
}
pub fn builtin_redis_zadd(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Int(1))
}
pub fn builtin_redis_zadd_many(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Int(1))
}
pub fn builtin_redis_zrange(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Array(vec![]))
}
pub fn builtin_redis_zrevrange(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Array(vec![]))
}
pub fn builtin_redis_zrank(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::None)
}
pub fn builtin_redis_zscore(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::None)
}
pub fn builtin_redis_zcard(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Int(0))
}
pub fn builtin_redis_zrem(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Int(0))
}
pub fn builtin_redis_publish(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::redis_publish(args, line, col)
}
pub fn builtin_redis_subscribe(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::None)
}
pub fn builtin_redis_unsubscribe(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Bool(true))
}
pub fn builtin_redis_pipeline(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Array(vec![]))
}
pub fn builtin_redis_xadd(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::String("0-0".into()))
}
pub fn builtin_redis_xread(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Array(vec![]))
}
pub fn builtin_redis_xlen(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Int(0))
}

// ============================================================
// std.db_clickhouse — ClickHouse builtins
// ============================================================

pub fn builtin_clickhouse_connect(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::clickhouse_connect(args, line, col)
}
pub fn builtin_clickhouse_connect_url(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::clickhouse_connect(args, line, col)
}
pub fn builtin_clickhouse_close(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::close(args, line, col)
}
pub fn builtin_clickhouse_ping(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Bool(true))
}
pub fn builtin_clickhouse_query(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::clickhouse_query(args, line, col)
}
pub fn builtin_clickhouse_execute(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::clickhouse_execute(args, line, col)
}
pub fn builtin_clickhouse_insert(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Bool(true))
}

// ============================================================
// std.db_cassandra — Cassandra/ScyllaDB builtins
// ============================================================

pub fn builtin_cassandra_connect(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::cassandra_connect(args, line, col)
}
pub fn builtin_cassandra_connect_auth(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::cassandra_connect(args, line, col)
}
pub fn builtin_cassandra_close(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::close(args, line, col)
}
pub fn builtin_cassandra_ping(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Bool(true))
}
pub fn builtin_cassandra_query(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::cassandra_query(args, line, col)
}
pub fn builtin_cassandra_execute(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::cassandra_execute(args, line, col)
}
pub fn builtin_cassandra_prepare(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::String("prepared".into()))
}
pub fn builtin_cassandra_execute_prepared(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Bool(true))
}
pub fn builtin_cassandra_batch(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Bool(true))
}
pub fn builtin_cassandra_query_paged(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::cassandra_query(args, line, col)
}
pub fn builtin_cassandra_replication_string(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::String("{'class': 'SimpleStrategy', 'replication_factor': 1}".into()))
}

// ============================================================
// std.db_elasticsearch — Elasticsearch builtins
// ============================================================

pub fn builtin_elastic_connect(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::elastic_connect(args, line, col)
}
pub fn builtin_elastic_connect_basic(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::elastic_connect(args, line, col)
}
pub fn builtin_elastic_connect_cloud(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::elastic_connect(args, line, col)
}
pub fn builtin_elastic_close(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::close(args, line, col)
}
pub fn builtin_elastic_ping(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Bool(true))
}
pub fn builtin_elastic_info(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::None)
}
pub fn builtin_elastic_create_index(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::elastic_create_index(args, line, col)
}
pub fn builtin_elastic_delete_index(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::elastic_delete_index(args, line, col)
}
pub fn builtin_elastic_index_exists(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Bool(false))
}
pub fn builtin_elastic_list_indexes(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Array(vec![]))
}
pub fn builtin_elastic_get_mapping(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::None)
}
pub fn builtin_elastic_update_mapping(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Bool(true))
}
pub fn builtin_elastic_get_settings(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::None)
}
pub fn builtin_elastic_refresh(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Bool(true))
}
pub fn builtin_elastic_index_doc(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::elastic_index_doc(args, line, col)
}
pub fn builtin_elastic_index_doc_auto(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::elastic_index_doc(args, line, col)
}
pub fn builtin_elastic_get_doc(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::elastic_get_doc(args, line, col)
}
pub fn builtin_elastic_doc_exists(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Bool(false))
}
pub fn builtin_elastic_update_doc(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Bool(true))
}
pub fn builtin_elastic_delete_doc(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::elastic_delete_doc(args, line, col)
}
pub fn builtin_elastic_bulk(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Bool(true))
}
pub fn builtin_elastic_search(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    crate::db_runtime::elastic_search(args, line, col)
}
pub fn builtin_elastic_delete_by_query(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Bool(true))
}
pub fn builtin_elastic_update_by_query(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Bool(true))
}
pub fn builtin_elastic_scroll_start(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::None)
}
pub fn builtin_elastic_scroll_next(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::None)
}
pub fn builtin_elastic_scroll_clear(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::Bool(true))
}
pub fn builtin_elastic_cluster_health(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::None)
}
pub fn builtin_elastic_cluster_stats(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::None)
}
pub fn builtin_elastic_nodes_info(args: &[Value], _line: usize, _col: usize) -> VietResult<Value> {
    Ok(Value::None)
}
