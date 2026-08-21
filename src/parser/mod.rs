/// VietLang Parser
/// Recursive descent parser that converts tokens into an AST.

pub mod ast;

use crate::error::{VietError, VietResult};
use crate::lexer::token::{Token, TokenKind, Span};
use ast::*;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        // Filter out newlines for simpler parsing (treat as whitespace)
        let filtered: Vec<Token> = tokens
            .into_iter()
            .filter(|t| !matches!(t.kind, TokenKind::Newline))
            .collect();
        Parser { tokens: filtered, pos: 0 }
    }

    /// Parse the entire token stream into a Program
    pub fn parse(&mut self) -> VietResult<Program> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        Ok(Program { statements })
    }

    // ========================================
    // Statement Parsing
    // ========================================

    fn parse_statement(&mut self) -> VietResult<Statement> {
        // Handle optional semicolons
        while self.check(&TokenKind::Semicolon) {
            self.advance();
        }

        if self.is_at_end() {
            return Err(self.error("Unexpected end of file"));
        }

        // Check for pub modifier
        let is_pub = if self.check(&TokenKind::Pub) {
            self.advance();
            true
        } else {
            false
        };

        let stmt = match &self.current().kind {
            TokenKind::Let => self.parse_let()?,
            TokenKind::Fn => self.parse_function(is_pub)?,
            TokenKind::Return => self.parse_return()?,
            TokenKind::If => self.parse_if()?,
            TokenKind::While => self.parse_while()?,
            TokenKind::For => self.parse_for()?,
            TokenKind::Break => {
                let span = self.current().span.clone();
                self.advance();
                Statement::Break { span }
            }
            TokenKind::Continue => {
                let span = self.current().span.clone();
                self.advance();
                Statement::Continue { span }
            }
            TokenKind::Struct => self.parse_struct(is_pub)?,
            TokenKind::Enum => self.parse_enum(is_pub)?,
            TokenKind::Impl => self.parse_impl()?,
            TokenKind::Import => self.parse_import()?,
            _ => {
                let expr = self.parse_expression()?;
                let span = expr.span().clone();

                // Check for assignment
                if self.check(&TokenKind::Assign) {
                    self.advance();
                    let value = self.parse_expression()?;
                    Statement::Assignment { target: expr, value, span }
                } else {
                    Statement::Expression { expr, span }
                }
            }
        };

        // Consume optional semicolons
        while self.check(&TokenKind::Semicolon) {
            self.advance();
        }

        Ok(stmt)
    }

    /// `let [mut] name [: type] = expr`
    fn parse_let(&mut self) -> VietResult<Statement> {
        let span = self.current().span.clone();
        self.expect(&TokenKind::Let)?;

        let mutable = if self.check(&TokenKind::Mut) {
            self.advance();
            true
        } else {
            false
        };

        let name = self.expect_identifier()?;

        let type_ann = if self.check(&TokenKind::Colon) {
            self.advance();
            Some(self.parse_type_annotation()?)
        } else {
            None
        };

        self.expect(&TokenKind::Assign)?;
        let value = self.parse_expression()?;

        Ok(Statement::Let { name, mutable, type_ann, value, span })
    }

    /// `fn name(params) [-> return_type] { body }`
    fn parse_function(&mut self, is_pub: bool) -> VietResult<Statement> {
        let span = self.current().span.clone();
        self.expect(&TokenKind::Fn)?;

        let name = self.expect_identifier()?;

        self.expect(&TokenKind::LParen)?;
        let params = self.parse_params()?;
        self.expect(&TokenKind::RParen)?;

        let return_type = if self.check(&TokenKind::Arrow) {
            self.advance();
            Some(self.parse_type_annotation()?)
        } else {
            None
        };

        self.expect(&TokenKind::LBrace)?;
        let body = self.parse_block_body()?;
        self.expect(&TokenKind::RBrace)?;

        Ok(Statement::Function { name, params, return_type, body, is_pub, span })
    }

    fn parse_params(&mut self) -> VietResult<Vec<Parameter>> {
        let mut params = Vec::new();
        if self.check(&TokenKind::RParen) {
            return Ok(params);
        }

        loop {
            let name = self.expect_identifier()?;
            let type_ann = if self.check(&TokenKind::Colon) {
                self.advance();
                Some(self.parse_type_annotation()?)
            } else {
                None
            };
            let default = if self.check(&TokenKind::Assign) {
                self.advance();
                Some(self.parse_expression()?)
            } else {
                None
            };
            params.push(Parameter { name, type_ann, default });

            if !self.check(&TokenKind::Comma) {
                break;
            }
            self.advance();
        }

        Ok(params)
    }

    /// `return [expr]`
    fn parse_return(&mut self) -> VietResult<Statement> {
        let span = self.current().span.clone();
        self.expect(&TokenKind::Return)?;

        let value = if !self.check(&TokenKind::RBrace) && !self.check(&TokenKind::Semicolon) && !self.is_at_end() {
            Some(self.parse_expression()?)
        } else {
            None
        };

        Ok(Statement::Return { value, span })
    }

    /// `if cond { body } [else { body }]`
    fn parse_if(&mut self) -> VietResult<Statement> {
        let span = self.current().span.clone();
        self.expect(&TokenKind::If)?;

        let condition = self.parse_expression()?;
        self.expect(&TokenKind::LBrace)?;
        let then_body = self.parse_block_body()?;
        self.expect(&TokenKind::RBrace)?;

        let else_body = if self.check(&TokenKind::Else) {
            self.advance();
            if self.check(&TokenKind::If) {
                // else if
                let else_if = self.parse_if()?;
                Some(vec![else_if])
            } else {
                self.expect(&TokenKind::LBrace)?;
                let body = self.parse_block_body()?;
                self.expect(&TokenKind::RBrace)?;
                Some(body)
            }
        } else {
            None
        };

        Ok(Statement::If { condition, then_body, else_body, span })
    }

    /// `while cond { body }`
    fn parse_while(&mut self) -> VietResult<Statement> {
        let span = self.current().span.clone();
        self.expect(&TokenKind::While)?;

        let condition = self.parse_expression()?;
        self.expect(&TokenKind::LBrace)?;
        let body = self.parse_block_body()?;
        self.expect(&TokenKind::RBrace)?;

        Ok(Statement::While { condition, body, span })
    }

    /// `for item in iterable { body }`
    fn parse_for(&mut self) -> VietResult<Statement> {
        let span = self.current().span.clone();
        self.expect(&TokenKind::For)?;

        let variable = self.expect_identifier()?;
        self.expect(&TokenKind::In)?;
        let iterable = self.parse_expression()?;
        self.expect(&TokenKind::LBrace)?;
        let body = self.parse_block_body()?;
        self.expect(&TokenKind::RBrace)?;

        Ok(Statement::For { variable, iterable, body, span })
    }

    /// `struct Name { fields }`
    fn parse_struct(&mut self, is_pub: bool) -> VietResult<Statement> {
        let span = self.current().span.clone();
        self.expect(&TokenKind::Struct)?;

        let name = self.expect_identifier()?;
        self.expect(&TokenKind::LBrace)?;

        let mut fields = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            let field_pub = if self.check(&TokenKind::Pub) {
                self.advance();
                true
            } else {
                false
            };
            let field_name = self.expect_identifier()?;
            self.expect(&TokenKind::Colon)?;
            let type_ann = self.parse_type_annotation()?;
            fields.push(StructField { name: field_name, type_ann, is_pub: field_pub });

            if !self.check(&TokenKind::Comma) {
                break;
            }
            self.advance();
        }
        self.expect(&TokenKind::RBrace)?;

        Ok(Statement::Struct { name, fields, is_pub, span })
    }

    /// `enum Name { variants }`
    fn parse_enum(&mut self, is_pub: bool) -> VietResult<Statement> {
        let span = self.current().span.clone();
        self.expect(&TokenKind::Enum)?;

        let name = self.expect_identifier()?;
        self.expect(&TokenKind::LBrace)?;

        let mut variants = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            let variant_name = self.expect_identifier()?;
            let fields = if self.check(&TokenKind::LParen) {
                self.advance();
                let mut types = Vec::new();
                while !self.check(&TokenKind::RParen) {
                    types.push(self.parse_type_annotation()?);
                    if !self.check(&TokenKind::Comma) {
                        break;
                    }
                    self.advance();
                }
                self.expect(&TokenKind::RParen)?;
                types
            } else {
                Vec::new()
            };
            variants.push(EnumVariant { name: variant_name, fields });

            if !self.check(&TokenKind::Comma) {
                break;
            }
            self.advance();
        }
        self.expect(&TokenKind::RBrace)?;

        Ok(Statement::Enum { name, variants, is_pub, span })
    }

    /// `impl TypeName { methods }`
    fn parse_impl(&mut self) -> VietResult<Statement> {
        let span = self.current().span.clone();
        self.expect(&TokenKind::Impl)?;

        let type_name = self.expect_identifier()?;
        self.expect(&TokenKind::LBrace)?;

        let mut methods = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            let is_pub = if self.check(&TokenKind::Pub) {
                self.advance();
                true
            } else {
                false
            };
            methods.push(self.parse_function(is_pub)?);
        }
        self.expect(&TokenKind::RBrace)?;

        Ok(Statement::Impl { type_name, methods, span })
    }

    /// `import path.to.module`
    fn parse_import(&mut self) -> VietResult<Statement> {
        let span = self.current().span.clone();
        self.expect(&TokenKind::Import)?;

        let mut path = Vec::new();
        path.push(self.expect_identifier()?);
        while self.check(&TokenKind::Dot) {
            self.advance();
            path.push(self.expect_identifier()?);
        }

        Ok(Statement::Import { path, alias: None, span })
    }

    // ========================================
    // Expression Parsing (Pratt Parser / Precedence Climbing)
    // ========================================

    fn parse_expression(&mut self) -> VietResult<Expression> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> VietResult<Expression> {
        let mut left = self.parse_and()?;
        while self.check(&TokenKind::Or) {
            let span = self.current().span.clone();
            self.advance();
            let right = self.parse_and()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op: BinaryOperator::Or,
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> VietResult<Expression> {
        let mut left = self.parse_equality()?;
        while self.check(&TokenKind::And) {
            let span = self.current().span.clone();
            self.advance();
            let right = self.parse_equality()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op: BinaryOperator::And,
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_equality(&mut self) -> VietResult<Expression> {
        let mut left = self.parse_comparison()?;
        while self.check(&TokenKind::Eq) || self.check(&TokenKind::NotEq) {
            let span = self.current().span.clone();
            let op = if self.check(&TokenKind::Eq) {
                BinaryOperator::Eq
            } else {
                BinaryOperator::NotEq
            };
            self.advance();
            let right = self.parse_comparison()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> VietResult<Expression> {
        let mut left = self.parse_range()?;
        while self.check(&TokenKind::Lt) || self.check(&TokenKind::Gt)
            || self.check(&TokenKind::LtEq) || self.check(&TokenKind::GtEq)
        {
            let span = self.current().span.clone();
            let op = match &self.current().kind {
                TokenKind::Lt => BinaryOperator::Lt,
                TokenKind::Gt => BinaryOperator::Gt,
                TokenKind::LtEq => BinaryOperator::LtEq,
                TokenKind::GtEq => BinaryOperator::GtEq,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_range()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_range(&mut self) -> VietResult<Expression> {
        let left = self.parse_additive()?;
        if self.check(&TokenKind::DotDot) {
            let span = self.current().span.clone();
            self.advance();
            let right = self.parse_additive()?;
            Ok(Expression::Range {
                start: Box::new(left),
                end: Box::new(right),
                span,
            })
        } else {
            Ok(left)
        }
    }

    fn parse_additive(&mut self) -> VietResult<Expression> {
        let mut left = self.parse_multiplicative()?;
        while self.check(&TokenKind::Plus) || self.check(&TokenKind::Minus) {
            let span = self.current().span.clone();
            let op = if self.check(&TokenKind::Plus) {
                BinaryOperator::Add
            } else {
                BinaryOperator::Sub
            };
            self.advance();
            let right = self.parse_multiplicative()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> VietResult<Expression> {
        let mut left = self.parse_unary()?;
        while self.check(&TokenKind::Star) || self.check(&TokenKind::Slash)
            || self.check(&TokenKind::Percent)
        {
            let span = self.current().span.clone();
            let op = match &self.current().kind {
                TokenKind::Star => BinaryOperator::Mul,
                TokenKind::Slash => BinaryOperator::Div,
                TokenKind::Percent => BinaryOperator::Mod,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_unary()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> VietResult<Expression> {
        if self.check(&TokenKind::Minus) {
            let span = self.current().span.clone();
            self.advance();
            let operand = self.parse_unary()?;
            Ok(Expression::UnaryOp {
                op: UnaryOperator::Neg,
                operand: Box::new(operand),
                span,
            })
        } else if self.check(&TokenKind::Not) {
            let span = self.current().span.clone();
            self.advance();
            let operand = self.parse_unary()?;
            Ok(Expression::UnaryOp {
                op: UnaryOperator::Not,
                operand: Box::new(operand),
                span,
            })
        } else {
            self.parse_postfix()
        }
    }

    fn parse_postfix(&mut self) -> VietResult<Expression> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.check(&TokenKind::Dot) {
                self.advance();
                let field = self.expect_identifier()?;
                let span = expr.span().clone();

                // Check for method call
                if self.check(&TokenKind::LParen) {
                    self.advance();
                    let arguments = self.parse_arguments()?;
                    self.expect(&TokenKind::RParen)?;
                    expr = Expression::MethodCall {
                        object: Box::new(expr),
                        method: field,
                        arguments,
                        span,
                    };
                } else {
                    expr = Expression::FieldAccess {
                        object: Box::new(expr),
                        field,
                        span,
                    };
                }
            } else if self.check(&TokenKind::LParen) {
                let span = expr.span().clone();
                self.advance();
                let arguments = self.parse_arguments()?;
                self.expect(&TokenKind::RParen)?;
                expr = Expression::Call {
                    callee: Box::new(expr),
                    arguments,
                    span,
                };
            } else if self.check(&TokenKind::LBracket) {
                let span = expr.span().clone();
                self.advance();
                let index = self.parse_expression()?;
                self.expect(&TokenKind::RBracket)?;
                expr = Expression::Index {
                    object: Box::new(expr),
                    index: Box::new(index),
                    span,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> VietResult<Expression> {
        let token = self.current().clone();

        match &token.kind {
            TokenKind::IntLiteral(v) => {
                let value = *v;
                self.advance();
                Ok(Expression::IntLiteral { value, span: token.span })
            }
            TokenKind::FloatLiteral(v) => {
                let value = *v;
                self.advance();
                Ok(Expression::FloatLiteral { value, span: token.span })
            }
            TokenKind::StringLiteral(v) => {
                let value = v.clone();
                self.advance();
                Ok(Expression::StringLiteral { value, span: token.span })
            }
            TokenKind::BoolLiteral(v) => {
                let value = *v;
                self.advance();
                Ok(Expression::BoolLiteral { value, span: token.span })
            }
            TokenKind::None => {
                self.advance();
                Ok(Expression::NoneLiteral { span: token.span })
            }
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.advance();

                // Check for struct literal: `Name { field: value }`
                if self.check(&TokenKind::LBrace) && name.chars().next().map_or(false, |c: char| c.is_uppercase()) {
                    self.advance();
                    let fields = self.parse_struct_literal_fields()?;
                    self.expect(&TokenKind::RBrace)?;
                    Ok(Expression::StructLiteral {
                        name,
                        fields,
                        span: token.span,
                    })
                } else {
                    Ok(Expression::Identifier { name, span: token.span })
                }
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(&TokenKind::RParen)?;
                Ok(expr)
            }
            TokenKind::LBracket => {
                self.advance();
                let mut elements = Vec::new();
                while !self.check(&TokenKind::RBracket) && !self.is_at_end() {
                    elements.push(self.parse_expression()?);
                    if !self.check(&TokenKind::Comma) {
                        break;
                    }
                    self.advance();
                }
                self.expect(&TokenKind::RBracket)?;
                Ok(Expression::ArrayLiteral { elements, span: token.span })
            }
            TokenKind::Match => {
                self.advance();
                self.parse_match_expression(token.span)
            }
            TokenKind::Fn => {
                // Lambda: fn(params) { body }
                self.advance();
                self.expect(&TokenKind::LParen)?;
                let params = self.parse_params()?;
                self.expect(&TokenKind::RParen)?;
                self.expect(&TokenKind::LBrace)?;
                let body_stmts = self.parse_block_body()?;
                self.expect(&TokenKind::RBrace)?;
                
                // Convert block to expression
                let body = Expression::Block {
                    statements: body_stmts,
                    final_expr: None,
                    span: token.span.clone(),
                };
                Ok(Expression::Lambda {
                    params,
                    body: Box::new(body),
                    span: token.span,
                })
            }
            _ => Err(self.error(&format!("Expected expression, found {:?}", token.kind))),
        }
    }

    fn parse_match_expression(&mut self, span: Span) -> VietResult<Expression> {
        let subject = self.parse_expression()?;
        self.expect(&TokenKind::LBrace)?;

        let mut arms = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            let pattern = self.parse_pattern()?;
            self.expect(&TokenKind::FatArrow)?;
            let body = self.parse_expression()?;
            arms.push(MatchArm { pattern, body });

            // Optional comma between arms
            if self.check(&TokenKind::Comma) {
                self.advance();
            }
        }
        self.expect(&TokenKind::RBrace)?;

        Ok(Expression::Match {
            subject: Box::new(subject),
            arms,
            span,
        })
    }

    fn parse_pattern(&mut self) -> VietResult<Pattern> {
        let token = self.current().clone();
        match &token.kind {
            TokenKind::IntLiteral(v) => {
                let value = *v;
                self.advance();
                Ok(Pattern::Literal(Expression::IntLiteral { value, span: token.span }))
            }
            TokenKind::StringLiteral(v) => {
                let value = v.clone();
                self.advance();
                Ok(Pattern::Literal(Expression::StringLiteral { value, span: token.span }))
            }
            TokenKind::BoolLiteral(v) => {
                let value = *v;
                self.advance();
                Ok(Pattern::Literal(Expression::BoolLiteral { value, span: token.span }))
            }
            TokenKind::Identifier(name) if name == "_" => {
                self.advance();
                Ok(Pattern::Wildcard)
            }
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.advance();
                // Check for enum variant pattern: Ok(value)
                if self.check(&TokenKind::LParen) {
                    self.advance();
                    let mut fields = Vec::new();
                    while !self.check(&TokenKind::RParen) {
                        fields.push(self.parse_pattern()?);
                        if !self.check(&TokenKind::Comma) {
                            break;
                        }
                        self.advance();
                    }
                    self.expect(&TokenKind::RParen)?;
                    Ok(Pattern::EnumVariant { name, fields })
                } else {
                    // Could be a variable binding or a simple enum variant
                    if name.chars().next().map_or(false, |c: char| c.is_uppercase()) {
                        Ok(Pattern::EnumVariant { name, fields: Vec::new() })
                    } else {
                        Ok(Pattern::Variable(name))
                    }
                }
            }
            _ => Err(self.error(&format!("Expected pattern, found {:?}", token.kind))),
        }
    }

    fn parse_struct_literal_fields(&mut self) -> VietResult<Vec<(String, Expression)>> {
        let mut fields = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            let name = self.expect_identifier()?;
            self.expect(&TokenKind::Colon)?;
            let value = self.parse_expression()?;
            fields.push((name, value));
            if !self.check(&TokenKind::Comma) {
                break;
            }
            self.advance();
        }
        Ok(fields)
    }

    fn parse_arguments(&mut self) -> VietResult<Vec<Expression>> {
        let mut args = Vec::new();
        if self.check(&TokenKind::RParen) {
            return Ok(args);
        }
        loop {
            args.push(self.parse_expression()?);
            if !self.check(&TokenKind::Comma) {
                break;
            }
            self.advance();
        }
        Ok(args)
    }

    // ========================================
    // Type Annotation Parsing
    // ========================================

    fn parse_type_annotation(&mut self) -> VietResult<TypeAnnotation> {
        // Check for nullable: ?Type
        if self.check(&TokenKind::Question) {
            self.advance();
            let inner = self.parse_type_annotation()?;
            return Ok(TypeAnnotation::Nullable(Box::new(inner)));
        }

        // Check for array: [Type]
        if self.check(&TokenKind::LBracket) {
            self.advance();
            let inner = self.parse_type_annotation()?;
            self.expect(&TokenKind::RBracket)?;
            return Ok(TypeAnnotation::Array(Box::new(inner)));
        }

        // Check for function type: fn(T1, T2) -> R
        if self.check(&TokenKind::Fn) {
            self.advance();
            self.expect(&TokenKind::LParen)?;
            let mut params = Vec::new();
            while !self.check(&TokenKind::RParen) {
                params.push(self.parse_type_annotation()?);
                if !self.check(&TokenKind::Comma) {
                    break;
                }
                self.advance();
            }
            self.expect(&TokenKind::RParen)?;
            self.expect(&TokenKind::Arrow)?;
            let return_type = self.parse_type_annotation()?;
            return Ok(TypeAnnotation::Function {
                params,
                return_type: Box::new(return_type),
            });
        }

        let name = self.expect_identifier()?;

        // Check for generic: Type<T1, T2>
        if self.check(&TokenKind::Lt) {
            self.advance();
            let mut params = Vec::new();
            while !self.check(&TokenKind::Gt) {
                params.push(self.parse_type_annotation()?);
                if !self.check(&TokenKind::Comma) {
                    break;
                }
                self.advance();
            }
            self.expect(&TokenKind::Gt)?;
            Ok(TypeAnnotation::Generic { name, params })
        } else {
            Ok(TypeAnnotation::Simple(name))
        }
    }

    // ========================================
    // Helpers
    // ========================================

    fn parse_block_body(&mut self) -> VietResult<Vec<Statement>> {
        let mut stmts = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            stmts.push(self.parse_statement()?);
        }
        Ok(stmts)
    }

    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.pos];
        if !self.is_at_end() {
            self.pos += 1;
        }
        token
    }

    fn check(&self, kind: &TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(&self.tokens[self.pos].kind) == std::mem::discriminant(kind)
    }

    fn expect(&mut self, kind: &TokenKind) -> VietResult<&Token> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            Err(self.error(&format!(
                "Expected {:?}, found {:?}",
                kind,
                self.current().kind
            )))
        }
    }

    fn expect_identifier(&mut self) -> VietResult<String> {
        let token = self.current().clone();
        match &token.kind {
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            // Also accept Self_ as identifier in certain contexts
            TokenKind::Self_ => {
                self.advance();
                Ok("self".to_string())
            }
            _ => Err(self.error(&format!("Expected identifier, found {:?}", token.kind))),
        }
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len() || matches!(self.tokens[self.pos].kind, TokenKind::Eof)
    }

    fn error(&self, message: &str) -> VietError {
        let token = if self.pos < self.tokens.len() {
            &self.tokens[self.pos]
        } else {
            self.tokens.last().unwrap()
        };
        VietError::parse_error(
            message.to_string(),
            token.span.line,
            token.span.column,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse(source: &str) -> Program {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse().unwrap()
    }

    #[test]
    fn test_let_statement() {
        let program = parse("let x = 42");
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Let { name, mutable, value, .. } => {
                assert_eq!(name, "x");
                assert!(!mutable);
                match value {
                    Expression::IntLiteral { value, .. } => assert_eq!(*value, 42),
                    _ => panic!("Expected IntLiteral"),
                }
            }
            _ => panic!("Expected Let statement"),
        }
    }

    #[test]
    fn test_let_mutable() {
        let program = parse("let mut count = 0");
        match &program.statements[0] {
            Statement::Let { name, mutable, .. } => {
                assert_eq!(name, "count");
                assert!(*mutable);
            }
            _ => panic!("Expected Let statement"),
        }
    }

    #[test]
    fn test_function() {
        let program = parse("fn add(a: Int, b: Int) -> Int { return a + b }");
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Function { name, params, .. } => {
                assert_eq!(name, "add");
                assert_eq!(params.len(), 2);
            }
            _ => panic!("Expected Function statement"),
        }
    }

    #[test]
    fn test_if_else() {
        let program = parse("if x > 0 { print(x) } else { print(0) }");
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::If { else_body, .. } => {
                assert!(else_body.is_some());
            }
            _ => panic!("Expected If statement"),
        }
    }

    #[test]
    fn test_struct() {
        let program = parse("struct User { name: String, age: Int }");
        match &program.statements[0] {
            Statement::Struct { name, fields, .. } => {
                assert_eq!(name, "User");
                assert_eq!(fields.len(), 2);
            }
            _ => panic!("Expected Struct"),
        }
    }

    #[test]
    fn test_binary_expression() {
        let program = parse("let x = 1 + 2 * 3");
        match &program.statements[0] {
            Statement::Let { value, .. } => {
                match value {
                    Expression::BinaryOp { op, .. } => {
                        assert_eq!(*op, BinaryOperator::Add);
                    }
                    _ => panic!("Expected BinaryOp"),
                }
            }
            _ => panic!("Expected Let"),
        }
    }

    #[test]
    fn test_array_literal() {
        let program = parse("let arr = [1, 2, 3]");
        match &program.statements[0] {
            Statement::Let { value, .. } => {
                match value {
                    Expression::ArrayLiteral { elements, .. } => {
                        assert_eq!(elements.len(), 3);
                    }
                    _ => panic!("Expected ArrayLiteral"),
                }
            }
            _ => panic!("Expected Let"),
        }
    }

    #[test]
    fn test_for_loop() {
        let program = parse("for i in 0..10 { print(i) }");
        match &program.statements[0] {
            Statement::For { variable, .. } => {
                assert_eq!(variable, "i");
            }
            _ => panic!("Expected For"),
        }
    }

    #[test]
    fn test_match_expression() {
        let program = parse(r#"
            let result = match x {
                1 => "one",
                2 => "two",
                _ => "other"
            }
        "#);
        assert_eq!(program.statements.len(), 1);
    }
}
