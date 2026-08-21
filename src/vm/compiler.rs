//! VietLang Bytecode Compiler
//! Compiles VietLang AST into a Bytecode Chunk.

use super::opcode::OpCode;
use crate::error::{VietError, VietResult};
use crate::interpreter::value::Value;
use crate::parser::ast::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk::default()
    }

    pub fn write_opcode(&mut self, op: OpCode, line: usize) {
        self.code.push(op as u8);
        self.lines.push(line);
    }

    pub fn write_byte(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}

pub struct Compiler {
    pub chunk: Chunk,
    scopes: Vec<HashMap<String, usize>>,
    next_slot: usize,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            scopes: vec![HashMap::new()],
            next_slot: 0,
        }
    }

    pub fn compile(&mut self, program: &Program) -> VietResult<Chunk> {
        for stmt in &program.statements {
            self.compile_statement(stmt)?;
        }
        self.chunk.write_opcode(OpCode::OpHalt, 0);
        Ok(self.chunk.clone())
    }

    fn compile_statement(&mut self, stmt: &Statement) -> VietResult<()> {
        match stmt {
            Statement::Let {
                name, value, span, ..
            } => {
                self.compile_expression(value)?;
                let idx = self.declare(name);
                self.chunk.write_opcode(OpCode::OpSetGlobal, span.line);
                self.chunk.write_byte((idx & 0xFF) as u8, span.line);
                self.chunk.write_byte(((idx >> 8) & 0xFF) as u8, span.line);
                self.chunk.write_opcode(OpCode::OpRecordResult, span.line);
            }
            Statement::Expression { expr, .. } => {
                self.compile_expression(expr)?;
                self.chunk
                    .write_opcode(OpCode::OpRecordResult, expr.span().line);
            }
            Statement::Return { value, span } => {
                if let Some(expr) = value {
                    self.compile_expression(expr)?;
                } else {
                    let const_idx = self.chunk.add_constant(Value::None);
                    self.chunk.write_opcode(OpCode::OpConstant, span.line);
                    self.chunk.write_byte((const_idx & 0xFF) as u8, span.line);
                    self.chunk
                        .write_byte(((const_idx >> 8) & 0xFF) as u8, span.line);
                }
                self.chunk.write_opcode(OpCode::OpReturn, span.line);
            }
            Statement::If {
                condition,
                then_body,
                else_body,
                span,
            } => {
                self.compile_expression(condition)?;
                self.chunk.write_opcode(OpCode::OpJumpIfFalse, span.line);
                let jump_false_pos = self.chunk.code.len();
                self.chunk.write_byte(0, span.line);
                self.chunk.write_byte(0, span.line);

                if then_body.is_empty() {
                    self.emit_none_result(span.line);
                } else {
                    self.compile_scoped_statements(then_body)?;
                }

                self.chunk.write_opcode(OpCode::OpJump, span.line);
                let jump_end_pos = self.chunk.code.len();
                self.chunk.write_byte(0, span.line);
                self.chunk.write_byte(0, span.line);

                // Patch false jump
                let else_target = self.chunk.code.len();
                self.chunk.code[jump_false_pos] = (else_target & 0xFF) as u8;
                self.chunk.code[jump_false_pos + 1] = ((else_target >> 8) & 0xFF) as u8;

                if let Some(else_stmts) = else_body {
                    if else_stmts.is_empty() {
                        self.emit_none_result(span.line);
                    } else {
                        self.compile_scoped_statements(else_stmts)?;
                    }
                } else {
                    self.emit_none_result(span.line);
                }

                // Patch end jump
                let end_target = self.chunk.code.len();
                self.chunk.code[jump_end_pos] = (end_target & 0xFF) as u8;
                self.chunk.code[jump_end_pos + 1] = ((end_target >> 8) & 0xFF) as u8;
            }
            Statement::Assignment {
                target,
                value,
                span,
            } => {
                if let Expression::Identifier { name, .. } = target {
                    self.compile_expression(value)?;
                    let idx = self.resolve(name).ok_or_else(|| {
                        VietError::name_error(
                            format!("Undefined variable '{}'", name),
                            span.line,
                            span.column,
                        )
                    })?;
                    self.chunk.write_opcode(OpCode::OpSetGlobal, span.line);
                    self.chunk.write_byte((idx & 0xFF) as u8, span.line);
                    self.chunk.write_byte(((idx >> 8) & 0xFF) as u8, span.line);
                    self.chunk.write_opcode(OpCode::OpRecordResult, span.line);
                } else {
                    return Err(VietError::runtime_error(
                        "VM does not support assignment targets other than variables".into(),
                        span.line,
                        span.column,
                    ));
                }
            }
            Statement::While {
                condition,
                body,
                span,
            } => {
                let loop_start = self.chunk.code.len();
                self.compile_expression(condition)?;
                self.chunk.write_opcode(OpCode::OpJumpIfFalse, span.line);
                let jump_false_pos = self.chunk.code.len();
                self.chunk.write_byte(0, span.line);
                self.chunk.write_byte(0, span.line);

                self.compile_scoped_statements(body)?;

                self.chunk.write_opcode(OpCode::OpJump, span.line);
                self.chunk.write_byte((loop_start & 0xFF) as u8, span.line);
                self.chunk
                    .write_byte(((loop_start >> 8) & 0xFF) as u8, span.line);

                let exit_target = self.chunk.code.len();
                self.chunk.code[jump_false_pos] = (exit_target & 0xFF) as u8;
                self.chunk.code[jump_false_pos + 1] = ((exit_target >> 8) & 0xFF) as u8;
                self.emit_none_result(span.line);
            }
            _ => {
                let (kind, span) = statement_kind_and_span(stmt);
                return Err(VietError::runtime_error(
                    format!("VM compiler does not support {} statements", kind),
                    span.line,
                    span.column,
                ));
            }
        }
        Ok(())
    }

    fn compile_expression(&mut self, expr: &Expression) -> VietResult<()> {
        match expr {
            Expression::IntLiteral { value, span } => {
                let idx = self.chunk.add_constant(Value::Int(*value));
                self.chunk.write_opcode(OpCode::OpConstant, span.line);
                self.chunk.write_byte((idx & 0xFF) as u8, span.line);
                self.chunk.write_byte(((idx >> 8) & 0xFF) as u8, span.line);
            }
            Expression::FloatLiteral { value, span } => {
                let idx = self.chunk.add_constant(Value::Float(*value));
                self.chunk.write_opcode(OpCode::OpConstant, span.line);
                self.chunk.write_byte((idx & 0xFF) as u8, span.line);
                self.chunk.write_byte(((idx >> 8) & 0xFF) as u8, span.line);
            }
            Expression::StringLiteral { value, span } => {
                let idx = self.chunk.add_constant(Value::String(value.clone()));
                self.chunk.write_opcode(OpCode::OpConstant, span.line);
                self.chunk.write_byte((idx & 0xFF) as u8, span.line);
                self.chunk.write_byte(((idx >> 8) & 0xFF) as u8, span.line);
            }
            Expression::BoolLiteral { value, span } => {
                let idx = self.chunk.add_constant(Value::Bool(*value));
                self.chunk.write_opcode(OpCode::OpConstant, span.line);
                self.chunk.write_byte((idx & 0xFF) as u8, span.line);
                self.chunk.write_byte(((idx >> 8) & 0xFF) as u8, span.line);
            }
            Expression::NoneLiteral { span } => {
                let idx = self.chunk.add_constant(Value::None);
                self.chunk.write_opcode(OpCode::OpConstant, span.line);
                self.chunk.write_byte((idx & 0xFF) as u8, span.line);
                self.chunk.write_byte(((idx >> 8) & 0xFF) as u8, span.line);
            }
            Expression::Identifier { name, span } => {
                if let Some(idx) = self.resolve(name) {
                    self.chunk.write_opcode(OpCode::OpGetGlobal, span.line);
                    self.chunk.write_byte((idx & 0xFF) as u8, span.line);
                    self.chunk.write_byte(((idx >> 8) & 0xFF) as u8, span.line);
                } else {
                    return Err(VietError::name_error(
                        format!("Undefined variable '{}'", name),
                        span.line,
                        span.column,
                    ));
                }
            }
            Expression::BinaryOp {
                left,
                op,
                right,
                span,
            } => {
                if matches!(op, BinaryOperator::And | BinaryOperator::Or) {
                    return self.compile_logical(left, op, right, span);
                }
                self.compile_expression(left)?;
                self.compile_expression(right)?;
                match op {
                    BinaryOperator::Add => self.chunk.write_opcode(OpCode::OpAdd, span.line),
                    BinaryOperator::Sub => self.chunk.write_opcode(OpCode::OpSub, span.line),
                    BinaryOperator::Mul => self.chunk.write_opcode(OpCode::OpMul, span.line),
                    BinaryOperator::Div => self.chunk.write_opcode(OpCode::OpDiv, span.line),
                    BinaryOperator::Mod => self.chunk.write_opcode(OpCode::OpMod, span.line),
                    BinaryOperator::Eq => self.chunk.write_opcode(OpCode::OpEqual, span.line),
                    BinaryOperator::NotEq => self.chunk.write_opcode(OpCode::OpNotEqual, span.line),
                    BinaryOperator::Gt => self.chunk.write_opcode(OpCode::OpGreaterThan, span.line),
                    BinaryOperator::Lt => self.chunk.write_opcode(OpCode::OpLessThan, span.line),
                    BinaryOperator::GtEq => {
                        self.chunk.write_opcode(OpCode::OpGreaterEqual, span.line)
                    }
                    BinaryOperator::LtEq => self.chunk.write_opcode(OpCode::OpLessEqual, span.line),
                    BinaryOperator::And | BinaryOperator::Or => unreachable!(),
                }
            }
            Expression::UnaryOp { op, operand, span } => {
                self.compile_expression(operand)?;
                self.chunk.write_opcode(
                    match op {
                        UnaryOperator::Neg => OpCode::OpNeg,
                        UnaryOperator::Not => OpCode::OpNot,
                    },
                    span.line,
                );
            }
            _ => {
                let span = expr.span();
                return Err(VietError::runtime_error(
                    format!(
                        "VM compiler does not support {} expressions",
                        expression_kind(expr)
                    ),
                    span.line,
                    span.column,
                ));
            }
        }
        Ok(())
    }

    fn emit_none_result(&mut self, line: usize) {
        let idx = self.chunk.add_constant(Value::None);
        self.chunk.write_opcode(OpCode::OpConstant, line);
        self.chunk.write_byte((idx & 0xFF) as u8, line);
        self.chunk.write_byte(((idx >> 8) & 0xFF) as u8, line);
        self.chunk.write_opcode(OpCode::OpRecordResult, line);
    }

    fn declare(&mut self, name: &str) -> usize {
        let slot = self.next_slot;
        self.next_slot += 1;
        self.scopes
            .last_mut()
            .expect("compiler always has a scope")
            .insert(name.to_string(), slot);
        slot
    }

    fn resolve(&self, name: &str) -> Option<usize> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).copied())
    }

    fn compile_scoped_statements(&mut self, statements: &[Statement]) -> VietResult<()> {
        self.scopes.push(HashMap::new());
        let result = statements
            .iter()
            .try_for_each(|statement| self.compile_statement(statement));
        self.scopes.pop();
        result
    }

    fn compile_logical(
        &mut self,
        left: &Expression,
        op: &BinaryOperator,
        right: &Expression,
        span: &crate::lexer::token::Span,
    ) -> VietResult<()> {
        self.compile_expression(left)?;
        self.chunk.write_opcode(OpCode::OpJumpIfFalse, span.line);
        let conditional_jump = self.chunk.code.len();
        self.chunk.write_byte(0, span.line);
        self.chunk.write_byte(0, span.line);

        match op {
            BinaryOperator::And => {
                self.compile_expression(right)?;
                self.chunk.write_opcode(OpCode::OpNot, span.line);
                self.chunk.write_opcode(OpCode::OpNot, span.line);
                self.chunk.write_opcode(OpCode::OpJump, span.line);
                let end_jump = self.chunk.code.len();
                self.chunk.write_byte(0, span.line);
                self.chunk.write_byte(0, span.line);
                let false_target = self.chunk.code.len();
                self.patch_jump(conditional_jump, false_target);
                self.emit_bool(false, span.line);
                let end_target = self.chunk.code.len();
                self.patch_jump(end_jump, end_target);
            }
            BinaryOperator::Or => {
                self.emit_bool(true, span.line);
                self.chunk.write_opcode(OpCode::OpJump, span.line);
                let end_jump = self.chunk.code.len();
                self.chunk.write_byte(0, span.line);
                self.chunk.write_byte(0, span.line);
                let right_target = self.chunk.code.len();
                self.patch_jump(conditional_jump, right_target);
                self.compile_expression(right)?;
                self.chunk.write_opcode(OpCode::OpNot, span.line);
                self.chunk.write_opcode(OpCode::OpNot, span.line);
                let end_target = self.chunk.code.len();
                self.patch_jump(end_jump, end_target);
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    fn emit_bool(&mut self, value: bool, line: usize) {
        let idx = self.chunk.add_constant(Value::Bool(value));
        self.chunk.write_opcode(OpCode::OpConstant, line);
        self.chunk.write_byte((idx & 0xFF) as u8, line);
        self.chunk.write_byte(((idx >> 8) & 0xFF) as u8, line);
    }

    fn patch_jump(&mut self, position: usize, target: usize) {
        self.chunk.code[position] = (target & 0xFF) as u8;
        self.chunk.code[position + 1] = ((target >> 8) & 0xFF) as u8;
    }
}

fn statement_kind_and_span(stmt: &Statement) -> (&'static str, &crate::lexer::token::Span) {
    match stmt {
        Statement::Let { span, .. } => ("let", span),
        Statement::Assignment { span, .. } => ("assignment", span),
        Statement::Expression { span, .. } => ("expression", span),
        Statement::Function { span, .. } => ("function", span),
        Statement::Return { span, .. } => ("return", span),
        Statement::If { span, .. } => ("if", span),
        Statement::While { span, .. } => ("while", span),
        Statement::For { span, .. } => ("for", span),
        Statement::Break { span } => ("break", span),
        Statement::Continue { span } => ("continue", span),
        Statement::Struct { span, .. } => ("struct", span),
        Statement::Enum { span, .. } => ("enum", span),
        Statement::Impl { span, .. } => ("impl", span),
        Statement::Import { span, .. } => ("import", span),
        Statement::TryCatch { span, .. } => ("try/catch", span),
    }
}

fn expression_kind(expr: &Expression) -> &'static str {
    match expr {
        Expression::IntLiteral { .. } => "integer literal",
        Expression::FloatLiteral { .. } => "float literal",
        Expression::StringLiteral { .. } => "string literal",
        Expression::BoolLiteral { .. } => "boolean literal",
        Expression::NoneLiteral { .. } => "none literal",
        Expression::Identifier { .. } => "identifier",
        Expression::BinaryOp { .. } => "binary",
        Expression::UnaryOp { .. } => "unary",
        Expression::Call { .. } => "call",
        Expression::MethodCall { .. } => "method call",
        Expression::FieldAccess { .. } => "field access",
        Expression::Index { .. } => "index",
        Expression::ArrayLiteral { .. } => "array literal",
        Expression::StructLiteral { .. } => "struct literal",
        Expression::Match { .. } => "match",
        Expression::Block { .. } => "block",
        Expression::Lambda { .. } => "lambda",
        Expression::Range { .. } => "range",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::Lexer, parser::Parser};

    fn compile_source(source: &str) -> VietResult<Chunk> {
        let tokens = Lexer::new(source).tokenize()?;
        let program = Parser::new(tokens).parse()?;
        Compiler::new().compile(&program)
    }

    #[test]
    fn unsupported_statement_is_a_compile_error() {
        let error = compile_source("fn greet() { return 1 }")
            .expect_err("functions are not implemented by the VM compiler");
        assert!(error
            .message
            .contains("does not support function statements"));
        assert_eq!(error.line, 1);
    }

    #[test]
    fn unsupported_expression_is_a_compile_error() {
        let error = compile_source("let values = [1, 2, 3]")
            .expect_err("arrays are not implemented by the VM compiler");
        assert!(error
            .message
            .contains("does not support array literal expressions"));
        assert_eq!(error.line, 1);
    }

    #[test]
    fn supported_subset_still_compiles() {
        let chunk =
            compile_source("let mut total = 1\ntotal = total + 2\nwhile total < 5 { total += 1 }")
                .expect("supported VM subset should compile");
        assert!(!chunk.code.is_empty());
    }
}
