/// VietLang Token Types
/// Defines all token types recognized by the lexer.

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // === Literals ===
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),

    // === Identifiers & Keywords ===
    Identifier(String),

    // Keywords
    Let,
    Mut,
    Fn,
    Return,
    If,
    Else,
    Match,
    For,
    While,
    In,
    Break,
    Continue,
    Struct,
    Enum,
    Impl,
    Trait,
    Pub,
    Import,
    Type,
    Spawn,
    Async,
    Await,
    Select,
    Try,
    True,
    False,
    None,
    Self_,

    // === Operators ===
    Plus,        // +
    Minus,       // -
    Star,        // *
    Slash,       // /
    Percent,     // %
    Assign,      // =
    Eq,          // ==
    NotEq,       // !=
    Lt,          // <
    Gt,          // >
    LtEq,       // <=
    GtEq,       // >=
    And,         // &&
    Or,          // ||
    Not,         // !
    Arrow,       // ->
    FatArrow,    // =>
    Question,    // ?
    Dot,         // .
    DotDot,      // ..
    Colon,       // :
    ColonColon,  // ::
    Semicolon,   // ;  (optional in most cases)
    Comma,       // ,
    Pipe,        // |
    Ampersand,   // &

    // === Delimiters ===
    LParen,      // (
    RParen,      // )
    LBrace,      // {
    RBrace,      // }
    LBracket,    // [
    RBracket,    // ]

    // === Special ===
    Newline,
    Eof,
}

impl TokenKind {
    pub fn is_keyword(ident: &str) -> Option<TokenKind> {
        match ident {
            "let" => Some(TokenKind::Let),
            "mut" => Some(TokenKind::Mut),
            "fn" => Some(TokenKind::Fn),
            "return" => Some(TokenKind::Return),
            "if" => Some(TokenKind::If),
            "else" => Some(TokenKind::Else),
            "match" => Some(TokenKind::Match),
            "for" => Some(TokenKind::For),
            "while" => Some(TokenKind::While),
            "in" => Some(TokenKind::In),
            "break" => Some(TokenKind::Break),
            "continue" => Some(TokenKind::Continue),
            "struct" => Some(TokenKind::Struct),
            "enum" => Some(TokenKind::Enum),
            "impl" => Some(TokenKind::Impl),
            "trait" => Some(TokenKind::Trait),
            "pub" => Some(TokenKind::Pub),
            "import" => Some(TokenKind::Import),
            "type" => Some(TokenKind::Type),
            // spawn, async, await, select, try — reserved for future use
            // Currently treated as builtin functions (identifiers)
            "true" => Some(TokenKind::True),
            "false" => Some(TokenKind::False),
            "none" => Some(TokenKind::None),
            "self" => Some(TokenKind::Self_),
            _ => None,
        }
    }
}

/// Source location for error reporting
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(line: usize, column: usize, start: usize, end: usize) -> Self {
        Span { line, column, start, end }
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// A token with its kind and source location
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub lexeme: String,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span, lexeme: String) -> Self {
        Token { kind, span, lexeme }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} '{}' at {}", self.kind, self.lexeme, self.span)
    }
}
