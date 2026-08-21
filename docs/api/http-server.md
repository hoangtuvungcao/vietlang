# Native HTTP server and client

VietLang's native transport path uses maintained Rust protocol libraries:
Reqwest/Rustls for outbound HTTP/HTTPS, and Tokio/Hyper/Axum/Rustls for inbound
HTTP/1.1 and HTTP/2. HTTP/3 uses Quinn and h3 as an opt-in experimental server
transport.

## HTTP/1.1 and HTTP/2 server

```vietlang
let mut config = map_new()
config = map_set(config, "addr", "0.0.0.0")
config = map_set(config, "port", 8080)
config = map_set(config, "workers", 4)
config = map_set(config, "max_concurrency", 256)
config = map_set(config, "max_body_bytes", 1048576)
config = map_set(config, "max_header_bytes", 65536)
config = map_set(config, "max_response_bytes", 8388608)
config = map_set(config, "request_timeout_ms", 30000)
config = map_set(config, "shutdown_timeout_ms", 10000)

http_listen(config, fn(req) {
    let mut response = map_new()
    response = map_set(response, "status_code", 200)
    response = map_set(response, "content_type", "application/json; charset=utf-8")
    response = map_set(response, "body", "{\"status\":\"ok\"}")
    return response
})
```

The handler receives `method`, `path`, `query`, `protocol`, `client_ip`,
`request_id`, `headers`, `body`, and `body_base64`. Handler state is cloned per
request. Use explicitly synchronized native services for shared mutable state.

The runtime rejects oversized headers and bodies, caps concurrent handlers,
times handlers out, limits response size, strips hop-by-hop response headers,
adds defensive browser headers, and shuts down gracefully on Ctrl-C.

## HTTPS and HTTP/3

```vietlang
config = map_set(config, "port", 8443)
config = map_set(config, "tls_cert_file", "certs/fullchain.pem")
config = map_set(config, "tls_key_file", "certs/private-key.pem")

// Optional: HTTP/3 over QUIC on UDP 8443.
config = map_set(config, "http3", true)
config = map_set(config, "http3_port", 8443)
```

Both TLS files are mandatory together. HTTP/3 also requires UDP access through
the firewall/load balancer. The server advertises it with `Alt-Svc`. The h3
dependency is still experimental, so HTTP/3 is not part of VietLang's current
production-readiness claim.

## Outbound HTTP/HTTPS

```vietlang
let mut options = map_new()
options = map_set(options, "timeout_ms", 10000)
options = map_set(options, "max_response_bytes", 2097152)
let response = http_fetch("https://api.example.com/v1/health", "GET", map_new(), "", options)
```

`http_fetch` supports HTTP/1.1 and HTTP/2 through negotiation, validates URLs,
methods, and headers, enforces TLS 1.2 or newer for HTTPS, does not follow
redirects automatically, pools connections, and bounds response reads. It does
not currently provide an HTTP/3 client.

## Production boundary

These changes remove the hand-written plaintext protocol path from normal
execution, but they do not by themselves make VietLang 0.3.0-alpha.1 production-certified.
CI covers bounded HTTP load, HTTP/2 TLS, HTTP/3 QUIC, async WebSocket framing,
deterministic fuzzing, SQL pool construction, and semantic/module checks. An
independent security review and externally operated extended failure-injection
campaign remain release gates. See `docs/upgrade-roadmap.md`.
