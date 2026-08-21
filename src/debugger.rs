//! Top-level source debugger with breakpoints and deterministic stepping.

use std::{
    collections::HashSet,
    io::{self, Write},
};

use crate::{
    error::VietResult,
    interpreter::Interpreter,
    parser::ast::{Program, Statement},
};

pub fn run(program: &Program, breakpoints: &HashSet<usize>, interactive: bool) -> VietResult<()> {
    let mut interpreter = Interpreter::new();
    let mut stepping = interactive;
    for statement in &program.statements {
        let line = statement_line(statement);
        if stepping || breakpoints.contains(&line) {
            eprintln!(
                "[debug] paused at line {}: {}",
                line,
                statement_kind(statement)
            );
            if interactive {
                loop {
                    eprint!("debug> ");
                    let _ = io::stderr().flush();
                    let mut command = String::new();
                    if io::stdin().read_line(&mut command).is_err() {
                        return Ok(());
                    }
                    match command.trim() {
                        "s" | "step" | "" => {
                            stepping = true;
                            break;
                        }
                        "c" | "continue" => {
                            stepping = false;
                            break;
                        }
                        "q" | "quit" => return Ok(()),
                        "h" | "help" => eprintln!("step|s, continue|c, quit|q"),
                        _ => eprintln!("unknown debugger command; use 'help'"),
                    }
                }
            }
        }
        interpreter.execute(&Program {
            statements: vec![statement.clone()],
        })?;
    }
    Ok(())
}

fn statement_line(statement: &Statement) -> usize {
    match statement {
        Statement::Let { span, .. }
        | Statement::Assignment { span, .. }
        | Statement::Expression { span, .. }
        | Statement::Function { span, .. }
        | Statement::Return { span, .. }
        | Statement::If { span, .. }
        | Statement::While { span, .. }
        | Statement::For { span, .. }
        | Statement::Break { span }
        | Statement::Continue { span }
        | Statement::Struct { span, .. }
        | Statement::Enum { span, .. }
        | Statement::Impl { span, .. }
        | Statement::Import { span, .. }
        | Statement::TryCatch { span, .. } => span.line,
    }
}

fn statement_kind(statement: &Statement) -> &'static str {
    match statement {
        Statement::Let { .. } => "let",
        Statement::Assignment { .. } => "assignment",
        Statement::Expression { .. } => "expression",
        Statement::Function { .. } => "function",
        Statement::Return { .. } => "return",
        Statement::If { .. } => "if",
        Statement::While { .. } => "while",
        Statement::For { .. } => "for",
        Statement::Break { .. } => "break",
        Statement::Continue { .. } => "continue",
        Statement::Struct { .. } => "struct",
        Statement::Enum { .. } => "enum",
        Statement::Impl { .. } => "impl",
        Statement::Import { .. } => "import",
        Statement::TryCatch { .. } => "try/catch",
    }
}
