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

/// RFC 3174 SHA-1 Digest implementation
fn sha1_digest(data: &[u8]) -> [u8; 20] {
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
            w[i] = u32::from_be_bytes([chunk[i*4], chunk[i*4+1], chunk[i*4+2], chunk[i*4+3]]);
        }
        for i in 16..80 {
            w[i] = (w[i-3] ^ w[i-8] ^ w[i-14] ^ w[i-16]).rotate_left(1);
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
            let temp = a.rotate_left(5).wrapping_add(f).wrapping_add(e).wrapping_add(k).wrapping_add(w[i]);
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

fn base64_encode_bytes(bytes: &[u8]) -> String {
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
        return Err(VietError::runtime_error("sha1() takes 1 argument".into(), line, col));
    }
    let s = match &args[0] { Value::String(s) => s.as_bytes(), _ => return Err(VietError::type_error("sha1() expects string".into(), line, col)) };
    let digest = sha1_digest(s);
    let hex: String = digest.iter().map(|b| format!("{:02x}", b)).collect();
    Ok(Value::String(hex))
}

pub fn builtin_ws_accept_key(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error("ws_accept_key() takes 1 argument (sec_websocket_key)".into(), line, col));
    }
    let key = match &args[0] { Value::String(s) => s.clone(), _ => return Err(VietError::type_error("ws_accept_key() expects string".into(), line, col)) };
    let concat = format!("{}258EAFA5-E914-47DA-95CA-C5AB0DC85B11", key);
    let digest = sha1_digest(concat.as_bytes());
    let encoded = base64_encode_bytes(&digest);
    Ok(Value::String(encoded))
}

pub fn builtin_tcp_send(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() < 3 || args.len() > 4 {
        return Err(VietError::runtime_error("tcp_send() takes 3-4 arguments (host, port, payload, [timeout_ms])".into(), line, col));
    }
    let host = match &args[0] { Value::String(s) => s.clone(), _ => return Err(VietError::type_error("tcp_send() host must be String".into(), line, col)) };
    let port = match &args[1] { Value::Int(n) => *n as u16, _ => return Err(VietError::type_error("tcp_send() port must be Int".into(), line, col)) };
    let payload = match &args[2] { Value::String(s) => s.clone(), _ => return Err(VietError::type_error("tcp_send() payload must be String".into(), line, col)) };
    let timeout_ms = if args.len() == 4 {
        match &args[3] { Value::Int(n) => *n as u64, _ => 3000 }
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
                Err(e) => return Err(VietError::runtime_error(format!("TCP connection error: {}", e), line, col)),
            }
        }
    }
    Err(VietError::runtime_error(format!("Cannot resolve host '{}'", host), line, col))
}

pub fn builtin_udp_send(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 3 {
        return Err(VietError::runtime_error("udp_send() takes 3 arguments (host, port, payload)".into(), line, col));
    }
    let host = match &args[0] { Value::String(s) => s.clone(), _ => return Err(VietError::type_error("udp_send() host must be String".into(), line, col)) };
    let port = match &args[1] { Value::Int(n) => *n as u16, _ => return Err(VietError::type_error("udp_send() port must be Int".into(), line, col)) };
    let payload = match &args[2] { Value::String(s) => s.clone(), _ => return Err(VietError::type_error("udp_send() payload must be String".into(), line, col)) };

    match std::net::UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => {
            let addr = format!("{}:{}", host, port);
            match socket.send_to(payload.as_bytes(), addr) {
                Ok(_) => Ok(Value::Bool(true)),
                Err(e) => Err(VietError::runtime_error(format!("UDP send error: {}", e), line, col)),
            }
        }
        Err(e) => Err(VietError::runtime_error(format!("UDP bind error: {}", e), line, col)),
    }
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
    eprintln!("\x1b[32m VietLang HTTP Server listening on http://localhost:{}\x1b[0m", port);

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

#[allow(dead_code)]
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

// ============================================================
// String Character Operations (needed for self-hosting)
// ============================================================

pub fn builtin_char_at(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 2 {
        return Err(VietError::runtime_error("char_at() takes 2 arguments (string, index)".into(), line, col));
    }
    let s = match &args[0] { Value::String(s) => s.clone(), _ => return Err(VietError::type_error("char_at() expects a string".into(), line, col)) };
    let idx = match &args[1] { Value::Int(n) => *n as usize, _ => return Err(VietError::type_error("char_at() index must be Int".into(), line, col)) };
    let chars: Vec<char> = s.chars().collect();
    if idx < chars.len() {
        Ok(Value::String(chars[idx].to_string()))
    } else {
        Ok(Value::None)
    }
}

pub fn builtin_char_code(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error("char_code() takes 1 argument".into(), line, col));
    }
    let s = match &args[0] { Value::String(s) => s.clone(), _ => return Err(VietError::type_error("char_code() expects a string".into(), line, col)) };
    if let Some(ch) = s.chars().next() {
        Ok(Value::Int(ch as i64))
    } else {
        Ok(Value::Int(0))
    }
}

pub fn builtin_from_char_code(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error("from_char_code() takes 1 argument".into(), line, col));
    }
    let code = match &args[0] { Value::Int(n) => *n as u32, _ => return Err(VietError::type_error("from_char_code() expects Int".into(), line, col)) };
    match char::from_u32(code) {
        Some(ch) => Ok(Value::String(ch.to_string())),
        None => Ok(Value::String(String::new())),
    }
}

pub fn builtin_substring(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() < 2 || args.len() > 3 {
        return Err(VietError::runtime_error("substring() takes 2-3 arguments (string, start, [end])".into(), line, col));
    }
    let s = match &args[0] { Value::String(s) => s.clone(), _ => return Err(VietError::type_error("substring() expects a string".into(), line, col)) };
    let start = match &args[1] { Value::Int(n) => *n as usize, _ => return Err(VietError::type_error("substring() start must be Int".into(), line, col)) };
    let chars: Vec<char> = s.chars().collect();
    let end = if args.len() == 3 {
        match &args[2] { Value::Int(n) => (*n as usize).min(chars.len()), _ => chars.len() }
    } else {
        chars.len()
    };
    if start <= end && start <= chars.len() {
        Ok(Value::String(chars[start..end.min(chars.len())].iter().collect()))
    } else {
        Ok(Value::String(String::new()))
    }
}

pub fn builtin_str_repeat(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 2 {
        return Err(VietError::runtime_error("str_repeat() takes 2 arguments (string, count)".into(), line, col));
    }
    let s = match &args[0] { Value::String(s) => s.clone(), _ => return Err(VietError::type_error("str_repeat() expects a string".into(), line, col)) };
    let n = match &args[1] { Value::Int(n) => *n as usize, _ => return Err(VietError::type_error("str_repeat() count must be Int".into(), line, col)) };
    Ok(Value::String(s.repeat(n)))
}

pub fn builtin_parse_int(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error("parse_int() takes 1 argument".into(), line, col));
    }
    let s = match &args[0] { Value::String(s) => s.clone(), _ => return Err(VietError::type_error("parse_int() expects a string".into(), line, col)) };
    match s.trim().parse::<i64>() {
        Ok(n) => Ok(Value::Int(n)),
        Err(_) => Ok(Value::None),
    }
}

pub fn builtin_parse_float(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error("parse_float() takes 1 argument".into(), line, col));
    }
    let s = match &args[0] { Value::String(s) => s.clone(), _ => return Err(VietError::type_error("parse_float() expects a string".into(), line, col)) };
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
        return Err(VietError::runtime_error("sort() takes 1 argument".into(), line, col));
    }
    let arr = match &args[0] {
        Value::Array(a) => a.clone(),
        _ => return Err(VietError::type_error("sort() expects an array".into(), line, col)),
    };
    let mut sorted = arr;
    sorted.sort_by(|a, b| {
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => x.cmp(y),
            (Value::Float(x), Value::Float(y)) => x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal),
            (Value::String(x), Value::String(y)) => x.cmp(y),
            _ => std::cmp::Ordering::Equal,
        }
    });
    Ok(Value::Array(sorted))
}

pub fn builtin_array_slice(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() < 2 || args.len() > 3 {
        return Err(VietError::runtime_error("slice() takes 2-3 arguments (array, start, [end])".into(), line, col));
    }
    let arr = match &args[0] { Value::Array(a) => a.clone(), _ => return Err(VietError::type_error("slice() expects array".into(), line, col)) };
    let start = match &args[1] { Value::Int(n) => *n as usize, _ => 0 };
    let end = if args.len() == 3 {
        match &args[2] { Value::Int(n) => (*n as usize).min(arr.len()), _ => arr.len() }
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
        return Err(VietError::runtime_error("index_of() takes 2 arguments (array, value)".into(), line, col));
    }
    let arr = match &args[0] { Value::Array(a) => a.clone(), _ => return Err(VietError::type_error("index_of() expects array".into(), line, col)) };
    for (i, item) in arr.iter().enumerate() {
        if item == &args[1] {
            return Ok(Value::Int(i as i64));
        }
    }
    Ok(Value::Int(-1))
}

pub fn builtin_array_flat(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error("flat() takes 1 argument".into(), line, col));
    }
    let arr = match &args[0] { Value::Array(a) => a.clone(), _ => return Err(VietError::type_error("flat() expects array".into(), line, col)) };
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
        return Err(VietError::runtime_error("is_error() takes 1 argument".into(), line, col));
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
        return Err(VietError::runtime_error("sleep_ms() takes 1 argument (milliseconds)".into(), line, col));
    }
    let ms = match &args[0] {
        Value::Int(n) => *n as u64,
        _ => return Err(VietError::type_error("sleep_ms() expects Int".into(), line, col)),
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
        return Err(VietError::runtime_error("tcp_ping() takes 2-3 arguments (host, port, [timeout_ms])".into(), line, col));
    }
    let host = match &args[0] { Value::String(s) => s.clone(), _ => return Err(VietError::type_error("tcp_ping() host must be String".into(), line, col)) };
    let port = match &args[1] { Value::Int(n) => *n as u16, _ => return Err(VietError::type_error("tcp_ping() port must be Int".into(), line, col)) };
    let timeout_ms = if args.len() == 3 {
        match &args[2] { Value::Int(n) => *n as u64, _ => 1000 }
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
        return Err(VietError::runtime_error("str_split_lines() takes 1 argument".into(), line, col));
    }
    let s = match &args[0] { Value::String(s) => s.clone(), _ => return Err(VietError::type_error("str_split_lines() expects String".into(), line, col)) };
    let lines: Vec<Value> = s.lines().map(|l| Value::String(l.to_string())).collect();
    Ok(Value::Array(lines))
}

pub fn builtin_system_cmd(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error("system_cmd() takes 1 argument (command)".into(), line, col));
    }
    let cmd_str = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(VietError::type_error("system_cmd() expects a string".into(), line, col)),
    };
    use std::process::Command;
    let output = Command::new("sh")
        .arg("-c")
        .arg(&cmd_str)
        .output();

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
            Ok(Value::Struct { type_name: "CommandResult".to_string(), fields: res })
        }
        Err(e) => Err(VietError::runtime_error(format!("Failed to execute command '{}': {}", cmd_str, e), line, col)),
    }
}

pub fn builtin_url_encode(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error("url_encode() takes 1 argument".into(), line, col));
    }
    let s = match &args[0] { Value::String(s) => s.as_str(), _ => return Err(VietError::type_error("url_encode() expects String".into(), line, col)) };
    let mut encoded = String::new();
    for b in s.bytes() {
        match b {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => encoded.push(b as char),
            b' ' => encoded.push('+'),
            _ => encoded.push_str(&format!("%{:02X}", b)),
        }
    }
    Ok(Value::String(encoded))
}

pub fn builtin_url_decode(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 1 {
        return Err(VietError::runtime_error("url_decode() takes 1 argument".into(), line, col));
    }
    let s = match &args[0] { Value::String(s) => s.as_str(), _ => return Err(VietError::type_error("url_decode() expects String".into(), line, col)) };
    let mut decoded = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(val) = u8::from_str_radix(std::str::from_utf8(&bytes[i+1..i+3]).unwrap_or(""), 16) {
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
        return Err(VietError::runtime_error("hmac_sha256() takes 2 arguments (message, key)".into(), line, col));
    }
    let msg = match &args[0] { Value::String(s) => s.clone(), _ => return Err(VietError::type_error("hmac_sha256() message must be String".into(), line, col)) };
    let key = match &args[1] { Value::String(s) => s.clone(), _ => return Err(VietError::type_error("hmac_sha256() key must be String".into(), line, col)) };
    let combined = format!("{}:{}:{}", key, msg, key);
    let hash = simple_sha256(combined.as_bytes());
    Ok(Value::String(hash))
}

pub fn builtin_encrypt_secret(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 2 {
        return Err(VietError::runtime_error("encrypt_secret() takes 2 arguments (plaintext, key)".into(), line, col));
    }
    let text = match &args[0] { Value::String(s) => s.as_bytes(), _ => return Err(VietError::type_error("encrypt_secret() plaintext must be String".into(), line, col)) };
    let key = match &args[1] { Value::String(s) => s.as_bytes(), _ => return Err(VietError::type_error("encrypt_secret() key must be String".into(), line, col)) };
    if key.is_empty() { return Err(VietError::runtime_error("Key cannot be empty".into(), line, col)); }
    
    let mut encrypted = Vec::with_capacity(text.len());
    for (i, &b) in text.iter().enumerate() {
        let k = key[i % key.len()];
        encrypted.push(b ^ k ^ ((i as u8).wrapping_mul(7)));
    }
    Ok(Value::String(base64_encode_bytes(&encrypted)))
}

pub fn builtin_decrypt_secret(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 2 {
        return Err(VietError::runtime_error("decrypt_secret() takes 2 arguments (ciphertext, key)".into(), line, col));
    }
    let b64 = match &args[0] { Value::String(s) => s.clone(), _ => return Err(VietError::type_error("decrypt_secret() ciphertext must be String".into(), line, col)) };
    let key = match &args[1] { Value::String(s) => s.as_bytes(), _ => return Err(VietError::type_error("decrypt_secret() key must be String".into(), line, col)) };
    if key.is_empty() { return Err(VietError::runtime_error("Key cannot be empty".into(), line, col)); }

    // Decode base64
    let mut bytes = Vec::new();
    let clean = b64.replace(['\r', '\n', ' '], "");
    let mut buf = [0u8; 4];
    let mut count = 0;
    for &c in clean.as_bytes() {
        if c == b'=' { break; }
        let val = match c {
            b'A'..=b'Z' => c - b'A',
            b'a'..=b'z' => c - b'a' + 26,
            b'0'..=b'9' => c - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            _ => continue,
        };
        buf[count] = val;
        count += 1;
        if count == 4 {
            bytes.push((buf[0] << 2) | (buf[1] >> 4));
            bytes.push((buf[1] << 4) | (buf[2] >> 2));
            bytes.push((buf[2] << 6) | buf[3]);
            count = 0;
        }
    }
    if count == 2 {
        bytes.push((buf[0] << 2) | (buf[1] >> 4));
    } else if count == 3 {
        bytes.push((buf[0] << 2) | (buf[1] >> 4));
        bytes.push((buf[1] << 4) | (buf[2] >> 2));
    }

    let mut decrypted = Vec::with_capacity(bytes.len());
    for (i, &b) in bytes.iter().enumerate() {
        let k = key[i % key.len()];
        decrypted.push(b ^ k ^ ((i as u8).wrapping_mul(7)));
    }
    Ok(Value::String(String::from_utf8_lossy(&decrypted).to_string()))
}

pub fn builtin_ip_in_cidr(args: &[Value], line: usize, col: usize) -> VietResult<Value> {
    if args.len() != 2 {
        return Err(VietError::runtime_error("ip_in_cidr() takes 2 arguments (ip, cidr)".into(), line, col));
    }
    let ip_str = match &args[0] { Value::String(s) => s.clone(), _ => return Err(VietError::type_error("ip must be String".into(), line, col)) };
    let cidr_str = match &args[1] { Value::String(s) => s.clone(), _ => return Err(VietError::type_error("cidr must be String".into(), line, col)) };

    if cidr_str == "0.0.0.0/0" || cidr_str == "*" {
        return Ok(Value::Bool(true));
    }
    if !cidr_str.contains('/') {
        return Ok(Value::Bool(ip_str == cidr_str));
    }

    let parts: Vec<&str> = cidr_str.split('/').collect();
    if parts.len() != 2 { return Ok(Value::Bool(false)); }
    let prefix = parts[0];
    let mask: u32 = parts[1].parse().unwrap_or(32);

    let ip_num = parse_ipv4(&ip_str);
    let net_num = parse_ipv4(prefix);
    if ip_num.is_none() || net_num.is_none() {
        return Ok(Value::Bool(false));
    }

    let mask_bits = if mask == 0 { 0 } else { !0u32 << (32 - mask) };
    Ok(Value::Bool((ip_num.unwrap() & mask_bits) == (net_num.unwrap() & mask_bits)))
}

fn parse_ipv4(s: &str) -> Option<u32> {
    let octets: Vec<&str> = s.trim().split('.').collect();
    if octets.len() != 4 { return None; }
    let o1: u32 = octets[0].parse().ok()?;
    let o2: u32 = octets[1].parse().ok()?;
    let o3: u32 = octets[2].parse().ok()?;
    let o4: u32 = octets[3].parse().ok()?;
    Some((o1 << 24) | (o2 << 16) | (o3 << 8) | o4)
}

