//! VietLang Abstract Syntax Tree (AST)
//! Defines all AST node types used by the parser and interpreter.

use crate::lexer::token::Span;

/// Top-level program: a list of statements
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

/// Statements
#[derive(Debug, Clone)]
pub enum Statement {
    /// `let [mut] name [: type] = expr`
    Let {
        name: String,
        mutable: bool,
        type_ann: Option<TypeAnnotation>,
        value: Expression,
        span: Span,
    },

    /// `name = expr` or `obj.field = expr`
    Assignment {
        target: Expression,
        value: Expression,
        span: Span,
    },

    /// Expression statement (expression used as statement)
    Expression { expr: Expression, span: Span },

    /// `fn name(params) [-> return_type] { body }`
    Function {
        name: String,
        params: Vec<Parameter>,
        return_type: Option<TypeAnnotation>,
        body: Vec<Statement>,
        is_pub: bool,
        span: Span,
    },

    /// `return [expr]`
    Return {
        value: Option<Expression>,
        span: Span,
    },

    /// `if cond { body } [else { body }]`
    If {
        condition: Expression,
        then_body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
        span: Span,
    },

    /// `while cond { body }`
    While {
        condition: Expression,
        body: Vec<Statement>,
        span: Span,
    },

    /// `for item in iterable { body }`
    For {
        variable: String,
        iterable: Expression,
        body: Vec<Statement>,
        span: Span,
    },

    /// `break`
    Break { span: Span },

    /// `continue`
    Continue { span: Span },

    /// `struct Name { fields }`
    Struct {
        name: String,
        fields: Vec<StructField>,
        is_pub: bool,
        span: Span,
    },

    /// `enum Name { variants }`
    Enum {
        name: String,
        variants: Vec<EnumVariant>,
        is_pub: bool,
        span: Span,
    },

    /// `impl TypeName { methods }`
    Impl {
        type_name: String,
        methods: Vec<Statement>,
        span: Span,
    },

    /// `import path.to.module`
    Import {
        path: Vec<String>,
        alias: Option<String>,
        span: Span,
    },

    /// `try { ... } catch err { ... }`
    TryCatch {
        try_body: Vec<Statement>,
        catch_var: String,
        catch_body: Vec<Statement>,
        span: Span,
    },
}

/// Expressions
#[derive(Debug, Clone)]
pub enum Expression {
    /// Integer literal: `42`
    IntLiteral { value: i64, span: Span },

    /// Float literal: `3.14`
    FloatLiteral { value: f64, span: Span },

    /// String literal: `"hello"`
    StringLiteral { value: String, span: Span },

    /// Boolean literal: `true` / `false`
    BoolLiteral { value: bool, span: Span },

    /// None literal
    NoneLiteral { span: Span },

    /// Variable reference: `x`
    Identifier { name: String, span: Span },

    /// Binary operation: `a + b`, `x == y`
    BinaryOp {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
        span: Span,
    },

    /// Unary operation: `-x`, `!flag`
    UnaryOp {
        op: UnaryOperator,
        operand: Box<Expression>,
        span: Span,
    },

    /// Function call: `foo(arg1, arg2)`
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
        span: Span,
    },

    /// Method call: `obj.method(args)`
    MethodCall {
        object: Box<Expression>,
        method: String,
        arguments: Vec<Expression>,
        span: Span,
    },

    /// Field access: `obj.field`
    FieldAccess {
        object: Box<Expression>,
        field: String,
        span: Span,
    },

    /// Index access: `arr[0]`
    Index {
        object: Box<Expression>,
        index: Box<Expression>,
        span: Span,
    },

    /// Array literal: `[1, 2, 3]`
    ArrayLiteral {
        elements: Vec<Expression>,
        span: Span,
    },

    /// Struct instantiation: `User { name: "John", age: 30 }`
    StructLiteral {
        name: String,
        fields: Vec<(String, Expression)>,
        span: Span,
    },

    /// Match expression
    Match {
        subject: Box<Expression>,
        arms: Vec<MatchArm>,
        span: Span,
    },

    /// Block expression: `{ stmts; expr }`
    Block {
        statements: Vec<Statement>,
        final_expr: Option<Box<Expression>>,
        span: Span,
    },

    /// Lambda: `fn(x, y) { x + y }`  or  `|x, y| x + y`
    Lambda {
        params: Vec<Parameter>,
        return_type: Option<TypeAnnotation>,
        body: Box<Expression>,
        span: Span,
    },

    /// Range: `1..10`
    Range {
        start: Box<Expression>,
        end: Box<Expression>,
        span: Span,
    },
}

impl Expression {
    pub fn span(&self) -> &Span {
        match self {
            Expression::IntLiteral { span, .. } => span,
            Expression::FloatLiteral { span, .. } => span,
            Expression::StringLiteral { span, .. } => span,
            Expression::BoolLiteral { span, .. } => span,
            Expression::NoneLiteral { span } => span,
            Expression::Identifier { span, .. } => span,
            Expression::BinaryOp { span, .. } => span,
            Expression::UnaryOp { span, .. } => span,
            Expression::Call { span, .. } => span,
            Expression::MethodCall { span, .. } => span,
            Expression::FieldAccess { span, .. } => span,
            Expression::Index { span, .. } => span,
            Expression::ArrayLiteral { span, .. } => span,
            Expression::StructLiteral { span, .. } => span,
            Expression::Match { span, .. } => span,
            Expression::Block { span, .. } => span,
            Expression::Lambda { span, .. } => span,
            Expression::Range { span, .. } => span,
        }
    }
}

/// Binary operators
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,   // +
    Sub,   // -
    Mul,   // *
    Div,   // /
    Mod,   // %
    Eq,    // ==
    NotEq, // !=
    Lt,    // <
    Gt,    // >
    LtEq,  // <=
    GtEq,  // >=
    And,   // &&
    Or,    // ||
}

/// Unary operators
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Neg, // -
    Not, // !
}

/// Function parameter
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub type_ann: Option<TypeAnnotation>,
    pub default: Option<Expression>,
}

/// Struct field definition
#[derive(Debug, Clone)]
pub struct StructField {
    pub name: String,
    pub type_ann: TypeAnnotation,
    pub is_pub: bool,
}

/// Enum variant
#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub fields: Vec<TypeAnnotation>, // tuple variant fields
}

/// Match arm: `pattern => expression`
#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Expression,
}

/// Patterns for match expressions
#[derive(Debug, Clone)]
pub enum Pattern {
    /// Literal value: `42`, `"hello"`, `true`
    Literal(Expression),
    /// Variable binding: `x`
    Variable(String),
    /// Enum variant: `Ok(value)`, `Err(e)`
    EnumVariant { name: String, fields: Vec<Pattern> },
    /// Wildcard: `_`
    Wildcard,
}

/// Type annotations
#[derive(Debug, Clone)]
pub enum TypeAnnotation {
    /// Simple type: `Int`, `String`
    Simple(String),
    /// Nullable type: `?Int`
    Nullable(Box<TypeAnnotation>),
    /// Array type: `[Int]`
    Array(Box<TypeAnnotation>),
    /// Generic type: `Result<T, E>`
    Generic {
        name: String,
        params: Vec<TypeAnnotation>,
    },
    /// Function type: `fn(Int, Int) -> Int`
    Function {
        params: Vec<TypeAnnotation>,
        return_type: Box<TypeAnnotation>,
    },
}
