use crate::ast::*;
use crate::errors::{LexError, ParseError};
use crate::lexer::{Lexer, Span, Token, TokenKind};

/// Recursive-descent parser with Pratt precedence climbing and panic-mode
/// error recovery: when a parse error is encountered, the parser records it,
/// skips to the next synchronisation token, and continues producing a partial AST.
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token,
    next: Token,
    /// Used for diagnostic messages.
    source: &'a str,
    /// Lex errors collected during parsing.
    lex_errors: Vec<LexError>,
    /// Parse errors collected during recovery. The first error is also returned
    /// by [`parse`] so that existing callers continue to see `Err`.
    parse_errors: Vec<ParseError>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Result<Self, ParseError> {
        let mut lexer = Lexer::new(source);
        let current = lexer.next_token().map_err(|e| ParseError::Lex { err: e })?;
        let next = lexer.next_token().map_err(|e| ParseError::Lex { err: e })?;
        Ok(Parser {
            lexer,
            current,
            next,
            source,
            lex_errors: Vec::new(),
            parse_errors: Vec::new(),
        })
    }

    /// Drain lex errors accumulated during parsing.
    pub fn drain_lex_errors(&mut self) -> Vec<LexError> {
        let mut errors = self.lexer.drain_errors();
        errors.append(&mut self.lex_errors);
        errors
    }

    /// Drain parse errors accumulated during error recovery.
    pub fn drain_parse_errors(&mut self) -> Vec<ParseError> {
        std::mem::take(&mut self.parse_errors)
    }

    /// Panic-mode recovery: skip tokens until one of the synchronisation
    /// tokens is found, then consume it (unless it is Eof).
    fn recover(&mut self, sync: &[TokenKind]) {
        loop {
            if self.check(&TokenKind::Eof) {
                return;
            }
            if sync.contains(&self.current.kind) {
                return;
            }
            self.advance_ignoring_errors();
        }
    }

    /// Advance past the current token even when the lexer returns an error.
    fn advance_ignoring_errors(&mut self) {
        self.current = std::mem::replace(
            &mut self.next,
            Token::new(TokenKind::Eof, Span::new(0, 0, 0)),
        );
        let next = match self.lexer.next_token() {
            Ok(tok) => tok,
            Err(e) => {
                self.lex_errors.push(e);
                Token::new(TokenKind::Eof, Span::new(1, 1, 0))
            }
        };
        self.next = next;
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let start_span = self.current.span;
        let mut statements = Vec::new();

        loop {
            if self.check(&TokenKind::Eof) {
                break;
            }
            if self.check(&TokenKind::Dedent) {
                let tok = self.current.clone();
                self.parse_errors
                    .push(ParseError::unexpected_token(self.source, &tok));
                self.advance();
                continue;
            }
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => {
                    self.parse_errors.push(e);
                    self.recover(&[TokenKind::Newline, TokenKind::Dedent, TokenKind::Eof]);
                }
            }
            if self.check(&TokenKind::Newline) {
                self.advance();
            }
        }

        let end_span = self.current.span;

        if self.parse_errors.is_empty() {
            Ok(Program {
                statements,
                span: Span::new_with_len(
                    start_span.line,
                    start_span.column,
                    start_span.byte_index,
                    end_span.byte_index + end_span.length - start_span.byte_index,
                ),
            })
        } else {
            // Return the first error (backward compat) without draining,
            // so drain_parse_errors() still has all accumulated errors.
            Err(self.parse_errors[0].clone())
        }
    }

    fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        let is_pub = if self.check(&TokenKind::Pub) {
            self.advance();
            true
        } else {
            false
        };

        let stmt = match &self.current.kind {
            TokenKind::Fn => {
                if matches!(&self.next.kind, TokenKind::LParen) {
                    let expr = self.parse_lambda()?;
                    Ok(Stmt::Expr(expr))
                } else {
                    self.parse_function_def()
                }
            }
            TokenKind::Defer => self.parse_defer(),
            TokenKind::Macro => self.parse_macro_def(),
            TokenKind::Extern => self.parse_extern_fn(),
            TokenKind::Struct => self.parse_struct_def(),
            TokenKind::Interface => self.parse_interface_def(),
            TokenKind::Let => self.parse_let_or_var(false),
            TokenKind::Var => self.parse_let_or_var(true),
            TokenKind::If => self.parse_if(),
            TokenKind::While => self.parse_while(),
            TokenKind::Break => self.parse_break(),
            TokenKind::Continue => self.parse_continue(),
            TokenKind::For => self.parse_for(),
            TokenKind::Return => self.parse_return(),
            TokenKind::Load => self.parse_load(is_pub),
            _ => {
                if is_pub {
                    let tok = self.current.clone();
                    return Err(ParseError::unexpected_token(self.source, &tok));
                }
                self.parse_expr_stmt()
            }
        }?;

        Ok(stmt)
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        // The `:` has already been consumed by the caller.
        if self.check(&TokenKind::Newline) {
            self.advance();
        }
        if !self.check(&TokenKind::Indent) {
            let tok = self.current.clone();
            return Err(ParseError::expected_indented_block(self.source, &tok));
        }
        self.advance(); // consume Indent

        let mut stmts = Vec::new();
        while !self.check(&TokenKind::Dedent) && !self.check(&TokenKind::Eof) {
            match self.parse_statement() {
                Ok(stmt) => stmts.push(stmt),
                Err(e) => {
                    self.parse_errors.push(e);
                    self.recover(&[TokenKind::Newline, TokenKind::Dedent, TokenKind::Eof]);
                }
            }
            if self.check(&TokenKind::Newline) {
                self.advance();
            }
        }

        if self.check(&TokenKind::Dedent) {
            self.advance();
        }
        Ok(stmts)
    }

    fn parse_extern_fn(&mut self) -> Result<Stmt, ParseError> {
        let start = self.current.span;
        self.advance(); // consume `extern`

        if !self.check(&TokenKind::Fn) {
            let tok = self.current.clone();
            return Err(ParseError::expected_token(self.source, &tok, "'fn'"));
        }
        self.advance(); // consume `fn`

        let name = self.expect_identifier()?;

        if !self.check(&TokenKind::LParen) {
            let tok = self.current.clone();
            return Err(ParseError::expected_token(self.source, &tok, "'('"));
        }
        self.advance(); // consume `(`

        let params = self.parse_params()?;

        if !self.check(&TokenKind::RParen) {
            let tok = self.current.clone();
            return Err(ParseError::expected_token(self.source, &tok, "')'"));
        }
        self.advance(); // consume `)`

        let return_type = if self.check(&TokenKind::Arrow) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        Ok(Stmt::ExternFn {
            name,
            params,
            return_type,
            span: self.merge_span(&start, &self.current.span),
        })
    }

    fn parse_struct_def(&mut self) -> Result<Stmt, ParseError> {
        let start = self.current.span;
        self.advance();
        let name = self.expect_identifier()?;
        if !self.check(&TokenKind::Colon) {
            return Err(ParseError::expected_token(
                self.source,
                &self.current,
                "':'",
            ));
        }
        self.advance();
        let body = self.parse_block()?;
        let mut fields = Vec::new();
        for stmt in body {
            match stmt {
                Stmt::Let {
                    name,
                    type_annot: Some(type_annot),
                    span,
                    ..
                }
                | Stmt::Var {
                    name,
                    type_annot: Some(type_annot),
                    span,
                    ..
                } => {
                    fields.push(Param {
                        name,
                        type_annot,
                        span,
                    });
                }
                _ => return Err(ParseError::unexpected_token(self.source, &self.current)),
            }
        }
        Ok(Stmt::StructDef {
            name,
            fields,
            span: self.merge_span(&start, &self.current.span),
        })
    }

    fn parse_interface_def(&mut self) -> Result<Stmt, ParseError> {
        let start = self.current.span;
        self.advance();
        let name = self.expect_identifier()?;
        if !self.check(&TokenKind::Colon) {
            return Err(ParseError::expected_token(
                self.source,
                &self.current,
                "':'",
            ));
        }
        self.advance();
        let mut methods = Vec::new();
        if self.check(&TokenKind::Newline) {
            self.advance();
        }
        if !self.check(&TokenKind::Indent) {
            return Err(ParseError::expected_indented_block(
                self.source,
                &self.current,
            ));
        }
        self.advance();
        while !self.check(&TokenKind::Dedent) && !self.check(&TokenKind::Eof) {
            let method_span = self.current.span;
            if !self.check(&TokenKind::Fn) {
                return Err(ParseError::expected_token(
                    self.source,
                    &self.current,
                    "'fn'",
                ));
            }
            self.advance();
            let method_name = self.expect_identifier()?;
            if !self.check(&TokenKind::LParen) {
                return Err(ParseError::expected_token(
                    self.source,
                    &self.current,
                    "'('",
                ));
            }
            self.advance();
            let params = self.parse_params()?;
            if !self.check(&TokenKind::RParen) {
                return Err(ParseError::expected_token(
                    self.source,
                    &self.current,
                    "')'",
                ));
            }
            self.advance();
            if self.check(&TokenKind::Arrow) {
                self.advance();
                let _ = self.parse_type()?;
            }
            if let Some(first) = params.into_iter().next() {
                methods.push(Param {
                    name: method_name,
                    type_annot: first.type_annot,
                    span: method_span,
                });
            } else {
                return Err(ParseError::expected_identifier(self.source, &self.current));
            }
            if self.check(&TokenKind::Newline) {
                self.advance();
            }
        }
        if self.check(&TokenKind::Dedent) {
            self.advance();
        }
        Ok(Stmt::InterfaceDef {
            name,
            methods,
            span: self.merge_span(&start, &self.current.span),
        })
    }

    fn parse_function_def(&mut self) -> Result<Stmt, ParseError> {
        let start = self.current.span;
        self.advance(); // consume `fn`

        let name = self.expect_identifier()?;

        if !self.check(&TokenKind::LParen) {
            let tok = self.current.clone();
            return Err(ParseError::expected_token(self.source, &tok, "'('"));
        }
        self.advance(); // consume `(`

        let params = self.parse_params()?;

        if !self.check(&TokenKind::RParen) {
            let tok = self.current.clone();
            return Err(ParseError::expected_token(self.source, &tok, "')'"));
        }
        self.advance(); // consume `)`

        // Optional return type: `-> Type`
        let return_type = if self.check(&TokenKind::Arrow) {
            self.advance(); // consume `->`
            Some(self.parse_type()?)
        } else {
            None
        };

        if !self.check(&TokenKind::Colon) {
            let tok = self.current.clone();
            return Err(ParseError::expected_token(self.source, &tok, "':'"));
        }
        self.advance(); // consume `:`

        let body = self.parse_block()?;

        Ok(Stmt::FunctionDef {
            name,
            params,
            return_type,
            body,
            span: self.merge_span(&start, &self.current.span),
        })
    }

    fn parse_params(&mut self) -> Result<Vec<Param>, ParseError> {
        let mut params = Vec::new();
        loop {
            if self.check(&TokenKind::RParen) || self.check(&TokenKind::Eof) {
                break;
            }
            let param_span = self.current.span;
            let name = self.expect_identifier()?;

            if !self.check(&TokenKind::Colon) {
                let tok = self.current.clone();
                return Err(ParseError::expected_token(self.source, &tok, "':'"));
            }
            self.advance(); // consume `:`
            let type_annot = self.parse_type()?;

            params.push(Param {
                name,
                type_annot,
                span: param_span,
            });

            if self.check(&TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        Ok(params)
    }

    fn parse_let_or_var(&mut self, mutable: bool) -> Result<Stmt, ParseError> {
        let start = self.current.span;
        self.advance(); // consume `let` / `var`

        let name = self.expect_identifier()?;

        // Optional type annotation: `: Type`
        let type_annot = if self.check(&TokenKind::Colon) {
            self.advance(); // consume `:`
            Some(self.parse_type()?)
        } else {
            None
        };

        if !self.check(&TokenKind::Equal) {
            let tok = self.current.clone();
            return Err(ParseError::expected_token(self.source, &tok, "'='"));
        }
        self.advance(); // consume `=`

        let value = self.parse_expr()?;

        let span = self.merge_span(&start, &self.current.span);
        if mutable {
            Ok(Stmt::Var {
                name,
                type_annot,
                value,
                span,
            })
        } else {
            Ok(Stmt::Let {
                name,
                type_annot,
                value,
                span,
            })
        }
    }

    fn parse_if(&mut self) -> Result<Stmt, ParseError> {
        let start = self.current.span;
        self.advance(); // consume `if`

        let condition = self.parse_expr()?;

        if !self.check(&TokenKind::Colon) {
            let tok = self.current.clone();
            return Err(ParseError::expected_token(self.source, &tok, "':'"));
        }
        self.advance(); // consume `:`

        let body = self.parse_block()?;

        // Parse `elif` branches
        let mut elifs = Vec::new();
        while self.check(&TokenKind::Elif) {
            self.advance();

            let elif_cond = self.parse_expr()?;

            if !self.check(&TokenKind::Colon) {
                let tok = self.current.clone();
                return Err(ParseError::expected_token(self.source, &tok, "':'"));
            }
            self.advance(); // consume `:`

            let elif_body = self.parse_block()?;
            elifs.push((elif_cond, elif_body));
        }

        // Parse optional `else` branch
        let else_body = if self.check(&TokenKind::Else) {
            let _else_span = self.current.span;
            self.advance(); // consume `else`

            if !self.check(&TokenKind::Colon) {
                let tok = self.current.clone();
                return Err(ParseError::expected_token(self.source, &tok, "':'"));
            }
            self.advance(); // consume `:`

            Some(self.parse_block()?)
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            body,
            elifs,
            else_body,
            span: self.merge_span(&start, &self.current.span),
        })
    }

    fn parse_while(&mut self) -> Result<Stmt, ParseError> {
        let start = self.current.span;
        self.advance(); // consume `while`

        let condition = self.parse_expr()?;

        if !self.check(&TokenKind::Colon) {
            let tok = self.current.clone();
            return Err(ParseError::expected_token(self.source, &tok, "':'"));
        }
        self.advance(); // consume `:`

        let body = self.parse_block()?;

        Ok(Stmt::While {
            condition,
            body,
            span: self.merge_span(&start, &self.current.span),
        })
    }

    fn parse_break(&mut self) -> Result<Stmt, ParseError> {
        let span = self.current.span;
        self.advance();
        Ok(Stmt::Break { span })
    }

    fn parse_continue(&mut self) -> Result<Stmt, ParseError> {
        let span = self.current.span;
        self.advance();
        Ok(Stmt::Continue { span })
    }

    fn parse_for(&mut self) -> Result<Stmt, ParseError> {
        let start = self.current.span;
        self.advance(); // consume `for`

        let variable = self.expect_identifier()?;

        if !self.check(&TokenKind::In) {
            let tok = self.current.clone();
            return Err(ParseError::expected_token(self.source, &tok, "'in'"));
        }
        self.advance(); // consume `in`

        let iterable = self.parse_expr()?;

        if !self.check(&TokenKind::Colon) {
            let tok = self.current.clone();
            return Err(ParseError::expected_token(self.source, &tok, "':'"));
        }
        self.advance(); // consume `:`

        let body = self.parse_block()?;

        Ok(Stmt::For {
            variable,
            iterable,
            body,
            span: self.merge_span(&start, &self.current.span),
        })
    }

    fn parse_return(&mut self) -> Result<Stmt, ParseError> {
        let start = self.current.span;
        self.advance(); // consume `return`

        // If the next token starts an expression, parse it; otherwise
        // it's a bare `return`.
        let value = if self.can_start_expr() {
            Some(self.parse_expr()?)
        } else {
            None
        };

        Ok(Stmt::Return {
            value,
            span: self.merge_span(&start, &self.current.span),
        })
    }

    fn parse_load(&mut self, is_pub: bool) -> Result<Stmt, ParseError> {
        let start = self.current.span;
        self.advance(); // consume `load`

        // Parse the module path (e.g. `std.io`, `./lexer`, `../parser.ast`, `c.printf`)
        let mut module_path = Vec::new();
        if self.check(&TokenKind::Dot) {
            module_path.push(self.parse_relative_prefix()?);
            module_path.push(self.expect_identifier()?);
        } else {
            module_path.push(self.expect_identifier()?);
        }

        while self.check(&TokenKind::Dot) || self.check(&TokenKind::Slash) {
            self.advance();
            module_path.push(self.expect_identifier()?);
        }

        // Parse optional `as alias`
        let alias = if self.check(&TokenKind::As) {
            self.advance(); // consume `as`
            Some(self.expect_identifier()?)
        } else {
            None
        };

        // Parse optional `::{sym1, sym2}` selective import
        let symbols = if self.check(&TokenKind::DoubleColon) {
            self.advance(); // consume `::`
            if !self.check(&TokenKind::LBrace) {
                let tok = self.current.clone();
                return Err(ParseError::expected_token(self.source, &tok, "'{'"));
            }
            self.advance(); // consume `{`
            let mut syms = Vec::new();
            loop {
                if self.check(&TokenKind::RBrace) || self.check(&TokenKind::Eof) {
                    break;
                }
                syms.push(self.expect_identifier()?);
                if self.check(&TokenKind::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
            if !self.check(&TokenKind::RBrace) {
                let tok = self.current.clone();
                return Err(ParseError::expected_token(self.source, &tok, "'}'"));
            }
            self.advance(); // consume `}`
            Some(syms)
        } else {
            None
        };

        Ok(Stmt::Load {
            module_path,
            symbols,
            alias,
            is_pub,
            span: self.merge_span(&start, &self.current.span),
        })
    }

    fn parse_relative_prefix(&mut self) -> Result<String, ParseError> {
        self.advance(); // consume `.`
        if self.check(&TokenKind::Dot) {
            self.advance(); // consume second `.`
            if !self.check(&TokenKind::Slash) {
                let tok = self.current.clone();
                return Err(ParseError::expected_token(
                    self.source,
                    &tok,
                    "'/' after '..'",
                ));
            }
            self.advance(); // consume `/`
            Ok("..".to_string())
        } else if self.check(&TokenKind::Slash) {
            self.advance(); // consume `/`
            Ok(".".to_string())
        } else {
            let tok = self.current.clone();
            Err(ParseError::expected_token(
                self.source,
                &tok,
                "'/' after '.'",
            ))
        }
    }

    fn parse_expr_stmt(&mut self) -> Result<Stmt, ParseError> {
        if !self.can_start_expr() {
            let tok = self.current.clone();
            return Err(ParseError::expected_expression(self.source, &tok));
        }
        let expr = self.parse_expr()?;
        Ok(Stmt::Expr(expr))
    }

    /// Precedence: 1=assign, 2=||, 3=&&, 4=eq, 5=comp, 6=add, 7=mul, 8=unary
    fn get_prefix_precedence(kind: &TokenKind) -> Option<u8> {
        match kind {
            TokenKind::Minus | TokenKind::Bang => Some(8),
            _ => None,
        }
    }

    fn get_infix_precedence(kind: &TokenKind) -> Option<(u8, bool)> {
        match kind {
            TokenKind::Dot => Some((10, false)),
            TokenKind::Equal
            | TokenKind::PlusEqual
            | TokenKind::MinusEqual
            | TokenKind::StarEqual
            | TokenKind::SlashEqual
            | TokenKind::PercentEqual => Some((1, true)),
            TokenKind::PipePipe => Some((2, false)),
            TokenKind::AmpersandAmpersand => Some((3, false)),
            TokenKind::EqualEqual | TokenKind::NotEqual => Some((4, false)),
            TokenKind::Less
            | TokenKind::Greater
            | TokenKind::LessEqual
            | TokenKind::GreaterEqual => Some((5, false)),
            TokenKind::Plus | TokenKind::Minus => Some((6, false)),
            TokenKind::Star | TokenKind::Slash | TokenKind::Percent => Some((7, false)),
            _ => None,
        }
    }

    fn can_start_expr(&self) -> bool {
        matches!(
            &self.current.kind,
            TokenKind::IntLiteral(_)
                | TokenKind::FloatLiteral(_)
                | TokenKind::StringLiteral(_)
                | TokenKind::True
                | TokenKind::False
                | TokenKind::Identifier(_)
                | TokenKind::LParen
                | TokenKind::Minus
                | TokenKind::Bang
                | TokenKind::Fn
        )
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_expr_prec(0)
    }

    fn parse_expr_prec(&mut self, min_prec: u8) -> Result<Expr, ParseError> {
        let mut left = self.parse_prefix()?;

        loop {
            // Check for postfix operators (function call, member access).
            if self.check(&TokenKind::LParen) {
                left = self.parse_call(left)?;
                continue;
            }

            if self.check(&TokenKind::Dot) {
                self.advance(); // consume `.`
                let member = self.expect_identifier()?;
                let span = self.merge_span(&left.span(), &self.current.span);
                left = Expr::MemberAccess {
                    object: Box::new(left),
                    member,
                    span,
                };
                continue;
            }

            if self.check(&TokenKind::As) {
                self.advance(); // consume `as`
                let target_type = self.parse_type()?;
                let span = self.merge_span(&left.span(), &target_type.span);
                left = Expr::Cast {
                    expr: Box::new(left),
                    target_type,
                    span,
                };
                continue;
            }

            if self.check(&TokenKind::LBrace)
                && let Expr::Identifier(ref name, _) = left
            {
                left = self.parse_struct_literal(name.clone(), left.span())?;
                continue;
            }

            // Check for binary operators.
            if let Some((prec, right_assoc)) = Self::get_infix_precedence(&self.current.kind)
                && prec >= min_prec
            {
                let next_min = if right_assoc { prec } else { prec + 1 };
                let op_token = self.current.clone();
                self.advance();
                let right = self.parse_expr_prec(next_min)?;
                let span = self.merge_span(&left.span(), &right.span());

                if matches!(&op_token.kind, TokenKind::Equal) {
                    left = Expr::Assign {
                        target: Box::new(left),
                        value: Box::new(right),
                        span,
                    };
                } else if matches!(
                    &op_token.kind,
                    TokenKind::PlusEqual
                        | TokenKind::MinusEqual
                        | TokenKind::StarEqual
                        | TokenKind::SlashEqual
                        | TokenKind::PercentEqual
                ) {
                    let compound_op = match &op_token.kind {
                        TokenKind::PlusEqual => BinaryOp::Add,
                        TokenKind::MinusEqual => BinaryOp::Sub,
                        TokenKind::StarEqual => BinaryOp::Mul,
                        TokenKind::SlashEqual => BinaryOp::Div,
                        TokenKind::PercentEqual => BinaryOp::Mod,
                        _ => {
                            return Err(ParseError::Internal {
                                msg: format!(
                                    "unexpected compound assignment token {:?}",
                                    op_token.kind
                                ),
                                src: self.source.to_string(),
                                span: (op_token.span.byte_index, op_token.span.length.max(1))
                                    .into(),
                            });
                        }
                    };
                    let target = Box::new(left.clone());
                    left = Expr::Assign {
                        target,
                        value: Box::new(Expr::Binary {
                            left: Box::new(left),
                            op: compound_op,
                            right: Box::new(right),
                            span,
                        }),
                        span,
                    };
                } else {
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: Self::token_to_binary_op(&op_token.kind),
                        right: Box::new(right),
                        span,
                    };
                }
                continue;
            }

            break;
        }

        Ok(left)
    }

    fn parse_prefix(&mut self) -> Result<Expr, ParseError> {
        // Unary operators: `-expr`, `!expr`
        if let Some(prec) = Self::get_prefix_precedence(&self.current.kind) {
            let op_token = self.current.clone();
            self.advance();
            let operand = self.parse_expr_prec(prec)?;
            let span = self.merge_span(&op_token.span, &operand.span());
            return Ok(Expr::Unary {
                op: Self::token_to_unary_op(&op_token.kind),
                operand: Box::new(operand),
                span,
            });
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let token = self.current.clone();
        match &token.kind {
            TokenKind::IntLiteral(n) => {
                self.advance();
                Ok(Expr::IntLiteral(*n, token.span))
            }
            TokenKind::FloatLiteral(f) => {
                self.advance();
                Ok(Expr::FloatLiteral(*f, token.span))
            }
            TokenKind::StringLiteral(s) => {
                self.advance();
                Ok(Expr::StringLiteral(s.clone(), token.span))
            }
            TokenKind::True => {
                self.advance();
                Ok(Expr::BoolLiteral(true, token.span))
            }
            TokenKind::False => {
                self.advance();
                Ok(Expr::BoolLiteral(false, token.span))
            }
            TokenKind::Identifier(s) => {
                self.advance();
                Ok(Expr::Identifier(s.clone(), token.span))
            }
            TokenKind::LParen => {
                self.advance(); // consume `(`
                let inner = self.parse_expr()?;
                if !self.check(&TokenKind::RParen) {
                    let lparen = token.span;
                    return Err(ParseError::UnclosedParen {
                        line: lparen.line,
                        column: lparen.column,
                        src: self.source.to_string(),
                        span: (lparen.byte_index, lparen.length.max(1)).into(),
                    });
                }
                let rparen = self.current.span;
                self.advance(); // consume `)`
                let span = self.merge_span(&token.span, &rparen);
                Ok(Expr::Grouping {
                    expr: Box::new(inner),
                    span,
                })
            }
            TokenKind::Fn => self.parse_lambda(),
            _ => Err(ParseError::expected_expression(self.source, &token)),
        }
    }

    fn parse_call(&mut self, callee: Expr) -> Result<Expr, ParseError> {
        let lparen = self.current.span;
        self.advance(); // consume `(`

        let mut args = Vec::new();
        loop {
            if self.check(&TokenKind::RParen) || self.check(&TokenKind::Eof) {
                break;
            }
            args.push(self.parse_expr()?);
            if self.check(&TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        if !self.check(&TokenKind::RParen) {
            // Unterminated argument list – report the error at the
            // opening `(`.
            return Err(ParseError::UnclosedParen {
                line: lparen.line,
                column: lparen.column,
                src: self.source.to_string(),
                span: (lparen.byte_index, lparen.length.max(1)).into(),
            });
        }
        let rparen = self.current.span;
        self.advance(); // consume `)`

        let span = self.merge_span(&callee.span(), &rparen);
        Ok(Expr::Call {
            callee: Box::new(callee),
            args,
            span,
        })
    }

    fn parse_struct_literal(&mut self, name: String, start: Span) -> Result<Expr, ParseError> {
        self.advance(); // consume {
        let mut fields = Vec::new();
        loop {
            if self.check(&TokenKind::RBrace) || self.check(&TokenKind::Eof) {
                break;
            }
            let field_name = self.expect_identifier()?;
            if !self.check(&TokenKind::Colon) {
                return Err(ParseError::expected_token(
                    self.source,
                    &self.current,
                    "':'",
                ));
            }
            self.advance();
            let expr = self.parse_expr()?;
            fields.push((field_name, expr));
            if self.check(&TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        if !self.check(&TokenKind::RBrace) {
            return Err(ParseError::expected_token(
                self.source,
                &self.current,
                "'}'",
            ));
        }
        let end = self.current.span;
        self.advance();
        Ok(Expr::StructLiteral {
            name,
            fields,
            span: self.merge_span(&start, &end),
        })
    }

    fn parse_type(&mut self) -> Result<Type, ParseError> {
        if !matches!(&self.current.kind, TokenKind::Identifier(_)) {
            let tok = self.current.clone();
            return Err(ParseError::ExpectedType {
                line: tok.span.line,
                column: tok.span.column,
                src: self.source.to_string(),
                span: (tok.span.byte_index, tok.span.length.max(1)).into(),
            });
        }
        let token = self.current.clone();
        let name = match &token.kind {
            TokenKind::Identifier(s) => s.clone(),
            _ => {
                return Err(ParseError::ExpectedType {
                    line: token.span.line,
                    column: token.span.column,
                    src: self.source.to_string(),
                    span: (token.span.byte_index, token.span.length.max(1)).into(),
                });
            }
        };
        self.advance();
        let mut args = Vec::new();
        if self.check(&TokenKind::LBracket) {
            self.advance();
            loop {
                if self.check(&TokenKind::RBracket) || self.check(&TokenKind::Eof) {
                    break;
                }
                args.push(self.parse_type()?);
                if self.check(&TokenKind::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
            if !self.check(&TokenKind::RBracket) {
                return Err(ParseError::expected_token(
                    self.source,
                    &self.current,
                    "']'",
                ));
            }
            self.advance();
        }
        Ok(Type {
            name,
            args,
            span: token.span,
        })
    }

    fn check(&self, kind: &TokenKind) -> bool {
        &self.current.kind == kind
    }

    fn advance(&mut self) -> Token {
        let next = match self.lexer.next_token() {
            Ok(tok) => tok,
            Err(e) => {
                self.lex_errors.push(e);
                Token::new(TokenKind::Eof, Span::new(1, 1, 0))
            }
        };
        std::mem::replace(&mut self.current, std::mem::replace(&mut self.next, next))
    }

    fn expect_identifier(&mut self) -> Result<String, ParseError> {
        if let TokenKind::Identifier(s) = &self.current.kind {
            let name = s.clone();
            self.advance();
            Ok(name)
        } else {
            let tok = self.current.clone();
            Err(ParseError::expected_identifier(self.source, &tok))
        }
    }

    fn merge_span(&self, from: &Span, to: &Span) -> Span {
        let end = to.byte_index + to.length;
        Span::new_with_len(
            from.line,
            from.column,
            from.byte_index,
            end - from.byte_index,
        )
    }

    fn token_to_binary_op(kind: &TokenKind) -> BinaryOp {
        match kind {
            TokenKind::Plus => BinaryOp::Add,
            TokenKind::Minus => BinaryOp::Sub,
            TokenKind::Star => BinaryOp::Mul,
            TokenKind::Slash => BinaryOp::Div,
            TokenKind::EqualEqual => BinaryOp::Equal,
            TokenKind::NotEqual => BinaryOp::NotEqual,
            TokenKind::Less => BinaryOp::Less,
            TokenKind::Greater => BinaryOp::Greater,
            TokenKind::LessEqual => BinaryOp::LessEqual,
            TokenKind::GreaterEqual => BinaryOp::GreaterEqual,
            TokenKind::AmpersandAmpersand => BinaryOp::And,
            TokenKind::PipePipe => BinaryOp::Or,
            TokenKind::Percent => BinaryOp::Mod,
            _ => BinaryOp::Add, // unreachable guarded by get_infix_precedence
        }
    }

    fn token_to_unary_op(kind: &TokenKind) -> UnaryOp {
        match kind {
            TokenKind::Minus => UnaryOp::Negate,
            TokenKind::Bang => UnaryOp::Not,
            _ => UnaryOp::Negate,
        }
    }

    fn parse_lambda(&mut self) -> Result<Expr, ParseError> {
        let start = self.current.span;
        self.advance(); // consume `fn`

        if !self.check(&TokenKind::LParen) {
            let tok = self.current.clone();
            return Err(ParseError::expected_token(self.source, &tok, "'('"));
        }
        self.advance(); // consume `(`

        let params = self.parse_params()?;

        if !self.check(&TokenKind::RParen) {
            let tok = self.current.clone();
            return Err(ParseError::expected_token(self.source, &tok, "')'"));
        }
        self.advance(); // consume `)`

        let return_type = if self.check(&TokenKind::Arrow) {
            self.advance(); // consume `->`
            Some(self.parse_type()?)
        } else {
            None
        };

        if !self.check(&TokenKind::Colon) {
            let tok = self.current.clone();
            return Err(ParseError::expected_token(self.source, &tok, "':'"));
        }
        self.advance(); // consume `:`

        let body = self.parse_block()?;

        Ok(Expr::Lambda {
            params,
            return_type,
            body,
            span: self.merge_span(&start, &self.current.span),
        })
    }

    fn parse_defer(&mut self) -> Result<Stmt, ParseError> {
        let start = self.current.span;
        self.advance(); // consume `defer`

        if self.check(&TokenKind::Colon) {
            self.advance(); // consume `:`
            let body = self.parse_block()?;
            Ok(Stmt::Defer {
                body,
                span: self.merge_span(&start, &self.current.span),
            })
        } else {
            let expr = self.parse_expr_stmt()?;
            let body = vec![expr];
            Ok(Stmt::Defer {
                body,
                span: self.merge_span(&start, &self.current.span),
            })
        }
    }

    fn parse_macro_params(&mut self) -> Result<Vec<String>, ParseError> {
        let mut params = Vec::new();
        loop {
            if self.check(&TokenKind::RParen) || self.check(&TokenKind::Eof) {
                break;
            }
            let name = self.expect_identifier()?;
            params.push(name);
            if self.check(&TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        Ok(params)
    }

    fn parse_macro_def(&mut self) -> Result<Stmt, ParseError> {
        let start = self.current.span;
        self.advance(); // consume `macro`

        let name = self.expect_identifier()?;

        if !self.check(&TokenKind::LParen) {
            let tok = self.current.clone();
            return Err(ParseError::expected_token(self.source, &tok, "'('"));
        }
        self.advance(); // consume `(`

        let params = self.parse_macro_params()?;

        if !self.check(&TokenKind::RParen) {
            let tok = self.current.clone();
            return Err(ParseError::expected_token(self.source, &tok, "')'"));
        }
        self.advance(); // consume `)`

        if !self.check(&TokenKind::Colon) {
            let tok = self.current.clone();
            return Err(ParseError::expected_token(self.source, &tok, "':'"));
        }
        self.advance(); // consume `:`

        let body = self.parse_block()?;

        Ok(Stmt::MacroDef {
            name,
            params,
            body,
            span: self.merge_span(&start, &self.current.span),
        })
    }
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::IntLiteral(_, s)
            | Expr::FloatLiteral(_, s)
            | Expr::StringLiteral(_, s)
            | Expr::BoolLiteral(_, s)
            | Expr::Identifier(_, s) => *s,
            Expr::Binary { span, .. }
            | Expr::Unary { span, .. }
            | Expr::Call { span, .. }
            | Expr::Assign { span, .. }
            | Expr::Grouping { span, .. }
            | Expr::MemberAccess { span, .. }
            | Expr::StructLiteral { span, .. }
            | Expr::Cast { span, .. }
            | Expr::Lambda { span, .. }
            | Expr::MacroInvocation { span, .. } => *span,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(source: &str) -> Program {
        let mut parser = Parser::new(source).expect("parser creation failed");
        parser.parse().expect("parse failed")
    }

    fn stmt_kinds(source: &str) -> Vec<&'static str> {
        let prog = parse(source);
        prog.statements.iter().map(|s| stmt_tag(s)).collect()
    }

    fn stmt_tag(stmt: &Stmt) -> &'static str {
        match stmt {
            Stmt::Let { .. } => "let",
            Stmt::Var { .. } => "var",
            Stmt::FunctionDef { .. } => "fn",
            Stmt::StructDef { .. } => "struct",
            Stmt::InterfaceDef { .. } => "interface",
            Stmt::If { .. } => "if",
            Stmt::While { .. } => "while",
            Stmt::Break { .. } => "break",
            Stmt::Continue { .. } => "continue",
            Stmt::For { .. } => "for",
            Stmt::Return { .. } => "return",
            Stmt::ExternFn { .. } => "extern fn",
            Stmt::Load { .. } => "load",
            Stmt::Expr(_) => "expr",
            Stmt::Defer { .. } => "defer",
            Stmt::MacroDef { .. } => "macro",
        }
    }

    #[test]
    fn load_statement() {
        let src = "load std.io\n";
        let prog = parse(src);
        match &prog.statements[0] {
            Stmt::Load { module_path, .. } => {
                assert_eq!(module_path, &["std", "io"]);
            }
            _ => panic!("expected Load statement"),
        }
    }

    #[test]
    fn load_pub_statement() {
        let src = "pub load std.io\n";
        let prog = parse(src);
        match &prog.statements[0] {
            Stmt::Load {
                module_path,
                is_pub,
                ..
            } => {
                assert_eq!(module_path, &["std", "io"]);
                assert!(*is_pub);
            }
            _ => panic!("expected Load statement"),
        }
    }

    #[test]
    fn load_relative_path() {
        let src = "load ./lexer\nload ../parser.ast\nload crate.parser\n";
        let prog = parse(src);
        assert_eq!(prog.statements.len(), 3);
        match &prog.statements[0] {
            Stmt::Load { module_path, .. } => assert_eq!(module_path, &[".", "lexer"]),
            _ => panic!("expected Load statement"),
        }
        match &prog.statements[1] {
            Stmt::Load { module_path, .. } => assert_eq!(module_path, &["..", "parser", "ast"]),
            _ => panic!("expected Load statement"),
        }
        match &prog.statements[2] {
            Stmt::Load { module_path, .. } => assert_eq!(module_path, &["crate", "parser"]),
            _ => panic!("expected Load statement"),
        }
    }

    #[test]
    fn let_decl() {
        let prog = parse("let x = 5\n");
        assert_eq!(prog.statements.len(), 1);
        match &prog.statements[0] {
            Stmt::Let {
                name,
                type_annot,
                value,
                ..
            } => {
                assert_eq!(name, "x");
                assert!(type_annot.is_none());
                assert!(matches!(value, Expr::IntLiteral(5, _)));
            }
            _ => panic!("expected Let"),
        }
    }

    #[test]
    fn let_with_type() {
        let prog = parse("let x: Int = 5\n");
        match &prog.statements[0] {
            Stmt::Let {
                name, type_annot, ..
            } => {
                assert_eq!(name, "x");
                assert_eq!(type_annot.as_ref().unwrap().name, "Int");
            }
            _ => panic!("expected Let"),
        }
    }

    #[test]
    fn let_with_generic_type() {
        let prog = parse("let xs: Box[Int] = value\n");
        match &prog.statements[0] {
            Stmt::Let { type_annot, .. } => {
                let ty = type_annot.as_ref().unwrap();
                assert_eq!(ty.name, "Box");
                assert_eq!(ty.args[0].name, "Int");
            }
            _ => panic!("expected Let"),
        }
    }

    #[test]
    fn interface_signature() {
        let prog = parse("interface Drawable:\n    fn draw(self: Drawable) -> Void\n");
        match &prog.statements[0] {
            Stmt::InterfaceDef { name, methods, .. } => {
                assert_eq!(name, "Drawable");
                assert_eq!(methods[0].name, "draw");
            }
            _ => panic!("expected InterfaceDef"),
        }
    }

    #[test]
    fn var_decl() {
        let prog = parse("var name: String = \"hello\"\n");
        match &prog.statements[0] {
            Stmt::Var {
                name,
                type_annot,
                value,
                ..
            } => {
                assert_eq!(name, "name");
                assert_eq!(type_annot.as_ref().unwrap().name, "String");
                assert!(matches!(value, Expr::StringLiteral(s, _) if s == "hello"));
            }
            _ => panic!("expected Var"),
        }
    }

    #[test]
    fn function_no_args() {
        let prog = parse("fn main():\n    return 0\n");
        assert_eq!(stmt_kinds("fn main():\n    return 0\n"), vec!["fn"]);
        match &prog.statements[0] {
            Stmt::FunctionDef {
                name,
                params,
                return_type,
                body,
                ..
            } => {
                assert_eq!(name, "main");
                assert!(params.is_empty());
                assert!(return_type.is_none());
                assert_eq!(body.len(), 1);
                assert!(matches!(body[0], Stmt::Return { .. }));
            }
            _ => panic!("expected FunctionDef"),
        }
    }

    #[test]
    fn function_with_params_and_return() {
        let src = "fn add(a: Int, b: Int) -> Int:\n    return a + b\n";
        let prog = parse(src);
        match &prog.statements[0] {
            Stmt::FunctionDef {
                name,
                params,
                return_type,
                body,
                ..
            } => {
                assert_eq!(name, "add");
                assert_eq!(params.len(), 2);
                assert_eq!(params[0].name, "a");
                assert_eq!(params[0].type_annot.name, "Int");
                assert_eq!(params[1].name, "b");
                assert_eq!(params[1].type_annot.name, "Int");
                assert_eq!(return_type.as_ref().unwrap().name, "Int");
                assert_eq!(body.len(), 1);
            }
            _ => panic!("expected FunctionDef"),
        }
    }

    #[test]
    fn if_statement() {
        let src = "if x > 10:\n    let y = 1\n";
        let prog = parse(src);
        match &prog.statements[0] {
            Stmt::If {
                condition,
                body,
                elifs,
                else_body,
                ..
            } => {
                assert!(matches!(condition, Expr::Binary { .. }));
                assert_eq!(body.len(), 1);
                assert!(elifs.is_empty());
                assert!(else_body.is_none());
            }
            _ => panic!("expected If"),
        }
    }

    #[test]
    fn if_elif_else_chain() {
        let src = "\
if x > 10:
    let y = 1
elif x > 5:
    let y = 2
else:
    let y = 3
";
        let prog = parse(src);
        match &prog.statements[0] {
            Stmt::If {
                body,
                elifs,
                else_body,
                ..
            } => {
                assert_eq!(body.len(), 1);
                assert_eq!(elifs.len(), 1);
                assert!(else_body.is_some());
                assert_eq!(else_body.as_ref().unwrap().len(), 1);
            }
            _ => panic!("expected If"),
        }
    }

    #[test]
    fn while_loop() {
        let src = "while x > 0:\n    x = x - 1\n";
        let prog = parse(src);
        match &prog.statements[0] {
            Stmt::While {
                condition, body, ..
            } => {
                assert!(matches!(condition, Expr::Binary { .. }));
                assert_eq!(body.len(), 1);
            }
            _ => panic!("expected While"),
        }
    }

    #[test]
    fn for_loop() {
        let src = "for i in range:\n    print(i)\n";
        let prog = parse(src);
        match &prog.statements[0] {
            Stmt::For {
                variable,
                iterable,
                body,
                ..
            } => {
                assert_eq!(variable, "i");
                assert!(matches!(iterable, Expr::Identifier(name, _) if name == "range"));
                assert_eq!(body.len(), 1);
            }
            _ => panic!("expected For"),
        }
    }

    #[test]
    fn binary_expression_precedence() {
        let prog = parse("let x = 1 + 2 * 3\n");
        match &prog.statements[0] {
            Stmt::Let { value, .. } => match value {
                Expr::Binary {
                    left, op, right, ..
                } => {
                    assert_eq!(*op, BinaryOp::Add);
                    assert!(matches!(left.as_ref(), Expr::IntLiteral(1, _)));
                    match right.as_ref() {
                        Expr::Binary {
                            left: rl,
                            op: rop,
                            right: rr,
                            ..
                        } => {
                            assert_eq!(*rop, BinaryOp::Mul);
                            assert!(matches!(rl.as_ref(), Expr::IntLiteral(2, _)));
                            assert!(matches!(rr.as_ref(), Expr::IntLiteral(3, _)));
                        }
                        _ => panic!("expected Mul binary"),
                    }
                }
                _ => panic!("expected Add binary"),
            },
            _ => panic!("expected Let"),
        }
    }

    #[test]
    fn comparison_chaining() {
        let prog = parse("let r = a == b\n");
        match &prog.statements[0] {
            Stmt::Let { value, .. } => {
                assert!(matches!(
                    value,
                    Expr::Binary {
                        op: BinaryOp::Equal,
                        ..
                    }
                ));
            }
            _ => panic!("expected Let"),
        }
    }

    #[test]
    fn logical_operators() {
        let prog = parse("let r = a && b || c\n");
        match &prog.statements[0] {
            Stmt::Let { value, .. } => match value {
                Expr::Binary {
                    op: BinaryOp::Or,
                    left,
                    ..
                } => match left.as_ref() {
                    Expr::Binary {
                        op: BinaryOp::And, ..
                    } => {}
                    _ => panic!("expected And left"),
                },
                _ => panic!("expected Or at top"),
            },
            _ => panic!("expected Let"),
        }
    }

    #[test]
    fn function_call() {
        let prog = parse("let r = foo(x, y + 1)\n");
        match &prog.statements[0] {
            Stmt::Let { value, .. } => match value {
                Expr::Call { callee, args, .. } => {
                    assert!(matches!(
                        callee.as_ref(),
                        Expr::Identifier(name, _) if name == "foo"
                    ));
                    assert_eq!(args.len(), 2);
                }
                _ => panic!("expected Call"),
            },
            _ => panic!("expected Let"),
        }
    }

    #[test]
    fn nested_calls() {
        let prog = parse("let r = f(g(x))\n");
        match &prog.statements[0] {
            Stmt::Let { value, .. } => match value {
                Expr::Call { callee, args, .. } => {
                    assert!(matches!(
                        callee.as_ref(),
                        Expr::Identifier(name, _) if name == "f"
                    ));
                    assert_eq!(args.len(), 1);
                    assert!(matches!(&args[0], Expr::Call { .. }));
                }
                _ => panic!("expected Call"),
            },
            _ => panic!("expected Let"),
        }
    }

    #[test]
    fn grouping() {
        let prog = parse("let r = (1 + 2) * 3\n");
        match &prog.statements[0] {
            Stmt::Let { value, .. } => match value {
                Expr::Binary {
                    op, left, right, ..
                } => {
                    assert_eq!(*op, BinaryOp::Mul);
                    match left.as_ref() {
                        Expr::Grouping { expr, .. } => match expr.as_ref() {
                            Expr::Binary { op: add_op, .. } => {
                                assert_eq!(*add_op, BinaryOp::Add);
                            }
                            _ => panic!("expected Add inside grouping"),
                        },
                        _ => panic!("expected Grouping"),
                    }
                    assert!(matches!(right.as_ref(), Expr::IntLiteral(3, _)));
                }
                _ => panic!("expected Mul"),
            },
            _ => panic!("expected Let"),
        }
    }

    #[test]
    fn unary_minus() {
        let prog = parse("let r = -5\n");
        match &prog.statements[0] {
            Stmt::Let { value, .. } => match value {
                Expr::Unary { op, operand, .. } => {
                    assert_eq!(*op, UnaryOp::Negate);
                    assert!(matches!(operand.as_ref(), Expr::IntLiteral(5, _)));
                }
                _ => panic!("expected Unary"),
            },
            _ => panic!("expected Let"),
        }
    }

    #[test]
    fn unary_not() {
        let prog = parse("let r = !true\n");
        match &prog.statements[0] {
            Stmt::Let { value, .. } => match value {
                Expr::Unary { op, operand, .. } => {
                    assert_eq!(*op, UnaryOp::Not);
                    assert!(matches!(operand.as_ref(), Expr::BoolLiteral(true, _)));
                }
                _ => panic!("expected Unary"),
            },
            _ => panic!("expected Let"),
        }
    }

    #[test]
    fn lambda_expression() {
        let src = "let f = fn(x: Int, y: Int) -> Int:\n    return x + y\n";
        let prog = parse(src);
        match &prog.statements[0] {
            Stmt::Let { value, .. } => match value {
                Expr::Lambda {
                    params,
                    return_type,
                    body,
                    ..
                } => {
                    assert_eq!(params.len(), 2);
                    assert_eq!(params[0].name, "x");
                    assert_eq!(params[1].name, "y");
                    assert!(return_type.is_some());
                    assert_eq!(body.len(), 1);
                }
                _ => panic!("expected Lambda"),
            },
            _ => panic!("expected Let"),
        }
    }

    #[test]
    fn lambda_no_return() {
        let src = "let f = fn(x: Int):\n    let y = x\n";
        let prog = parse(src);
        match &prog.statements[0] {
            Stmt::Let { value, .. } => match value {
                Expr::Lambda {
                    params,
                    return_type,
                    body,
                    ..
                } => {
                    assert_eq!(params.len(), 1);
                    assert_eq!(params[0].name, "x");
                    assert!(return_type.is_none());
                    assert_eq!(body.len(), 1);
                }
                _ => panic!("expected Lambda"),
            },
            _ => panic!("expected Let"),
        }
    }

    #[test]
    fn defer_statement() {
        let src = "defer:\n    cleanup()\n";
        let prog = parse(src);
        match &prog.statements[0] {
            Stmt::Defer { body, .. } => {
                assert_eq!(body.len(), 1);
            }
            _ => panic!("expected Defer"),
        }
    }

    #[test]
    fn defer_single_stmt() {
        let src = "defer cleanup()\n";
        let prog = parse(src);
        match &prog.statements[0] {
            Stmt::Defer { body, .. } => {
                assert_eq!(body.len(), 1);
                assert!(matches!(&body[0], Stmt::Expr(Expr::Call { .. })));
            }
            _ => panic!("expected Defer"),
        }
    }

    #[test]
    fn macro_definition() {
        let src = "macro my_macro(x, y):\n    let result = x + y\n    return result\n";
        let prog = parse(src);
        match &prog.statements[0] {
            Stmt::MacroDef {
                name, params, body, ..
            } => {
                assert_eq!(name, "my_macro");
                assert_eq!(params.len(), 2);
                assert_eq!(params[0], "x");
                assert_eq!(params[1], "y");
                assert_eq!(body.len(), 2);
            }
            _ => panic!("expected MacroDef"),
        }
    }

    #[test]
    fn bare_return() {
        let prog = parse("fn f():\n    return\n");
        match &prog.statements[0] {
            Stmt::FunctionDef { body, .. } => match &body[0] {
                Stmt::Return { value, .. } => {
                    assert!(value.is_none());
                }
                _ => panic!("expected Return"),
            },
            _ => panic!("expected FunctionDef"),
        }
    }

    #[test]
    fn error_unclosed_paren() {
        let src = "let x = (1 + 2\n";
        let mut parser = Parser::new(src).expect("parser creation failed");
        let result = parser.parse();
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = format!("{}", err);
        assert!(msg.contains("Unclosed"), "got: {}", msg);
    }

    #[test]
    fn error_missing_colon() {
        let src = "if x > 10\n    let y = 1\n";
        let mut parser = Parser::new(src).expect("parser creation failed");
        let result = parser.parse();
        assert!(result.is_err(), "expected parse error");
    }

    #[test]
    fn error_expected_expr() {
        let src = "let x = \n";
        let mut parser = Parser::new(src).expect("parser creation failed");
        let result = parser.parse();
        assert!(result.is_err(), "expected parse error");
    }

    // ── Error recovery tests ────────────────────────────────────────────────

    #[test]
    fn parse_error_recovery_collects_multiple_errors() {
        // Two malformed statements: the parser should recover after the first
        // and report both via drain_parse_errors().
        let src = "let x = \nlet y = \n";
        let mut parser = Parser::new(src).expect("parser creation failed");
        let result = parser.parse();
        assert!(result.is_err(), "expected parse error");
        assert!(
            parser.parse_errors.len() >= 1,
            "should have collected errors"
        );
    }

    #[test]
    fn parse_error_recovery_continues_after_bad_stmt() {
        // A bad statement followed by a good one.
        let src = "let x = \nlet y = 1\n";
        let mut parser = Parser::new(src).expect("parser creation failed");
        let _result = parser.parse();
        let errors = parser.drain_parse_errors();
        assert!(!errors.is_empty(), "expected errors");
        // Should have recovered and produced partial AST
    }

    #[test]
    fn parse_error_recovery_bad_if_body() {
        // Malformed inside if body: parser should recover per-statement
        // and continue parsing the rest of the block.
        let src = "if true:\n    let x = \n    let y = 42\n";
        let mut parser = Parser::new(src).expect("parser creation failed");
        let _result = parser.parse();
        let errors = parser.drain_parse_errors();
        assert!(!errors.is_empty(), "expected parse errors for bad let");
    }

    #[test]
    fn parse_error_recovery_unclosed_paren_in_expr() {
        // Unclosed paren inside an expression should be caught, recovered,
        // and subsequent statements still parsed.
        let src = "let x = (1 + 2\nlet y = 3\n";
        let mut parser = Parser::new(src).expect("parser creation failed");
        let result = parser.parse();
        assert!(result.is_err(), "expected parse error on unclosed paren");
        let errors = parser.drain_parse_errors();
        assert!(!errors.is_empty(), "expected errors");
    }

    #[test]
    fn parse_error_recovery_top_level_stray_dedent() {
        // A stray Dedent at the top level should be recovered, letting the
        // parser continue to the next statement.
        let src = "let x = 1\n\ndedent\nlet y = 2\n";
        // The blank line in the middle might produce a Dedent.
        let mut parser = Parser::new(src).expect("parser creation failed");
        let _result = parser.parse();
        // Should have produced some AST even if there were errors.
    }

    #[test]
    fn drain_parse_errors_clears_buffer() {
        let src = "let x = \nlet y = \n";
        let mut parser = Parser::new(src).expect("parser creation failed");
        let _ = parser.parse();
        let errors1 = parser.drain_parse_errors();
        assert!(!errors1.is_empty());
        let errors2 = parser.drain_parse_errors();
        assert!(
            errors2.is_empty(),
            "drain_parse_errors should clear the buffer"
        );
    }
}
