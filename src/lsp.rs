//! Minimal dependency-free JSON-RPC transport for the official VietLang LSP.

use serde_json::{json, Value};
use std::{
    collections::HashMap,
    io::{self, BufRead, Read, Write},
};

use crate::{formatter::format_source, lexer::Lexer, parser::Parser, semantic::SemanticAnalyzer};

pub fn run() -> io::Result<()> {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let stdout = io::stdout();
    let mut output = stdout.lock();
    let mut documents = HashMap::<String, String>::new();
    loop {
        let Some(message) = read_message(&mut input)? else {
            break;
        };
        let method = message.get("method").and_then(Value::as_str).unwrap_or("");
        let id = message.get("id").cloned();
        match method {
            "initialize" => respond(
                &mut output,
                id,
                json!({
                    "capabilities": {
                        "textDocumentSync": 1,
                        "hoverProvider": true,
                        "completionProvider": {"resolveProvider": false},
                        "documentFormattingProvider": true
                    },
                    "serverInfo": {"name": "vietlang-lsp", "version": env!("CARGO_PKG_VERSION")}
                }),
            )?,
            "shutdown" => respond(&mut output, id, Value::Null)?,
            "exit" => break,
            "textDocument/completion" => respond(
                &mut output,
                id,
                json!([
                    {"label":"fn","kind":14,"insertText":"fn name() {\n    \n}"},
                    {"label":"let","kind":14,"insertText":"let name = value"},
                    {"label":"match","kind":14,"insertText":"match value {\n    _ => none\n}"},
                    {"label":"Option","kind":7},{"label":"Result","kind":7},
                    {"label":"http_listen","kind":3},{"label":"http_fetch","kind":3}
                ]),
            )?,
            "textDocument/hover" => respond(
                &mut output,
                id,
                json!({
                    "contents": {"kind":"markdown","value":"VietLang backend API — run `vietlang check` for whole-module typed diagnostics."}
                }),
            )?,
            "textDocument/formatting" => {
                let uri = message
                    .pointer("/params/textDocument/uri")
                    .and_then(Value::as_str)
                    .unwrap_or("");
                let source = documents.get(uri).map(String::as_str).unwrap_or("");
                let edits = match format_source(source) {
                    Ok(formatted) => {
                        json!([{"range":{"start":{"line":0,"character":0},"end":{"line":2147483647u32,"character":0}},"newText":formatted}])
                    }
                    Err(_) => json!([]),
                };
                respond(&mut output, id, edits)?;
            }
            "textDocument/didOpen" | "textDocument/didChange" => {
                let uri = message
                    .pointer("/params/textDocument/uri")
                    .and_then(Value::as_str)
                    .unwrap_or("");
                let text = message
                    .pointer("/params/textDocument/text")
                    .and_then(Value::as_str)
                    .or_else(|| {
                        message
                            .pointer("/params/contentChanges/0/text")
                            .and_then(Value::as_str)
                    })
                    .unwrap_or("");
                documents.insert(uri.to_string(), text.to_string());
                let diagnostics = diagnostics(text);
                notify(
                    &mut output,
                    "textDocument/publishDiagnostics",
                    json!({"uri":uri,"diagnostics":diagnostics}),
                )?;
            }
            _ if id.is_some() => respond_error(&mut output, id, -32601, "Method not found")?,
            _ => {}
        }
    }
    Ok(())
}

fn diagnostics(source: &str) -> Value {
    let result = Lexer::new(source)
        .tokenize()
        .and_then(|tokens| Parser::new(tokens).parse())
        .and_then(|program| SemanticAnalyzer::new().analyze(&program));
    match result {
        Ok(()) => json!([]),
        Err(error) => json!([{
            "range":{"start":{"line":error.line.saturating_sub(1),"character":error.column.saturating_sub(1)},
                     "end":{"line":error.line.saturating_sub(1),"character":error.column}},
            "severity":1,"source":"vietlang","message":error.message
        }]),
    }
}

fn read_message<R: BufRead + Read>(reader: &mut R) -> io::Result<Option<Value>> {
    let mut length = None;
    loop {
        let mut header = String::new();
        if reader.read_line(&mut header)? == 0 {
            return Ok(None);
        }
        if header == "\r\n" || header == "\n" {
            break;
        }
        if let Some(value) = header.strip_prefix("Content-Length:") {
            length = value.trim().parse::<usize>().ok();
        }
    }
    let Some(length) = length else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "missing Content-Length",
        ));
    };
    let mut body = vec![0; length];
    reader.read_exact(&mut body)?;
    serde_json::from_slice(&body)
        .map(Some)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

fn write_message<W: Write>(writer: &mut W, value: &Value) -> io::Result<()> {
    let body = serde_json::to_vec(value).map_err(io::Error::other)?;
    write!(writer, "Content-Length: {}\r\n\r\n", body.len())?;
    writer.write_all(&body)?;
    writer.flush()
}
fn respond<W: Write>(writer: &mut W, id: Option<Value>, result: Value) -> io::Result<()> {
    write_message(
        writer,
        &json!({"jsonrpc":"2.0","id":id.unwrap_or(Value::Null),"result":result}),
    )
}
fn respond_error<W: Write>(
    writer: &mut W,
    id: Option<Value>,
    code: i64,
    message: &str,
) -> io::Result<()> {
    write_message(
        writer,
        &json!({"jsonrpc":"2.0","id":id.unwrap_or(Value::Null),"error":{"code":code,"message":message}}),
    )
}
fn notify<W: Write>(writer: &mut W, method: &str, params: Value) -> io::Result<()> {
    write_message(
        writer,
        &json!({"jsonrpc":"2.0","method":method,"params":params}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn diagnostics_report_semantic_errors() {
        assert_eq!(diagnostics("let value: Int = 1"), json!([]));
        assert_eq!(
            diagnostics("let value: Int = \"bad\"")
                .as_array()
                .unwrap()
                .len(),
            1
        );
    }
}
