//! VietLang Virtual Machine (VM)
//! High-performance stack-based bytecode interpreter for backend workloads.

pub mod opcode;
pub mod compiler;

use std::collections::HashMap;
use crate::error::{VietError, VietResult};
use crate::interpreter::value::Value;
use opcode::OpCode;
use compiler::Chunk;

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
    globals: HashMap<usize, Value>,
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        VM {
            chunk,
            ip: 0,
            stack: Vec::with_capacity(256),
            globals: HashMap::new(),
        }
    }

    pub fn run(&mut self) -> VietResult<Value> {
        while self.ip < self.chunk.code.len() {
            let byte = self.chunk.code[self.ip];
            self.ip += 1;

            let op = match OpCode::from_u8(byte) {
                Some(o) => o,
                None => return Err(VietError::runtime_error(format!("Invalid opcode: 0x{:02X}", byte), 0, 0)),
            };

            match op {
                OpCode::OpConstant => {
                    let idx = self.read_u16();
                    let val = self.chunk.constants.get(idx).cloned().unwrap_or(Value::None);
                    self.stack.push(val);
                }
                OpCode::OpPop => {
                    self.stack.pop();
                }
                OpCode::OpAdd => {
                    let b = self.stack.pop().unwrap_or(Value::None);
                    let a = self.stack.pop().unwrap_or(Value::None);
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Int(x + y)),
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Float(x + y)),
                        (Value::String(x), Value::String(y)) => self.stack.push(Value::String(format!("{}{}", x, y))),
                        _ => return Err(VietError::type_error("Cannot add unsupported operand types in VM".into(), 0, 0)),
                    }
                }
                OpCode::OpSub => {
                    let b = self.stack.pop().unwrap_or(Value::None);
                    let a = self.stack.pop().unwrap_or(Value::None);
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Int(x - y)),
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Float(x - y)),
                        _ => return Err(VietError::type_error("Cannot subtract unsupported operand types in VM".into(), 0, 0)),
                    }
                }
                OpCode::OpMul => {
                    let b = self.stack.pop().unwrap_or(Value::None);
                    let a = self.stack.pop().unwrap_or(Value::None);
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Int(x * y)),
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Float(x * y)),
                        _ => return Err(VietError::type_error("Cannot multiply unsupported operand types in VM".into(), 0, 0)),
                    }
                }
                OpCode::OpDiv => {
                    let b = self.stack.pop().unwrap_or(Value::None);
                    let a = self.stack.pop().unwrap_or(Value::None);
                    match (a, b) {
                        (Value::Int(_), Value::Int(0)) => return Err(VietError::runtime_error("Division by zero in VM".into(), 0, 0)),
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Int(x / y)),
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Float(x / y)),
                        _ => return Err(VietError::type_error("Cannot divide unsupported operand types in VM".into(), 0, 0)),
                    }
                }
                OpCode::OpMod => {
                    let b = self.stack.pop().unwrap_or(Value::None);
                    let a = self.stack.pop().unwrap_or(Value::None);
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Int(x % y)),
                        _ => return Err(VietError::type_error("Cannot modulo non-integers in VM".into(), 0, 0)),
                    }
                }
                OpCode::OpEqual => {
                    let b = self.stack.pop().unwrap_or(Value::None);
                    let a = self.stack.pop().unwrap_or(Value::None);
                    self.stack.push(Value::Bool(a == b));
                }
                OpCode::OpNotEqual => {
                    let b = self.stack.pop().unwrap_or(Value::None);
                    let a = self.stack.pop().unwrap_or(Value::None);
                    self.stack.push(Value::Bool(a != b));
                }
                OpCode::OpGreaterThan => {
                    let b = self.stack.pop().unwrap_or(Value::None);
                    let a = self.stack.pop().unwrap_or(Value::None);
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Bool(x > y)),
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Bool(x > y)),
                        _ => self.stack.push(Value::Bool(false)),
                    }
                }
                OpCode::OpLessThan => {
                    let b = self.stack.pop().unwrap_or(Value::None);
                    let a = self.stack.pop().unwrap_or(Value::None);
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Bool(x < y)),
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Bool(x < y)),
                        _ => self.stack.push(Value::Bool(false)),
                    }
                }
                OpCode::OpGreaterEqual => {
                    let b = self.stack.pop().unwrap_or(Value::None);
                    let a = self.stack.pop().unwrap_or(Value::None);
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Bool(x >= y)),
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Bool(x >= y)),
                        _ => self.stack.push(Value::Bool(false)),
                    }
                }
                OpCode::OpLessEqual => {
                    let b = self.stack.pop().unwrap_or(Value::None);
                    let a = self.stack.pop().unwrap_or(Value::None);
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Bool(x <= y)),
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Bool(x <= y)),
                        _ => self.stack.push(Value::Bool(false)),
                    }
                }
                OpCode::OpNot => {
                    let a = self.stack.pop().unwrap_or(Value::None);
                    self.stack.push(Value::Bool(!a.is_truthy()));
                }
                OpCode::OpNeg => {
                    let a = self.stack.pop().unwrap_or(Value::None);
                    match a {
                        Value::Int(x) => self.stack.push(Value::Int(-x)),
                        Value::Float(x) => self.stack.push(Value::Float(-x)),
                        _ => return Err(VietError::type_error("Cannot negate non-number in VM".into(), 0, 0)),
                    }
                }
                OpCode::OpSetGlobal => {
                    let idx = self.read_u16();
                    let val = self.stack.pop().unwrap_or(Value::None);
                    self.globals.insert(idx, val);
                }
                OpCode::OpGetGlobal => {
                    let idx = self.read_u16();
                    let val = self.globals.get(&idx).cloned().unwrap_or(Value::None);
                    self.stack.push(val);
                }
                OpCode::OpJump => {
                    let target = self.read_u16();
                    self.ip = target;
                }
                OpCode::OpJumpIfFalse => {
                    let target = self.read_u16();
                    let top = self.stack.pop().unwrap_or(Value::None);
                    if !top.is_truthy() {
                        self.ip = target;
                    }
                }
                OpCode::OpReturn => {
                    let val = self.stack.pop().unwrap_or(Value::None);
                    return Ok(val);
                }
                OpCode::OpHalt => {
                    let result = self.stack.pop().unwrap_or(Value::None);
                    return Ok(result);
                }
                _ => {}
            }
        }

        Ok(self.stack.pop().unwrap_or(Value::None))
    }

    fn read_u16(&mut self) -> usize {
        let b1 = self.chunk.code[self.ip] as usize;
        let b2 = self.chunk.code[self.ip + 1] as usize;
        self.ip += 2;
        b1 | (b2 << 8)
    }
}
