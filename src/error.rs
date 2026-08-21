/// VietLang Error Types
/// Unified error handling for the compiler/interpreter.

use std::fmt;
use crate::interpreter::value::Value;

#[derive(Debug, Clone)]
pub enum ErrorKind {
    LexerError,
    ParseError,
    TypeError,
    RuntimeError,
    NameError,
    Return(Value),
    Break,
    Continue,
}

#[derive(Debug, Clone)]
pub struct VietError {
    pub kind: ErrorKind,
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl VietError {
    pub fn new(kind: ErrorKind, message: String, line: usize, column: usize) -> Self {
        VietError { kind, message, line, column }
    }

    pub fn lexer_error(message: String, line: usize, column: usize) -> Self {
        VietError::new(ErrorKind::LexerError, message, line, column)
    }

    pub fn parse_error(message: String, line: usize, column: usize) -> Self {
        VietError::new(ErrorKind::ParseError, message, line, column)
    }

    pub fn type_error(message: String, line: usize, column: usize) -> Self {
        VietError::new(ErrorKind::TypeError, message, line, column)
    }

    pub fn runtime_error(message: String, line: usize, column: usize) -> Self {
        VietError::new(ErrorKind::RuntimeError, message, line, column)
    }

    pub fn name_error(message: String, line: usize, column: usize) -> Self {
        VietError::new(ErrorKind::NameError, message, line, column)
    }

    pub fn return_signal(val: Value) -> Self {
        VietError {
            kind: ErrorKind::Return(val),
            message: String::new(),
            line: 0,
            column: 0,
        }
    }

    pub fn break_signal() -> Self {
        VietError {
            kind: ErrorKind::Break,
            message: String::new(),
            line: 0,
            column: 0,
        }
    }

    pub fn continue_signal() -> Self {
        VietError {
            kind: ErrorKind::Continue,
            message: String::new(),
            line: 0,
            column: 0,
        }
    }
}

impl fmt::Display for VietError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind_str = match &self.kind {
            ErrorKind::LexerError => "LexerError",
            ErrorKind::ParseError => "ParseError",
            ErrorKind::TypeError => "TypeError",
            ErrorKind::RuntimeError => "RuntimeError",
            ErrorKind::NameError => "NameError",
            ErrorKind::Return(_) => "Return",
            ErrorKind::Break => "Break",
            ErrorKind::Continue => "Continue",
        };
        write!(
            f,
            "\x1b[31m{}\x1b[0m at line {}:{}: {}",
            kind_str, self.line, self.column, self.message
        )
    }
}

impl std::error::Error for VietError {}

pub type VietResult<T> = Result<T, VietError>;
