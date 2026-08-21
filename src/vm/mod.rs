//! VietLang Virtual Machine (VM)
//! Experimental stack-based bytecode interpreter for a documented AST subset.

pub mod compiler;
pub mod opcode;

use crate::error::{VietError, VietResult};
use crate::interpreter::value::Value;
use compiler::Chunk;
use opcode::OpCode;
use std::collections::HashMap;

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
    globals: HashMap<usize, Value>,
    last_result: Value,
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        VM {
            chunk,
            ip: 0,
            stack: Vec::with_capacity(256),
            globals: HashMap::new(),
            last_result: Value::None,
        }
    }

    pub fn run(&mut self) -> VietResult<Value> {
        while self.ip < self.chunk.code.len() {
            let byte = self.chunk.code[self.ip];
            self.ip += 1;

            let op = match OpCode::from_u8(byte) {
                Some(o) => o,
                None => {
                    return Err(VietError::runtime_error(
                        format!("Invalid opcode: 0x{:02X}", byte),
                        0,
                        0,
                    ))
                }
            };

            match op {
                OpCode::OpConstant => {
                    let idx = self.read_u16()?;
                    let val = self.chunk.constants.get(idx).cloned().ok_or_else(|| {
                        VietError::runtime_error(
                            format!("Invalid constant index: {}", idx),
                            self.current_line(),
                            0,
                        )
                    })?;
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
                        (Value::Int(x), Value::Float(y)) => {
                            self.stack.push(Value::Float(x as f64 + y))
                        }
                        (Value::Float(x), Value::Int(y)) => {
                            self.stack.push(Value::Float(x + y as f64))
                        }
                        (Value::String(x), Value::String(y)) => {
                            self.stack.push(Value::String(format!("{}{}", x, y)))
                        }
                        (Value::String(x), y) => {
                            self.stack.push(Value::String(format!("{}{}", x, y)))
                        }
                        (x, Value::String(y)) => {
                            self.stack.push(Value::String(format!("{}{}", x, y)))
                        }
                        _ => {
                            return Err(VietError::type_error(
                                "Cannot add unsupported operand types in VM".into(),
                                0,
                                0,
                            ))
                        }
                    }
                }
                OpCode::OpSub => {
                    let b = self.stack.pop().unwrap_or(Value::None);
                    let a = self.stack.pop().unwrap_or(Value::None);
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Int(x - y)),
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Float(x - y)),
                        (Value::Int(x), Value::Float(y)) => {
                            self.stack.push(Value::Float(x as f64 - y))
                        }
                        (Value::Float(x), Value::Int(y)) => {
                            self.stack.push(Value::Float(x - y as f64))
                        }
                        _ => {
                            return Err(VietError::type_error(
                                "Cannot subtract unsupported operand types in VM".into(),
                                0,
                                0,
                            ))
                        }
                    }
                }
                OpCode::OpMul => {
                    let b = self.stack.pop().unwrap_or(Value::None);
                    let a = self.stack.pop().unwrap_or(Value::None);
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Int(x * y)),
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Float(x * y)),
                        (Value::Int(x), Value::Float(y)) => {
                            self.stack.push(Value::Float(x as f64 * y))
                        }
                        (Value::Float(x), Value::Int(y)) => {
                            self.stack.push(Value::Float(x * y as f64))
                        }
                        (Value::String(value), Value::Int(times))
                        | (Value::Int(times), Value::String(value))
                            if times >= 0 =>
                        {
                            self.stack.push(Value::String(value.repeat(times as usize)))
                        }
                        _ => {
                            return Err(VietError::type_error(
                                "Cannot multiply unsupported operand types in VM".into(),
                                0,
                                0,
                            ))
                        }
                    }
                }
                OpCode::OpDiv => {
                    let b = self.stack.pop().unwrap_or(Value::None);
                    let a = self.stack.pop().unwrap_or(Value::None);
                    match (a, b) {
                        (Value::Int(_), Value::Int(0)) => {
                            return Err(VietError::runtime_error(
                                "Division by zero in VM".into(),
                                0,
                                0,
                            ))
                        }
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Int(x / y)),
                        (Value::Float(_), Value::Float(0.0))
                        | (Value::Int(_), Value::Float(0.0))
                        | (Value::Float(_), Value::Int(0)) => {
                            return Err(VietError::runtime_error(
                                "Division by zero in VM".into(),
                                0,
                                0,
                            ))
                        }
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Float(x / y)),
                        (Value::Int(x), Value::Float(y)) => {
                            self.stack.push(Value::Float(x as f64 / y))
                        }
                        (Value::Float(x), Value::Int(y)) => {
                            self.stack.push(Value::Float(x / y as f64))
                        }
                        _ => {
                            return Err(VietError::type_error(
                                "Cannot divide unsupported operand types in VM".into(),
                                0,
                                0,
                            ))
                        }
                    }
                }
                OpCode::OpMod => {
                    let b = self.stack.pop().unwrap_or(Value::None);
                    let a = self.stack.pop().unwrap_or(Value::None);
                    match (a, b) {
                        (Value::Int(_), Value::Int(0)) => {
                            return Err(VietError::runtime_error(
                                "Division by zero in VM".into(),
                                0,
                                0,
                            ))
                        }
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Int(x % y)),
                        _ => {
                            return Err(VietError::type_error(
                                "Cannot modulo non-integers in VM".into(),
                                0,
                                0,
                            ))
                        }
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
                    self.comparison_op(|a, b| a > b, |a, b| a > b)?;
                }
                OpCode::OpLessThan => {
                    self.comparison_op(|a, b| a < b, |a, b| a < b)?;
                }
                OpCode::OpGreaterEqual => {
                    self.comparison_op(|a, b| a >= b, |a, b| a >= b)?;
                }
                OpCode::OpLessEqual => {
                    self.comparison_op(|a, b| a <= b, |a, b| a <= b)?;
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
                        _ => {
                            return Err(VietError::type_error(
                                "Cannot negate non-number in VM".into(),
                                0,
                                0,
                            ))
                        }
                    }
                }
                OpCode::OpSetGlobal => {
                    let idx = self.read_u16()?;
                    let val = self.stack.last().cloned().unwrap_or(Value::None);
                    self.globals.insert(idx, val);
                }
                OpCode::OpGetGlobal => {
                    let idx = self.read_u16()?;
                    let val = self.globals.get(&idx).cloned().ok_or_else(|| {
                        VietError::runtime_error(
                            format!("Undefined VM slot: {}", idx),
                            self.current_line(),
                            0,
                        )
                    })?;
                    self.stack.push(val);
                }
                OpCode::OpJump => {
                    let target = self.read_u16()?;
                    self.validate_jump(target)?;
                    self.ip = target;
                }
                OpCode::OpJumpIfFalse => {
                    let target = self.read_u16()?;
                    self.validate_jump(target)?;
                    let top = self.stack.pop().unwrap_or(Value::None);
                    if !top.is_truthy() {
                        self.ip = target;
                    }
                }
                OpCode::OpReturn => {
                    let val = self.stack.pop().unwrap_or(Value::None);
                    return Ok(val);
                }
                OpCode::OpRecordResult => {
                    self.last_result = self.stack.pop().unwrap_or(Value::None);
                }
                OpCode::OpHalt => {
                    return Ok(self.last_result.clone());
                }
                _ => {}
            }
        }

        Ok(self.last_result.clone())
    }

    fn read_u16(&mut self) -> VietResult<usize> {
        if self.ip + 1 >= self.chunk.code.len() {
            return Err(VietError::runtime_error(
                "Truncated bytecode operand".into(),
                self.current_line(),
                0,
            ));
        }
        let b1 = self.chunk.code[self.ip] as usize;
        let b2 = self.chunk.code[self.ip + 1] as usize;
        self.ip += 2;
        Ok(b1 | (b2 << 8))
    }

    fn validate_jump(&self, target: usize) -> VietResult<()> {
        if target > self.chunk.code.len() {
            Err(VietError::runtime_error(
                format!("Invalid bytecode jump target: {}", target),
                self.current_line(),
                0,
            ))
        } else {
            Ok(())
        }
    }

    fn current_line(&self) -> usize {
        self.ip
            .checked_sub(1)
            .and_then(|index| self.chunk.lines.get(index))
            .copied()
            .unwrap_or(0)
    }

    fn comparison_op<I, F>(&mut self, int_cmp: I, float_cmp: F) -> VietResult<()>
    where
        I: Fn(i64, i64) -> bool,
        F: Fn(f64, f64) -> bool,
    {
        let right = self.stack.pop().unwrap_or(Value::None);
        let left = self.stack.pop().unwrap_or(Value::None);
        let result = match (&left, &right) {
            (Value::Int(a), Value::Int(b)) => int_cmp(*a, *b),
            (Value::Float(a), Value::Float(b)) => float_cmp(*a, *b),
            (Value::Int(a), Value::Float(b)) => float_cmp(*a as f64, *b),
            (Value::Float(a), Value::Int(b)) => float_cmp(*a, *b as f64),
            (Value::String(a), Value::String(b)) => int_cmp(a.cmp(b) as i64, 0),
            _ => {
                return Err(VietError::type_error(
                    format!(
                        "Cannot compare {} and {} in VM",
                        left.type_name(),
                        right.type_name()
                    ),
                    0,
                    0,
                ))
            }
        };
        self.stack.push(Value::Bool(result));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        interpreter::Interpreter, lexer::Lexer, parser::Parser, semantic::SemanticAnalyzer,
    };

    fn execute_both(source: &str) -> (Value, Value) {
        let tokens = Lexer::new(source).tokenize().unwrap();
        let program = Parser::new(tokens).parse().unwrap();
        SemanticAnalyzer::new().analyze(&program).unwrap();
        let interpreted = Interpreter::new().execute(&program).unwrap();
        let chunk = compiler::Compiler::new().compile(&program).unwrap();
        let vm = VM::new(chunk).run().unwrap();
        (interpreted, vm)
    }

    fn assert_conforms(source: &str) {
        let (interpreted, vm) = execute_both(source);
        assert_eq!(interpreted, vm, "execution mismatch for: {source}");
    }

    #[test]
    fn interpreter_and_vm_conform_for_expressions_and_bindings() {
        assert_conforms("1 + 2 * 3");
        assert_conforms("1 + 2.5");
        assert_conforms("\"viet\" + \"lang\"");
        assert_conforms("-(3 + 2)");
        assert_conforms("1 < 2.5");
        assert_conforms("let mut value = 2\nvalue = value * 5\nvalue");
        assert_conforms("none");
    }

    #[test]
    fn interpreter_and_vm_conform_for_control_flow_and_scopes() {
        assert_conforms("let mut total = 0\nwhile total < 5 { total += 1 }\ntotal");
        assert_conforms("let value = 1\nif true { let value = 99 }\nvalue");
        assert_conforms("let mut value = 1\nif false { value = 2 } else { value = 3 }\nvalue");
    }

    #[test]
    fn interpreter_and_vm_short_circuit_consistently() {
        assert_conforms("false && (1 / 0 > 0)");
        assert_conforms("true || (1 / 0 > 0)");
        assert_conforms("true && !false");
    }

    #[test]
    fn malformed_bytecode_returns_errors_instead_of_panicking() {
        let truncated = Chunk {
            code: vec![OpCode::OpConstant as u8],
            constants: vec![],
            lines: vec![7],
        };
        assert!(VM::new(truncated)
            .run()
            .unwrap_err()
            .message
            .contains("Truncated"));

        let invalid_constant = Chunk {
            code: vec![OpCode::OpConstant as u8, 9, 0, OpCode::OpHalt as u8],
            constants: vec![],
            lines: vec![3; 4],
        };
        assert!(VM::new(invalid_constant)
            .run()
            .unwrap_err()
            .message
            .contains("constant index"));

        let invalid_jump = Chunk {
            code: vec![OpCode::OpJump as u8, 99, 0],
            constants: vec![],
            lines: vec![5; 3],
        };
        assert!(VM::new(invalid_jump)
            .run()
            .unwrap_err()
            .message
            .contains("jump target"));
    }
}
