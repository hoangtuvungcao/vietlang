//! VietLang Interpreter
//! Tree-walking interpreter that executes VietLang AST.

pub mod value;
pub mod environment;

use std::collections::{HashMap, HashSet};
use std::net::TcpListener;
use std::io::{BufRead, Read, Write};
use crate::error::{VietError, VietResult, ErrorKind};
use crate::lexer::token::Span;
use crate::parser::ast::*;
use value::Value;
use environment::Environment;

#[derive(Clone)]
pub struct Interpreter {
    env: Environment,
    /// Struct definitions
    struct_defs: HashMap<String, Vec<StructField>>,
    /// Enum definitions
    enum_defs: HashMap<String, Vec<EnumVariant>>,
    /// Track loaded module paths to prevent circular / duplicate execution
    loaded_modules: HashSet<String>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interp = Interpreter {
            env: Environment::new(),
            struct_defs: HashMap::new(),
            enum_defs: HashMap::new(),
            loaded_modules: HashSet::new(),
        };
        interp.register_builtins();
        interp
    }

    fn register_builtins(&mut self) {
        // Standard builtin functions
        let builtins = vec![
            // Core
            ("print", None),       // variadic
            ("println", None),     // variadic
            ("len", Some(1)),
            ("type_of", Some(1)),
            ("to_string", Some(1)),
            ("to_int", Some(1)),
            ("to_float", Some(1)),
            ("push", Some(2)),
            ("pop", Some(1)),
            ("input", Some(1)),
            ("abs", Some(1)),
            ("min", Some(2)),
            ("max", Some(2)),
            ("assert", None),
            ("exit", None),
            ("format", None),
            ("range", None),

            // std.io — File I/O
            ("file_read", Some(1)),
            ("file_write", Some(2)),
            ("file_append", Some(2)),
            ("file_exists", Some(1)),
            ("file_delete", Some(1)),
            ("dir_list", Some(1)),
            ("dir_create", Some(1)),

            // std.json
            ("json_parse", Some(1)),
            ("json_stringify", None),

            // std.env
            ("env_get", None),
            ("env_set", Some(2)),
            ("env_all", Some(0)),

            // std.time
            ("time_now", Some(0)),
            ("time_now_ms", Some(0)),
            ("sleep", Some(1)),
            ("timer_start", Some(0)),

            // std.crypto
            ("sha256", Some(1)),
            ("sha1", Some(1)),
            ("ws_accept_key", Some(1)),
            ("uuid", Some(0)),
            ("base64_encode", Some(1)),
            ("random_int", Some(2)),

            // std.log
            ("log_debug", None),
            ("log_info", None),
            ("log_warn", None),
            ("log_error", None),

            // std.collections (Map)
            ("map_new", Some(0)),
            ("map_set", Some(3)),
            ("map_get", None),
            ("map_has", Some(2)),
            ("map_keys", Some(1)),
            ("map_values", Some(1)),
            ("map_remove", Some(2)),

            // std.http
            ("http_listen", None),

            // std.db & std.db_sqlite (Real Binary SQLite)
            ("db_query", None),
            ("db_table", Some(1)),
            ("sqlite_open", None),
            ("sqlite_exec", Some(2)),
            ("sqlite_execute", None),
            ("sqlite_query", None),
            ("sqlite_close", Some(1)),
            ("builtin_sqlite_open", None),
            ("builtin_sqlite_exec", Some(2)),
            ("builtin_sqlite_execute", None),
            ("builtin_sqlite_query", None),
            ("builtin_sqlite_close", Some(1)),

            // MySQL Native Driver Builtins
            ("mysql_connect", None),
            ("mysql_exec", Some(2)),
            ("mysql_execute", None),
            ("mysql_query", None),
            ("mysql_close", Some(1)),
            ("builtin_mysql_connect", None),
            ("builtin_mysql_exec", Some(2)),
            ("builtin_mysql_execute", None),
            ("builtin_mysql_query", None),
            // HTTP & Network Clients & WebSockets & Security
            ("http_fetch", None),
            ("csv_parse", Some(1)),
            ("csv_stringify", Some(1)),
            ("ws_enable", None),
            ("ws_broadcast", None),
            ("html_escape", Some(1)),

            // Concurrency
            ("spawn", None),
            ("channel", None),
            ("channel_new", None),
            ("channel_send", None),
            ("channel_recv", None),
            ("channel_try_recv", None),
            ("channel_close", None),
            ("thread_sleep", None),
            ("mutex_new", None),

            // String character operations
            ("char_at", Some(2)),
            ("char_code", Some(1)),
            ("from_char_code", Some(1)),
            ("substring", None),
            ("str_repeat", Some(2)),
            ("parse_int", Some(1)),
            ("parse_float", Some(1)),

            // Array operations
            ("sort", Some(1)),
            ("slice", None),
            ("index_of", Some(2)),
            ("flat", Some(1)),

            // Error handling & Reflection
            ("throw", None),
            ("is_error", Some(1)),
            ("typeof", Some(1)),
            ("type_of", Some(1)),

            // System & Timers
            ("get_args", Some(0)),
            ("platform", Some(0)),
            ("arch", Some(0)),
            ("sleep_ms", Some(1)),
            ("time_now_us", Some(0)),
            ("tcp_ping", None),
            ("tcp_send", None),
            ("udp_send", Some(3)),
            ("str_split_lines", Some(1)),
            ("system_cmd", Some(1)),
            ("url_encode", Some(1)),
            ("url_decode", Some(1)),
            ("to_uppercase", Some(1)),
            ("to_lowercase", Some(1)),
            ("trim", Some(1)),
            ("starts_with", Some(2)),
            ("ends_with", Some(2)),
            ("contains", Some(2)),
            ("hmac_sha256", Some(2)),
            ("hmac_sha512", Some(2)),
            ("encrypt_secret", Some(2)),
            ("decrypt_secret", Some(2)),
            ("ip_in_cidr", Some(2)),
            ("hex_encode", Some(1)),
            ("hex_decode", Some(1)),
            ("crypto_random_hex", None),
        ];

        for (name, arity) in builtins {
            self.env.define(
                name,
                Value::BuiltinFunction {
                    name: name.to_string(),
                    arity,
                },
                false,
            );
        }
    }

    /// Execute a complete program
    pub fn execute(&mut self, program: &Program) -> VietResult<Value> {
        let mut last_value = Value::None;
        for stmt in &program.statements {
            match self.execute_statement(stmt) {
                Ok(val) => last_value = val,
                Err(e) => return Err(e),
            }
        }
        Ok(last_value)
    }

    // ========================================
    // Statement Execution
    // ========================================

    fn execute_statement(&mut self, stmt: &Statement) -> VietResult<Value> {
        match stmt {
            Statement::Let { name, mutable, value, .. } => {
                let val = self.evaluate_expression(value)?;
                self.env.define(name, val.clone(), *mutable);
                Ok(val)
            }

            Statement::Assignment { target, value, span } => {
                let val = self.evaluate_expression(value)?;
                match target {
                    Expression::Identifier { name, .. } => {
                        self.env.set(name, val.clone()).map_err(|mut e| {
                            e.line = span.line;
                            e.column = span.column;
                            e
                        })?;
                    }
                    Expression::FieldAccess { object, field, .. } => {
                        // Get the struct, modify field, set back
                        let obj_val = self.evaluate_expression(object)?;
                        match obj_val {
                            Value::Struct { type_name, mut fields } => {
                                fields.insert(field.clone(), val.clone());
                                if let Expression::Identifier { name, .. } = object.as_ref() {
                                    self.env.set(name, Value::Struct { type_name, fields }).map_err(|mut e| {
                                        e.line = span.line;
                                        e.column = span.column;
                                        e
                                    })?;
                                }
                            }
                            _ => return Err(VietError::runtime_error(
                                "Cannot set field on non-struct value".to_string(),
                                span.line, span.column,
                            )),
                        }
                    }
                    Expression::Index { object, index, .. } => {
                        let idx_val = self.evaluate_expression(index)?;
                        let obj_val = self.evaluate_expression(object)?;
                        match (obj_val, idx_val) {
                            (Value::Array(mut arr), Value::Int(i)) => {
                                let idx = if i < 0 { (arr.len() as i64 + i) as usize } else { i as usize };
                                if idx < arr.len() {
                                    arr[idx] = val.clone();
                                    if let Expression::Identifier { name, .. } = object.as_ref() {
                                        self.env.set(name, Value::Array(arr)).map_err(|mut e| {
                                            e.line = span.line;
                                            e.column = span.column;
                                            e
                                        })?;
                                    }
                                } else {
                                    return Err(VietError::runtime_error(
                                        format!("Index {} out of bounds (length {})", i, arr.len()),
                                        span.line, span.column,
                                    ));
                                }
                            }
                            _ => return Err(VietError::runtime_error(
                                "Invalid index assignment".to_string(),
                                span.line, span.column,
                            )),
                        }
                    }
                    _ => return Err(VietError::runtime_error(
                        "Invalid assignment target".to_string(),
                        span.line, span.column,
                    )),
                }
                Ok(val)
            }

            Statement::Expression { expr, .. } => {
                self.evaluate_expression(expr)
            }

            Statement::Function { name, params, body, .. } => {
                let func = Value::Function {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                    closure_env: None,
                };
                self.env.define(name, func, false);
                Ok(Value::None)
            }

            Statement::Return { value, .. } => {
                let val = match value {
                    Some(expr) => self.evaluate_expression(expr)?,
                    None => Value::None,
                };
                Err(VietError::return_signal(val))
            }

            Statement::If { condition, then_body, else_body, .. } => {
                let cond = self.evaluate_expression(condition)?;
                if cond.is_truthy() {
                    self.execute_block(then_body)
                } else if let Some(else_stmts) = else_body {
                    self.execute_block(else_stmts)
                } else {
                    Ok(Value::None)
                }
            }

            Statement::While { condition, body, .. } => {
                loop {
                    let cond = self.evaluate_expression(condition)?;
                    if !cond.is_truthy() {
                        break;
                    }
                    match self.execute_block(body) {
                        Ok(_) => {}
                        Err(VietError { kind: ErrorKind::Break, .. }) => break,
                        Err(VietError { kind: ErrorKind::Continue, .. }) => continue,
                        Err(e) => return Err(e),
                    }
                }
                Ok(Value::None)
            }

            Statement::For { variable, iterable, body, .. } => {
                let iter_val = self.evaluate_expression(iterable)?;
                let items = match iter_val {
                    Value::Array(arr) => arr,
                    Value::Range { start, end } => {
                        (start..end).map(Value::Int).collect()
                    }
                    Value::String(s) => {
                        s.chars().map(|c| Value::String(c.to_string())).collect()
                    }
                    _ => return Err(VietError::runtime_error(
                        format!("Cannot iterate over {}", iter_val.type_name()),
                        0, 0,
                    )),
                };

                for item in items {
                    self.env.push_scope();
                    self.env.define(variable, item, false);
                    match self.execute_block_no_scope(body) {
                        Ok(_) => {}
                        Err(VietError { kind: ErrorKind::Break, .. }) => {
                            self.env.pop_scope();
                            break;
                        }
                        Err(VietError { kind: ErrorKind::Continue, .. }) => {
                            self.env.pop_scope();
                            continue;
                        }
                        Err(e) => {
                            self.env.pop_scope();
                            return Err(e);
                        }
                    }
                    self.env.pop_scope();
                }
                Ok(Value::None)
            }

            Statement::Break { .. } => {
                Err(VietError::break_signal())
            }

            Statement::Continue { .. } => {
                Err(VietError::continue_signal())
            }

            Statement::Struct { name, fields, .. } => {
                self.struct_defs.insert(name.clone(), fields.clone());
                Ok(Value::None)
            }

            Statement::Enum { name, variants, .. } => {
                // Register enum variants as constructors
                for variant in variants {
                    let variant_name = format!("{}.{}", name, variant.name);
                    if variant.fields.is_empty() {
                        // Simple variant - define as a value
                        self.env.define(
                            &variant.name,
                            Value::EnumVariant {
                                type_name: name.clone(),
                                variant: variant.name.clone(),
                                fields: Vec::new(),
                            },
                            false,
                        );
                    }
                    // Store for pattern matching
                    let _ = variant_name; // used for matching later
                }
                self.enum_defs.insert(name.clone(), variants.clone());
                Ok(Value::None)
            }

            Statement::Impl { type_name, methods, .. } => {
                for method in methods {
                    if let Statement::Function { name: method_name, params, body, .. } = method {
                        let full_name = format!("{}::{}", type_name, method_name);
                        let func = Value::Function {
                            name: full_name.clone(),
                            params: params.clone(),
                            body: body.clone(),
                            closure_env: None,
                        };
                        self.env.define(&full_name, func, false);
                    }
                }
                Ok(Value::None)
            }

            Statement::Import { path, span, .. } => {
                let joined = path.join("/");
                let mut search_paths = vec![
                    format!("{}.vl", joined),
                    format!("src/{}.vl", joined),
                    format!("std/{}.vl", joined),
                    format!("modules/{}.vl", joined),
                    format!("modules/{}/src/main.vl", joined),
                    format!("modules/{}/src/lib.vl", joined),
                    format!("modules/{}/mod.vl", joined),
                ];

                // If path contains 'src', also search from 'src' onwards
                if let Some(src_pos) = path.iter().position(|seg| seg == "src") {
                    let from_src = path[src_pos..].join("/");
                    let after_src = path[src_pos+1..].join("/");
                    search_paths.push(format!("{}.vl", from_src));
                    search_paths.push(format!("{}.vl", after_src));
                    search_paths.push(format!("src/{}.vl", after_src));
                }

                // If path contains 'examples', also search relative to current repo
                if let Some(app_pos) = path.iter().position(|seg| seg == "agricultural_ecommerce" || seg == "viet_fintech_gateway" || seg == "viet_finance_desktop") {
                    let from_app = path[app_pos+1..].join("/");
                    search_paths.push(format!("{}.vl", from_app));
                    search_paths.push(format!("src/{}.vl", from_app));
                }

                if path.len() > 1 && path[0] == "std" {
                    search_paths.push(format!("std/{}.vl", path[1..].join("/")));
                }

                // Global ~/.vietlang paths
                if let Ok(home) = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")) {
                    search_paths.push(format!("{}/.vietlang/std/{}.vl", home, joined));
                    search_paths.push(format!("{}/.vietlang/modules/{}.vl", home, joined));
                    if path.len() > 1 && path[0] == "std" {
                        search_paths.push(format!("{}/.vietlang/std/{}.vl", home, path[1..].join("/")));
                    }
                }

                // Executable directory relative paths
                if let Ok(exe_path) = std::env::current_exe() {
                    if let Some(exe_dir) = exe_path.parent() {
                        let exe_dir_str = exe_dir.to_string_lossy();
                        search_paths.push(format!("{}/std/{}.vl", exe_dir_str, joined));
                        search_paths.push(format!("{}/modules/{}.vl", exe_dir_str, joined));
                        if path.len() > 1 && path[0] == "std" {
                            search_paths.push(format!("{}/std/{}.vl", exe_dir_str, path[1..].join("/")));
                        }
                    }
                }

                let mut found_path = None;
                for sp in search_paths {
                    if !sp.is_empty() && std::path::Path::new(&sp).exists() {
                        found_path = Some(sp);
                        break;
                    }
                }

                if let Some(resolved_path) = found_path {
                    let canonical = std::fs::canonicalize(&resolved_path)
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_else(|_| resolved_path.clone());

                    if !self.loaded_modules.contains(&canonical) {
                        self.loaded_modules.insert(canonical);
                        let source = std::fs::read_to_string(&resolved_path).map_err(|e|
                            VietError::runtime_error(format!("Cannot import '{}': {}", resolved_path, e), span.line, span.column)
                        )?;
                        let mut lexer = crate::lexer::Lexer::new(&source);
                        let tokens = lexer.tokenize()?;
                        let mut parser = crate::parser::Parser::new(tokens);
                        let program = parser.parse()?;
                        self.execute(&program)?;
                    }
                    Ok(Value::None)
                } else {
                    // Built-in virtual module
                    Ok(Value::None)
                }
            }

            Statement::TryCatch { try_body, catch_var, catch_body, .. } => {
                match self.execute_block(try_body) {
                    Ok(val) => Ok(val),
                    Err(err) => {
                        if matches!(err.kind, ErrorKind::Return(_) | ErrorKind::Break | ErrorKind::Continue) {
                            // Control flow signals pass through
                            return Err(err);
                        }
                        self.env.push_scope();
                        self.env.define(catch_var, Value::String(err.message.clone()), false);
                        let result = self.execute_block_no_scope(catch_body);
                        self.env.pop_scope();
                        result
                    }
                }
            }
        }
    }

    fn execute_block(&mut self, statements: &[Statement]) -> VietResult<Value> {
        self.env.push_scope();
        let result = self.execute_block_no_scope(statements);
        self.env.pop_scope();
        result
    }

    fn execute_block_no_scope(&mut self, statements: &[Statement]) -> VietResult<Value> {
        let mut last = Value::None;
        for stmt in statements {
            last = self.execute_statement(stmt)?;
        }
        Ok(last)
    }

    // ========================================
    // Expression Evaluation
    // ========================================

    fn evaluate_expression(&mut self, expr: &Expression) -> VietResult<Value> {
        match expr {
            Expression::IntLiteral { value, .. } => Ok(Value::Int(*value)),
            Expression::FloatLiteral { value, .. } => Ok(Value::Float(*value)),
            Expression::StringLiteral { value, .. } => Ok(Value::String(value.clone())),
            Expression::BoolLiteral { value, .. } => Ok(Value::Bool(*value)),
            Expression::NoneLiteral { .. } => Ok(Value::None),

            Expression::Identifier { name, span } => {
                self.env.get(name).cloned().map_err(|mut e| {
                    e.line = span.line;
                    e.column = span.column;
                    e
                })
            }

            Expression::BinaryOp { left, op, right, span } => {
                match op {
                    BinaryOperator::And => {
                        let lval = self.evaluate_expression(left)?;
                        if !lval.is_truthy() {
                            Ok(Value::Bool(false))
                        } else {
                            let rval = self.evaluate_expression(right)?;
                            Ok(Value::Bool(rval.is_truthy()))
                        }
                    }
                    BinaryOperator::Or => {
                        let lval = self.evaluate_expression(left)?;
                        if lval.is_truthy() {
                            Ok(Value::Bool(true))
                        } else {
                            let rval = self.evaluate_expression(right)?;
                            Ok(Value::Bool(rval.is_truthy()))
                        }
                    }
                    _ => {
                        let lval = self.evaluate_expression(left)?;
                        let rval = self.evaluate_expression(right)?;
                        self.evaluate_binary_op(&lval, op, &rval, span)
                    }
                }
            }

            Expression::UnaryOp { op, operand, span } => {
                let val = self.evaluate_expression(operand)?;
                match op {
                    UnaryOperator::Neg => match val {
                        Value::Int(n) => Ok(Value::Int(-n)),
                        Value::Float(f) => Ok(Value::Float(-f)),
                        _ => Err(VietError::type_error(
                            format!("Cannot negate {}", val.type_name()),
                            span.line, span.column,
                        )),
                    },
                    UnaryOperator::Not => Ok(Value::Bool(!val.is_truthy())),
                }
            }

            Expression::Call { callee, arguments, span } => {
                let func = self.evaluate_expression(callee)?;
                let mut args = Vec::new();
                for arg in arguments {
                    args.push(self.evaluate_expression(arg)?);
                }
                self.call_function(&func, &args, span)
            }

            Expression::MethodCall { object, method, arguments, span } => {
                let obj = self.evaluate_expression(object)?;
                let mut args = Vec::new();
                for arg in arguments {
                    args.push(self.evaluate_expression(arg)?);
                }
                self.call_method(&obj, method, &args, span)
            }

            Expression::FieldAccess { object, field, span } => {
                let obj = self.evaluate_expression(object)?;
                match &obj {
                    Value::Struct { fields, .. } => {
                        fields.get(field).cloned().ok_or_else(|| {
                            VietError::runtime_error(
                                format!("Struct has no field '{}'", field),
                                span.line, span.column,
                            )
                        })
                    }
                    _ => Err(VietError::type_error(
                        format!("Cannot access field on {}", obj.type_name()),
                        span.line, span.column,
                    )),
                }
            }

            Expression::Index { object, index, span } => {
                let obj = self.evaluate_expression(object)?;
                let idx = self.evaluate_expression(index)?;
                match (&obj, &idx) {
                    (Value::Array(arr), Value::Int(i)) => {
                        let index = if *i < 0 {
                            (arr.len() as i64 + *i) as usize
                        } else {
                            *i as usize
                        };
                        arr.get(index).cloned().ok_or_else(|| {
                            VietError::runtime_error(
                                format!("Index {} out of bounds (length {})", i, arr.len()),
                                span.line, span.column,
                            )
                        })
                    }
                    (Value::String(s), Value::Int(i)) => {
                        let index = if *i < 0 {
                            (s.len() as i64 + *i) as usize
                        } else {
                            *i as usize
                        };
                        s.chars().nth(index)
                            .map(|c| Value::String(c.to_string()))
                            .ok_or_else(|| {
                                VietError::runtime_error(
                                    format!("Index {} out of bounds", i),
                                    span.line, span.column,
                                )
                            })
                    }
                    _ => Err(VietError::type_error(
                        format!("Cannot index {} with {}", obj.type_name(), idx.type_name()),
                        span.line, span.column,
                    )),
                }
            }

            Expression::ArrayLiteral { elements, .. } => {
                let mut values = Vec::new();
                for elem in elements {
                    values.push(self.evaluate_expression(elem)?);
                }
                Ok(Value::Array(values))
            }

            Expression::StructLiteral { name, fields, span } => {
                // Verify struct exists
                if !self.struct_defs.contains_key(name) {
                    return Err(VietError::type_error(
                        format!("Unknown struct type: '{}'", name),
                        span.line, span.column,
                    ));
                }

                let mut field_values = HashMap::new();
                for (fname, fexpr) in fields {
                    let val = self.evaluate_expression(fexpr)?;
                    field_values.insert(fname.clone(), val);
                }
                Ok(Value::Struct {
                    type_name: name.clone(),
                    fields: field_values,
                })
            }

            Expression::Match { subject, arms, span } => {
                let subject_val = self.evaluate_expression(subject)?;
                for arm in arms {
                    if let Some(bindings) = self.match_pattern(&arm.pattern, &subject_val) {
                        self.env.push_scope();
                        for (name, val) in bindings {
                            self.env.define(&name, val, false);
                        }
                        let result = self.evaluate_expression(&arm.body);
                        self.env.pop_scope();
                        return result;
                    }
                }
                Err(VietError::runtime_error(
                    "Non-exhaustive match expression".to_string(),
                    span.line, span.column,
                ))
            }

            Expression::Block { statements, final_expr, .. } => {
                self.env.push_scope();
                for stmt in statements {
                    self.execute_statement(stmt)?;
                }
                let result = match final_expr {
                    Some(expr) => self.evaluate_expression(expr),
                    None => Ok(Value::None),
                };
                self.env.pop_scope();
                result
            }

            Expression::Lambda { params, body, .. } => {
                Ok(Value::Function {
                    name: "<lambda>".to_string(),
                    params: params.clone(),
                    body: vec![Statement::Return {
                        value: Some(*body.clone()),
                        span: body.span().clone(),
                    }],
                    closure_env: Some(self.env.depth()),
                })
            }

            Expression::Range { start, end, span } => {
                let start_val = self.evaluate_expression(start)?;
                let end_val = self.evaluate_expression(end)?;
                match (&start_val, &end_val) {
                    (Value::Int(s), Value::Int(e)) => {
                        Ok(Value::Range { start: *s, end: *e })
                    }
                    _ => Err(VietError::type_error(
                        "Range bounds must be integers".to_string(),
                        span.line, span.column,
                    )),
                }
            }
        }
    }

    // ========================================
    // Binary Operations
    // ========================================

    fn evaluate_binary_op(
        &self,
        left: &Value,
        op: &BinaryOperator,
        right: &Value,
        span: &Span,
    ) -> VietResult<Value> {
        match op {
            // Arithmetic
            BinaryOperator::Add => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + *b as f64)),
                (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                (Value::String(a), b) => Ok(Value::String(format!("{}{}", a, b))),
                (a, Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                (Value::Array(a), Value::Array(b)) => {
                    let mut combined = a.clone();
                    combined.extend(b.clone());
                    Ok(Value::Array(combined))
                }
                _ => Err(VietError::type_error(
                    format!("Cannot add {} and {}", left.type_name(), right.type_name()),
                    span.line, span.column,
                )),
            },
            BinaryOperator::Sub => self.numeric_op(left, right, |a, b| a - b, |a, b| a - b, span),
            BinaryOperator::Mul => match (left, right) {
                (Value::String(s), Value::Int(n)) | (Value::Int(n), Value::String(s)) => {
                    Ok(Value::String(s.repeat(*n as usize)))
                }
                _ => self.numeric_op(left, right, |a, b| a * b, |a, b| a * b, span),
            },
            BinaryOperator::Div => {
                // Check for division by zero
                match right {
                    Value::Int(0) => Err(VietError::runtime_error(
                        "Division by zero".to_string(),
                        span.line, span.column,
                    )),
                    Value::Float(f) if *f == 0.0 => Err(VietError::runtime_error(
                        "Division by zero".to_string(),
                        span.line, span.column,
                    )),
                    _ => self.numeric_op(left, right, |a, b| a / b, |a, b| a / b, span),
                }
            }
            BinaryOperator::Mod => self.numeric_op(left, right, |a, b| a % b, |a, b| a % b, span),

            // Comparison
            BinaryOperator::Eq => Ok(Value::Bool(left == right)),
            BinaryOperator::NotEq => Ok(Value::Bool(left != right)),
            BinaryOperator::Lt => self.comparison_op(left, right, |a, b| a < b, |a, b| a < b, span),
            BinaryOperator::Gt => self.comparison_op(left, right, |a, b| a > b, |a, b| a > b, span),
            BinaryOperator::LtEq => self.comparison_op(left, right, |a, b| a <= b, |a, b| a <= b, span),
            BinaryOperator::GtEq => self.comparison_op(left, right, |a, b| a >= b, |a, b| a >= b, span),

            // Logical
            BinaryOperator::And => Ok(Value::Bool(left.is_truthy() && right.is_truthy())),
            BinaryOperator::Or => Ok(Value::Bool(left.is_truthy() || right.is_truthy())),
        }
    }

    fn numeric_op<F1, F2>(
        &self,
        left: &Value,
        right: &Value,
        int_op: F1,
        float_op: F2,
        span: &Span,
    ) -> VietResult<Value>
    where
        F1: Fn(i64, i64) -> i64,
        F2: Fn(f64, f64) -> f64,
    {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(int_op(*a, *b))),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(float_op(*a, *b))),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(float_op(*a as f64, *b))),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(float_op(*a, *b as f64))),
            _ => Err(VietError::type_error(
                format!("Cannot perform arithmetic on {} and {}", left.type_name(), right.type_name()),
                span.line, span.column,
            )),
        }
    }

    fn comparison_op<F1, F2>(
        &self,
        left: &Value,
        right: &Value,
        int_cmp: F1,
        float_cmp: F2,
        span: &Span,
    ) -> VietResult<Value>
    where
        F1: Fn(i64, i64) -> bool,
        F2: Fn(f64, f64) -> bool,
    {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(int_cmp(*a, *b))),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(float_cmp(*a, *b))),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool(float_cmp(*a as f64, *b))),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(float_cmp(*a, *b as f64))),
            (Value::String(a), Value::String(b)) => Ok(Value::Bool(int_cmp(a.cmp(b) as i64, 0))),
            _ => Err(VietError::type_error(
                format!("Cannot compare {} and {}", left.type_name(), right.type_name()),
                span.line, span.column,
            )),
        }
    }

    // ========================================
    // Function Calls
    // ========================================

    fn call_function(
        &mut self,
        func: &Value,
        args: &[Value],
        span: &Span,
    ) -> VietResult<Value> {
        match func {
            Value::Function { params, body, .. } => {
                self.env.push_scope();

                // Bind parameters
                for (i, param) in params.iter().enumerate() {
                    let val = if i < args.len() {
                        args[i].clone()
                    } else if let Some(default) = &param.default {
                        self.evaluate_expression(default)?
                    } else {
                        Value::None
                    };
                    self.env.define(&param.name, val, false);
                }

                // Execute body
                let result = self.execute_block_no_scope(body);
                self.env.pop_scope();

                match result {
                    Ok(val) => Ok(val),
                    Err(VietError { kind: ErrorKind::Return(val), .. }) => Ok(val),
                    Err(e) => Err(e),
                }
            }
            Value::BuiltinFunction { name, .. } => {
                self.call_builtin(name, args, span)
            }
            _ => Err(VietError::type_error(
                format!("'{}' is not callable", func.type_name()),
                span.line, span.column,
            )),
        }
    }

    fn call_method(
        &mut self,
        object: &Value,
        method: &str,
        args: &[Value],
        span: &Span,
    ) -> VietResult<Value> {
        // Try to find impl method first
        let type_name = object.type_name().to_string();
        let full_name = format!("{}::{}", type_name, method);
        if let Ok(func) = self.env.get(&full_name).cloned() {
            let mut all_args = vec![object.clone()];
            all_args.extend_from_slice(args);
            return self.call_function(&func, &all_args, span);
        }

        // Built-in methods on types
        match (object, method) {
            // String methods
            (Value::String(s), "len") => Ok(Value::Int(s.len() as i64)),
            (Value::String(s), "contains") => {
                if let Some(Value::String(sub)) = args.first() {
                    Ok(Value::Bool(s.contains(sub.as_str())))
                } else {
                    Err(VietError::type_error("contains() expects a string argument".to_string(), span.line, span.column))
                }
            }
            (Value::String(s), "split") => {
                if let Some(Value::String(sep)) = args.first() {
                    Ok(Value::Array(s.split(sep.as_str()).map(|p| Value::String(p.to_string())).collect()))
                } else {
                    Err(VietError::type_error("split() expects a string argument".to_string(), span.line, span.column))
                }
            }
            (Value::String(s), "trim") => Ok(Value::String(s.trim().to_string())),
            (Value::String(s), "to_upper" | "to_uppercase") => Ok(Value::String(s.to_uppercase())),
            (Value::String(s), "to_lower" | "to_lowercase") => Ok(Value::String(s.to_lowercase())),
            (Value::String(s), "starts_with") => {
                if let Some(Value::String(prefix)) = args.first() {
                    Ok(Value::Bool(s.starts_with(prefix.as_str())))
                } else {
                    Err(VietError::type_error("starts_with() expects a string argument".to_string(), span.line, span.column))
                }
            }
            (Value::String(s), "ends_with") => {
                if let Some(Value::String(suffix)) = args.first() {
                    Ok(Value::Bool(s.ends_with(suffix.as_str())))
                } else {
                    Err(VietError::type_error("ends_with() expects a string argument".to_string(), span.line, span.column))
                }
            }
            (Value::String(s), "replace") => {
                if args.len() == 2 {
                    if let (Value::String(from), Value::String(to)) = (&args[0], &args[1]) {
                        Ok(Value::String(s.replace(from.as_str(), to.as_str())))
                    } else {
                        Err(VietError::type_error("replace() expects two string arguments".to_string(), span.line, span.column))
                    }
                } else {
                    Err(VietError::type_error("replace() expects exactly 2 arguments".to_string(), span.line, span.column))
                }
            }
            (Value::String(s), "chars") => {
                Ok(Value::Array(s.chars().map(|c| Value::String(c.to_string())).collect()))
            }

            // Array methods
            (Value::Array(arr), "len") => Ok(Value::Int(arr.len() as i64)),
            (Value::Array(arr), "is_empty") => Ok(Value::Bool(arr.is_empty())),
            (Value::Array(arr), "push") => {
                if let Some(item) = args.first() {
                    let mut new_arr = arr.clone();
                    new_arr.push(item.clone());
                    Ok(Value::Array(new_arr))
                } else {
                    Err(VietError::type_error("push() expects an item argument".to_string(), span.line, span.column))
                }
            }
            (Value::Array(arr), "first") => {
                Ok(arr.first().cloned().unwrap_or(Value::None))
            }
            (Value::Array(arr), "last") => {
                Ok(arr.last().cloned().unwrap_or(Value::None))
            }
            (Value::Array(arr), "contains") => {
                if let Some(needle) = args.first() {
                    Ok(Value::Bool(arr.contains(needle)))
                } else {
                    Err(VietError::type_error("contains() expects an argument".to_string(), span.line, span.column))
                }
            }
            (Value::Array(arr), "join") => {
                if let Some(Value::String(sep)) = args.first() {
                    let joined: Vec<String> = arr.iter().map(|v| format!("{}", v)).collect();
                    Ok(Value::String(joined.join(sep)))
                } else {
                    let joined: Vec<String> = arr.iter().map(|v| format!("{}", v)).collect();
                    Ok(Value::String(joined.join("")))
                }
            }
            (Value::Array(arr), "reversed") => {
                let mut rev = arr.clone();
                rev.reverse();
                Ok(Value::Array(rev))
            }
            (Value::Array(arr), "map") => {
                if let Some(func) = args.first() {
                    let mut result = Vec::new();
                    for item in arr {
                        let val = self.call_function(func, &[item.clone()], span)?;
                        result.push(val);
                    }
                    Ok(Value::Array(result))
                } else {
                    Err(VietError::type_error("map() expects a function argument".to_string(), span.line, span.column))
                }
            }
            (Value::Array(arr), "filter") => {
                if let Some(func) = args.first() {
                    let mut result = Vec::new();
                    for item in arr {
                        let val = self.call_function(func, &[item.clone()], span)?;
                        if val.is_truthy() {
                            result.push(item.clone());
                        }
                    }
                    Ok(Value::Array(result))
                } else {
                    Err(VietError::type_error("filter() expects a function argument".to_string(), span.line, span.column))
                }
            }
            (Value::Array(arr), "reduce") => {
                if args.len() == 2 {
                    let func = &args[0];
                    let mut acc = args[1].clone();
                    for item in arr {
                        acc = self.call_function(func, &[acc, item.clone()], span)?;
                    }
                    Ok(acc)
                } else {
                    Err(VietError::type_error("reduce() expects (function, initial) arguments".to_string(), span.line, span.column))
                }
            }

            // Int/Float methods
            (Value::Int(_) | Value::Float(_), "abs") => {
                match object {
                    Value::Int(n) => Ok(Value::Int(n.abs())),
                    Value::Float(f) => Ok(Value::Float(f.abs())),
                    _ => unreachable!(),
                }
            }
            (Value::Int(n), "to_float") => Ok(Value::Float(*n as f64)),
            (Value::Float(f), "to_int") => Ok(Value::Int(*f as i64)),

            _ => Err(VietError::runtime_error(
                format!("No method '{}' on type '{}'", method, object.type_name()),
                span.line, span.column,
            )),
        }
    }

    // ========================================
    // Built-in Functions
    // ========================================

    fn call_builtin(
        &mut self,
        name: &str,
        args: &[Value],
        span: &Span,
    ) -> VietResult<Value> {
        match name {
            "print" => {
                let output: Vec<String> = args.iter().map(|a| format!("{}", a)).collect();
                print!("{}", output.join(" "));
                Ok(Value::None)
            }
            "println" => {
                let output: Vec<String> = args.iter().map(|a| format!("{}", a)).collect();
                println!("{}", output.join(" "));
                Ok(Value::None)
            }
            "len" => {
                if args.len() != 1 {
                    return Err(VietError::runtime_error("len() takes exactly 1 argument".to_string(), span.line, span.column));
                }
                match &args[0] {
                    Value::String(s) => Ok(Value::Int(s.len() as i64)),
                    Value::Array(a) => Ok(Value::Int(a.len() as i64)),
                    _ => Err(VietError::type_error(
                        format!("len() not supported for {}", args[0].type_name()),
                        span.line, span.column,
                    )),
                }
            }
            "type_of" | "typeof" => {
                if args.len() != 1 {
                    return Err(VietError::runtime_error("type_of() takes exactly 1 argument".to_string(), span.line, span.column));
                }
                Ok(Value::String(args[0].type_name().to_string()))
            }
            "to_string" => {
                if args.len() != 1 {
                    return Err(VietError::runtime_error("to_string() takes exactly 1 argument".to_string(), span.line, span.column));
                }
                Ok(Value::String(format!("{}", args[0])))
            }
            "to_int" => {
                if args.len() != 1 {
                    return Err(VietError::runtime_error("to_int() takes exactly 1 argument".to_string(), span.line, span.column));
                }
                match &args[0] {
                    Value::Int(n) => Ok(Value::Int(*n)),
                    Value::Float(f) => Ok(Value::Int(*f as i64)),
                    Value::String(s) => s.parse::<i64>()
                        .map(Value::Int)
                        .map_err(|_| VietError::runtime_error(
                            format!("Cannot convert '{}' to Int", s),
                            span.line, span.column,
                        )),
                    Value::Bool(b) => Ok(Value::Int(if *b { 1 } else { 0 })),
                    _ => Err(VietError::type_error(
                        format!("Cannot convert {} to Int", args[0].type_name()),
                        span.line, span.column,
                    )),
                }
            }
            "to_float" => {
                if args.len() != 1 {
                    return Err(VietError::runtime_error("to_float() takes exactly 1 argument".to_string(), span.line, span.column));
                }
                match &args[0] {
                    Value::Int(n) => Ok(Value::Float(*n as f64)),
                    Value::Float(f) => Ok(Value::Float(*f)),
                    Value::String(s) => s.parse::<f64>()
                        .map(Value::Float)
                        .map_err(|_| VietError::runtime_error(
                            format!("Cannot convert '{}' to Float", s),
                            span.line, span.column,
                        )),
                    _ => Err(VietError::type_error(
                        format!("Cannot convert {} to Float", args[0].type_name()),
                        span.line, span.column,
                    )),
                }
            }
            "push" => {
                if args.len() != 2 {
                    return Err(VietError::runtime_error("push() takes exactly 2 arguments (array, value)".to_string(), span.line, span.column));
                }
                match &args[0] {
                    Value::Array(arr) => {
                        let mut new_arr = arr.clone();
                        new_arr.push(args[1].clone());
                        Ok(Value::Array(new_arr))
                    }
                    _ => Err(VietError::type_error("push() first argument must be an array".to_string(), span.line, span.column)),
                }
            }
            "pop" => {
                if args.len() != 1 {
                    return Err(VietError::runtime_error("pop() takes exactly 1 argument".to_string(), span.line, span.column));
                }
                match &args[0] {
                    Value::Array(arr) => {
                        let mut new_arr = arr.clone();
                        let popped = new_arr.pop().unwrap_or(Value::None);
                        // Returns the popped value
                        Ok(popped)
                    }
                    _ => Err(VietError::type_error("pop() argument must be an array".to_string(), span.line, span.column)),
                }
            }
            "input" => {
                if args.len() != 1 {
                    return Err(VietError::runtime_error("input() takes exactly 1 argument (prompt)".to_string(), span.line, span.column));
                }
                print!("{}", args[0]);
                use std::io::{self, Write};
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                Ok(Value::String(input.trim_end().to_string()))
            }
            "abs" => {
                if args.len() != 1 {
                    return Err(VietError::runtime_error("abs() takes exactly 1 argument".to_string(), span.line, span.column));
                }
                match &args[0] {
                    Value::Int(n) => Ok(Value::Int(n.abs())),
                    Value::Float(f) => Ok(Value::Float(f.abs())),
                    _ => Err(VietError::type_error("abs() requires a numeric argument".to_string(), span.line, span.column)),
                }
            }
            "min" => {
                if args.len() != 2 {
                    return Err(VietError::runtime_error("min() takes exactly 2 arguments".to_string(), span.line, span.column));
                }
                match (&args[0], &args[1]) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Int(*a.min(b))),
                    (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.min(*b))),
                    _ => Err(VietError::type_error("min() requires numeric arguments".to_string(), span.line, span.column)),
                }
            }
            "max" => {
                if args.len() != 2 {
                    return Err(VietError::runtime_error("max() takes exactly 2 arguments".to_string(), span.line, span.column));
                }
                match (&args[0], &args[1]) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Int(*a.max(b))),
                    (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.max(*b))),
                    _ => Err(VietError::type_error("max() requires numeric arguments".to_string(), span.line, span.column)),
                }
            }

            // === Standard Library (routed to stdlib module) ===

            // std.io
            "file_read" => crate::stdlib::builtin_file_read(args, span.line, span.column),
            "file_write" => crate::stdlib::builtin_file_write(args, span.line, span.column),
            "file_append" => crate::stdlib::builtin_file_append(args, span.line, span.column),
            "file_exists" => crate::stdlib::builtin_file_exists(args, span.line, span.column),
            "file_delete" => crate::stdlib::builtin_file_delete(args, span.line, span.column),
            "dir_list" => crate::stdlib::builtin_dir_list(args, span.line, span.column),
            "dir_create" => crate::stdlib::builtin_dir_create(args, span.line, span.column),

            // std.json
            "json_parse" => crate::stdlib::builtin_json_parse(args, span.line, span.column),
            "json_stringify" => crate::stdlib::builtin_json_stringify(args, span.line, span.column),

            // std.env
            "env_get" => crate::stdlib::builtin_env_get(args, span.line, span.column),
            "env_set" => crate::stdlib::builtin_env_set(args, span.line, span.column),
            "env_all" => crate::stdlib::builtin_env_all(args, span.line, span.column),

            // std.time
            "time_now" => crate::stdlib::builtin_time_now(args, span.line, span.column),
            "time_now_ms" => crate::stdlib::builtin_time_now_ms(args, span.line, span.column),
            "sleep" => crate::stdlib::builtin_time_sleep(args, span.line, span.column),
            "timer_start" => crate::stdlib::builtin_time_measure(args, span.line, span.column),

            // std.crypto
            "sha256" => crate::stdlib::builtin_hash_sha256(args, span.line, span.column),
            "sha1" => crate::stdlib::builtin_sha1(args, span.line, span.column),
            "ws_accept_key" => crate::stdlib::builtin_ws_accept_key(args, span.line, span.column),
            "uuid" => crate::stdlib::builtin_uuid(args, span.line, span.column),
            "base64_encode" => crate::stdlib::builtin_base64_encode(args, span.line, span.column),
            "random_int" => crate::stdlib::builtin_random_int(args, span.line, span.column),

            // std.log
            "log_debug" => crate::stdlib::builtin_log(args, span.line, span.column, "DEBUG"),
            "log_info" => crate::stdlib::builtin_log(args, span.line, span.column, "INFO"),
            "log_warn" => crate::stdlib::builtin_log(args, span.line, span.column, "WARN"),
            "log_error" => crate::stdlib::builtin_log(args, span.line, span.column, "ERROR"),

            // std.collections
            "map_new" => crate::stdlib::builtin_map_new(args, span.line, span.column),
            "map_set" => crate::stdlib::builtin_map_set(args, span.line, span.column),
            "map_get" => crate::stdlib::builtin_map_get(args, span.line, span.column),
            "map_has" => crate::stdlib::builtin_map_has(args, span.line, span.column),
            "map_keys" => crate::stdlib::builtin_map_keys(args, span.line, span.column),
            "map_values" => crate::stdlib::builtin_map_values(args, span.line, span.column),
            "map_remove" => crate::stdlib::builtin_map_remove(args, span.line, span.column),

            // std.http
            "http_listen" => self.eval_http_listen(args, span),

            // std.db
            "db_query" => crate::stdlib::builtin_db_query(args, span.line, span.column),
            "db_table" => crate::stdlib::builtin_db_table(args, span.line, span.column),

            // Concurrency
            "spawn" => self.eval_spawn(args, span),
            "channel" | "channel_new" => crate::stdlib::builtin_channel(args, span.line, span.column),
            "channel_send" => crate::stdlib::builtin_channel_send(args, span.line, span.column),
            "channel_recv" => crate::stdlib::builtin_channel_recv(args, span.line, span.column),
            "channel_try_recv" => crate::stdlib::builtin_channel_try_recv(args, span.line, span.column),
            "channel_close" => crate::stdlib::builtin_channel_close(args, span.line, span.column),
            "thread_sleep" => crate::stdlib::builtin_thread_sleep(args, span.line, span.column),
            "mutex_new" => crate::stdlib::builtin_mutex_new(args, span.line, span.column),

            // Utility
            "assert" => crate::stdlib::builtin_assert(args, span.line, span.column),
            "exit" => crate::stdlib::builtin_exit(args, span.line, span.column),
            "format" => crate::stdlib::builtin_format(args, span.line, span.column),
            "range" => crate::stdlib::builtin_range(args, span.line, span.column),

            // String character operations
            "char_at" => crate::stdlib::builtin_char_at(args, span.line, span.column),
            "char_code" => crate::stdlib::builtin_char_code(args, span.line, span.column),
            "from_char_code" => crate::stdlib::builtin_from_char_code(args, span.line, span.column),
            "substring" => crate::stdlib::builtin_substring(args, span.line, span.column),
            "str_repeat" => crate::stdlib::builtin_str_repeat(args, span.line, span.column),
            "parse_int" => crate::stdlib::builtin_parse_int(args, span.line, span.column),
            "parse_float" => crate::stdlib::builtin_parse_float(args, span.line, span.column),

            // Array operations
            "sort" => crate::stdlib::builtin_array_sort(args, span.line, span.column),
            "slice" => crate::stdlib::builtin_array_slice(args, span.line, span.column),
            "index_of" => crate::stdlib::builtin_array_index_of(args, span.line, span.column),
            "flat" => crate::stdlib::builtin_array_flat(args, span.line, span.column),

            // Error handling & Reflection
            "throw" => crate::stdlib::builtin_throw(args, span.line, span.column),
            "is_error" => crate::stdlib::builtin_is_error(args, span.line, span.column),

            // System & Timers
            "get_args" => crate::stdlib::builtin_args(args, span.line, span.column),
            "platform" => crate::stdlib::builtin_platform(args, span.line, span.column),
            "arch" => crate::stdlib::builtin_arch(args, span.line, span.column),
            "sleep_ms" => crate::stdlib::builtin_sleep_ms(args, span.line, span.column),
            "time_now_us" => crate::stdlib::builtin_time_now_us(args, span.line, span.column),
            "tcp_ping" => crate::stdlib::builtin_tcp_ping(args, span.line, span.column),
            "tcp_send" => crate::stdlib::builtin_tcp_send(args, span.line, span.column),
            "udp_send" => crate::stdlib::builtin_udp_send(args, span.line, span.column),
            "str_split_lines" => crate::stdlib::builtin_str_split_lines(args, span.line, span.column),
            "system_cmd" => crate::stdlib::builtin_system_cmd(args, span.line, span.column),
            "url_encode" => crate::stdlib::builtin_url_encode(args, span.line, span.column),
            "url_decode" => crate::stdlib::builtin_url_decode(args, span.line, span.column),
            "to_uppercase" => crate::stdlib::builtin_to_uppercase(args, span.line, span.column),
            "to_lowercase" => crate::stdlib::builtin_to_lowercase(args, span.line, span.column),
            "trim" => crate::stdlib::builtin_trim(args, span.line, span.column),
            "starts_with" => crate::stdlib::builtin_starts_with(args, span.line, span.column),
            "ends_with" => crate::stdlib::builtin_ends_with(args, span.line, span.column),
            "contains" => crate::stdlib::builtin_contains(args, span.line, span.column),
            "hmac_sha256" => crate::stdlib::builtin_hmac_sha256(args, span.line, span.column),
            "hmac_sha512" => crate::stdlib::builtin_hmac_sha512(args, span.line, span.column),
            "encrypt_secret" => crate::stdlib::builtin_encrypt_secret(args, span.line, span.column),
            "decrypt_secret" => crate::stdlib::builtin_decrypt_secret(args, span.line, span.column),
            "ip_in_cidr" => crate::stdlib::builtin_ip_in_cidr(args, span.line, span.column),
            "hex_encode" => crate::stdlib::builtin_hex_encode(args, span.line, span.column),
            "hex_decode" => crate::stdlib::builtin_hex_decode(args, span.line, span.column),
            "crypto_random_hex" => crate::stdlib::builtin_crypto_random_hex(args, span.line, span.column),
            "uuid_v4" => crate::stdlib::builtin_uuid_v4(args, span.line, span.column),
            "time_unix_ms" => crate::stdlib::builtin_time_unix_ms(args, span.line, span.column),
            "sqlite_open" | "builtin_sqlite_open" => crate::stdlib::builtin_sqlite_open(args, span.line, span.column),
            "sqlite_exec" | "builtin_sqlite_exec" => crate::stdlib::builtin_sqlite_exec(args, span.line, span.column),
            "sqlite_execute" | "builtin_sqlite_execute" => crate::stdlib::builtin_sqlite_execute(args, span.line, span.column),
            "sqlite_query" | "builtin_sqlite_query" => crate::stdlib::builtin_sqlite_query(args, span.line, span.column),
            "sqlite_close" | "builtin_sqlite_close" => crate::stdlib::builtin_sqlite_close(args, span.line, span.column),

            "mysql_connect" | "builtin_mysql_connect" => crate::stdlib::builtin_mysql_connect(args, span.line, span.column),
            "mysql_exec" | "builtin_mysql_exec" => crate::stdlib::builtin_mysql_exec(args, span.line, span.column),
            "mysql_execute" | "builtin_mysql_execute" => crate::stdlib::builtin_mysql_execute(args, span.line, span.column),
            "mysql_query" | "builtin_mysql_query" => crate::stdlib::builtin_mysql_query(args, span.line, span.column),
            "mysql_close" | "builtin_mysql_close" => crate::stdlib::builtin_mysql_close(args, span.line, span.column),

            "http_fetch" | "builtin_http_fetch" => crate::stdlib::builtin_http_fetch(args, span.line, span.column),
            "csv_parse" | "builtin_csv_parse" => crate::stdlib::builtin_csv_parse(args, span.line, span.column),
            "csv_stringify" | "builtin_csv_stringify" => crate::stdlib::builtin_csv_stringify(args, span.line, span.column),
            "ws_enable" | "builtin_ws_enable" => crate::stdlib::builtin_ws_enable(args, span.line, span.column),
            "ws_broadcast" | "builtin_ws_broadcast" => crate::stdlib::builtin_ws_broadcast(args, span.line, span.column),
            "html_escape" | "builtin_html_escape" => crate::stdlib::builtin_html_escape(args, span.line, span.column),

            _ => Err(VietError::runtime_error(
                format!("Unknown builtin function: '{}'", name),
                span.line, span.column,
            )),
        }
    }

    // ========================================
    // Pattern Matching
    // ========================================

    fn match_pattern(&self, pattern: &Pattern, value: &Value) -> Option<Vec<(String, Value)>> {
        match pattern {
            Pattern::Wildcard => Some(Vec::new()),
            Pattern::Variable(name) => {
                Some(vec![(name.clone(), value.clone())])
            }
            Pattern::Literal(expr) => {
                let pattern_val = match expr {
                    Expression::IntLiteral { value, .. } => Value::Int(*value),
                    Expression::FloatLiteral { value, .. } => Value::Float(*value),
                    Expression::StringLiteral { value, .. } => Value::String(value.clone()),
                    Expression::BoolLiteral { value, .. } => Value::Bool(*value),
                    _ => return None,
                };
                if &pattern_val == value {
                    Some(Vec::new())
                } else {
                    None
                }
            }
            Pattern::EnumVariant { name, fields } => {
                match value {
                    Value::EnumVariant { variant, fields: val_fields, .. } => {
                        if variant == name && fields.len() == val_fields.len() {
                            let mut bindings = Vec::new();
                            for (pat, val) in fields.iter().zip(val_fields.iter()) {
                                match self.match_pattern(pat, val) {
                                    Some(mut b) => bindings.append(&mut b),
                                    None => return None,
                                }
                            }
                            Some(bindings)
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
        }
    }

    // ========================================
    // Generic HTTP Server Engine
    // ========================================

    fn eval_http_listen(&mut self, args: &[Value], span: &Span) -> VietResult<Value> {
        if args.is_empty() {
            return Err(VietError::runtime_error("http_listen() takes 1+ arguments (port_or_addr_or_config, handler_fn)".into(), span.line, span.column));
        }

        let mut bind_ip = "0.0.0.0".to_string();
        let mut port = 8080u16;
        let mut protocol = "HTTP/1.1, HTTP/2, HTTP/3 (Alt-Svc)".to_string();
        let mut handler = None;

        match &args[0] {
            Value::Int(n) => {
                port = *n as u16;
                if args.len() >= 2 {
                    handler = Some(args[1].clone());
                }
            }
            Value::String(s) => {
                if s.contains(':') {
                    let parts: Vec<&str> = s.split(':').collect();
                    bind_ip = parts[0].to_string();
                    port = parts[1].parse::<u16>().unwrap_or(8080);
                } else {
                    port = s.parse::<u16>().unwrap_or(8080);
                }
                if args.len() >= 2 {
                    handler = Some(args[1].clone());
                }
            }
            Value::Struct { fields, .. } => {
                if let Some(Value::String(a)) = fields.get("addr") {
                    if a.contains(':') {
                        let parts: Vec<&str> = a.split(':').collect();
                        bind_ip = parts[0].to_string();
                        port = parts[1].parse::<u16>().unwrap_or(8080);
                    }
                }
                if let Some(Value::Int(p)) = fields.get("port") {
                    port = *p as u16;
                }
                if let Some(Value::String(pr)) = fields.get("protocol") {
                    protocol = pr.clone();
                }
                if args.len() >= 2 {
                    handler = Some(args[1].clone());
                }
            }
            _ => return Err(VietError::type_error("http_listen() first argument must be Int, String or Config Map".into(), span.line, span.column)),
        }

        let addr = format!("{}:{}", bind_ip, port);
        eprintln!("\x1b[32m[VietLang HTTP Engine]\x1b[0m Listening on http://{}:{} [Protocols: {}]", bind_ip, port, protocol);

        let listener = TcpListener::bind(&addr).map_err(|e|
            VietError::runtime_error(format!("Cannot bind to {}: {}", addr, e), span.line, span.column)
        )?;

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let client_ip = stream.peer_addr().map(|a| a.to_string()).unwrap_or_else(|_| "127.0.0.1".to_string());
                    let mut reader = std::io::BufReader::new(&stream);
                    let mut request_line = String::new();
                    let _ = reader.read_line(&mut request_line);

                    let parts: Vec<&str> = request_line.trim().split_whitespace().collect();
                    let method = parts.first().unwrap_or(&"GET").to_string();
                    let full_path = parts.get(1).unwrap_or(&"/").to_string();
                    let http_proto = parts.get(2).unwrap_or(&"HTTP/1.1").to_string();

                    let path_parts: Vec<&str> = full_path.split('?').collect();
                    let path = path_parts[0].to_string();
                    let query_str = path_parts.get(1).unwrap_or(&"").to_string();

                    // Read headers
                    let mut headers_map = HashMap::new();
                    let mut content_length = 0usize;
                    loop {
                        let mut header_line = String::new();
                        let _ = reader.read_line(&mut header_line);
                        let header_line = header_line.trim().to_string();
                        if header_line.is_empty() { break; }
                        if let Some(pos) = header_line.find(':') {
                            let key = header_line[..pos].trim().to_lowercase();
                            let val = header_line[pos+1..].trim().to_string();
                            if key == "content-length" {
                                content_length = val.parse().unwrap_or(0);
                            }
                            headers_map.insert(key, Value::String(val));
                        }
                    }

                    // Read body
                    let mut body = String::new();
                    if content_length > 0 {
                        let mut buf = vec![0u8; content_length];
                        let _ = reader.read_exact(&mut buf);
                        body = String::from_utf8_lossy(&buf).to_string();
                    }

                    // WebSocket RFC 6455 Handshake Upgrade (Only if explicitly enabled by user via std.ws)
                    let ws_is_active = crate::stdlib::WS_ENABLED.load(std::sync::atomic::Ordering::SeqCst);
                    let ws_path_matches = {
                        let guard = crate::stdlib::WS_ENDPOINT.lock().unwrap();
                        match &*guard {
                            Some(ep) => ep == &path,
                            None => true,
                        }
                    };

                    if ws_is_active && ws_path_matches && (headers_map.contains_key("sec-websocket-key") || headers_map.get("upgrade").map(|v| match v { Value::String(s) => s.to_lowercase().contains("websocket"), _ => false }).unwrap_or(false)) {
                        let ws_key = match headers_map.get("sec-websocket-key") {
                            Some(Value::String(s)) => s.clone(),
                            _ => "".to_string(),
                        };
                        if !ws_key.is_empty() {
                            let concat = format!("{}258EAFA5-E914-47DA-95CA-C5AB0DC85B11", ws_key);
                            let digest = crate::stdlib::sha1_digest(concat.as_bytes());
                            let accept_key = crate::stdlib::base64_encode_bytes(&digest);
                            let handshake_resp = format!(
                                "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\nServer: VietLang-WebSocket/1.0\r\n\r\n",
                                accept_key
                            );
                            let _ = stream.write_all(handshake_resp.as_bytes());
                            let _ = stream.flush();

                            let welcome = format!("{{\"type\":\"connection_established\",\"message\":\"VietLang Real-Time WebSocket Connected\",\"timestamp\":{}}}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs());
                            let welcome_frame = crate::stdlib::encode_ws_text_frame(&welcome);
                            let _ = stream.write_all(&welcome_frame);
                            let _ = stream.flush();

                            let stream_arc = std::sync::Arc::new(std::sync::Mutex::new(stream));
                            crate::stdlib::register_ws_client(stream_arc.clone());

                            std::thread::spawn(move || {
                                let mut buf = [0u8; 4096];
                                loop {
                                    let res = if let Ok(mut s) = stream_arc.lock() {
                                        s.read(&mut buf)
                                    } else {
                                        break;
                                    };
                                    match res {
                                        Ok(0) | Err(_) => break,
                                        Ok(_) => {}
                                    }
                                }
                            });

                            continue;
                        }
                    }

                    // CORS OPTIONS Preflight
                    if method == "OPTIONS" {
                        let preflight = format!(
                            "HTTP/1.1 204 No Content\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE, PATCH, OPTIONS\r\nAccess-Control-Allow-Headers: *\r\nAlt-Svc: h3=\":{}\"; ma=86400, h2=\":{}\"\r\nServer: VietLang/0.1.0\r\nConnection: close\r\n\r\n",
                            port, port
                        );
                        let _ = stream.write_all(preflight.as_bytes());
                        let _ = stream.flush();
                        continue;
                    }

                    // Pure VietLang Request Dispatching
                    let (status_code, content_type, response_body) = if let Some(ref h) = handler {
                        let mut req_map = HashMap::new();
                        req_map.insert("method".to_string(), Value::String(method.clone()));
                        req_map.insert("path".to_string(), Value::String(path.clone()));
                        req_map.insert("query".to_string(), Value::String(query_str));
                        req_map.insert("protocol".to_string(), Value::String(http_proto));
                        req_map.insert("client_ip".to_string(), Value::String(client_ip));
                        req_map.insert("headers".to_string(), Value::Struct { type_name: "Map".to_string(), fields: headers_map });
                        req_map.insert("body".to_string(), Value::String(body));

                        let req_val = Value::Struct { type_name: "Map".to_string(), fields: req_map };

                        match self.call_function(h, &[req_val], span) {
                            Ok(res_val) => {
                                match &res_val {
                                    Value::String(s) => {
                                        let ct = if s.starts_with("<!DOCTYPE") || s.starts_with("<html") {
                                            "text/html; charset=utf-8".to_string()
                                        } else {
                                            "application/json; charset=utf-8".to_string()
                                        };
                                        (200, ct, s.clone())
                                    }
                                    Value::Struct { fields: m, .. } => {
                                        let code = if let Some(Value::Int(c)) = m.get("status_code") {
                                            *c as usize
                                        } else {
                                            200
                                        };
                                        let ct = if let Some(Value::String(c)) = m.get("content_type") {
                                            c.clone()
                                        } else {
                                            "application/json; charset=utf-8".to_string()
                                        };
                                        let b = if let Some(Value::String(body_val)) = m.get("body") {
                                            body_val.clone()
                                        } else {
                                            match crate::stdlib::builtin_json_stringify(&[res_val.clone()], 0, 0) {
                                                Ok(Value::String(s)) => s,
                                                _ => format!("{}", res_val),
                                            }
                                        };
                                        (code, ct, b)
                                    }
                                    other => {
                                        let b = match crate::stdlib::builtin_json_stringify(&[other.clone()], 0, 0) {
                                            Ok(Value::String(s)) => s,
                                            _ => format!("{}", other),
                                        };
                                        (200, "application/json; charset=utf-8".to_string(), b)
                                    }
                                }
                            }
                            Err(e) => {
                                (500, "application/json; charset=utf-8".to_string(), format!("{{\"error\":\"Internal Server Error: {}\"}}", e))
                            }
                        }
                    } else {
                        (200, "application/json; charset=utf-8".to_string(), format!("{{\"status\":\"OK\",\"path\":\"{}\",\"server\":\"VietLang/0.1.0\"}}", path))
                    };

                    let response = format!(
                        "HTTP/1.1 {} OK\r\nContent-Type: {}\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE, PATCH, OPTIONS\r\nAccess-Control-Allow-Headers: *\r\nX-Content-Type-Options: nosniff\r\nX-Frame-Options: SAMEORIGIN\r\nX-XSS-Protection: 1; mode=block\r\nReferrer-Policy: strict-origin-when-cross-origin\r\nPermissions-Policy: camera=(), microphone=(), geolocation=()\r\nAlt-Svc: h3=\":{}\"; ma=86400, h2=\":{}\"\r\nServer: VietLang-Enterprise/0.1.0\r\nX-Powered-By: VietLang-Backend\r\nConnection: close\r\n\r\n{}",
                        status_code, content_type, response_body.len(), port, port, response_body
                    );
                    let _ = stream.write_all(response.as_bytes());
                    let _ = stream.flush();
                    eprintln!("\x1b[36m[HTTP API]\x1b[0m {} {} -> {}", method, path, status_code);
                }
                Err(e) => {
                    eprintln!("\x1b[31m[HTTP Error]\x1b[0m {}", e);
                }
            }
        }

        Ok(Value::None)
    }

    fn eval_spawn(&mut self, args: &[Value], span: &Span) -> VietResult<Value> {
        if args.is_empty() {
            return Err(VietError::runtime_error("spawn() takes at least 1 function or closure argument".into(), span.line, span.column));
        }
        let func = args[0].clone();
        let call_args = args[1..].to_vec();

        let mut sub_interpreter = self.clone();
        let span_clone = span.clone();

        static TASK_ID_GEN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        let task_id = TASK_ID_GEN.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        std::thread::spawn(move || {
            let _ = sub_interpreter.call_function(&func, &call_args, &span_clone);
        });

        let mut fields = HashMap::new();
        fields.insert("id".to_string(), Value::Int(task_id as i64));
        fields.insert("type".to_string(), Value::String("Task".to_string()));
        Ok(Value::Struct {
            type_name: "Task".to_string(),
            fields,
        })
    }
}
