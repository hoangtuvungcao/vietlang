//! Gradual semantic analysis for VietLang.
//!
//! The analyzer rejects contradictions that can be proven from source-level
//! declarations. Values supplied by imports or dynamic native APIs remain
//! `Unknown` until module signatures and a typed IR are available.

use std::collections::{HashMap, HashSet};

use crate::{
    error::{VietError, VietResult},
    lexer::token::Span,
    parser::ast::{
        BinaryOperator, Expression, MatchArm, Parameter, Pattern, Program, Statement,
        TypeAnnotation, UnaryOperator,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Type {
    Unknown,
    None,
    Int,
    Float,
    String,
    Bool,
    Array(Box<Type>),
    Nullable(Box<Type>),
    Named(String),
    Generic(String, Vec<Type>),
    Function(FunctionType),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FunctionType {
    params: Vec<Type>,
    required: usize,
    return_type: Box<Type>,
    variadic: bool,
}

#[derive(Debug, Clone)]
struct Binding {
    ty: Type,
    mutable: bool,
}

#[derive(Debug, Clone)]
struct StructType {
    fields: HashMap<String, Type>,
}

#[derive(Debug, Clone)]
struct EnumType {
    variants: HashMap<String, Vec<Type>>,
}

#[derive(Clone)]
pub struct SemanticAnalyzer {
    scopes: Vec<HashMap<String, Binding>>,
    structs: HashMap<String, StructType>,
    enums: HashMap<String, EnumType>,
    methods: HashMap<String, HashMap<String, FunctionType>>,
    current_return: Vec<Type>,
    observed_returns: Vec<Vec<Type>>,
    loop_depth: usize,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = Self {
            scopes: vec![HashMap::new()],
            structs: HashMap::new(),
            enums: HashMap::new(),
            methods: HashMap::new(),
            current_return: Vec::new(),
            observed_returns: Vec::new(),
            loop_depth: 0,
        };
        analyzer.register_builtins();
        analyzer
    }

    pub fn analyze(&mut self, program: &Program) -> VietResult<()> {
        self.predeclare(&program.statements)?;
        self.analyze_block(&program.statements)
    }

    /// Returns the canonical type assigned to a top-level symbol.  This is the
    /// stable bridge from semantic analysis into the typed IR and language
    /// tooling; callers never need access to analyzer-internal type nodes.
    pub fn symbol_type(&self, name: &str) -> Option<String> {
        self.scopes
            .first()?
            .get(name)
            .map(|binding| display_type(&binding.ty))
    }

    fn register_builtins(&mut self) {
        self.define_builtin("len", vec![Type::Unknown], 1, Type::Int, false);
        self.define_builtin("type_of", vec![Type::Unknown], 1, Type::String, false);
        self.define_builtin("to_string", vec![Type::Unknown], 1, Type::String, false);
        self.define_builtin("to_int", vec![Type::Unknown], 1, Type::Int, false);
        self.define_builtin("to_float", vec![Type::Unknown], 1, Type::Float, false);
        self.define_builtin("println", vec![], 0, Type::None, true);
        self.define_builtin("print", vec![], 0, Type::None, true);
        self.define_builtin("assert", vec![], 1, Type::None, true);
        self.define_builtin("map_new", vec![], 0, Type::Named("Map".into()), false);
        self.define_builtin(
            "map_set",
            vec![Type::Named("Map".into()), Type::String, Type::Unknown],
            3,
            Type::Named("Map".into()),
            false,
        );
        self.define_builtin(
            "map_get",
            vec![Type::Named("Map".into()), Type::String],
            2,
            Type::Unknown,
            false,
        );
        self.define_builtin(
            "push",
            vec![Type::Array(Box::new(Type::Unknown)), Type::Unknown],
            2,
            Type::Array(Box::new(Type::Unknown)),
            false,
        );
        self.define_builtin("http_listen", vec![], 1, Type::None, true);
        self.define_builtin(
            "http_fetch",
            vec![],
            1,
            Type::Named("HttpResponse".into()),
            true,
        );
    }

    fn define_builtin(
        &mut self,
        name: &str,
        params: Vec<Type>,
        required: usize,
        return_type: Type,
        variadic: bool,
    ) {
        self.scopes[0].insert(
            name.into(),
            Binding {
                ty: Type::Function(FunctionType {
                    params,
                    required,
                    return_type: Box::new(return_type),
                    variadic,
                }),
                mutable: false,
            },
        );
    }

    fn predeclare(&mut self, statements: &[Statement]) -> VietResult<()> {
        for statement in statements {
            match statement {
                Statement::Struct {
                    name, fields, span, ..
                } => {
                    if self.structs.contains_key(name) || self.enums.contains_key(name) {
                        return Err(name_error(
                            format!("Type '{}' is already declared", name),
                            span,
                        ));
                    }
                    let mut declared = HashMap::new();
                    for field in fields {
                        if declared
                            .insert(field.name.clone(), type_from_annotation(&field.type_ann))
                            .is_some()
                        {
                            return Err(name_error(
                                format!("Duplicate field '{}.{}'", name, field.name),
                                span,
                            ));
                        }
                    }
                    self.structs
                        .insert(name.clone(), StructType { fields: declared });
                }
                Statement::Enum {
                    name,
                    variants,
                    span,
                    ..
                } => {
                    if self.structs.contains_key(name) || self.enums.contains_key(name) {
                        return Err(name_error(
                            format!("Type '{}' is already declared", name),
                            span,
                        ));
                    }
                    let mut declared = HashMap::new();
                    for variant in variants {
                        let field_types: Vec<_> =
                            variant.fields.iter().map(type_from_annotation).collect();
                        if declared
                            .insert(variant.name.clone(), field_types.clone())
                            .is_some()
                        {
                            return Err(name_error(
                                format!("Duplicate enum variant '{}.{}'", name, variant.name),
                                span,
                            ));
                        }
                        let variant_type = if field_types.is_empty() {
                            Type::Named(name.clone())
                        } else {
                            Type::Function(FunctionType {
                                required: field_types.len(),
                                params: field_types,
                                return_type: Box::new(Type::Named(name.clone())),
                                variadic: false,
                            })
                        };
                        self.define_unique(&variant.name, variant_type, false, span)?;
                    }
                    self.enums
                        .insert(name.clone(), EnumType { variants: declared });
                }
                Statement::Function {
                    name,
                    params,
                    return_type,
                    span,
                    ..
                } => {
                    let signature = self.function_type(params, return_type.as_ref(), span)?;
                    self.define_unique(name, Type::Function(signature), false, span)?;
                }
                Statement::Impl {
                    type_name,
                    methods,
                    span,
                } => {
                    let mut declarations = self.methods.remove(type_name).unwrap_or_default();
                    for method in methods {
                        let Statement::Function {
                            name,
                            params,
                            return_type,
                            span: method_span,
                            ..
                        } = method
                        else {
                            return Err(type_error("impl blocks may contain only functions", span));
                        };
                        if params.first().map(|param| param.name.as_str()) != Some("self") {
                            return Err(type_error(
                                format!(
                                    "Method '{}.{}' must declare 'self' as its first parameter",
                                    type_name, name
                                ),
                                method_span,
                            ));
                        }
                        let mut signature =
                            self.function_type(params, return_type.as_ref(), method_span)?;
                        signature.params.remove(0);
                        signature.required = signature.required.saturating_sub(1);
                        if declarations.insert(name.clone(), signature).is_some() {
                            return Err(name_error(
                                format!("Duplicate method '{}.{}'", type_name, name),
                                method_span,
                            ));
                        }
                    }
                    self.methods.insert(type_name.clone(), declarations);
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn function_type(
        &self,
        params: &[Parameter],
        return_type: Option<&TypeAnnotation>,
        span: &Span,
    ) -> VietResult<FunctionType> {
        let mut names = HashSet::new();
        let mut saw_default = false;
        let mut required = 0;
        for param in params {
            if !names.insert(&param.name) {
                return Err(name_error(
                    format!("Duplicate parameter '{}'", param.name),
                    span,
                ));
            }
            if param.default.is_some() {
                saw_default = true;
            } else {
                if saw_default {
                    return Err(type_error(
                        "Required parameters cannot follow default parameters",
                        span,
                    ));
                }
                required += 1;
            }
        }
        Ok(FunctionType {
            params: params
                .iter()
                .map(|param| {
                    param
                        .type_ann
                        .as_ref()
                        .map(type_from_annotation)
                        .unwrap_or(Type::Unknown)
                })
                .collect(),
            required,
            return_type: Box::new(
                return_type
                    .map(type_from_annotation)
                    .unwrap_or(Type::Unknown),
            ),
            variadic: false,
        })
    }

    fn analyze_block(&mut self, statements: &[Statement]) -> VietResult<()> {
        for statement in statements {
            self.analyze_statement(statement)?;
        }
        Ok(())
    }

    fn analyze_statement(&mut self, statement: &Statement) -> VietResult<()> {
        match statement {
            Statement::Let {
                name,
                mutable,
                type_ann,
                value,
                span,
            } => {
                let actual = self.expression_type(value)?;
                let declared = type_ann
                    .as_ref()
                    .map(type_from_annotation)
                    .unwrap_or_else(|| actual.clone());
                ensure_assignable(
                    &declared,
                    &actual,
                    span,
                    &format!("initializer for '{}'", name),
                )?;
                self.current_scope_mut().insert(
                    name.clone(),
                    Binding {
                        ty: declared,
                        mutable: *mutable,
                    },
                );
            }
            Statement::Assignment {
                target,
                value,
                span,
            } => {
                let value_type = self.expression_type(value)?;
                match target {
                    Expression::Identifier { name, .. } => {
                        let binding = self.lookup(name).ok_or_else(|| {
                            name_error(
                                format!("Cannot assign undeclared variable '{}'", name),
                                span,
                            )
                        })?;
                        if !binding.mutable {
                            return Err(type_error(
                                format!("Cannot assign immutable variable '{}'", name),
                                span,
                            ));
                        }
                        ensure_assignable(
                            &binding.ty,
                            &value_type,
                            span,
                            &format!("assignment to '{}'", name),
                        )?;
                    }
                    Expression::FieldAccess { object, field, .. } => {
                        if let Expression::Identifier { name, .. } = object.as_ref() {
                            let binding = self.lookup(name).ok_or_else(|| {
                                name_error(
                                    format!("Cannot assign through undeclared variable '{}'", name),
                                    span,
                                )
                            })?;
                            if !binding.mutable {
                                return Err(type_error(
                                    format!(
                                        "Cannot mutate field through immutable variable '{}'",
                                        name
                                    ),
                                    span,
                                ));
                            }
                        }
                        let object_type = self.expression_type(object)?;
                        if let Type::Named(name) = object_type {
                            if let Some(struct_type) = self.structs.get(&name) {
                                let expected = struct_type.fields.get(field).ok_or_else(|| {
                                    type_error(
                                        format!("Struct '{}' has no field '{}'", name, field),
                                        span,
                                    )
                                })?;
                                ensure_assignable(
                                    expected,
                                    &value_type,
                                    span,
                                    &format!("field '{}.{}'", name, field),
                                )?;
                            }
                        }
                    }
                    _ => {
                        return Err(type_error(
                            "Assignment target must be a variable or struct field",
                            span,
                        ))
                    }
                }
            }
            Statement::Expression { expr, .. } => {
                self.expression_type(expr)?;
            }
            Statement::Function {
                params,
                return_type,
                body,
                span,
                ..
            } => {
                self.push_scope();
                for param in params {
                    let ty = param
                        .type_ann
                        .as_ref()
                        .map(type_from_annotation)
                        .unwrap_or(Type::Unknown);
                    if let Some(default) = &param.default {
                        let actual = self.expression_type(default)?;
                        ensure_assignable(
                            &ty,
                            &actual,
                            span,
                            &format!("default value for '{}'", param.name),
                        )?;
                    }
                    self.current_scope_mut()
                        .insert(param.name.clone(), Binding { ty, mutable: false });
                }
                let expected_return = return_type
                    .as_ref()
                    .map(type_from_annotation)
                    .unwrap_or(Type::Unknown);
                self.current_return.push(expected_return);
                self.observed_returns.push(Vec::new());
                self.predeclare(body)?;
                let result = self.analyze_block(body);
                self.current_return.pop();
                self.observed_returns.pop();
                self.pop_scope();
                result?;
                if return_type.is_some() && !block_guarantees_return(body) {
                    return Err(type_error(
                        "Function with an explicit return type may fall through without returning",
                        span,
                    ));
                }
            }
            Statement::Return { value, span } => {
                let expected =
                    self.current_return.last().cloned().ok_or_else(|| {
                        type_error("return is only valid inside a function", span)
                    })?;
                let actual = match value {
                    Some(value) => self.expression_type(value)?,
                    None => Type::None,
                };
                ensure_assignable(&expected, &actual, span, "return value")?;
                if let Some(returns) = self.observed_returns.last_mut() {
                    returns.push(actual);
                }
            }
            Statement::If {
                condition,
                then_body,
                else_body,
                span,
            } => {
                let condition_type = self.expression_type(condition)?;
                ensure_condition(&condition_type, span)?;
                self.with_scope(then_body)?;
                if let Some(body) = else_body {
                    self.with_scope(body)?;
                }
            }
            Statement::While {
                condition,
                body,
                span,
            } => {
                let condition_type = self.expression_type(condition)?;
                ensure_condition(&condition_type, span)?;
                self.loop_depth += 1;
                let result = self.with_scope(body);
                self.loop_depth -= 1;
                result?;
            }
            Statement::For {
                variable,
                iterable,
                body,
                span,
            } => {
                let iterable_type = self.expression_type(iterable)?;
                let item_type = match iterable_type {
                    Type::Array(inner) => *inner,
                    Type::String => Type::String,
                    Type::Unknown => Type::Unknown,
                    other => {
                        return Err(type_error(
                            format!("Cannot iterate over {}", display_type(&other)),
                            span,
                        ))
                    }
                };
                self.push_scope();
                self.current_scope_mut().insert(
                    variable.clone(),
                    Binding {
                        ty: item_type,
                        mutable: false,
                    },
                );
                self.loop_depth += 1;
                let result = self.analyze_block(body);
                self.loop_depth -= 1;
                self.pop_scope();
                result?;
            }
            Statement::Break { span } | Statement::Continue { span } => {
                if self.loop_depth == 0 {
                    return Err(type_error(
                        "break/continue is only valid inside a loop",
                        span,
                    ));
                }
            }
            Statement::Struct { .. } | Statement::Enum { .. } | Statement::Import { .. } => {}
            Statement::Impl {
                type_name,
                methods,
                span,
            } => {
                if !self.structs.contains_key(type_name) && !self.enums.contains_key(type_name) {
                    return Err(type_error(
                        format!("Cannot implement unknown type '{}'", type_name),
                        span,
                    ));
                }
                for method in methods {
                    let Statement::Function {
                        params,
                        return_type,
                        body,
                        span,
                        ..
                    } = method
                    else {
                        return Err(type_error("impl blocks may contain only functions", span));
                    };
                    self.function_type(params, return_type.as_ref(), span)?;
                    self.push_scope();
                    self.current_scope_mut().insert(
                        "self".into(),
                        Binding {
                            ty: Type::Named(type_name.clone()),
                            mutable: false,
                        },
                    );
                    for param in params {
                        if param.name == "self" {
                            continue;
                        }
                        self.current_scope_mut().insert(
                            param.name.clone(),
                            Binding {
                                ty: param
                                    .type_ann
                                    .as_ref()
                                    .map(type_from_annotation)
                                    .unwrap_or(Type::Unknown),
                                mutable: false,
                            },
                        );
                    }
                    self.current_return.push(
                        return_type
                            .as_ref()
                            .map(type_from_annotation)
                            .unwrap_or(Type::Unknown),
                    );
                    self.observed_returns.push(Vec::new());
                    self.predeclare(body)?;
                    let result = self.analyze_block(body);
                    self.current_return.pop();
                    self.observed_returns.pop();
                    self.pop_scope();
                    result?;
                    if return_type.is_some() && !block_guarantees_return(body) {
                        return Err(type_error(
                            "Method with an explicit return type may fall through without returning",
                            span,
                        ));
                    }
                }
            }
            Statement::TryCatch {
                try_body,
                catch_var,
                catch_body,
                ..
            } => {
                self.with_scope(try_body)?;
                self.push_scope();
                self.current_scope_mut().insert(
                    catch_var.clone(),
                    Binding {
                        ty: Type::String,
                        mutable: false,
                    },
                );
                let result = self.analyze_block(catch_body);
                self.pop_scope();
                result?;
            }
        }
        Ok(())
    }

    fn expression_type(&mut self, expression: &Expression) -> VietResult<Type> {
        match expression {
            Expression::IntLiteral { .. } => Ok(Type::Int),
            Expression::FloatLiteral { .. } => Ok(Type::Float),
            Expression::StringLiteral { .. } => Ok(Type::String),
            Expression::BoolLiteral { .. } => Ok(Type::Bool),
            Expression::NoneLiteral { .. } => Ok(Type::None),
            Expression::Identifier { name, .. } => Ok(self
                .lookup(name)
                .map(|binding| binding.ty.clone())
                .or_else(|| match name.as_str() {
                    "None" => Some(Type::Generic("Option".into(), vec![Type::Unknown])),
                    _ => None,
                })
                .unwrap_or(Type::Unknown)),
            Expression::BinaryOp {
                left,
                op,
                right,
                span,
            } => {
                let left_type = self.expression_type(left)?;
                let right_type = self.expression_type(right)?;
                binary_type(&left_type, op, &right_type, span)
            }
            Expression::UnaryOp { op, operand, span } => {
                let operand_type = self.expression_type(operand)?;
                match op {
                    UnaryOperator::Neg
                        if is_numeric(&operand_type) || operand_type == Type::Unknown =>
                    {
                        Ok(operand_type)
                    }
                    UnaryOperator::Neg => Err(type_error(
                        format!("Cannot negate {}", display_type(&operand_type)),
                        span,
                    )),
                    UnaryOperator::Not => Ok(Type::Bool),
                }
            }
            Expression::Call {
                callee,
                arguments,
                span,
            } => {
                let argument_types = arguments
                    .iter()
                    .map(|arg| self.expression_type(arg))
                    .collect::<VietResult<Vec<_>>>()?;
                if let Expression::Identifier { name, .. } = callee.as_ref() {
                    if self.lookup(name).is_none() {
                        let result = match (name.as_str(), argument_types.as_slice()) {
                            ("Some", [value]) => {
                                Some(Type::Generic("Option".into(), vec![value.clone()]))
                            }
                            ("Ok", [value]) => Some(Type::Generic(
                                "Result".into(),
                                vec![value.clone(), Type::Unknown],
                            )),
                            ("Err", [error]) => Some(Type::Generic(
                                "Result".into(),
                                vec![Type::Unknown, error.clone()],
                            )),
                            ("Some" | "Ok" | "Err", _) => {
                                return Err(type_error(
                                    format!("{}() expects exactly 1 argument", name),
                                    span,
                                ));
                            }
                            _ => None,
                        };
                        if let Some(result) = result {
                            return Ok(result);
                        }
                    }
                }
                let callee_type = self.expression_type(callee)?;
                match callee_type {
                    Type::Function(signature) => {
                        if argument_types.len() < signature.required
                            || (!signature.variadic
                                && argument_types.len() > signature.params.len())
                        {
                            let maximum = if signature.variadic {
                                "variadic".into()
                            } else {
                                signature.params.len().to_string()
                            };
                            return Err(type_error(
                                format!(
                                    "Call expects {}..={} argument(s), received {}",
                                    signature.required,
                                    maximum,
                                    argument_types.len()
                                ),
                                span,
                            ));
                        }
                        for (index, (expected, actual)) in signature
                            .params
                            .iter()
                            .zip(argument_types.iter())
                            .enumerate()
                        {
                            ensure_assignable(
                                expected,
                                actual,
                                span,
                                &format!("argument {}", index + 1),
                            )?;
                        }
                        Ok(*signature.return_type)
                    }
                    Type::Unknown => Ok(Type::Unknown),
                    other => Err(type_error(
                        format!("{} is not callable", display_type(&other)),
                        span,
                    )),
                }
            }
            Expression::MethodCall {
                object,
                method,
                arguments,
                span,
            } => {
                let object_type = self.expression_type(object)?;
                let argument_types = arguments
                    .iter()
                    .map(|argument| self.expression_type(argument))
                    .collect::<VietResult<Vec<_>>>()?;
                if let Type::Named(type_name) = object_type {
                    if self.structs.contains_key(&type_name) || self.enums.contains_key(&type_name)
                    {
                        let signature = self
                            .methods
                            .get(&type_name)
                            .and_then(|methods| methods.get(method))
                            .cloned()
                            .ok_or_else(|| {
                                type_error(
                                    format!("Type '{}' has no method '{}'", type_name, method),
                                    span,
                                )
                            })?;
                        return check_call(&signature, &argument_types, span);
                    }
                }
                Ok(Type::Unknown)
            }
            Expression::FieldAccess {
                object,
                field,
                span,
            } => {
                let object_type = self.expression_type(object)?;
                match object_type {
                    Type::Named(name) => match self.structs.get(&name) {
                        Some(struct_type) => {
                            struct_type.fields.get(field).cloned().ok_or_else(|| {
                                type_error(
                                    format!("Struct '{}' has no field '{}'", name, field),
                                    span,
                                )
                            })
                        }
                        None => Ok(Type::Unknown),
                    },
                    Type::Unknown => Ok(Type::Unknown),
                    other => Err(type_error(
                        format!(
                            "Cannot access field '{}' on {}",
                            field,
                            display_type(&other)
                        ),
                        span,
                    )),
                }
            }
            Expression::Index {
                object,
                index,
                span,
            } => {
                let object_type = self.expression_type(object)?;
                let index_type = self.expression_type(index)?;
                if !matches!(index_type, Type::Int | Type::Unknown) {
                    return Err(type_error("Index must be Int", span));
                }
                match object_type {
                    Type::Array(inner) => Ok(*inner),
                    Type::Named(name) if name == "Array" => Ok(Type::Unknown),
                    Type::String => Ok(Type::String),
                    Type::Unknown => Ok(Type::Unknown),
                    other => Err(type_error(
                        format!("Cannot index {}", display_type(&other)),
                        span,
                    )),
                }
            }
            Expression::ArrayLiteral { elements, span } => {
                let mut element_type = Type::Unknown;
                for element in elements {
                    let actual = self.expression_type(element)?;
                    if element_type == Type::Unknown {
                        element_type = actual;
                    } else {
                        ensure_assignable(&element_type, &actual, span, "array element")?;
                    }
                }
                Ok(Type::Array(Box::new(element_type)))
            }
            Expression::StructLiteral { name, fields, span } => {
                let Some(struct_type) = self.structs.get(name).cloned() else {
                    return Ok(Type::Unknown);
                };
                let mut supplied = HashSet::new();
                for (field, value) in fields {
                    if !supplied.insert(field) {
                        return Err(type_error(
                            format!("Duplicate field '{}.{}'", name, field),
                            span,
                        ));
                    }
                    let expected = struct_type.fields.get(field).ok_or_else(|| {
                        type_error(format!("Struct '{}' has no field '{}'", name, field), span)
                    })?;
                    let actual = self.expression_type(value)?;
                    ensure_assignable(
                        expected,
                        &actual,
                        span,
                        &format!("field '{}.{}'", name, field),
                    )?;
                }
                let missing: Vec<_> = struct_type
                    .fields
                    .keys()
                    .filter(|field| !supplied.contains(field))
                    .cloned()
                    .collect();
                if !missing.is_empty() {
                    return Err(type_error(
                        format!(
                            "Struct '{}' is missing field(s): {}",
                            name,
                            missing.join(", ")
                        ),
                        span,
                    ));
                }
                Ok(Type::Named(name.clone()))
            }
            Expression::Match {
                subject,
                arms,
                span,
            } => {
                let subject_type = self.expression_type(subject)?;
                self.check_match_exhaustiveness(&subject_type, arms, span)?;
                let mut result_type = Type::Unknown;
                for arm in arms {
                    self.push_scope();
                    self.bind_pattern(&arm.pattern, &subject_type, span)?;
                    let arm_type = self.expression_type(&arm.body);
                    self.pop_scope();
                    let arm_type = arm_type?;
                    if result_type == Type::Unknown {
                        result_type = arm_type;
                    } else {
                        ensure_assignable(&result_type, &arm_type, span, "match arm")?;
                    }
                }
                Ok(result_type)
            }
            Expression::Block {
                statements,
                final_expr,
                ..
            } => {
                self.push_scope();
                self.predeclare(statements)?;
                let result = self
                    .analyze_block(statements)
                    .and_then(|_| match final_expr {
                        Some(expression) => self.expression_type(expression),
                        None => Ok(Type::None),
                    });
                self.pop_scope();
                result
            }
            Expression::Lambda {
                params,
                return_type,
                body,
                span,
            } => {
                let signature = self.function_type(params, return_type.as_ref(), span)?;
                self.push_scope();
                for (param, ty) in params.iter().zip(signature.params.iter()) {
                    self.current_scope_mut().insert(
                        param.name.clone(),
                        Binding {
                            ty: ty.clone(),
                            mutable: false,
                        },
                    );
                }
                let expected_return = return_type
                    .as_ref()
                    .map(type_from_annotation)
                    .unwrap_or(Type::Unknown);
                self.current_return.push(expected_return.clone());
                self.observed_returns.push(Vec::new());
                let body_type = self.expression_type(body);
                self.current_return.pop();
                let observed = self.observed_returns.pop().unwrap_or_default();
                self.pop_scope();
                let body_type = body_type?;
                if return_type.is_some() {
                    if let Expression::Block { statements, .. } = body.as_ref() {
                        if !block_guarantees_return(statements) {
                            return Err(type_error(
                                "Lambda with an explicit return type may fall through without returning",
                                span,
                            ));
                        }
                    }
                }
                let inferred_return = if return_type.is_some() {
                    expected_return
                } else if let Some(first) = observed.first() {
                    for actual in observed.iter().skip(1) {
                        ensure_assignable(first, actual, span, "lambda return value")?;
                    }
                    first.clone()
                } else {
                    body_type
                };
                Ok(Type::Function(FunctionType {
                    return_type: Box::new(inferred_return),
                    ..signature
                }))
            }
            Expression::Range { start, end, span } => {
                let start_type = self.expression_type(start)?;
                let end_type = self.expression_type(end)?;
                ensure_assignable(&Type::Int, &start_type, span, "range start")?;
                ensure_assignable(&Type::Int, &end_type, span, "range end")?;
                Ok(Type::Array(Box::new(Type::Int)))
            }
        }
    }

    fn check_match_exhaustiveness(
        &self,
        subject: &Type,
        arms: &[MatchArm],
        span: &Span,
    ) -> VietResult<()> {
        if arms
            .iter()
            .any(|arm| matches!(arm.pattern, Pattern::Wildcard | Pattern::Variable(_)))
        {
            return Ok(());
        }
        match subject {
            Type::Bool => {
                let mut seen_true = false;
                let mut seen_false = false;
                for arm in arms {
                    if let Pattern::Literal(Expression::BoolLiteral { value, .. }) = &arm.pattern {
                        if *value {
                            seen_true = true
                        } else {
                            seen_false = true
                        }
                    }
                }
                if seen_true && seen_false {
                    Ok(())
                } else {
                    Err(type_error("Non-exhaustive Bool match", span))
                }
            }
            Type::Named(name) | Type::Generic(name, _)
                if self.enums.contains_key(name) || builtin_adt_variants(name).is_some() =>
            {
                let builtin;
                let expected = if let Some(declared) = self.enums.get(name) {
                    &declared.variants
                } else {
                    builtin = builtin_adt_variants(name).expect("guarded builtin ADT");
                    &builtin
                };
                for arm in arms {
                    if let Pattern::EnumVariant { name: variant, .. } = &arm.pattern {
                        if !expected.contains_key(variant) {
                            return Err(type_error(
                                format!("Unknown enum variant '{}'", variant),
                                span,
                            ));
                        }
                    }
                }
                let seen: HashSet<_> = arms
                    .iter()
                    .filter_map(|arm| match &arm.pattern {
                        Pattern::EnumVariant { name, .. } => Some(name.as_str()),
                        _ => None,
                    })
                    .collect();
                let missing: Vec<_> = expected
                    .keys()
                    .filter(|variant| !seen.contains(variant.as_str()))
                    .cloned()
                    .collect();
                if missing.is_empty() {
                    Ok(())
                } else {
                    Err(type_error(
                        format!("Non-exhaustive match; missing: {}", missing.join(", ")),
                        span,
                    ))
                }
            }
            Type::Unknown => Ok(()),
            _ => Err(type_error(
                "Non-exhaustive match requires a wildcard arm",
                span,
            )),
        }
    }

    fn bind_pattern(&mut self, pattern: &Pattern, subject: &Type, span: &Span) -> VietResult<()> {
        match pattern {
            Pattern::Variable(name) => {
                self.current_scope_mut().insert(
                    name.clone(),
                    Binding {
                        ty: subject.clone(),
                        mutable: false,
                    },
                );
            }
            Pattern::EnumVariant { name, fields } => {
                let (enum_name, generic_params) = match subject {
                    Type::Named(name) => (Some(name.clone()), Vec::new()),
                    Type::Generic(name, params) => (Some(name.clone()), params.clone()),
                    _ => (None, Vec::new()),
                };
                let mut field_types = enum_name
                    .as_ref()
                    .and_then(|enum_name| self.enums.get(enum_name))
                    .and_then(|enum_type| enum_type.variants.get(name))
                    .cloned();
                if field_types.is_none() {
                    field_types =
                        builtin_adt_field_types(enum_name.as_deref(), name, &generic_params);
                }
                if let Some(field_types) = field_types {
                    if fields.len() != field_types.len() {
                        return Err(type_error(
                            format!(
                                "Pattern '{}' expects {} field(s), received {}",
                                name,
                                field_types.len(),
                                fields.len()
                            ),
                            span,
                        ));
                    }
                    for (field, field_type) in fields.iter().zip(field_types.iter()) {
                        self.bind_pattern(field, field_type, span)?;
                    }
                } else if enum_name.is_some() {
                    return Err(type_error(format!("Unknown enum variant '{}'", name), span));
                } else {
                    for field in fields {
                        self.bind_pattern(field, &Type::Unknown, span)?;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn with_scope(&mut self, statements: &[Statement]) -> VietResult<()> {
        self.push_scope();
        self.predeclare(statements)?;
        let result = self.analyze_block(statements);
        self.pop_scope();
        result
    }

    fn define_unique(
        &mut self,
        name: &str,
        ty: Type,
        mutable: bool,
        span: &Span,
    ) -> VietResult<()> {
        if self.current_scope().contains_key(name) {
            return Err(name_error(
                format!("'{}' is already declared in this scope", name),
                span,
            ));
        }
        self.current_scope_mut()
            .insert(name.into(), Binding { ty, mutable });
        Ok(())
    }

    fn lookup(&self, name: &str) -> Option<&Binding> {
        self.scopes.iter().rev().find_map(|scope| scope.get(name))
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    fn pop_scope(&mut self) {
        self.scopes.pop();
    }
    fn current_scope(&self) -> &HashMap<String, Binding> {
        self.scopes.last().expect("semantic scope")
    }
    fn current_scope_mut(&mut self) -> &mut HashMap<String, Binding> {
        self.scopes.last_mut().expect("semantic scope")
    }
}

fn builtin_adt_variants(name: &str) -> Option<HashMap<String, Vec<Type>>> {
    let variants = match name {
        "Option" => [
            ("Some".to_string(), vec![Type::Unknown]),
            ("None".to_string(), vec![]),
        ],
        "Result" => [
            ("Ok".to_string(), vec![Type::Unknown]),
            ("Err".to_string(), vec![Type::Unknown]),
        ],
        _ => return None,
    };
    Some(variants.into_iter().collect())
}

fn builtin_adt_field_types(
    enum_name: Option<&str>,
    variant: &str,
    params: &[Type],
) -> Option<Vec<Type>> {
    match (enum_name?, variant) {
        ("Option", "Some") => Some(vec![params.first().cloned().unwrap_or(Type::Unknown)]),
        ("Option", "None") => Some(vec![]),
        ("Result", "Ok") => Some(vec![params.first().cloned().unwrap_or(Type::Unknown)]),
        ("Result", "Err") => Some(vec![params.get(1).cloned().unwrap_or(Type::Unknown)]),
        ("Option" | "Result", _) => None,
        _ => None,
    }
}

fn type_from_annotation(annotation: &TypeAnnotation) -> Type {
    match annotation {
        TypeAnnotation::Simple(name) => match name.as_str() {
            "Any" => Type::Unknown,
            "Int" => Type::Int,
            "Float" => Type::Float,
            "String" => Type::String,
            "Bool" => Type::Bool,
            "None" => Type::None,
            _ => Type::Named(name.clone()),
        },
        TypeAnnotation::Nullable(inner) => Type::Nullable(Box::new(type_from_annotation(inner))),
        TypeAnnotation::Array(inner) => Type::Array(Box::new(type_from_annotation(inner))),
        TypeAnnotation::Generic { name, params } => Type::Generic(
            name.clone(),
            params.iter().map(type_from_annotation).collect(),
        ),
        TypeAnnotation::Function {
            params,
            return_type,
        } => Type::Function(FunctionType {
            required: params.len(),
            params: params.iter().map(type_from_annotation).collect(),
            return_type: Box::new(type_from_annotation(return_type)),
            variadic: false,
        }),
    }
}

fn ensure_assignable(expected: &Type, actual: &Type, span: &Span, context: &str) -> VietResult<()> {
    if is_assignable(expected, actual) {
        Ok(())
    } else {
        Err(type_error(
            format!(
                "Type mismatch in {}: expected {}, found {}",
                context,
                display_type(expected),
                display_type(actual)
            ),
            span,
        ))
    }
}

fn check_call(signature: &FunctionType, arguments: &[Type], span: &Span) -> VietResult<Type> {
    if arguments.len() < signature.required
        || (!signature.variadic && arguments.len() > signature.params.len())
    {
        let maximum = if signature.variadic {
            "variadic".into()
        } else {
            signature.params.len().to_string()
        };
        return Err(type_error(
            format!(
                "Call expects {}..={} argument(s), received {}",
                signature.required,
                maximum,
                arguments.len()
            ),
            span,
        ));
    }
    for (index, (expected, actual)) in signature.params.iter().zip(arguments).enumerate() {
        ensure_assignable(expected, actual, span, &format!("argument {}", index + 1))?;
    }
    Ok((*signature.return_type).clone())
}

fn is_assignable(expected: &Type, actual: &Type) -> bool {
    match (expected, actual) {
        (Type::Unknown, _) | (_, Type::Unknown) => true,
        (Type::Float, Type::Int) => true,
        (Type::Nullable(_), Type::None) => true,
        (Type::Nullable(expected), Type::Nullable(actual)) => is_assignable(expected, actual),
        (Type::Nullable(expected), actual) => is_assignable(expected, actual),
        (Type::Named(name), Type::Array(_)) if name == "Array" => true,
        (Type::Generic(expected, _), Type::Named(actual)) => expected == actual,
        (Type::Named(expected), Type::Generic(actual, _)) => expected == actual,
        (Type::Array(expected), Type::Array(actual)) => is_assignable(expected, actual),
        (Type::Generic(en, ep), Type::Generic(an, ap)) => {
            en == an && ep.len() == ap.len() && ep.iter().zip(ap).all(|(e, a)| is_assignable(e, a))
        }
        _ => expected == actual,
    }
}

fn ensure_condition(ty: &Type, span: &Span) -> VietResult<()> {
    if matches!(ty, Type::Bool | Type::Unknown) {
        Ok(())
    } else {
        Err(type_error(
            format!("Condition must be Bool, found {}", display_type(ty)),
            span,
        ))
    }
}

fn binary_type(left: &Type, op: &BinaryOperator, right: &Type, span: &Span) -> VietResult<Type> {
    if *left == Type::Unknown || *right == Type::Unknown {
        return Ok(match op {
            BinaryOperator::Eq
            | BinaryOperator::NotEq
            | BinaryOperator::Lt
            | BinaryOperator::Gt
            | BinaryOperator::LtEq
            | BinaryOperator::GtEq
            | BinaryOperator::And
            | BinaryOperator::Or => Type::Bool,
            _ => Type::Unknown,
        });
    }
    match op {
        BinaryOperator::Add if *left == Type::String && *right == Type::String => Ok(Type::String),
        BinaryOperator::Add | BinaryOperator::Sub | BinaryOperator::Mul | BinaryOperator::Div
            if is_numeric(left) && is_numeric(right) =>
        {
            if *left == Type::Float || *right == Type::Float {
                Ok(Type::Float)
            } else {
                Ok(Type::Int)
            }
        }
        BinaryOperator::Mod if *left == Type::Int && *right == Type::Int => Ok(Type::Int),
        BinaryOperator::Eq | BinaryOperator::NotEq
            if is_assignable(left, right) || is_assignable(right, left) =>
        {
            Ok(Type::Bool)
        }
        BinaryOperator::Lt | BinaryOperator::Gt | BinaryOperator::LtEq | BinaryOperator::GtEq
            if (is_numeric(left) && is_numeric(right))
                || (*left == Type::String && *right == Type::String) =>
        {
            Ok(Type::Bool)
        }
        BinaryOperator::And | BinaryOperator::Or if *left == Type::Bool && *right == Type::Bool => {
            Ok(Type::Bool)
        }
        _ => Err(type_error(
            format!(
                "Operator {:?} is not defined for {} and {}",
                op,
                display_type(left),
                display_type(right)
            ),
            span,
        )),
    }
}

fn is_numeric(ty: &Type) -> bool {
    matches!(ty, Type::Int | Type::Float)
}

fn block_guarantees_return(statements: &[Statement]) -> bool {
    statements.iter().any(statement_guarantees_return)
}

fn statement_guarantees_return(statement: &Statement) -> bool {
    match statement {
        Statement::Return { .. } => true,
        Statement::If {
            then_body,
            else_body: Some(else_body),
            ..
        } => block_guarantees_return(then_body) && block_guarantees_return(else_body),
        Statement::TryCatch {
            try_body,
            catch_body,
            ..
        } => block_guarantees_return(try_body) && block_guarantees_return(catch_body),
        _ => false,
    }
}

fn display_type(ty: &Type) -> String {
    match ty {
        Type::Unknown => "Unknown".into(),
        Type::None => "None".into(),
        Type::Int => "Int".into(),
        Type::Float => "Float".into(),
        Type::String => "String".into(),
        Type::Bool => "Bool".into(),
        Type::Array(inner) => format!("[{}]", display_type(inner)),
        Type::Nullable(inner) => format!("?{}", display_type(inner)),
        Type::Named(name) => name.clone(),
        Type::Generic(name, params) => format!(
            "{}<{}>",
            name,
            params
                .iter()
                .map(display_type)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Type::Function(_) => "Function".into(),
    }
}

fn type_error(message: impl Into<String>, span: &Span) -> VietError {
    VietError::type_error(message.into(), span.line, span.column)
}

fn name_error(message: impl Into<String>, span: &Span) -> VietError {
    VietError::name_error(message.into(), span.line, span.column)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::Lexer, parser::Parser};

    fn analyze(source: &str) -> VietResult<()> {
        let tokens = Lexer::new(source).tokenize()?;
        let program = Parser::new(tokens).parse()?;
        SemanticAnalyzer::new().analyze(&program)
    }

    #[test]
    fn rejects_annotated_initializer_and_return_mismatches() {
        assert!(analyze("let value: Int = \"wrong\"")
            .unwrap_err()
            .message
            .contains("initializer"));
        assert!(analyze("fn value() -> Int { return \"wrong\" }")
            .unwrap_err()
            .message
            .contains("return value"));
        assert!(analyze("fn value() -> Int { let value = 1 }")
            .unwrap_err()
            .message
            .contains("fall through"));
        analyze("fn value(ok: Bool) -> Int { if ok { return 1 } else { return 0 } }").unwrap();
    }

    #[test]
    fn validates_function_calls_and_defaults() {
        analyze("fn add(a: Int, b: Int = 2) -> Int { return a + b }\nlet result: Int = add(3)")
            .unwrap();
        assert!(
            analyze("fn add(a: Int, b: Int) -> Int { return a + b }\nadd(1)")
                .unwrap_err()
                .message
                .contains("Call expects")
        );
        assert!(analyze("fn bad(a: Int = 1, b: Int) { }")
            .unwrap_err()
            .message
            .contains("Required parameters"));
    }

    #[test]
    fn validates_struct_fields_and_types() {
        analyze(
            "struct User { name: String, age: Int }\nlet user = User { name: \"Lan\", age: 20 }",
        )
        .unwrap();
        assert!(
            analyze("struct User { name: String, age: Int }\nUser { name: \"Lan\" }")
                .unwrap_err()
                .message
                .contains("missing field")
        );
        assert!(analyze("struct User { age: Int }\nUser { age: \"old\" }")
            .unwrap_err()
            .message
            .contains("field 'User.age'"));
    }

    #[test]
    fn validates_control_flow_context_and_match_exhaustiveness() {
        assert!(analyze("break")
            .unwrap_err()
            .message
            .contains("inside a loop"));
        analyze("let ok: Bool = match true { true => true, false => false }").unwrap();
        assert!(analyze("match true { true => 1 }")
            .unwrap_err()
            .message
            .contains("Non-exhaustive Bool"));
    }

    #[test]
    fn rejects_assignment_to_immutable_or_wrong_type() {
        assert!(analyze("let value: Int = 1\nvalue = 2")
            .unwrap_err()
            .message
            .contains("immutable"));
        assert!(analyze("let mut value: Int = 1\nvalue = \"bad\"")
            .unwrap_err()
            .message
            .contains("assignment"));
    }

    #[test]
    fn validates_oop_methods_and_struct_mutation() {
        analyze(
            "struct User { name: String }\nimpl User { fn label(self, prefix: String) -> String { return prefix + self.name } }\nlet user = User { name: \"Lan\" }\nlet label: String = user.label(\"Hi \" )",
        )
        .unwrap();
        assert!(analyze(
            "struct User { name: String }\nimpl User { fn label(prefix: String) -> String { return prefix } }"
        )
        .unwrap_err()
        .message
        .contains("must declare 'self'"));
        assert!(analyze(
            "struct User { name: String }\nimpl User { fn label(self, prefix: String) -> String { return prefix } }\nlet user = User { name: \"Lan\" }\nuser.label(1)"
        )
        .unwrap_err()
        .message
        .contains("argument 1"));
        assert!(analyze(
            "struct User { name: String }\nlet user = User { name: \"Lan\" }\nuser.name = \"Mai\""
        )
        .unwrap_err()
        .message
        .contains("immutable variable"));
    }

    #[test]
    fn validates_enum_constructors_payloads_and_exhaustiveness() {
        analyze(
            "enum Result { Ok(Int), Err(String) }\nfn load() -> Result<Int, String> { return Ok(7) }\nlet result: Result<Int, String> = load()\nlet value: Int = match result { Ok(number) => number, Err(message) => 0 }",
        )
        .unwrap();
        assert!(analyze("enum Result { Ok(Int), Err(String) }\nOk(\"bad\")")
            .unwrap_err()
            .message
            .contains("argument 1"));
        assert!(analyze("enum Result { Ok(Int), Err(String) }\nlet result: Result = Ok(1)\nmatch result { Ok(value) => value }")
            .unwrap_err()
            .message
            .contains("Non-exhaustive match"));
        assert!(analyze("enum Result { Ok(Int), Err(String) }\nlet result: Result = Ok(1)\nmatch result { Missing(value) => value, Err(message) => 0 }")
            .unwrap_err()
            .message
            .contains("Unknown enum variant"));
    }

    #[test]
    fn validates_annotated_and_inferred_lambda_returns() {
        analyze(
            "let double: fn(Int) -> Int = fn(value: Int) -> Int { return value * 2 }\ndouble(4)",
        )
        .unwrap();
        analyze("let double: fn(Int) -> Int = fn(value: Int) { return value * 2 }\ndouble(4)")
            .unwrap();
        assert!(analyze("let bad = fn() -> Int { return \"wrong\" }")
            .unwrap_err()
            .message
            .contains("return value"));
        assert!(analyze("let bad = fn() -> Int { let value = 1 }")
            .unwrap_err()
            .message
            .contains("fall through"));
    }
}
