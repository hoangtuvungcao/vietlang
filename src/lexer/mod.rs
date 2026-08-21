//! VietLang Lexer
//! Converts source code into a stream of tokens.

pub mod token;

use crate::error::{VietError, VietResult};
use token::{Token, TokenKind, Span};

pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
            tokens: Vec::new(),
        }
    }

    /// Tokenize the entire source code
    pub fn tokenize(&mut self) -> VietResult<Vec<Token>> {
        while !self.is_at_end() {
            self.skip_whitespace_and_comments();
            if self.is_at_end() {
                break;
            }

            let token = self.scan_token()?;
            self.tokens.push(token);
        }

        // Add EOF token
        self.tokens.push(Token::new(
            TokenKind::Eof,
            Span::new(self.line, self.column, self.pos, self.pos),
            String::new(),
        ));

        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> VietResult<Token> {
        let start = self.pos;
        let start_line = self.line;
        let start_col = self.column;
        let ch = self.advance();

        let kind = match ch {
            // Single-character tokens
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semicolon,
            '?' => TokenKind::Question,
            '&' => {
                if self.match_char('&') {
                    TokenKind::And
                } else {
                    TokenKind::Ampersand
                }
            }
            '|' => {
                if self.match_char('|') {
                    TokenKind::Or
                } else {
                    TokenKind::Pipe
                }
            }

            // Two-character and compound assignment tokens
            '/' => {
                if self.match_char('=') {
                    TokenKind::SlashAssign
                } else {
                    TokenKind::Slash
                }
            }
            '+' => {
                if self.match_char('=') {
                    TokenKind::PlusAssign
                } else {
                    TokenKind::Plus
                }
            }
            '-' => {
                if self.match_char('>') {
                    TokenKind::Arrow
                } else if self.match_char('=') {
                    TokenKind::MinusAssign
                } else {
                    TokenKind::Minus
                }
            }
            '*' => {
                if self.match_char('=') {
                    TokenKind::StarAssign
                } else {
                    TokenKind::Star
                }
            }
            '%' => {
                if self.match_char('=') {
                    TokenKind::PercentAssign
                } else {
                    TokenKind::Percent
                }
            }
            '=' => {
                if self.match_char('=') {
                    TokenKind::Eq
                } else if self.match_char('>') {
                    TokenKind::FatArrow
                } else {
                    TokenKind::Assign
                }
            }
            '!' => {
                if self.match_char('=') {
                    TokenKind::NotEq
                } else {
                    TokenKind::Not
                }
            }
            '<' => {
                if self.match_char('=') {
                    TokenKind::LtEq
                } else if self.match_char('<') {
                    TokenKind::ShiftLeft
                } else {
                    TokenKind::Lt
                }
            }
            '>' => {
                if self.match_char('=') {
                    TokenKind::GtEq
                } else if self.match_char('>') {
                    TokenKind::ShiftRight
                } else {
                    TokenKind::Gt
                }
            }
            ':' => {
                if self.match_char(':') {
                    TokenKind::ColonColon
                } else {
                    TokenKind::Colon
                }
            }
            '.' => {
                if self.match_char('.') {
                    TokenKind::DotDot
                } else {
                    TokenKind::Dot
                }
            }
            '^' => TokenKind::Caret,
            '~' => TokenKind::Tilde,

            // Newline
            '\n' => {
                self.line += 1;
                self.column = 1;
                TokenKind::Newline
            }

            // String literals
            '"' => self.scan_string()?,

            // Number literals
            c if c.is_ascii_digit() => self.scan_number(c)?,

            // Identifiers and keywords
            c if c.is_alphabetic() || c == '_' => self.scan_identifier(c),

            _ => {
                return Err(VietError::lexer_error(
                    format!("Unexpected character: '{}'", ch),
                    start_line,
                    start_col,
                ));
            }
        };

        let lexeme: String = self.source[start..self.pos].iter().collect();
        let span = Span::new(start_line, start_col, start, self.pos);
        Ok(Token::new(kind, span, lexeme))
    }

    fn scan_string(&mut self) -> VietResult<TokenKind> {
        let start_line = self.line;
        let start_col = self.column;
        let mut value = String::new();

        while !self.is_at_end() && self.peek() != '"' {
            let ch = self.advance();
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            }
            if ch == '\\' {
                // Escape sequences
                if self.is_at_end() {
                    return Err(VietError::lexer_error(
                        "Unterminated escape sequence".to_string(),
                        start_line,
                        start_col,
                    ));
                }
                let escaped = self.advance();
                match escaped {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    'e' => value.push('\x1b'),
                    'x' => {
                        let mut hex = String::new();
                        if !self.is_at_end() { hex.push(self.advance()); }
                        if !self.is_at_end() { hex.push(self.advance()); }
                        if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                            value.push(byte as char);
                        } else {
                            return Err(VietError::lexer_error(
                                format!("Invalid hex escape sequence: \\x{}", hex),
                                self.line,
                                self.column,
                            ));
                        }
                    }
                    '\\' => value.push('\\'),
                    '"' => value.push('"'),
                    '{' => value.push('{'),
                    '}' => value.push('}'),
                    _ => {
                        return Err(VietError::lexer_error(
                            format!("Invalid escape sequence: \\{}", escaped),
                            self.line,
                            self.column,
                        ));
                    }
                }
            } else {
                value.push(ch);
            }
        }

        if self.is_at_end() {
            return Err(VietError::lexer_error(
                "Unterminated string".to_string(),
                start_line,
                start_col,
            ));
        }

        self.advance(); // consume closing "
        Ok(TokenKind::StringLiteral(value))
    }

    fn scan_number(&mut self, first: char) -> VietResult<TokenKind> {
        let mut num_str = String::from(first);
        let mut is_float = false;

        while !self.is_at_end() && (self.peek().is_ascii_digit() || self.peek() == '_') {
            let ch = self.advance();
            if ch != '_' {
                num_str.push(ch);
            }
        }

        // Check for float
        if !self.is_at_end() && self.peek() == '.' {
            // Look ahead to make sure it's not a method call (e.g., 5.to_string())
            if self.pos + 1 < self.source.len() && self.source[self.pos + 1].is_ascii_digit() {
                is_float = true;
                num_str.push(self.advance()); // consume '.'
                while !self.is_at_end() && (self.peek().is_ascii_digit() || self.peek() == '_') {
                    let ch = self.advance();
                    if ch != '_' {
                        num_str.push(ch);
                    }
                }
            }
        }

        if is_float {
            let value: f64 = num_str.parse().map_err(|_| {
                VietError::lexer_error(
                    format!("Invalid float literal: {}", num_str),
                    self.line,
                    self.column,
                )
            })?;
            Ok(TokenKind::FloatLiteral(value))
        } else {
            let value: i64 = num_str.parse().map_err(|_| {
                VietError::lexer_error(
                    format!("Invalid integer literal: {}", num_str),
                    self.line,
                    self.column,
                )
            })?;
            Ok(TokenKind::IntLiteral(value))
        }
    }

    fn scan_identifier(&mut self, first: char) -> TokenKind {
        let mut ident = String::from(first);

        while !self.is_at_end() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            ident.push(self.advance());
        }

        // Check for keywords
        match TokenKind::is_keyword(&ident) {
            Some(TokenKind::True) => TokenKind::BoolLiteral(true),
            Some(TokenKind::False) => TokenKind::BoolLiteral(false),
            Some(keyword) => keyword,
            None => TokenKind::Identifier(ident),
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '/' if self.peek_next() == Some('/') => {
                    // Line comment
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                }
                '/' if self.peek_next() == Some('*') => {
                    // Block comment
                    self.advance(); // /
                    self.advance(); // *
                    let mut depth = 1;
                    while !self.is_at_end() && depth > 0 {
                        if self.peek() == '/' && self.peek_next() == Some('*') {
                            depth += 1;
                            self.advance();
                            self.advance();
                        } else if self.peek() == '*' && self.peek_next() == Some('/') {
                            depth -= 1;
                            self.advance();
                            self.advance();
                        } else {
                            if self.peek() == '\n' {
                                self.line += 1;
                                self.column = 0;
                            }
                            self.advance();
                        }
                    }
                }
                _ => break,
            }
        }
    }

    // === Helper methods ===

    fn advance(&mut self) -> char {
        let ch = self.source[self.pos];
        self.pos += 1;
        self.column += 1;
        ch
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.pos]
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.pos + 1 < self.source.len() {
            Some(self.source[self.pos + 1])
        } else {
            None
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.pos] != expected {
            false
        } else {
            self.pos += 1;
            self.column += 1;
            true
        }
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.source.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(source: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(source);
        lexer.tokenize().unwrap()
    }

    fn token_kinds(source: &str) -> Vec<TokenKind> {
        lex(source).into_iter().map(|t| t.kind).collect()
    }

    #[test]
    fn test_numbers() {
        assert_eq!(
            token_kinds("42 3.14"),
            vec![
                TokenKind::IntLiteral(42),
                TokenKind::FloatLiteral(3.14),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_strings() {
        assert_eq!(
            token_kinds(r#""hello world""#),
            vec![
                TokenKind::StringLiteral("hello world".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_string_escape() {
        assert_eq!(
            token_kinds(r#""hello\nworld""#),
            vec![
                TokenKind::StringLiteral("hello\nworld".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_keywords() {
        assert_eq!(
            token_kinds("let fn if else match struct"),
            vec![
                TokenKind::Let,
                TokenKind::Fn,
                TokenKind::If,
                TokenKind::Else,
                TokenKind::Match,
                TokenKind::Struct,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_identifiers() {
        assert_eq!(
            token_kinds("foo bar_baz _test"),
            vec![
                TokenKind::Identifier("foo".to_string()),
                TokenKind::Identifier("bar_baz".to_string()),
                TokenKind::Identifier("_test".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_operators() {
        assert_eq!(
            token_kinds("+ - * / == != <= >= -> =>"),
            vec![
                TokenKind::Plus,
                TokenKind::Minus,
                TokenKind::Star,
                TokenKind::Slash,
                TokenKind::Eq,
                TokenKind::NotEq,
                TokenKind::LtEq,
                TokenKind::GtEq,
                TokenKind::Arrow,
                TokenKind::FatArrow,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_let_statement() {
        assert_eq!(
            token_kinds("let x = 42"),
            vec![
                TokenKind::Let,
                TokenKind::Identifier("x".to_string()),
                TokenKind::Assign,
                TokenKind::IntLiteral(42),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_function() {
        let tokens = token_kinds("fn add(a: Int, b: Int) -> Int { return a + b }");
        assert_eq!(tokens[0], TokenKind::Fn);
        assert_eq!(tokens[1], TokenKind::Identifier("add".to_string()));
        assert_eq!(tokens[2], TokenKind::LParen);
    }

    #[test]
    fn test_comments() {
        assert_eq!(
            token_kinds("let x = 5 // this is a comment\nlet y = 10"),
            vec![
                TokenKind::Let,
                TokenKind::Identifier("x".to_string()),
                TokenKind::Assign,
                TokenKind::IntLiteral(5),
                TokenKind::Newline,
                TokenKind::Let,
                TokenKind::Identifier("y".to_string()),
                TokenKind::Assign,
                TokenKind::IntLiteral(10),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_block_comments() {
        assert_eq!(
            token_kinds("let /* comment */ x = 5"),
            vec![
                TokenKind::Let,
                TokenKind::Identifier("x".to_string()),
                TokenKind::Assign,
                TokenKind::IntLiteral(5),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_booleans() {
        assert_eq!(
            token_kinds("true false"),
            vec![
                TokenKind::BoolLiteral(true),
                TokenKind::BoolLiteral(false),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_number_underscore() {
        assert_eq!(
            token_kinds("1_000_000"),
            vec![
                TokenKind::IntLiteral(1000000),
                TokenKind::Eof,
            ]
        );
    }
}
