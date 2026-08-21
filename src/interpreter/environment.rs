//! VietLang Environment
//! Manages variable scoping with a stack of hash maps.

use super::value::Value;
use crate::error::{VietError, VietResult};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Environment for variable storage with lexical scoping
#[derive(Debug, Clone)]
pub struct Environment {
    scopes: Vec<Scope>,
}

#[derive(Debug, Clone)]
struct Scope {
    variables: HashMap<String, SharedVariable>,
}

#[derive(Debug, Clone)]
struct Variable {
    value: Value,
    mutable: bool,
}

type SharedVariable = Arc<Mutex<Variable>>;

/// A lexical snapshot keeps bindings alive after their declaring block exits.
/// Individual bindings are shared so mutations remain visible to sibling
/// closures and are synchronized when a closure is moved to an OS thread.
#[derive(Debug, Clone)]
pub struct CapturedEnvironment {
    scopes: Vec<HashMap<String, SharedVariable>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            scopes: vec![Scope {
                variables: HashMap::new(),
            }],
        }
    }

    /// Push a new scope
    pub fn push_scope(&mut self) {
        self.scopes.push(Scope {
            variables: HashMap::new(),
        });
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
        scope.variables.insert(
            name.to_string(),
            Arc::new(Mutex::new(Variable { value, mutable })),
        );
    }

    /// Get a variable's value, searching from innermost to outermost scope
    pub fn get(&self, name: &str) -> VietResult<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(var) = scope.variables.get(name) {
                return var
                    .lock()
                    .map(|variable| variable.value.clone())
                    .map_err(|_| {
                        VietError::runtime_error(
                            format!("Variable '{}' is unavailable after a poisoned lock", name),
                            0,
                            0,
                        )
                    });
            }
        }
        Err(VietError::name_error(
            format!("Undefined variable: '{}'", name),
            0,
            0,
        ))
    }

    /// Set a variable's value (must already exist and be mutable)
    pub fn set(&mut self, name: &str, value: Value) -> VietResult<()> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(var) = scope.variables.get_mut(name) {
                let mut variable = var.lock().map_err(|_| {
                    VietError::runtime_error(
                        format!("Variable '{}' is unavailable after a poisoned lock", name),
                        0,
                        0,
                    )
                })?;
                if !variable.mutable {
                    return Err(VietError::runtime_error(
                        format!("Cannot assign to immutable variable: '{}'", name),
                        0,
                        0,
                    ));
                }
                variable.value = value;
                return Ok(());
            }
        }
        Err(VietError::name_error(
            format!("Undefined variable: '{}'", name),
            0,
            0,
        ))
    }

    pub fn capture(&self) -> CapturedEnvironment {
        CapturedEnvironment {
            scopes: self
                .scopes
                .iter()
                .map(|scope| scope.variables.clone())
                .collect(),
        }
    }

    pub fn from_capture(captured: &CapturedEnvironment) -> Self {
        Self {
            scopes: captured
                .scopes
                .iter()
                .cloned()
                .map(|variables| Scope { variables })
                .collect(),
        }
    }

    /// Make declarations added later to the module/global scope visible while
    /// preserving lexical lookup for every captured local scope.
    pub fn merge_missing_globals_from(&mut self, caller: &Environment) {
        let Some(target) = self.scopes.first_mut() else {
            return;
        };
        let Some(source) = caller.scopes.first() else {
            return;
        };
        for (name, variable) in &source.variables {
            target
                .variables
                .entry(name.clone())
                .or_insert_with(|| variable.clone());
        }
    }
}
