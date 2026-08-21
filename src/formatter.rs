//! Official, deterministic VietLang formatter.

use crate::{error::VietResult, lexer::Lexer, parser::Parser};

pub fn format_source(source: &str) -> VietResult<String> {
    // Refuse to rewrite malformed input.  Comments are intentionally preserved
    // verbatim; formatting changes indentation and trailing whitespace only.
    Parser::new(Lexer::new(source).tokenize()?).parse()?;
    let mut indent = 0usize;
    let mut output = String::new();
    for raw_line in source.lines() {
        let trimmed = raw_line.trim();
        if trimmed.is_empty() {
            if !output.ends_with("\n\n") {
                output.push('\n');
            }
            continue;
        }
        let closes_first =
            trimmed.starts_with('}') || trimmed.starts_with("else") || trimmed.starts_with("catch");
        if closes_first {
            indent = indent.saturating_sub(1);
        }
        output.push_str(&"    ".repeat(indent));
        output.push_str(trimmed);
        output.push('\n');
        let (opens, closes) = brace_delta(trimmed);
        indent = indent.saturating_add(opens);
        let consumed_close = usize::from(closes_first);
        indent = indent.saturating_sub(closes.saturating_sub(consumed_close));
    }
    while output.ends_with("\n\n\n") {
        output.pop();
    }
    Parser::new(Lexer::new(&output).tokenize()?).parse()?;
    Ok(output)
}

fn brace_delta(line: &str) -> (usize, usize) {
    let mut opens = 0;
    let mut closes = 0;
    let mut quoted = false;
    let mut escaped = false;
    let chars: Vec<_> = line.chars().collect();
    let mut index = 0;
    while index < chars.len() {
        let ch = chars[index];
        if !quoted && ch == '/' && chars.get(index + 1) == Some(&'/') {
            break;
        }
        if ch == '"' && !escaped {
            quoted = !quoted;
        }
        if !quoted {
            if ch == '{' {
                opens += 1;
            }
            if ch == '}' {
                closes += 1;
            }
        }
        escaped = ch == '\\' && !escaped;
        if ch != '\\' {
            escaped = false;
        }
        index += 1;
    }
    (opens, closes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatting_is_comment_preserving_and_idempotent() {
        let source = "fn main(){\n// keep me\nlet text = \"{\"\nif true {\nprintln(text)\n}\n}\n";
        let once = format_source(source).unwrap();
        let twice = format_source(&once).unwrap();
        assert_eq!(once, twice);
        assert!(once.contains("    // keep me"));
        assert!(once.contains("        println(text)"));
    }
}
