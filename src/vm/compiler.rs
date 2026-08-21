/// VietLang Bytecode Compiler
/// Compiles VietLang AST into a Bytecode Chunk.

use std::collections::HashMap;
use crate::error::{VietError, VietResult};
use crate::interpreter::value::Value;
use crate::parser::ast::*;
use super::opcode::OpCode;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
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
    globals: HashMap<String, usize>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            chunk: Chunk::new(),
            globals: HashMap::new(),
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
            Statement::Let { name, value, span, .. } => {
                self.compile_expression(value)?;
                let next_idx = self.globals.len();
                let idx = *self.globals.entry(name.clone()).or_insert(next_idx);
                self.chunk.write_opcode(OpCode::OpSetGlobal, span.line);
                self.chunk.write_byte((idx & 0xFF) as u8, span.line);
                self.chunk.write_byte(((idx >> 8) & 0xFF) as u8, span.line);
            }
            Statement::Expression { expr, .. } => {
                self.compile_expression(expr)?;
                self.chunk.write_opcode(OpCode::OpPop, 0);
            }
            Statement::Return { value, span } => {
                if let Some(expr) = value {
                    self.compile_expression(expr)?;
                } else {
                    let const_idx = self.chunk.add_constant(Value::None);
                    self.chunk.write_opcode(OpCode::OpConstant, span.line);
                    self.chunk.write_byte((const_idx & 0xFF) as u8, span.line);
                    self.chunk.write_byte(((const_idx >> 8) & 0xFF) as u8, span.line);
                }
                self.chunk.write_opcode(OpCode::OpReturn, span.line);
            }
            Statement::If { condition, then_body, else_body, span } => {
                self.compile_expression(condition)?;
                self.chunk.write_opcode(OpCode::OpJumpIfFalse, span.line);
                let jump_false_pos = self.chunk.code.len();
                self.chunk.write_byte(0, span.line);
                self.chunk.write_byte(0, span.line);

                for s in then_body {
                    self.compile_statement(s)?;
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
                    for s in else_stmts {
                        self.compile_statement(s)?;
                    }
                }

                // Patch end jump
                let end_target = self.chunk.code.len();
                self.chunk.code[jump_end_pos] = (end_target & 0xFF) as u8;
                self.chunk.code[jump_end_pos + 1] = ((end_target >> 8) & 0xFF) as u8;
            }
            Statement::Assignment { target, value, span } => {
                self.compile_expression(value)?;
                if let Expression::Identifier { name, .. } = target {
                    let next_idx = self.globals.len();
                    let idx = *self.globals.entry(name.clone()).or_insert(next_idx);
                    self.chunk.write_opcode(OpCode::OpSetGlobal, span.line);
                    self.chunk.write_byte((idx & 0xFF) as u8, span.line);
                    self.chunk.write_byte(((idx >> 8) & 0xFF) as u8, span.line);
                }
            }
            Statement::While { condition, body, span } => {
                let loop_start = self.chunk.code.len();
                self.compile_expression(condition)?;
                self.chunk.write_opcode(OpCode::OpJumpIfFalse, span.line);
                let jump_false_pos = self.chunk.code.len();
                self.chunk.write_byte(0, span.line);
                self.chunk.write_byte(0, span.line);

                for s in body {
                    self.compile_statement(s)?;
                }

                self.chunk.write_opcode(OpCode::OpJump, span.line);
                self.chunk.write_byte((loop_start & 0xFF) as u8, span.line);
                self.chunk.write_byte(((loop_start >> 8) & 0xFF) as u8, span.line);

                let exit_target = self.chunk.code.len();
                self.chunk.code[jump_false_pos] = (exit_target & 0xFF) as u8;
                self.chunk.code[jump_false_pos + 1] = ((exit_target >> 8) & 0xFF) as u8;
            }
            _ => {
                // Fallback for other statements in VM
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
            Expression::Identifier { name, span } => {
                if let Some(&idx) = self.globals.get(name) {
                    self.chunk.write_opcode(OpCode::OpGetGlobal, span.line);
                    self.chunk.write_byte((idx & 0xFF) as u8, span.line);
                    self.chunk.write_byte(((idx >> 8) & 0xFF) as u8, span.line);
                } else {
                    return Err(VietError::name_error(format!("Undefined variable '{}'", name), span.line, span.column));
                }
            }
            Expression::BinaryOp { left, op, right, span } => {
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
                    BinaryOperator::GtEq => self.chunk.write_opcode(OpCode::OpGreaterEqual, span.line),
                    BinaryOperator::LtEq => self.chunk.write_opcode(OpCode::OpLessEqual, span.line),
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }
}
