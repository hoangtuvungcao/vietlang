//! VietLang Environment
//! Manages variable scoping with a stack of hash maps.

use std::collections::HashMap;
use super::value::Value;
use crate::error::{VietError, VietResult};

/// Environment for variable storage with lexical scoping
#[derive(Debug, Clone)]
pub struct Environment {
    scopes: Vec<Scope>,
}

#[derive(Debug, Clone)]
struct Scope {
    variables: HashMap<String, Variable>,
}

#[derive(Debug, Clone)]
struct Variable {
    value: Value,
    mutable: bool,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            scopes: vec![Scope { variables: HashMap::new() }],
        }
    }

    /// Push a new scope
    pub fn push_scope(&mut self) {
        self.scopes.push(Scope { variables: HashMap::new() });
    }

    /// Pop the current scope
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    /// Define a new variable in the current scope
    pub fn define(&mut self, name: &str, value: Value, mutable: bool) {
        let scope = self.scopes.last_mut().unwrap();
        scope.variables.insert(name.to_string(), Variable { value, mutable });
    }

    /// Get a variable's value, searching from innermost to outermost scope
    pub fn get(&self, name: &str) -> VietResult<&Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(var) = scope.variables.get(name) {
                return Ok(&var.value);
            }
        }
        Err(VietError::name_error(
            format!("Undefined variable: '{}'", name),
            0, 0,
        ))
    }

    /// Set a variable's value (must already exist and be mutable)
    pub fn set(&mut self, name: &str, value: Value) -> VietResult<()> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(var) = scope.variables.get_mut(name) {
                if !var.mutable {
                    return Err(VietError::runtime_error(
                        format!("Cannot assign to immutable variable: '{}'", name),
                        0, 0,
                    ));
                }
                var.value = value;
                return Ok(());
            }
        }
        Err(VietError::name_error(
            format!("Undefined variable: '{}'", name),
            0, 0,
        ))
    }

    /// Get the number of scopes (for closure support)
    pub fn depth(&self) -> usize {
        self.scopes.len()
    }
}
