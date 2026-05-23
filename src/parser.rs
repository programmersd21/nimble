use crate::ast::*;
use crate::errors::ParseError;
use crate::lexer::{Lexer, Span, Token, TokenKind};

// Parser

/// A hand‑written recursive‑descent parser for the Nimble language.
///
/// The parser consumes a token stream produced by [`Lexer`] and builds an
/// AST ([`Program`]).  It uses Pratt‑style precedence climbing for
/// expressions and explicit `Indent` / `Dedent` tokens for block structure.
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    /// One‑token look‑ahead buffer.
    current: Token,
    next: Token,
    /// The raw source, used for diagnostic messages.
    source: &'a str,
}

impl<'a> Parser<'a> {
    /// Create a new parser from a source string.
    ///
    /// # Errors
    /// Returns a lexer error if the first two tokens cannot be read.
    pub fn new(source: &'a str) -> Result<Self, ParseError> {
        let mut lexer = Lexer::new(source);
        let current = lexer.next_token().map_err(|msg| {
            ParseError::Internal { src: source.to_string(), span: (0usize, 0usize).into(), msg }
        })?;
        let next = lexer.next_token().map_err(|msg| {
            ParseError::Internal { src: source.to_string(), span: (0usize, 0usize).into(), msg }
        })?;
        Ok(Parser { lexer, current, next, source })
    }

    /// Parse the entire source into a [`Program`].
    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let start_span = self.current.span;
        let mut statements = Vec::new();

        loop {
            if self.check(&TokenKind::Eof) {
                break;
            }
            if self.check(&TokenKind::Dedent) {
                // A top‑level Dedent means we have an extra dedent – this
                // can happen if the file starts with indented content, which
                // is not valid Nimble at the top level.
                let tok = self.current.clone();
                return Err(ParseError::unexpected_token(self.source, &tok));
            }
            statements.push(self.parse_statement()?);
            if self.check(&TokenKind::Newline) {
                self.advance();
            }
        }

        let end_span = self.current.span;
        Ok(Program {
            statements,
            span: Span::new_with_len(
                start_span.line,
                start_span.column,
                start_span.byte_index,
                end_span.byte_index + end_span.length - start_span.byte_index,
            ),
        })
    }

    // ═══════════════════════════════════════════════════════════════════
    // Statement dispatch
    // ═══════════════════════════════════════════════════════════════════

    /// Route to the appropriate `parse_*` method based on the current token.
    fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        let is_pub = if self.check(&TokenKind::Pub) {
            self.advance();
            true
        } else {
            false
        };

        let stmt = match &self.current.kind {
            TokenKind::Fn => self.parse_function_def(),
            TokenKind::Extern => self.parse_extern_fn(),
            TokenKind::Let => self.parse_let_or_var(false),
            TokenKind::Var => self.parse_let_or_var(true),
            TokenKind::If => self.parse_if(),
            TokenKind::While => self.parse_while(),
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

    // ── Block helper ───────────────────────────────────────────────────

    /// Parse a block: `: [Newline] Indent stmts+ Dedent`
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
            stmts.push(self.parse_statement()?);
            if self.check(&TokenKind::Newline) {
                self.advance();
            }
        }

        if self.check(&TokenKind::Dedent) {
            self.advance();
        }
        Ok(stmts)
    }

    // ── `extern fn name(params) [-> RetType]` ─────────────────────────

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

    // ── `fn name(params) [-> RetType]:` ────────────────────────────────

    fn parse_function_def(&mut self) -> Result<Stmt, ParseError> {
        let start = self.current.span;
        self.advance(); // consume `fn`

        let name = self.expect_identifier()?;

        if !self.check(&TokenKind::LParen) {
            let tok = self.current.clone();
            return Err(ParseError::expected_token(
                self.source,
                &tok,
                "'('",
            ));
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

    /// Parse `param ("," param)*` inside a function signature.
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

    // ── `let` / `var` declarations ─────────────────────────────────────

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
            Ok(Stmt::Var { name, type_annot, value, span })
        } else {
            Ok(Stmt::Let { name, type_annot, value, span })
        }
    }

    // ── `if` / `elif` / `else` ─────────────────────────────────────────

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
            let _elif_span = self.current.span;
            self.advance(); // consume `elif`

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

    // ── `while` ────────────────────────────────────────────────────────

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

    // ── `for` ──────────────────────────────────────────────────────────

    fn parse_for(&mut self) -> Result<Stmt, ParseError> {
        let start = self.current.span;
        self.advance(); // consume `for`

        let variable = self.expect_identifier()?;

        if !self.check(&TokenKind::In) {
            let tok = self.current.clone();
            return Err(ParseError::expected_token(
                self.source,
                &tok,
                "'in'",
            ));
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

    // ── `return` ───────────────────────────────────────────────────────

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

    // ── `load` module import ──────────────────────────────────────────

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
                return Err(ParseError::expected_token(self.source, &tok, "'/' after '..'"));
            }
            self.advance(); // consume `/`
            Ok("..".to_string())
        } else if self.check(&TokenKind::Slash) {
            self.advance(); // consume `/`
            Ok(".".to_string())
        } else {
            let tok = self.current.clone();
            Err(ParseError::expected_token(self.source, &tok, "'/' after '.'"))
        }
    }

    // ── Expression statement ───────────────────────────────────────────

    fn parse_expr_stmt(&mut self) -> Result<Stmt, ParseError> {
        if !self.can_start_expr() {
            let tok = self.current.clone();
            return Err(ParseError::expected_expression(self.source, &tok));
        }
        let expr = self.parse_expr()?;
        Ok(Stmt::Expr(expr))
    }

    // ═══════════════════════════════════════════════════════════════════
    // Expression parser (Pratt / precedence climbing)
    // ═══════════════════════════════════════════════════════════════════

    /// Expression precedence levels.
    ///
    /// | Level | Operators           | Assoc |
    /// |-------|---------------------|-------|
    /// | 1     | `=`                 | right |
    /// | 2     | `\|\|`              | left  |
    /// | 3     | `&&`                | left  |
    /// | 4     | `==` `!=`           | left  |
    /// | 5     | `<` `>` `<=` `>=`   | left  |
    /// | 6     | `+` `-`             | left  |
    /// | 7     | `*` `/`             | left  |
    /// | 8     | unary `-` `!`       | right (prefix) |
    /// | 9     | `()` call           | left  |
    fn get_prefix_precedence(kind: &TokenKind) -> Option<u8> {
        match kind {
            TokenKind::Minus | TokenKind::Bang => Some(8),
            _ => None,
        }
    }

    fn get_infix_precedence(kind: &TokenKind) -> Option<(u8, bool)> {
        // Returns (precedence, right_associative)
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
        )
    }

    /// Parse any expression (entry point).
    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_expr_prec(0)
    }

    /// Core Pratt‑parsing loop with minimum precedence `min_prec`.
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

            // Check for binary operators.
            if let Some((prec, right_assoc)) = Self::get_infix_precedence(&self.current.kind) {
                if prec >= min_prec {
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
                    } else if matches!(&op_token.kind, TokenKind::PlusEqual
                        | TokenKind::MinusEqual
                        | TokenKind::StarEqual
                        | TokenKind::SlashEqual
                        | TokenKind::PercentEqual)
                    {
                        let compound_op = match &op_token.kind {
                            TokenKind::PlusEqual => BinaryOp::Add,
                            TokenKind::MinusEqual => BinaryOp::Sub,
                            TokenKind::StarEqual => BinaryOp::Mul,
                            TokenKind::SlashEqual => BinaryOp::Div,
                            TokenKind::PercentEqual => BinaryOp::Mod,
                            _ => unreachable!(),
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
            }
            break;
        }

        Ok(left)
    }

    /// Parse a prefix (primary / unary) expression.
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

    /// Parse a primary expression: literals, identifiers, grouping.
    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let token = self.current.clone();
        let expr = match &token.kind {
            TokenKind::IntLiteral(n) => Expr::IntLiteral(*n, token.span),
            TokenKind::FloatLiteral(f) => Expr::FloatLiteral(*f, token.span),
            TokenKind::StringLiteral(s) => {
                Expr::StringLiteral(s.clone(), token.span)
            }
            TokenKind::True => Expr::BoolLiteral(true, token.span),
            TokenKind::False => Expr::BoolLiteral(false, token.span),
            TokenKind::Identifier(s) => {
                Expr::Identifier(s.clone(), token.span)
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
                return Ok(Expr::Grouping {
                    expr: Box::new(inner),
                    span,
                });
            }
            _ => {
                return Err(ParseError::expected_expression(self.source, &token));
            }
        };

        self.advance();
        Ok(expr)
    }

    /// Parse a function / method call: `callee(args...)`
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

    // ── Type parsing ───────────────────────────────────────────────────

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
        let name = if let TokenKind::Identifier(s) = &token.kind {
            s.clone()
        } else {
            unreachable!()
        };
        self.advance();
        Ok(Type { name, span: token.span })
    }

    // ═══════════════════════════════════════════════════════════════════
    // Token helpers
    // ═══════════════════════════════════════════════════════════════════

    /// Check if the current token matches `kind`.
    fn check(&self, kind: &TokenKind) -> bool {
        &self.current.kind == kind
    }

    /// Advance to the next token, returning the old current token.
    fn advance(&mut self) -> Token {
        let prev = std::mem::replace(
            &mut self.current,
            std::mem::replace(
                &mut self.next,
                self.lexer.next_token().unwrap_or_else(|_| {
                    Token::new(TokenKind::Eof, Span::new(1, 1, 0))
                }),
            ),
        );
        prev
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
}


impl Expr {
    /// Convenience accessor to borrow the `Span` stored in any expression
    /// variant.
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
            | Expr::Cast { span, .. } => *span,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Parse `source`, panicking on errors.
    fn parse(source: &str) -> Program {
        let mut parser = Parser::new(source).expect("parser creation failed");
        parser.parse().expect("parse failed")
    }

    /// Parse `source` and return just the statement kinds.
    fn stmt_kinds(source: &str) -> Vec<&'static str> {
        let prog = parse(source);
        prog.statements.iter().map(|s| stmt_tag(s)).collect()
    }

    fn stmt_tag(stmt: &Stmt) -> &'static str {
        match stmt {
            Stmt::Let { .. } => "let",
            Stmt::Var { .. } => "var",
            Stmt::FunctionDef { .. } => "fn",
            Stmt::If { .. } => "if",
            Stmt::While { .. } => "while",
            Stmt::For { .. } => "for",
            Stmt::Return { .. } => "return",
            Stmt::ExternFn { .. } => "extern fn",
            Stmt::Load { .. } => "load",
            Stmt::Expr(_) => "expr",
        }
    }

    // ── Declarations ───────────────────────────────────────────────────

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
            Stmt::Load { module_path, is_pub, .. } => {
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
            Stmt::Let { name, type_annot, value, .. } => {
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
            Stmt::Let { name, type_annot, .. } => {
                assert_eq!(name, "x");
                assert_eq!(type_annot.as_ref().unwrap().name, "Int");
            }
            _ => panic!("expected Let"),
        }
    }

    #[test]
    fn var_decl() {
        let prog = parse("var name: String = \"hello\"\n");
        match &prog.statements[0] {
            Stmt::Var { name, type_annot, value, .. } => {
                assert_eq!(name, "name");
                assert_eq!(type_annot.as_ref().unwrap().name, "String");
                assert!(matches!(value, Expr::StringLiteral(s, _) if s == "hello"));
            }
            _ => panic!("expected Var"),
        }
    }

    // ── Function definitions ───────────────────────────────────────────

    #[test]
    fn function_no_args() {
        let prog = parse("fn main():\n    return 0\n");
        assert_eq!(stmt_kinds("fn main():\n    return 0\n"), vec!["fn"]);
        match &prog.statements[0] {
            Stmt::FunctionDef { name, params, return_type, body, .. } => {
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
            Stmt::FunctionDef { name, params, return_type, body, .. } => {
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

    // ── If / elif / else ───────────────────────────────────────────────

    #[test]
    fn if_statement() {
        let src = "if x > 10:\n    let y = 1\n";
        let prog = parse(src);
        match &prog.statements[0] {
            Stmt::If { condition, body, elifs, else_body, .. } => {
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
            Stmt::If { body, elifs, else_body, .. } => {
                assert_eq!(body.len(), 1);
                assert_eq!(elifs.len(), 1);
                assert!(else_body.is_some());
                assert_eq!(else_body.as_ref().unwrap().len(), 1);
            }
            _ => panic!("expected If"),
        }
    }

    // ── While / for ────────────────────────────────────────────────────

    #[test]
    fn while_loop() {
        let src = "while x > 0:\n    x = x - 1\n";
        let prog = parse(src);
        match &prog.statements[0] {
            Stmt::While { condition, body, .. } => {
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
            Stmt::For { variable, iterable, body, .. } => {
                assert_eq!(variable, "i");
                assert!(matches!(iterable, Expr::Identifier(name, _) if name == "range"));
                assert_eq!(body.len(), 1);
            }
            _ => panic!("expected For"),
        }
    }

    // ── Expressions ────────────────────────────────────────────────────

    #[test]
    fn binary_expression_precedence() {
        let prog = parse("let x = 1 + 2 * 3\n");
        match &prog.statements[0] {
            Stmt::Let { value, .. } => {
                // 1 + (2 * 3) – multiplication binds tighter.
                match value {
                    Expr::Binary { left, op, right, .. } => {
                        assert_eq!(*op, BinaryOp::Add);
                        assert!(matches!(left.as_ref(), Expr::IntLiteral(1, _)));
                        match right.as_ref() {
                            Expr::Binary { left: rl, op: rop, right: rr, .. } => {
                                assert_eq!(*rop, BinaryOp::Mul);
                                assert!(matches!(rl.as_ref(), Expr::IntLiteral(2, _)));
                                assert!(matches!(rr.as_ref(), Expr::IntLiteral(3, _)));
                            }
                            _ => panic!("expected Mul binary"),
                        }
                    }
                    _ => panic!("expected Add binary"),
                }
            }
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
                    Expr::Binary { op: BinaryOp::Equal, .. }
                ));
            }
            _ => panic!("expected Let"),
        }
    }

    #[test]
    fn logical_operators() {
        let prog = parse("let r = a && b || c\n");
        match &prog.statements[0] {
            Stmt::Let { value, .. } => {
                // `&&` binds tighter than `||`
                match value {
                    Expr::Binary { op: BinaryOp::Or, left, .. } => {
                        match left.as_ref() {
                            Expr::Binary { op: BinaryOp::And, .. } => {}
                            _ => panic!("expected And left"),
                        }
                    }
                    _ => panic!("expected Or at top"),
                }
            }
            _ => panic!("expected Let"),
        }
    }

    #[test]
    fn function_call() {
        let prog = parse("let r = foo(x, y + 1)\n");
        match &prog.statements[0] {
            Stmt::Let { value, .. } => {
                match value {
                    Expr::Call { callee, args, .. } => {
                        assert!(matches!(
                            callee.as_ref(),
                            Expr::Identifier(name, _) if name == "foo"
                        ));
                        assert_eq!(args.len(), 2);
                    }
                    _ => panic!("expected Call"),
                }
            }
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
                Expr::Binary { op, left, right, .. } => {
                    assert_eq!(*op, BinaryOp::Mul);
                    match left.as_ref() {
                        Expr::Grouping { expr, .. } => {
                            match expr.as_ref() {
                                Expr::Binary { op: add_op, .. } => {
                                    assert_eq!(*add_op, BinaryOp::Add);
                                }
                                _ => panic!("expected Add inside grouping"),
                            }
                        }
                        _ => panic!("expected Grouping"),
                    }
                    assert!(matches!(right.as_ref(), Expr::IntLiteral(3, _)));
                }
                _ => panic!("expected Mul"),
            },
            _ => panic!("expected Let"),
        }
    }

    // ── Unary ──────────────────────────────────────────────────────────

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
                    assert!(matches!(
                        operand.as_ref(),
                        Expr::BoolLiteral(true, _)
                    ));
                }
                _ => panic!("expected Unary"),
            },
            _ => panic!("expected Let"),
        }
    }

    // ── Return ─────────────────────────────────────────────────────────

    #[test]
    fn bare_return() {
        let prog = parse("fn f():\n    return\n");
        match &prog.statements[0] {
            Stmt::FunctionDef { body, .. } => {
                match &body[0] {
                    Stmt::Return { value, .. } => {
                        assert!(value.is_none());
                    }
                    _ => panic!("expected Return"),
                }
            }
            _ => panic!("expected FunctionDef"),
        }
    }

    // ── Error recovery ─────────────────────────────────────────────────

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
}
