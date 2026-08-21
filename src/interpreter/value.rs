//! VietLang Runtime Values
//! Defines all value types that can exist at runtime.

use super::environment::CapturedEnvironment;
use crate::parser::ast::{Parameter, Statement};
use std::collections::HashMap;
use std::fmt;

/// Runtime value types
#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    None,
    Array(Vec<Value>),
    Struct {
        type_name: String,
        fields: HashMap<String, Value>,
    },
    EnumVariant {
        type_name: String,
        variant: String,
        fields: Vec<Value>,
    },
    EnumConstructor {
        type_name: String,
        variant: String,
        arity: usize,
    },
    Function {
        name: String,
        params: Vec<Parameter>,
        body: Vec<Statement>,
        closure_env: CapturedEnvironment,
    },
    BuiltinFunction {
        name: String,
        arity: Option<usize>, // None = variadic
    },
    /// Represents a range (start..end)
    Range {
        start: i64,
        end: i64,
    },
}

impl Value {
    pub fn type_name(&self) -> &str {
        match self {
            Value::Int(_) => "Int",
            Value::Float(_) => "Float",
            Value::String(_) => "String",
            Value::Bool(_) => "Bool",
            Value::None => "None",
            Value::Array(_) => "Array",
            Value::Struct { type_name, .. } => type_name,
            Value::EnumVariant { type_name, .. } => type_name,
            Value::EnumConstructor { .. } => "EnumConstructor",
            Value::Function { .. } => "Function",
            Value::BuiltinFunction { .. } => "BuiltinFunction",
            Value::Range { .. } => "Range",
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::None => false,
            Value::Int(n) => *n != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            _ => true,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            Value::Int(n) => Some(*n as f64),
            _ => None,
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            _ => format!("{}", self),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::None => write!(f, "none"),
            Value::Array(elements) => {
                write!(f, "[")?;
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", elem)?;
                }
                write!(f, "]")
            }
            Value::Struct { type_name, fields } => {
                write!(f, "{} {{ ", type_name)?;
                for (i, (key, val)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", key, val)?;
                }
                write!(f, " }}")
            }
            Value::EnumVariant {
                variant, fields, ..
            } => {
                if fields.is_empty() {
                    write!(f, "{}", variant)
                } else {
                    write!(f, "{}(", variant)?;
                    for (i, field) in fields.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", field)?;
                    }
                    write!(f, ")")
                }
            }
            Value::EnumConstructor {
                type_name, variant, ..
            } => {
                write!(f, "<enum constructor {}.{}>", type_name, variant)
            }
            Value::Function { name, .. } => write!(f, "<fn {}>", name),
            Value::BuiltinFunction { name, .. } => write!(f, "<builtin {}>", name),
            Value::Range { start, end } => write!(f, "{}..{}", start, end),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Int(a), Value::Float(b)) => (*a as f64) == *b,
            (Value::Float(a), Value::Int(b)) => *a == (*b as f64),
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::None, Value::None) => true,
            (Value::Array(a), Value::Array(b)) => a == b,
            (
                Value::EnumVariant {
                    type_name: t1,
                    variant: v1,
                    fields: f1,
                },
                Value::EnumVariant {
                    type_name: t2,
                    variant: v2,
                    fields: f2,
                },
            ) => t1 == t2 && v1 == v2 && f1 == f2,
            _ => false,
        }
    }
}
