//! HTTP/1.1, HTTP/2, HTTPS, and opt-in HTTP/3 server runtime.
//!
//! The runtime clones a baseline interpreter for every request. Request-local
//! mutations are isolated; shared state must use an explicitly synchronized
//! native service such as the database or channel APIs.

use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
    sync::Arc,
    time::Duration,
};

use axum::{
    body::{to_bytes, Body},
    extract::{ws::WebSocketUpgrade, ConnectInfo, FromRequestParts, Request, State},
    http::{header, HeaderName, HeaderValue, StatusCode, Version},
    response::Response,
    routing::any,
    Router,
};
use axum_server::{tls_rustls::RustlsConfig, Handle};
use bytes::{Buf, Bytes};
use h3_quinn::quinn;
use tokio::sync::Semaphore;

use crate::{
    error::{VietError, VietResult},
    interpreter::{value::Value, Interpreter},
    lexer::token::Span,
};

#[derive(Clone)]
struct ServerState {
    interpreter: Interpreter,
    handler: Option<Value>,
    span: Span,
    max_body_bytes: usize,
    max_header_bytes: usize,
    max_response_bytes: usize,
    request_timeout: Duration,
    concurrency: Arc<Semaphore>,
    cors_allow_origin: Option<String>,
    is_tls: bool,
    http3_port: Option<u16>,
}

#[derive(Debug)]
struct ServerConfig {
    address: SocketAddr,
    workers: usize,
    max_concurrency: usize,
    max_body_bytes: usize,
    max_header_bytes: usize,
    max_response_bytes: usize,
    request_timeout: Duration,
    shutdown_timeout: Duration,
    tls_cert_file: Option<String>,
    tls_key_file: Option<String>,
    cors_allow_origin: Option<String>,
    http3_port: Option<u16>,
}

pub(crate) fn run_http_server(
    interpreter: Interpreter,
    args: &[Value],
    span: &Span,
) -> VietResult<Value> {
    // Several dependencies expose different rustls providers. Select one once
    // so TLS configuration never depends on feature-unification order.
    let _ = rustls::crypto::ring::default_provider().install_default();
    let (config, handler) = ServerConfig::from_args(args, span)?;
    let state = ServerState {
        interpreter,
        handler,
        span: span.clone(),
        max_body_bytes: config.max_body_bytes,
        max_header_bytes: config.max_header_bytes,
        max_response_bytes: config.max_response_bytes,
        request_timeout: config.request_timeout,
        concurrency: Arc::new(Semaphore::new(config.max_concurrency)),
        cors_allow_origin: config.cors_allow_origin.clone(),
        is_tls: config.tls_cert_file.is_some(),
        http3_port: config.http3_port,
    };
    let app = Router::new()
        .fallback(any(dispatch))
        .with_state(state.clone());
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(config.workers)
        .thread_name("vietlang-http")
        .enable_all()
        .build()
        .map_err(|error| {
            VietError::runtime_error(
                format!("Cannot initialize HTTP runtime: {}", error),
                span.line,
                span.column,
            )
        })?;

    let scheme = if config.tls_cert_file.is_some() {
        "https"
    } else {
        "http"
    };
    eprintln!(
        "\x1b[32m[VietLang HTTP]\x1b[0m {}://{} (HTTP/1.1 + HTTP/2, workers={}, max_concurrency={})",
        scheme, config.address, config.workers, config.max_concurrency,
    );

    runtime.block_on(async move {
        let handle = Handle::new();
        let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);
        install_shutdown_signal(handle.clone(), config.shutdown_timeout, shutdown_tx.clone());
        let http3_task = if let Some(http3_port) = config.http3_port {
            let cert = config
                .tls_cert_file
                .as_deref()
                .expect("HTTP/3 requires TLS certificate");
            let key = config
                .tls_key_file
                .as_deref()
                .expect("HTTP/3 requires TLS private key");
            let http3_address = SocketAddr::new(config.address.ip(), http3_port);
            let endpoint =
                build_http3_endpoint(http3_address, cert, key, config.max_concurrency, span)?;
            eprintln!(
                "\x1b[32m[VietLang HTTP/3]\x1b[0m https://{} (QUIC, experimental h3 transport)",
                http3_address
            );
            Some(tokio::spawn(serve_http3(
                endpoint,
                state.clone(),
                shutdown_rx,
            )))
        } else {
            None
        };
        let service = app.into_make_service_with_connect_info::<SocketAddr>();

        let result = match (config.tls_cert_file, config.tls_key_file) {
            (Some(cert), Some(key)) => {
                let tls = RustlsConfig::from_pem_file(cert, key)
                    .await
                    .map_err(|error| {
                        VietError::runtime_error(
                            format!("Cannot load TLS certificate/private key: {}", error),
                            span.line,
                            span.column,
                        )
                    })?;
                axum_server::bind_rustls(config.address, tls)
                    .handle(handle)
                    .serve(service)
                    .await
            }
            (None, None) => {
                axum_server::bind(config.address)
                    .handle(handle)
                    .serve(service)
                    .await
            }
            _ => unreachable!("TLS paths are validated together"),
        };

        let _ = shutdown_tx.send(true);
        if let Some(task) = http3_task {
            let _ = tokio::time::timeout(config.shutdown_timeout, task).await;
        }
        result.map_err(|error| {
            VietError::runtime_error(
                format!("HTTP server failed: {}", error),
                span.line,
                span.column,
            )
        })?;
        Ok(Value::None)
    })
}

impl ServerConfig {
    fn from_args(args: &[Value], span: &Span) -> VietResult<(Self, Option<Value>)> {
        if args.is_empty() || args.len() > 2 {
            return Err(VietError::runtime_error(
                "http_listen() takes 1-2 arguments (port_or_address_or_config, handler)".into(),
                span.line,
                span.column,
            ));
        }

        let mut bind_ip = "0.0.0.0".to_string();
        let mut port = 8080i64;
        let mut workers = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(2);
        workers = workers.clamp(2, 64);
        let mut max_concurrency = 256i64;
        let mut max_body_bytes = 1_048_576i64;
        let mut max_header_bytes = 65_536i64;
        let mut max_response_bytes = 8_388_608i64;
        let mut request_timeout_ms = 30_000i64;
        let mut shutdown_timeout_ms = 10_000i64;
        let mut tls_cert_file = None;
        let mut tls_key_file = None;
        let mut cors_allow_origin = None;
        let mut enable_http3 = false;
        let mut http3_port = None;

        match &args[0] {
            Value::Int(value) => port = *value,
            Value::String(value) => {
                if let Ok(address) = value.parse::<SocketAddr>() {
                    bind_ip = address.ip().to_string();
                    port = address.port() as i64;
                } else if let Ok(value_port) = value.parse::<i64>() {
                    port = value_port;
                } else {
                    return Err(VietError::runtime_error(
                        format!("Invalid HTTP bind address '{}'", value),
                        span.line,
                        span.column,
                    ));
                }
            }
            Value::Struct { fields, .. } => {
                if let Some(Value::String(value)) = fields.get("addr") {
                    if let Ok(address) = value.parse::<SocketAddr>() {
                        bind_ip = address.ip().to_string();
                        port = address.port() as i64;
                    } else {
                        bind_ip = value.clone();
                    }
                }
                read_int(fields, "port", &mut port);
                if let Some(Value::Int(value)) = fields.get("workers") {
                    workers = usize::try_from(*value).unwrap_or(0);
                }
                read_int(fields, "max_concurrency", &mut max_concurrency);
                read_int(fields, "max_body_bytes", &mut max_body_bytes);
                read_int(fields, "max_header_bytes", &mut max_header_bytes);
                read_int(fields, "max_response_bytes", &mut max_response_bytes);
                read_int(fields, "request_timeout_ms", &mut request_timeout_ms);
                read_int(fields, "shutdown_timeout_ms", &mut shutdown_timeout_ms);
                tls_cert_file = read_nonempty_string(fields, "tls_cert_file");
                tls_key_file = read_nonempty_string(fields, "tls_key_file");
                cors_allow_origin = read_nonempty_string(fields, "cors_allow_origin");
                if let Some(Value::Bool(value)) = fields.get("http3") {
                    enable_http3 = *value;
                }
                if let Some(Value::Int(value)) = fields.get("http3_port") {
                    http3_port = u16::try_from(*value).ok();
                    enable_http3 = true;
                }
            }
            _ => {
                return Err(VietError::type_error(
                    "http_listen() first argument must be Int, String, or Config Map".into(),
                    span.line,
                    span.column,
                ))
            }
        }

        if !(1..=65_535).contains(&port) {
            return Err(config_error("port must be between 1 and 65535", span));
        }
        if !(1..=256).contains(&workers) {
            return Err(config_error("workers must be between 1 and 256", span));
        }
        if !(1..=1_000_000).contains(&max_concurrency) {
            return Err(config_error(
                "max_concurrency must be between 1 and 1000000",
                span,
            ));
        }
        if !(1..=67_108_864).contains(&max_body_bytes) {
            return Err(config_error(
                "max_body_bytes must be between 1 and 67108864",
                span,
            ));
        }
        if !(1_024..=1_048_576).contains(&max_header_bytes) {
            return Err(config_error(
                "max_header_bytes must be between 1024 and 1048576",
                span,
            ));
        }
        if !(1..=67_108_864).contains(&max_response_bytes) {
            return Err(config_error(
                "max_response_bytes must be between 1 and 67108864",
                span,
            ));
        }
        if !(1..=300_000).contains(&request_timeout_ms) {
            return Err(config_error(
                "request_timeout_ms must be between 1 and 300000",
                span,
            ));
        }
        if !(1..=300_000).contains(&shutdown_timeout_ms) {
            return Err(config_error(
                "shutdown_timeout_ms must be between 1 and 300000",
                span,
            ));
        }
        if tls_cert_file.is_some() != tls_key_file.is_some() {
            return Err(config_error(
                "tls_cert_file and tls_key_file must be configured together",
                span,
            ));
        }
        if enable_http3 && tls_cert_file.is_none() {
            return Err(config_error(
                "HTTP/3 requires tls_cert_file and tls_key_file",
                span,
            ));
        }
        if enable_http3 && http3_port.is_none() {
            http3_port = Some(port as u16);
        }
        if let Some(origin) = &cors_allow_origin {
            HeaderValue::from_str(origin)
                .map_err(|_| config_error("cors_allow_origin is not a valid header value", span))?;
        }

        let ip = bind_ip.parse::<IpAddr>().map_err(|_| {
            config_error("addr must be a literal IPv4 or IPv6 address; hostnames are not accepted for server binds", span)
        })?;
        Ok((
            Self {
                address: SocketAddr::new(ip, port as u16),
                workers,
                max_concurrency: max_concurrency as usize,
                max_body_bytes: max_body_bytes as usize,
                max_header_bytes: max_header_bytes as usize,
                max_response_bytes: max_response_bytes as usize,
                request_timeout: Duration::from_millis(request_timeout_ms as u64),
                shutdown_timeout: Duration::from_millis(shutdown_timeout_ms as u64),
                tls_cert_file,
                tls_key_file,
                cors_allow_origin,
                http3_port,
            },
            args.get(1).cloned(),
        ))
    }
}

pub(crate) fn validate_config_for_fuzz(value: Value) -> VietResult<()> {
    ServerConfig::from_args(&[value], &Span::new(1, 1, 0, 0)).map(|_| ())
}

async fn dispatch(
    State(state): State<ServerState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    request: Request,
) -> Response<Body> {
    if crate::ws_runtime::endpoint().as_deref() == Some(request.uri().path()) {
        let (mut parts, _body) = request.into_parts();
        return match WebSocketUpgrade::from_request_parts(&mut parts, &state).await {
            Ok(upgrade) => upgrade
                .max_message_size(state.max_body_bytes)
                .max_frame_size(state.max_body_bytes)
                .on_upgrade(crate::ws_runtime::serve),
            Err(_) => error_response(
                StatusCode::BAD_REQUEST,
                "websocket_upgrade_required",
                "A valid RFC 6455 WebSocket upgrade is required",
                &state,
            ),
        };
    }
    let permit = match state.concurrency.clone().try_acquire_owned() {
        Ok(permit) => permit,
        Err(_) => {
            return error_response(
                StatusCode::SERVICE_UNAVAILABLE,
                "server_busy",
                "Server concurrency limit reached",
                &state,
            )
        }
    };

    let (parts, body) = request.into_parts();
    let header_bytes = parts
        .headers
        .iter()
        .map(|(name, value)| name.as_str().len() + value.as_bytes().len())
        .sum::<usize>();
    if header_bytes > state.max_header_bytes {
        return error_response(
            StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE,
            "headers_too_large",
            "Request headers exceed configured limit",
            &state,
        );
    }
    let body = match to_bytes(body, state.max_body_bytes).await {
        Ok(body) => body,
        Err(_) => {
            return error_response(
                StatusCode::PAYLOAD_TOO_LARGE,
                "payload_too_large",
                "Request body exceeds configured limit",
                &state,
            )
        }
    };
    let request_id = match crate::stdlib::builtin_uuid(&[], 0, 0) {
        Ok(Value::String(value)) => value,
        _ => "unknown".to_string(),
    };
    let mut headers = HashMap::new();
    for (name, value) in &parts.headers {
        headers.insert(
            name.as_str().to_string(),
            Value::String(value.to_str().unwrap_or("<non-utf8>").to_string()),
        );
    }
    let mut request_fields = HashMap::new();
    request_fields.insert("method".into(), Value::String(parts.method.to_string()));
    request_fields.insert("path".into(), Value::String(parts.uri.path().to_string()));
    request_fields.insert(
        "query".into(),
        Value::String(parts.uri.query().unwrap_or("").to_string()),
    );
    request_fields.insert(
        "protocol".into(),
        Value::String(protocol_name(parts.version).into()),
    );
    request_fields.insert("client_ip".into(), Value::String(peer.ip().to_string()));
    request_fields.insert("request_id".into(), Value::String(request_id.clone()));
    request_fields.insert(
        "headers".into(),
        Value::Struct {
            type_name: "Map".into(),
            fields: headers,
        },
    );
    request_fields.insert(
        "body".into(),
        Value::String(String::from_utf8_lossy(&body).to_string()),
    );
    request_fields.insert(
        "body_base64".into(),
        Value::String(base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            &body,
        )),
    );
    let request_value = Value::Struct {
        type_name: "HttpRequest".into(),
        fields: request_fields,
    };

    let handler = state.handler.clone();
    let span = state.span.clone();
    let mut interpreter = state.interpreter.clone();
    let task = tokio::task::spawn_blocking(move || {
        let _permit = permit;
        match handler {
            Some(handler) => interpreter.call_function(&handler, &[request_value], &span),
            None => Ok(Value::String("{\"status\":\"ok\"}".into())),
        }
    });
    let result = match tokio::time::timeout(state.request_timeout, task).await {
        Ok(Ok(Ok(value))) => value,
        Ok(Ok(Err(error))) => {
            eprintln!(
                "[VietLang HTTP] request {} handler error: {}",
                request_id, error
            );
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "handler_error",
                "Request handler failed",
                &state,
            );
        }
        Ok(Err(error)) => {
            eprintln!(
                "[VietLang HTTP] request {} task failure: {}",
                request_id, error
            );
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "task_failure",
                "Request task failed",
                &state,
            );
        }
        Err(_) => {
            return error_response(
                StatusCode::GATEWAY_TIMEOUT,
                "handler_timeout",
                "Request handler timed out",
                &state,
            )
        }
    };
    value_response(result, request_id, &state)
}

fn value_response(value: Value, request_id: String, state: &ServerState) -> Response<Body> {
    let (status, content_type, body, extra_headers) = match &value {
        Value::String(body) => {
            let content_type = if body.trim_start().starts_with('<') {
                "text/html; charset=utf-8".to_string()
            } else {
                "application/json; charset=utf-8".to_string()
            };
            (StatusCode::OK, content_type, body.clone(), None)
        }
        Value::Struct { fields, .. } => {
            let status = fields
                .get("status_code")
                .and_then(Value::as_int)
                .and_then(|code| u16::try_from(code).ok())
                .and_then(|code| StatusCode::from_u16(code).ok())
                .unwrap_or(StatusCode::OK);
            let content_type = match fields.get("content_type") {
                Some(Value::String(value)) => value.clone(),
                _ => "application/json; charset=utf-8".to_string(),
            };
            let body = match fields.get("body") {
                Some(Value::String(value)) => value.clone(),
                _ => json_string(&value),
            };
            let headers = match fields.get("headers") {
                Some(Value::Struct { fields, .. }) => Some(fields.clone()),
                _ => None,
            };
            (status, content_type, body, headers)
        }
        _ => (
            StatusCode::OK,
            "application/json; charset=utf-8".into(),
            json_string(&value),
            None,
        ),
    };

    if body.len() > state.max_response_bytes {
        return error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "response_too_large",
            "Handler response exceeds configured limit",
            state,
        );
    }
    let mut response = Response::builder().status(status);
    let headers = response.headers_mut().expect("response builder headers");
    apply_default_headers(headers, state, &request_id);
    if let Ok(value) = HeaderValue::from_str(&content_type) {
        headers.insert(header::CONTENT_TYPE, value);
    }
    if let Some(extra_headers) = extra_headers {
        for (name, value) in extra_headers {
            let Value::String(value) = value else {
                continue;
            };
            let Ok(name) = HeaderName::from_bytes(name.as_bytes()) else {
                continue;
            };
            if is_hop_by_hop(&name) {
                continue;
            }
            if let Ok(value) = HeaderValue::from_str(&value) {
                headers.insert(name, value);
            }
        }
    }
    response
        .body(Body::from(body))
        .unwrap_or_else(|_| Response::new(Body::empty()))
}

fn error_response(
    status: StatusCode,
    code: &str,
    message: &str,
    state: &ServerState,
) -> Response<Body> {
    let body = format!("{{\"error\":\"{}\",\"message\":\"{}\"}}", code, message);
    let mut response = Response::builder().status(status);
    let headers = response.headers_mut().expect("response builder headers");
    apply_default_headers(headers, state, "unassigned");
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json; charset=utf-8"),
    );
    response
        .body(Body::from(body))
        .unwrap_or_else(|_| Response::new(Body::empty()))
}

fn apply_default_headers(
    headers: &mut axum::http::HeaderMap,
    state: &ServerState,
    request_id: &str,
) {
    headers.insert(header::SERVER, HeaderValue::from_static("VietLang"));
    headers.insert(
        "x-content-type-options",
        HeaderValue::from_static("nosniff"),
    );
    headers.insert("x-frame-options", HeaderValue::from_static("DENY"));
    headers.insert("referrer-policy", HeaderValue::from_static("no-referrer"));
    headers.insert(
        "permissions-policy",
        HeaderValue::from_static("camera=(), microphone=(), geolocation=()"),
    );
    if state.is_tls {
        headers.insert(
            "strict-transport-security",
            HeaderValue::from_static("max-age=31536000; includeSubDomains"),
        );
    }
    if let Some(port) = state.http3_port {
        if let Ok(value) = HeaderValue::from_str(&format!("h3=\":{}\"; ma=86400", port)) {
            headers.insert("alt-svc", value);
        }
    }
    if let Ok(value) = HeaderValue::from_str(request_id) {
        headers.insert("x-request-id", value);
    }
    if let Some(origin) = &state.cors_allow_origin {
        if let Ok(value) = HeaderValue::from_str(origin) {
            headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, value);
            headers.insert(header::VARY, HeaderValue::from_static("Origin"));
        }
    }
}

fn is_hop_by_hop(name: &HeaderName) -> bool {
    matches!(
        name.as_str(),
        "connection"
            | "keep-alive"
            | "proxy-authenticate"
            | "proxy-authorization"
            | "te"
            | "trailer"
            | "transfer-encoding"
            | "upgrade"
            | "content-length"
    )
}

fn json_string(value: &Value) -> String {
    match crate::stdlib::builtin_json_stringify(&[value.clone()], 0, 0) {
        Ok(Value::String(value)) => value,
        _ => format!("\"{}\"", value),
    }
}

fn protocol_name(version: Version) -> &'static str {
    match version {
        Version::HTTP_09 => "HTTP/0.9",
        Version::HTTP_10 => "HTTP/1.0",
        Version::HTTP_11 => "HTTP/1.1",
        Version::HTTP_2 => "HTTP/2",
        Version::HTTP_3 => "HTTP/3",
        _ => "HTTP/unknown",
    }
}

fn install_shutdown_signal(
    handle: Handle<SocketAddr>,
    timeout: Duration,
    shutdown: tokio::sync::watch::Sender<bool>,
) {
    tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            eprintln!("\n[VietLang HTTP] graceful shutdown requested");
            let _ = shutdown.send(true);
            handle.graceful_shutdown(Some(timeout));
        }
    });
}

fn build_http3_endpoint(
    address: SocketAddr,
    cert_file: &str,
    key_file: &str,
    max_concurrency: usize,
    span: &Span,
) -> VietResult<quinn::Endpoint> {
    use rustls::pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer};

    let cert_data = std::fs::read(cert_file).map_err(|error| {
        VietError::runtime_error(
            format!(
                "Cannot read HTTP/3 TLS certificate '{}': {}",
                cert_file, error
            ),
            span.line,
            span.column,
        )
    })?;
    let key_data = std::fs::read(key_file).map_err(|error| {
        VietError::runtime_error(
            format!(
                "Cannot read HTTP/3 TLS private key '{}': {}",
                key_file, error
            ),
            span.line,
            span.column,
        )
    })?;
    let certificates = CertificateDer::pem_slice_iter(&cert_data)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| {
            VietError::runtime_error(
                format!("Invalid HTTP/3 certificate PEM: {}", error),
                span.line,
                span.column,
            )
        })?;
    let key = PrivateKeyDer::from_pem_slice(&key_data).map_err(|error| {
        VietError::runtime_error(
            format!("Invalid HTTP/3 private key PEM: {}", error),
            span.line,
            span.column,
        )
    })?;

    build_http3_endpoint_from_identity(address, certificates, key, max_concurrency, span)
}

fn build_http3_endpoint_from_identity(
    address: SocketAddr,
    certificates: Vec<rustls::pki_types::CertificateDer<'static>>,
    key: rustls::pki_types::PrivateKeyDer<'static>,
    max_concurrency: usize,
    span: &Span,
) -> VietResult<quinn::Endpoint> {
    use h3_quinn::quinn::{self, crypto::rustls::QuicServerConfig};

    let mut tls = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certificates, key)
        .map_err(|error| {
            VietError::runtime_error(
                format!("Invalid HTTP/3 TLS identity: {}", error),
                span.line,
                span.column,
            )
        })?;
    tls.alpn_protocols = vec![b"h3".to_vec()];
    let crypto = QuicServerConfig::try_from(tls).map_err(|error| {
        VietError::runtime_error(
            format!("Cannot configure QUIC TLS: {}", error),
            span.line,
            span.column,
        )
    })?;
    let mut server_config = quinn::ServerConfig::with_crypto(Arc::new(crypto));
    let transport = Arc::get_mut(&mut server_config.transport).ok_or_else(|| {
        VietError::runtime_error(
            "Cannot configure QUIC transport limits".into(),
            span.line,
            span.column,
        )
    })?;
    transport.max_concurrent_bidi_streams(quinn::VarInt::from_u32(
        max_concurrency.min(u32::MAX as usize) as u32,
    ));
    transport.max_concurrent_uni_streams(quinn::VarInt::from_u32(128));
    transport.max_idle_timeout(Some(
        Duration::from_secs(30)
            .try_into()
            .expect("valid QUIC idle timeout"),
    ));
    quinn::Endpoint::server(server_config, address).map_err(|error| {
        VietError::runtime_error(
            format!("Cannot bind HTTP/3 UDP endpoint {}: {}", address, error),
            span.line,
            span.column,
        )
    })
}

async fn serve_http3(
    endpoint: quinn::Endpoint,
    state: ServerState,
    mut shutdown: tokio::sync::watch::Receiver<bool>,
) {
    loop {
        let incoming = tokio::select! {
            incoming = endpoint.accept() => incoming,
            changed = shutdown.changed() => {
                if changed.is_ok() { endpoint.close(quinn::VarInt::from_u32(0), b"graceful shutdown"); }
                break;
            }
        };
        let Some(incoming) = incoming else { break };
        let state = state.clone();
        tokio::spawn(async move {
            let connection = match incoming.await {
                Ok(connection) => connection,
                Err(error) => {
                    eprintln!("[VietLang HTTP/3] QUIC handshake failed: {}", error);
                    return;
                }
            };
            let peer = connection.remote_address();
            let mut h3_connection =
                match h3::server::Connection::new(h3_quinn::Connection::new(connection)).await {
                    Ok(connection) => connection,
                    Err(error) => {
                        eprintln!("[VietLang HTTP/3] connection setup failed: {}", error);
                        return;
                    }
                };
            loop {
                match h3_connection.accept().await {
                    Ok(Some(resolver)) => {
                        let state = state.clone();
                        tokio::spawn(async move {
                            if let Err(error) = handle_http3_request(resolver, state, peer).await {
                                eprintln!("[VietLang HTTP/3] request failed: {}", error);
                            }
                        });
                    }
                    Ok(None) => break,
                    Err(error) => {
                        eprintln!("[VietLang HTTP/3] accept failed: {}", error);
                        break;
                    }
                }
            }
        });
    }
    endpoint.wait_idle().await;
}

async fn handle_http3_request<C>(
    resolver: h3::server::RequestResolver<C, Bytes>,
    state: ServerState,
    peer: SocketAddr,
) -> Result<(), String>
where
    C: h3::quic::Connection<Bytes>,
    <C as h3::quic::OpenStreams<Bytes>>::BidiStream: h3::quic::BidiStream<Bytes> + Send + 'static,
{
    let (request, mut stream) = resolver
        .resolve_request()
        .await
        .map_err(|error| error.to_string())?;
    let header_bytes = request
        .headers()
        .iter()
        .map(|(name, value)| name.as_str().len() + value.as_bytes().len())
        .sum::<usize>();
    if header_bytes > state.max_header_bytes {
        return send_http3_response(
            stream,
            error_response(
                StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE,
                "headers_too_large",
                "Request headers exceed configured limit",
                &state,
            ),
            state.max_response_bytes,
        )
        .await;
    }

    let mut body = Vec::new();
    while let Some(mut chunk) = stream
        .recv_data()
        .await
        .map_err(|error| error.to_string())?
    {
        if body.len().saturating_add(chunk.remaining()) > state.max_body_bytes {
            return send_http3_response(
                stream,
                error_response(
                    StatusCode::PAYLOAD_TOO_LARGE,
                    "payload_too_large",
                    "Request body exceeds configured limit",
                    &state,
                ),
                state.max_response_bytes,
            )
            .await;
        }
        while chunk.has_remaining() {
            let bytes = chunk.copy_to_bytes(chunk.remaining());
            body.extend_from_slice(&bytes);
        }
    }

    let permit = match state.concurrency.clone().try_acquire_owned() {
        Ok(permit) => permit,
        Err(_) => {
            return send_http3_response(
                stream,
                error_response(
                    StatusCode::SERVICE_UNAVAILABLE,
                    "server_busy",
                    "Server concurrency limit reached",
                    &state,
                ),
                state.max_response_bytes,
            )
            .await
        }
    };
    let request_id = match crate::stdlib::builtin_uuid(&[], 0, 0) {
        Ok(Value::String(value)) => value,
        _ => "unknown".to_string(),
    };
    let mut headers = HashMap::new();
    for (name, value) in request.headers() {
        headers.insert(
            name.as_str().to_string(),
            Value::String(value.to_str().unwrap_or("<non-utf8>").to_string()),
        );
    }
    let mut fields = HashMap::new();
    fields.insert("method".into(), Value::String(request.method().to_string()));
    fields.insert(
        "path".into(),
        Value::String(request.uri().path().to_string()),
    );
    fields.insert(
        "query".into(),
        Value::String(request.uri().query().unwrap_or("").to_string()),
    );
    fields.insert("protocol".into(), Value::String("HTTP/3".into()));
    fields.insert("client_ip".into(), Value::String(peer.ip().to_string()));
    fields.insert("request_id".into(), Value::String(request_id.clone()));
    fields.insert(
        "headers".into(),
        Value::Struct {
            type_name: "Map".into(),
            fields: headers,
        },
    );
    fields.insert(
        "body".into(),
        Value::String(String::from_utf8_lossy(&body).to_string()),
    );
    fields.insert(
        "body_base64".into(),
        Value::String(base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            &body,
        )),
    );
    let request_value = Value::Struct {
        type_name: "HttpRequest".into(),
        fields,
    };
    let handler = state.handler.clone();
    let span = state.span.clone();
    let mut interpreter = state.interpreter.clone();
    let task = tokio::task::spawn_blocking(move || {
        let _permit = permit;
        match handler {
            Some(handler) => interpreter.call_function(&handler, &[request_value], &span),
            None => Ok(Value::String("{\"status\":\"ok\"}".into())),
        }
    });
    let value = match tokio::time::timeout(state.request_timeout, task).await {
        Ok(Ok(Ok(value))) => value,
        Ok(Ok(Err(_))) => {
            return send_http3_response(
                stream,
                error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "handler_error",
                    "Request handler failed",
                    &state,
                ),
                state.max_response_bytes,
            )
            .await
        }
        Ok(Err(_)) => {
            return send_http3_response(
                stream,
                error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "task_failure",
                    "Request task failed",
                    &state,
                ),
                state.max_response_bytes,
            )
            .await
        }
        Err(_) => {
            return send_http3_response(
                stream,
                error_response(
                    StatusCode::GATEWAY_TIMEOUT,
                    "handler_timeout",
                    "Request handler timed out",
                    &state,
                ),
                state.max_response_bytes,
            )
            .await
        }
    };
    send_http3_response(
        stream,
        value_response(value, request_id, &state),
        state.max_response_bytes,
    )
    .await
}

async fn send_http3_response<S>(
    mut stream: h3::server::RequestStream<S, Bytes>,
    response: Response<Body>,
    max_response_bytes: usize,
) -> Result<(), String>
where
    S: h3::quic::BidiStream<Bytes>,
{
    let (parts, body) = response.into_parts();
    let bytes = to_bytes(body, max_response_bytes)
        .await
        .map_err(|error| error.to_string())?;
    let mut response = http::Response::builder().status(parts.status);
    *response
        .headers_mut()
        .ok_or_else(|| "cannot build HTTP/3 response headers".to_string())? = parts.headers;
    stream
        .send_response(response.body(()).map_err(|error| error.to_string())?)
        .await
        .map_err(|error| error.to_string())?;
    if !bytes.is_empty() {
        stream
            .send_data(bytes)
            .await
            .map_err(|error| error.to_string())?;
    }
    stream.finish().await.map_err(|error| error.to_string())
}

fn read_int(fields: &HashMap<String, Value>, key: &str, target: &mut i64) {
    if let Some(Value::Int(value)) = fields.get(key) {
        *target = *value;
    }
}

fn read_nonempty_string(fields: &HashMap<String, Value>, key: &str) -> Option<String> {
    match fields.get(key) {
        Some(Value::String(value)) if !value.trim().is_empty() => Some(value.clone()),
        _ => None,
    }
}

fn config_error(message: &str, span: &Span) -> VietError {
    VietError::runtime_error(
        format!("Invalid HTTP server config: {}", message),
        span.line,
        span.column,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span() -> Span {
        Span::new(1, 1, 0, 0)
    }

    fn config(fields: HashMap<String, Value>) -> Value {
        Value::Struct {
            type_name: "Map".into(),
            fields,
        }
    }

    fn test_state(is_tls: bool, http3_port: Option<u16>) -> ServerState {
        ServerState {
            interpreter: Interpreter::new(),
            handler: None,
            span: span(),
            max_body_bytes: 1024,
            max_header_bytes: 4096,
            max_response_bytes: 4096,
            request_timeout: Duration::from_secs(1),
            concurrency: Arc::new(Semaphore::new(4)),
            cors_allow_origin: Some("https://example.com".into()),
            is_tls,
            http3_port,
        }
    }

    #[test]
    fn default_config_is_bounded_and_http_only() {
        let (config, handler) = ServerConfig::from_args(&[Value::Int(8080)], &span()).unwrap();
        assert_eq!(config.address, "0.0.0.0:8080".parse().unwrap());
        assert_eq!(config.max_concurrency, 256);
        assert_eq!(config.max_body_bytes, 1_048_576);
        assert_eq!(config.max_response_bytes, 8_388_608);
        assert!(config.tls_cert_file.is_none());
        assert!(config.http3_port.is_none());
        assert!(handler.is_none());
    }

    #[test]
    fn tls_identity_must_be_complete() {
        let mut fields = HashMap::new();
        fields.insert("port".into(), Value::Int(8443));
        fields.insert("tls_cert_file".into(), Value::String("cert.pem".into()));
        let error = ServerConfig::from_args(&[config(fields)], &span()).unwrap_err();
        assert!(error.to_string().contains("configured together"));
    }

    #[test]
    fn http3_requires_tls() {
        let mut fields = HashMap::new();
        fields.insert("port".into(), Value::Int(8443));
        fields.insert("http3".into(), Value::Bool(true));
        let error = ServerConfig::from_args(&[config(fields)], &span()).unwrap_err();
        assert!(error.to_string().contains("HTTP/3 requires"));
    }

    #[test]
    fn invalid_limits_are_rejected() {
        let mut fields = HashMap::new();
        fields.insert("port".into(), Value::Int(8080));
        fields.insert("max_body_bytes".into(), Value::Int(0));
        let error = ServerConfig::from_args(&[config(fields)], &span()).unwrap_err();
        assert!(error.to_string().contains("max_body_bytes"));
    }

    #[test]
    fn secure_headers_and_h3_advertisement_are_applied() {
        let state = test_state(true, Some(8443));
        let response = value_response(Value::String("{}".into()), "request-1".into(), &state);
        let headers = response.headers();
        assert_eq!(headers["x-content-type-options"], "nosniff");
        assert_eq!(
            headers["strict-transport-security"],
            "max-age=31536000; includeSubDomains"
        );
        assert_eq!(
            headers["access-control-allow-origin"],
            "https://example.com"
        );
        assert_eq!(headers["alt-svc"], "h3=\":8443\"; ma=86400");
        assert_eq!(headers["x-request-id"], "request-1");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn https_listener_negotiates_http2() {
        let _ = rustls::crypto::ring::default_provider().install_default();
        let certified = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
        let certificate = certified.cert.der().to_vec();
        let private_key = certified.signing_key.serialize_der();
        let tls = RustlsConfig::from_der(vec![certificate], private_key)
            .await
            .unwrap();
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let address = listener.local_addr().unwrap();
        let handle = Handle::new();
        let app = Router::new()
            .fallback(any(dispatch))
            .with_state(test_state(true, None));
        let server = axum_server::from_tcp_rustls(listener, tls)
            .unwrap()
            .handle(handle.clone())
            .serve(app.into_make_service_with_connect_info::<SocketAddr>());
        let server_task = tokio::spawn(server);

        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .http2_prior_knowledge()
            .build()
            .unwrap();
        let response = client
            .get(format!("https://localhost:{}/health", address.port()))
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.version(), reqwest::Version::HTTP_2);
        assert_eq!(
            response.headers()["strict-transport-security"],
            "max-age=31536000; includeSubDomains"
        );

        handle.graceful_shutdown(None);
        server_task.await.unwrap().unwrap();
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn http3_listener_serves_a_real_quic_request() {
        use h3_quinn::quinn::crypto::rustls::QuicClientConfig;

        let _ = rustls::crypto::ring::default_provider().install_default();
        let certified = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
        let certificate = certified.cert.der().clone();
        let private_key =
            rustls::pki_types::PrivatePkcs8KeyDer::from(certified.signing_key.serialize_der());
        let server_endpoint = build_http3_endpoint_from_identity(
            "127.0.0.1:0".parse().unwrap(),
            vec![certificate.clone()],
            private_key.into(),
            8,
            &span(),
        )
        .unwrap();
        let server_address = server_endpoint.local_addr().unwrap();
        let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);
        let server_task = tokio::spawn(serve_http3(
            server_endpoint,
            test_state(true, Some(server_address.port())),
            shutdown_rx,
        ));

        let mut roots = rustls::RootCertStore::empty();
        roots.add(certificate).unwrap();
        let mut client_tls = rustls::ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth();
        client_tls.alpn_protocols = vec![b"h3".to_vec()];
        let client_crypto = QuicClientConfig::try_from(client_tls).unwrap();
        let mut client_endpoint = quinn::Endpoint::client("127.0.0.1:0".parse().unwrap()).unwrap();
        client_endpoint
            .set_default_client_config(quinn::ClientConfig::new(Arc::new(client_crypto)));
        let connection = client_endpoint
            .connect(server_address, "localhost")
            .unwrap()
            .await
            .unwrap();
        let (mut driver, mut sender) = h3::client::new(h3_quinn::Connection::new(connection))
            .await
            .unwrap();
        let driver_task = tokio::spawn(async move {
            std::future::poll_fn(|context| driver.poll_close(context)).await
        });
        let request = http::Request::get(format!(
            "https://localhost:{}/health",
            server_address.port()
        ))
        .body(())
        .unwrap();
        let mut stream = sender.send_request(request).await.unwrap();
        stream.finish().await.unwrap();
        let response = stream.recv_response().await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers()["alt-svc"],
            format!("h3=\":{}\"; ma=86400", server_address.port())
        );

        drop(stream);
        drop(sender);
        client_endpoint.close(quinn::VarInt::from_u32(0), b"test complete");
        let _ = driver_task.await;
        shutdown_tx.send(true).unwrap();
        tokio::time::timeout(Duration::from_secs(2), server_task)
            .await
            .unwrap()
            .unwrap();
    }
}
