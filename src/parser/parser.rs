use super::ast::*;
use crate::error::ParseError;
use crate::lexer::{Lexer, Span, Token};
use crate::types::Type;

// ── Parser ────────────────────────────────────────────────────────────────────

pub struct Parser {
    tokens: Vec<(Token, Span)>,
    current: usize,
    errors: Vec<ParseError>,
}

impl Parser {
    pub fn new(tokens: Vec<(Token, Span)>) -> Self {
        // Guarantee a final EOF so every peek() is always in-bounds.
        let mut tokens = tokens;
        if !matches!(tokens.last(), Some((Token::Eof, _))) {
            let span = tokens.last().map(|(_, s)| s.clone()).unwrap_or_default();
            tokens.push((Token::Eof, span));
        }
        Self {
            tokens,
            current: 0,
            errors: Vec::new(),
        }
    }

    // ── Navigation ────────────────────────────────────────────────────────────

    #[inline]
    fn peek(&self) -> &Token {
        // Safe: constructor guarantees EOF at end; clamp to last token.
        &self.tokens[self.current.min(self.tokens.len() - 1)].0
    }

    #[inline]
    fn peek_span(&self) -> Span {
        self.tokens[self.current.min(self.tokens.len() - 1)]
            .1
            .clone()
    }

    fn peek_nth(&self, n: usize) -> &Token {
        self.tokens
            .get(self.current + n)
            .map(|(t, _)| t)
            .unwrap_or(&Token::Eof)
    }

    /// Consume and return the current token. Never panics.
    fn advance(&mut self) -> Token {
        let idx = self.current.min(self.tokens.len() - 1);
        let tok = self.tokens[idx].0.clone();
        if !self.is_at_end() {
            self.current += 1;
        }
        tok
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::Eof)
    }

    fn check(&self, token: &Token) -> bool {
        !self.is_at_end() && self.peek() == token
    }

    fn match_token(&mut self, token: Token) -> bool {
        if self.check(&token) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume(&mut self, token: Token, ctx: &str) -> Result<(), ParseError> {
        if self.check(&token) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::new(
                format!("{}: expected {:?}, found {:?}", ctx, token, self.peek()),
                self.peek_span(),
            ))
        }
    }

    fn consume_ident(&mut self, ctx: &str) -> Result<String, ParseError> {
        match self.peek().clone() {
            Token::Ident(name) => {
                self.advance();
                Ok(name)
            }
            other => Err(ParseError::new(
                format!("{}: expected identifier, found {:?}", ctx, other),
                self.peek_span(),
            )),
        }
    }

    fn consume_str_literal(&mut self, ctx: &str) -> Result<String, ParseError> {
        match self.peek().clone() {
            Token::Str(s) => {
                self.advance();
                Ok(s)
            }
            other => Err(ParseError::new(
                format!("{}: expected string literal, found {:?}", ctx, other),
                self.peek_span(),
            )),
        }
    }

    fn consume_newlines(&mut self) {
        while self.match_token(Token::Newline) {}
    }

    // ── Error recovery ────────────────────────────────────────────────────────

    fn record(&mut self, err: ParseError) {
        self.errors.push(err);
    }

    /// Skip tokens until we reach a safe statement boundary.
    fn synchronize(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                Token::Newline => {
                    self.advance();
                    return;
                }
                Token::Dedent => return,
                Token::Fn
                | Token::Cls
                | Token::If
                | Token::While
                | Token::For
                | Token::Return
                | Token::Load
                | Token::Export => return,
                _ => {
                    self.advance();
                }
            }
        }
    }

    // ── Public API ────────────────────────────────────────────────────────────

    /// Parse all statements, collecting recoverable errors. Returns `Err` only
    /// when at least one error was recorded; successfully parsed statements are
    /// always included in the `Ok` branch too via `parse_partial`.
    pub fn parse(&mut self) -> Result<Vec<Stmt>, Vec<ParseError>> {
        let (stmts, errors) = self.parse_partial();
        if errors.is_empty() {
            Ok(stmts)
        } else {
            Err(errors)
        }
    }

    /// Always returns whatever was successfully parsed alongside every error.
    pub fn parse_partial(&mut self) -> (Vec<Stmt>, Vec<ParseError>) {
        let mut stmts = Vec::new();
        while !self.is_at_end() {
            if self.match_token(Token::Newline) {
                continue;
            }
            match self.parse_statement() {
                Ok(stmt) => stmts.push(stmt),
                Err(err) => {
                    self.record(err);
                    self.synchronize();
                }
            }
        }
        (stmts, std::mem::take(&mut self.errors))
    }

    // ── Statements ────────────────────────────────────────────────────────────

    fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(Token::Export) {
            return Ok(Stmt::Export(Box::new(self.parse_statement()?)));
        }
        if self.match_token(Token::Fn) {
            return self.parse_fn_def();
        }
        if self.match_token(Token::Cls) {
            return self.parse_cls_def();
        }
        if self.match_token(Token::If) {
            return self.parse_if();
        }
        if self.match_token(Token::While) {
            return self.parse_while();
        }
        if self.match_token(Token::For) {
            return self.parse_for();
        }
        if self.match_token(Token::Load) {
            return self.parse_load();
        }
        if self.match_token(Token::Return) {
            return self.parse_return();
        }
        if self.match_token(Token::Break) {
            self.expect_stmt_end("after break")?;
            return Ok(Stmt::Break);
        }
        if self.match_token(Token::Continue) {
            self.expect_stmt_end("after continue")?;
            return Ok(Stmt::Continue);
        }
        self.parse_assignment_or_expr()
    }

    fn parse_return(&mut self) -> Result<Stmt, ParseError> {
        let has_value =
            !self.is_at_end() && !self.check(&Token::Newline) && !self.check(&Token::Dedent);
        let val = if has_value {
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.expect_stmt_end("after return")?;
        Ok(Stmt::Return(val))
    }

    /// Consume a newline that terminates a statement, unless we're already at a
    /// dedent or EOF (both are also valid statement terminators).
    fn expect_stmt_end(&mut self, ctx: &str) -> Result<(), ParseError> {
        if self.is_at_end() || self.check(&Token::Dedent) {
            return Ok(());
        }
        self.consume(Token::Newline, &format!("expected newline {}", ctx))
    }

    fn parse_assignment_or_expr(&mut self) -> Result<Stmt, ParseError> {
        let lhs_span = self.peek_span();
        let expr = self.parse_expr()?;

        // Detect assignment operator (plain or compound).
        let compound: Option<Option<BinOp>> = if self.match_token(Token::Assign) {
            Some(None)
        } else if self.match_token(Token::PlusEq) {
            Some(Some(BinOp::Plus))
        } else if self.match_token(Token::MinusEq) {
            Some(Some(BinOp::Minus))
        } else if self.match_token(Token::StarEq) {
            Some(Some(BinOp::Star))
        } else if self.match_token(Token::SlashEq) {
            Some(Some(BinOp::Slash))
        } else {
            None
        };

        if let Some(op) = compound {
            let rhs = self.parse_expr()?;
            let value = match op {
                Some(bin) => Expr::BinOp {
                    op: bin,
                    left: Box::new(expr.clone()),
                    right: Box::new(rhs),
                },
                None => rhs,
            };
            self.expect_stmt_end("after assignment")?;
            return match expr {
                Expr::Ident(name) => Ok(Stmt::Assign {
                    target: name,
                    ty: None,
                    value,
                }),
                Expr::Field { obj, field } => Ok(Stmt::FieldAssign {
                    obj: *obj,
                    field,
                    value,
                }),
                Expr::Index { obj, idx } => Ok(Stmt::IndexAssign {
                    obj: *obj,
                    idx: *idx,
                    value,
                }),
                _ => Err(ParseError::new("invalid assignment target", lhs_span)),
            };
        }

        self.finish_expr_stmt(expr)
    }

    fn finish_expr_stmt(&mut self, expr: Expr) -> Result<Stmt, ParseError> {
        // Typed assignment: `name Type = value`
        if let Expr::Ident(name) = &expr {
            let checkpoint = self.current;
            if let Some(ty) = self.try_parse_type()? {
                if self.match_token(Token::Assign) {
                    let val = self.parse_expr()?;
                    self.expect_stmt_end("after typed assignment")?;
                    return Ok(Stmt::Assign {
                        target: name.clone(),
                        ty: Some(ty),
                        value: val,
                    });
                }
                // Not a typed assignment — silently roll back.
                self.current = checkpoint;
            }
        }
        self.expect_stmt_end("after expression")?;
        Ok(Stmt::Expr(expr))
    }

    // ── Expressions ───────────────────────────────────────────────────────────

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_prec(Precedence::Lowest)?;

        // Postfix ternary: `value if cond else alt`
        if self.match_token(Token::If) {
            let cond = self.parse_expr()?;
            self.consume(Token::Else, "expected 'else' in conditional expression")?;
            let alt = self.parse_expr()?;
            return Ok(Expr::Ternary {
                cond: Box::new(cond),
                then: Box::new(expr),
                else_: Box::new(alt),
            });
        }
        Ok(expr)
    }

    fn parse_prec(&mut self, min_prec: Precedence) -> Result<Expr, ParseError> {
        let mut left = self.parse_prefix()?;
        loop {
            let prec = Precedence::of(self.peek());
            if prec <= min_prec {
                break;
            }
            left = self.parse_infix(left)?;
        }
        Ok(left)
    }

    fn parse_prefix(&mut self) -> Result<Expr, ParseError> {
        let span = self.peek_span();
        let tok = self.advance();
        match tok {
            Token::Int(n) => Ok(Expr::Int(n)),
            Token::Float(n) => Ok(Expr::Float(n)),
            Token::Bool(b) => Ok(Expr::Bool(b)),
            Token::Null => Ok(Expr::Null),
            Token::Ident(s) => Ok(Expr::Ident(s)),
            Token::Error => Ok(Expr::Ident("error".into())),

            Token::Str(s) => self
                .parse_interpolated_string(&s)
                .map_err(|err| ParseError::new(err.message, span)),

            Token::Minus => Ok(Expr::UnaryOp {
                op: UnaryOp::Minus,
                expr: Box::new(self.parse_prec(Precedence::Prefix)?),
            }),
            Token::Not => Ok(Expr::UnaryOp {
                op: UnaryOp::Not,
                expr: Box::new(self.parse_prec(Precedence::Prefix)?),
            }),

            Token::LParen => {
                let e = self.parse_expr()?;
                self.consume(Token::RParen, "expected ')' to close group")?;
                Ok(e)
            }
            Token::LBracket => self.parse_list(),
            Token::LBrace => self.parse_map(),
            Token::Fn => self.parse_lambda(),

            Token::Spawn => Ok(Expr::Spawn(Box::new(self.parse_expr()?))),

            other => Err(ParseError::new(
                format!("unexpected token {:?} in expression", other),
                span,
            )),
        }
    }

    fn parse_list(&mut self) -> Result<Expr, ParseError> {
        let mut items = Vec::new();
        while !self.check(&Token::RBracket) && !self.is_at_end() {
            items.push(self.parse_expr()?);
            if !self.match_token(Token::Comma) {
                break;
            }
            // trailing comma: fall through to check `]`
        }
        self.consume(Token::RBracket, "expected ']' to close list")?;
        Ok(Expr::List(items))
    }

    fn parse_map(&mut self) -> Result<Expr, ParseError> {
        let mut pairs = Vec::new();
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            let key = self.parse_expr()?;
            self.consume(Token::Colon, "expected ':' in map literal")?;
            let val = self.parse_expr()?;
            pairs.push((key, val));
            if !self.match_token(Token::Comma) {
                break;
            }
        }
        self.consume(Token::RBrace, "expected '}' to close map")?;
        Ok(Expr::Map(pairs))
    }

    fn parse_infix(&mut self, left: Expr) -> Result<Expr, ParseError> {
        let span = self.peek_span();
        let tok = self.advance();

        match &tok {
            // Binary operators
            Token::Plus
            | Token::Minus
            | Token::Star
            | Token::Slash
            | Token::Percent
            | Token::EqEq
            | Token::BangEq
            | Token::Lt
            | Token::Gt
            | Token::LtEq
            | Token::GtEq
            | Token::And
            | Token::Or
            | Token::DotDot => {
                let op = BinOp::from_token(&tok).unwrap();
                let prec = Precedence::of(&tok);
                let right = self.parse_prec(prec)?;
                Ok(Expr::BinOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }

            // Call
            Token::LParen => {
                let args = self.parse_call_args()?;
                self.consume(Token::RParen, "expected ')' to close call")?;
                Ok(Expr::Call {
                    callee: Box::new(left),
                    args,
                })
            }

            // Field access
            Token::Dot => {
                let field = self.consume_ident("expected field name after '.'")?;
                Ok(Expr::Field {
                    obj: Box::new(left),
                    field,
                })
            }

            // Index
            Token::LBracket => {
                let idx = self.parse_expr()?;
                self.consume(Token::RBracket, "expected ']' to close index")?;
                Ok(Expr::Index {
                    obj: Box::new(left),
                    idx: Box::new(idx),
                })
            }

            // Error propagation
            Token::Question => Ok(Expr::Propagate(Box::new(left))),

            other => Err(ParseError::new(
                format!("unexpected token {:?} in infix position", other),
                span,
            )),
        }
    }

    fn parse_call_args(&mut self) -> Result<Vec<CallArg>, ParseError> {
        let mut args = Vec::new();
        while !self.check(&Token::RParen) && !self.is_at_end() {
            // Named argument: `name = expr`
            let arg = if matches!(self.peek(), Token::Ident(_))
                && matches!(self.peek_nth(1), Token::Assign)
            {
                let name = self.consume_ident("expected argument name")?;
                self.advance(); // consume `=`
                CallArg {
                    name: Some(name),
                    expr: self.parse_expr()?,
                }
            } else {
                CallArg {
                    name: None,
                    expr: self.parse_expr()?,
                }
            };
            args.push(arg);
            if !self.match_token(Token::Comma) {
                break;
            }
        }
        Ok(args)
    }

    // ── Functions & Lambdas ───────────────────────────────────────────────────

    fn parse_fn_def(&mut self) -> Result<Stmt, ParseError> {
        let name = self.consume_ident("expected function name")?;
        self.consume(Token::LParen, "expected '(' after function name")?;
        let params = self.parse_param_list()?;
        self.consume(Token::RParen, "expected ')' after parameters")?;
        let ret_ty = self.parse_ret_ty()?;

        let body = if self.match_token(Token::Assign) {
            // Short form: `fn f() = expr`
            let expr = self.parse_expr()?;
            self.expect_stmt_end("after single-expression function")?;
            vec![Stmt::Return(Some(expr))]
        } else {
            self.consume(Token::Colon, "expected ':' before function body")?;
            self.consume_newlines();
            self.parse_block()?
        };

        Ok(Stmt::FnDef(FnDef {
            name,
            params,
            ret_ty,
            body,
        }))
    }

    fn parse_lambda(&mut self) -> Result<Expr, ParseError> {
        self.consume(Token::LParen, "expected '(' for lambda parameters")?;
        let params = self.parse_param_list()?;
        self.consume(Token::RParen, "expected ')' after lambda parameters")?;
        let ret_ty = self.parse_ret_ty()?;

        let body = if self.match_token(Token::Assign) {
            let expr = self.parse_expr()?;
            vec![Stmt::Return(Some(expr))]
        } else {
            self.consume(Token::Colon, "expected ':' before lambda body")?;
            self.consume_newlines();
            self.parse_block()?
        };

        Ok(Expr::Lambda {
            params,
            ret_ty,
            body,
        })
    }

    fn parse_param_list(&mut self) -> Result<Vec<Param>, ParseError> {
        let mut params = Vec::new();
        while !self.check(&Token::RParen) && !self.is_at_end() {
            let name = self.consume_ident("expected parameter name")?;
            let ty = self.try_parse_type()?;
            params.push(Param { name, ty });
            if !self.match_token(Token::Comma) {
                break;
            }
        }
        Ok(params)
    }

    fn parse_ret_ty(&mut self) -> Result<Option<Type>, ParseError> {
        if self.match_token(Token::Arrow) {
            Ok(Some(self.parse_type()?))
        } else {
            Ok(None)
        }
    }

    // ── Classes ───────────────────────────────────────────────────────────────

    fn parse_cls_def(&mut self) -> Result<Stmt, ParseError> {
        let name = self.consume_ident("expected class name")?;
        self.consume(Token::Colon, "expected ':' after class name")?;
        self.consume_newlines();
        self.consume(Token::Indent, "expected indented block for class body")?;

        let mut fields = Vec::new();
        while !self.check(&Token::Dedent) && !self.is_at_end() {
            if self.match_token(Token::Newline) {
                continue;
            }
            let field_name = self.consume_ident("expected field name")?;
            let ty = self.try_parse_type()?.ok_or_else(|| {
                ParseError::new(
                    format!("field '{}' requires a type annotation", field_name),
                    self.peek_span(),
                )
            })?;
            fields.push(Param {
                name: field_name,
                ty: Some(ty),
            });
            if !self.check(&Token::Dedent) {
                self.expect_stmt_end("after class field")?;
            }
        }
        self.consume(Token::Dedent, "expected dedent after class body")?;
        Ok(Stmt::ClsDef(ClsDef { name, fields }))
    }

    // ── Blocks ────────────────────────────────────────────────────────────────

    fn parse_block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        self.consume(Token::Indent, "expected indented block")?;
        let mut stmts = Vec::new();

        while !self.check(&Token::Dedent) && !self.is_at_end() {
            if self.match_token(Token::Newline) {
                continue;
            }

            let before = self.current;
            match self.parse_statement() {
                Ok(stmt) => stmts.push(stmt),
                Err(err) => {
                    self.record(err);
                    self.synchronize();
                }
            }
            // Progress guard: if nothing was consumed, force advance to
            // prevent an infinite loop on a completely unknown token.
            if self.current == before {
                self.advance();
            }
        }

        self.consume(Token::Dedent, "expected dedent after block")?;
        Ok(stmts)
    }

    // ── Control flow ──────────────────────────────────────────────────────────

    fn parse_if(&mut self) -> Result<Stmt, ParseError> {
        let cond = self.parse_expr()?;
        self.consume(Token::Colon, "expected ':' after if condition")?;
        self.consume_newlines();
        let then = self.parse_block()?;

        let mut elifs = Vec::new();
        while self.match_token(Token::Elif) {
            let c = self.parse_expr()?;
            self.consume(Token::Colon, "expected ':' after elif condition")?;
            self.consume_newlines();
            elifs.push((c, self.parse_block()?));
        }

        let else_ = if self.match_token(Token::Else) {
            self.consume(Token::Colon, "expected ':' after else")?;
            self.consume_newlines();
            Some(self.parse_block()?)
        } else {
            None
        };

        Ok(Stmt::If {
            cond,
            then,
            elifs,
            else_,
        })
    }

    fn parse_while(&mut self) -> Result<Stmt, ParseError> {
        let cond = self.parse_expr()?;
        self.consume(Token::Colon, "expected ':' after while condition")?;
        self.consume_newlines();
        let body = self.parse_block()?;
        Ok(Stmt::While { cond, body })
    }

    fn parse_for(&mut self) -> Result<Stmt, ParseError> {
        let var1 = self.consume_ident("expected loop variable")?;

        if self.match_token(Token::Comma) {
            let var2 = self.consume_ident("expected second loop variable")?;
            self.consume(Token::In, "expected 'in' in for-kv loop")?;
            let iter = self.parse_expr()?;
            if self.check(&Token::Step) {
                return Err(ParseError::new(
                    "'step' is not supported in key-value for loops",
                    self.peek_span(),
                ));
            }
            self.consume(Token::Colon, "expected ':' after for-kv expression")?;
            self.consume_newlines();
            let body = self.parse_block()?;
            return Ok(Stmt::ForKV {
                key: var1,
                val: var2,
                iter,
                body,
            });
        }

        self.consume(Token::In, "expected 'in' in for loop")?;
        let iter = self.parse_expr()?;
        let step = if self.match_token(Token::Step) {
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.consume(Token::Colon, "expected ':' after for-in expression")?;
        self.consume_newlines();
        let body = self.parse_block()?;
        Ok(Stmt::For {
            var: var1,
            iter,
            step,
            body,
        })
    }

    fn parse_load(&mut self) -> Result<Stmt, ParseError> {
        let alias = self.consume_ident("expected module name in load")?;
        let source = if self.match_token(Token::From) {
            self.consume_str_literal("expected source string after 'from'")?
        } else {
            alias.clone()
        };
        self.expect_stmt_end("after load")?;
        Ok(Stmt::Load { alias, source })
    }

    // ── Types ─────────────────────────────────────────────────────────────────

    fn is_type_start(&self) -> bool {
        matches!(
            self.peek(),
            Token::Ident(_) | Token::LBracket | Token::LBrace | Token::Error
        )
    }

    /// Attempt to parse a type, rolling back silently if the tokens don't form
    /// a valid type. Never returns a hard error.
    fn try_parse_type(&mut self) -> Result<Option<Type>, ParseError> {
        if !self.is_type_start() {
            return Ok(None);
        }
        let checkpoint = self.current;
        match self.parse_type() {
            Ok(ty) => Ok(Some(ty)),
            Err(_) => {
                self.current = checkpoint;
                Ok(None)
            }
        }
    }

    fn parse_type(&mut self) -> Result<Type, ParseError> {
        let mut tys = vec![self.parse_type_primary()?];
        while self.match_token(Token::Pipe) {
            tys.push(self.parse_type_primary()?);
        }
        Ok(if tys.len() == 1 {
            tys.remove(0)
        } else {
            Type::Union(tys)
        })
    }

    fn parse_type_primary(&mut self) -> Result<Type, ParseError> {
        let span = self.peek_span();

        if self.match_token(Token::LBracket) {
            let inner = self.parse_type()?;
            self.consume(Token::RBracket, "expected ']' in list type")?;
            return Ok(Type::List(Box::new(inner)));
        }

        if self.match_token(Token::LBrace) {
            let key = self.parse_type()?;
            self.consume(Token::Colon, "expected ':' in map type")?;
            let val = self.parse_type()?;
            self.consume(Token::RBrace, "expected '}' in map type")?;
            return Ok(Type::Map(Box::new(key), Box::new(val)));
        }

        match self.advance() {
            Token::Error => Ok(Type::Error(Box::new(Type::Any))),
            Token::Ident(name) => Ok(match name.as_str() {
                "int" => Type::Int,
                "float" => Type::Float,
                "str" => Type::Str,
                "bool" => Type::Bool,
                "null" => Type::Null,
                "any" => Type::Any,
                _ => Type::Struct(name),
            }),
            Token::Null => Ok(Type::Null),
            other => Err(ParseError::new(
                format!("expected type annotation, found {:?}", other),
                span,
            )),
        }
    }

    // ── String interpolation ──────────────────────────────────────────────────

    fn parse_interpolated_string(&mut self, raw: &str) -> Result<Expr, ParseError> {
        if !raw.contains('{') {
            return Ok(Expr::Str(raw.to_string()));
        }

        let mut parts: Vec<InterpPart> = Vec::new();
        let mut buf = String::new();
        let mut chars = raw.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '{' if matches!(chars.peek(), Some('{')) => {
                    chars.next();
                    buf.push('{');
                }
                '{' => {
                    if !buf.is_empty() {
                        parts.push(InterpPart::Str(std::mem::take(&mut buf)));
                    }
                    let src = Self::consume_interpolation(&mut chars)?;
                    let expr = Self::parse_inline_expr(&src)?;
                    parts.push(InterpPart::Expr(expr));
                }
                '}' if matches!(chars.peek(), Some('}')) => {
                    chars.next();
                    buf.push('}');
                }
                other => buf.push(other),
            }
        }

        if !buf.is_empty() {
            parts.push(InterpPart::Str(buf));
        }

        Ok(match parts.as_slice() {
            [InterpPart::Str(s)] => Expr::Str(s.clone()),
            _ => Expr::Interp(parts),
        })
    }

    fn consume_interpolation(
        chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
    ) -> Result<String, ParseError> {
        let mut out = String::new();
        let mut depth: usize = 0;
        let mut in_str: Option<char> = None;

        while let Some(c) = chars.next() {
            if let Some(q) = in_str {
                out.push(c);
                if c == '\\' {
                    // Consume escape so a `\"` inside `{}` doesn't close the string.
                    if let Some(esc) = chars.next() {
                        out.push(esc);
                    }
                    continue;
                }
                if c == q {
                    in_str = None;
                }
                continue;
            }
            match c {
                '"' | '\'' => {
                    in_str = Some(c);
                    out.push(c);
                }
                '{' => {
                    depth += 1;
                    out.push(c);
                }
                '}' if depth > 0 => {
                    depth -= 1;
                    out.push(c);
                }
                '}' => return Ok(out), // depth == 0: end of interpolation
                _ => out.push(c),
            }
        }

        Err(ParseError::new(
            "unterminated string interpolation",
            Span::default(),
        ))
    }

    fn parse_inline_expr(src: &str) -> Result<Expr, ParseError> {
        let tokens = Lexer::new(src)
            .tokenize()
            .map_err(|e| ParseError::new(e.message, e.span))?;
        let mut p = Parser::new(tokens);
        let expr = p.parse_expr()?;
        if !p.is_at_end() && !matches!(p.peek(), Token::Newline) {
            return Err(ParseError::new(
                format!("unexpected token {:?} in interpolation", p.peek()),
                p.peek_span(),
            ));
        }
        Ok(expr)
    }
}

// ── Precedence ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Lowest,
    Or,
    And,
    Equals,
    LessGreater,
    Range,
    Sum,
    Product,
    Prefix,
    Call,
    Field,
    Post,
}

impl Precedence {
    pub fn of(token: &Token) -> Self {
        match token {
            Token::Or => Self::Or,
            Token::And => Self::And,
            Token::EqEq | Token::BangEq => Self::Equals,
            Token::Lt | Token::Gt | Token::LtEq | Token::GtEq => Self::LessGreater,
            Token::DotDot => Self::Range,
            Token::Plus | Token::Minus => Self::Sum,
            Token::Star | Token::Slash | Token::Percent => Self::Product,
            Token::LParen => Self::Call,
            Token::Dot | Token::LBracket => Self::Field,
            Token::Question => Self::Post,
            _ => Self::Lowest,
        }
    }
}

// ── BinOp helper ─────────────────────────────────────────────────────────────

impl BinOp {
    fn from_token(token: &Token) -> Option<Self> {
        Some(match token {
            Token::Plus => Self::Plus,
            Token::Minus => Self::Minus,
            Token::Star => Self::Star,
            Token::Slash => Self::Slash,
            Token::Percent => Self::Percent,
            Token::EqEq => Self::EqEq,
            Token::BangEq => Self::BangEq,
            Token::Lt => Self::Lt,
            Token::Gt => Self::Gt,
            Token::LtEq => Self::LtEq,
            Token::GtEq => Self::GtEq,
            Token::And => Self::And,
            Token::Or => Self::Or,
            Token::DotDot => Self::DotDot,
            _ => return None,
        })
    }
}
