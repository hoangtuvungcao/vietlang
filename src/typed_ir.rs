//! Typed, module-level intermediate representation used by checks and tooling.
//!
//! VietLang still executes its AST, but every project entrypoint is now lowered
//! into this stable representation after semantic analysis.  The IR deliberately
//! contains no interpreter values, so formatters, LSP clients and future bytecode
//! backends can consume it without executing user code.

use std::path::PathBuf;

use crate::{
    parser::ast::{Program, Statement},
    semantic::SemanticAnalyzer,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedModule {
    pub path: PathBuf,
    pub imports: Vec<String>,
    pub declarations: Vec<TypedDeclaration>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedDeclaration {
    pub name: String,
    pub kind: DeclarationKind,
    pub ty: String,
    pub public: bool,
    pub line: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclarationKind {
    Variable,
    Function,
    Struct,
    Enum,
    Implementation,
}

impl TypedModule {
    pub fn lower(path: PathBuf, program: &Program, analyzer: &SemanticAnalyzer) -> Self {
        let imports = program
            .statements
            .iter()
            .filter_map(|statement| match statement {
                Statement::Import { path, .. } => Some(path.join(".")),
                _ => None,
            })
            .collect();
        let declarations = program
            .statements
            .iter()
            .filter_map(|statement| {
                let (name, kind, public, line) = match statement {
                    Statement::Let { name, span, .. } => {
                        (name.clone(), DeclarationKind::Variable, false, span.line)
                    }
                    Statement::Function {
                        name, is_pub, span, ..
                    } => (name.clone(), DeclarationKind::Function, *is_pub, span.line),
                    Statement::Struct {
                        name, is_pub, span, ..
                    } => (name.clone(), DeclarationKind::Struct, *is_pub, span.line),
                    Statement::Enum {
                        name, is_pub, span, ..
                    } => (name.clone(), DeclarationKind::Enum, *is_pub, span.line),
                    Statement::Impl {
                        type_name, span, ..
                    } => (
                        type_name.clone(),
                        DeclarationKind::Implementation,
                        false,
                        span.line,
                    ),
                    _ => return None,
                };
                Some(TypedDeclaration {
                    ty: analyzer.symbol_type(&name).unwrap_or_else(|| name.clone()),
                    name,
                    kind,
                    public,
                    line,
                })
            })
            .collect();
        Self {
            path,
            imports,
            declarations,
        }
    }
}
