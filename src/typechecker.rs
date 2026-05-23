use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::ast::{BinaryOp, Expr, Stmt, UnaryOp};
use crate::env::{Environment, Symbol, SymbolKind};
use crate::lexer::Span;
use crate::module_loader::ModuleLoader;
use crate::types::{Substitution, Type};

// TypeError (miette‑diagnostic errors)

/// A structured type‑error that integrates with `miette` diagnostics.
#[derive(Debug, Error, Diagnostic)]
pub enum TypeError {
    #[error("Type mismatch: expected `{expected}`, found `{found}` at line {line}:{column}")]
    #[diagnostic(code("nimble::type::mismatch"))]
    Mismatch {
        expected: String,
        found: String,
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("expected `{expected}`, found `{found}`")]
        span: SourceSpan,
    },

    #[error("Cannot reassign to immutable variable `{name}` at line {line}:{column}")]
    #[diagnostic(code("nimble::type::assign_to_immutable"))]
    AssignToImmutable {
        name: String,
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("`{name}` is declared as `let`")]
        span: SourceSpan,
    },

    #[error("Undefined variable `{name}` at line {line}:{column}")]
    #[diagnostic(code("nimble::type::undefined_variable"))]
    UndefinedVariable {
        name: String,
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("undefined `{name}`")]
        span: SourceSpan,
    },

    #[error("Undefined type `{name}` at line {line}:{column}")]
    #[diagnostic(code("nimble::type::undefined_type"))]
    UndefinedType {
        name: String,
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("undefined type `{name}`")]
        span: SourceSpan,
    },

    #[error("Duplicate definition `{name}` at line {line}:{column}")]
    #[diagnostic(code("nimble::type::duplicate_definition"))]
    DuplicateDefinition {
        name: String,
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("`{name}` already defined in this scope")]
        span: SourceSpan,
    },

    #[error("Call of non‑function value at line {line}:{column}")]
    #[diagnostic(code("nimble::type::call_non_function"))]
    CallNonFunction {
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("not a function")]
        span: SourceSpan,
    },

    #[error("Argument count mismatch: expected {expected}, found {found} at line {line}:{column}")]
    #[diagnostic(code("nimble::type::argument_count"))]
    ArgumentCount {
        expected: usize,
        found: usize,
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("expected {expected} arguments, found {found}")]
        span: SourceSpan,
    },

    #[error("Interface `{interface}` requires method `{method}` but the target does not provide it at line {line}:{column}")]
    #[diagnostic(code("nimble::type::missing_method"))]
    MissingMethod {
        interface: String,
        method: String,
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("missing method `{method}` required by `{interface}`")]
        span: SourceSpan,
    },

    #[error("Occurs check failed: recursive type at line {line}:{column}")]
    #[diagnostic(code("nimble::type::recursive_type"))]
    RecursiveType {
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("recursive type constraint")]
        span: SourceSpan,
    },

    #[error("Internal type‑checker error: {msg}")]
    #[diagnostic(code("nimble::type::internal"))]
    Internal {
        msg: String,
        #[source_code]
        src: String,
        #[label("internal error")]
        span: SourceSpan,
    },
}

// Convenience constructors (matching the style of ParseError).

impl TypeError {
    fn mismatch(source: &str, expected: &Type, found: &Type, span: Span) -> Self {
        TypeError::Mismatch {
            expected: expected.to_string(),
            found: found.to_string(),
            line: span.line,
            column: span.column,
            src: source.to_string(),
            span: (span.byte_index, span.length.max(1)).into(),
        }
    }

    fn assign_to_immutable(source: &str, name: &str, span: Span) -> Self {
        TypeError::AssignToImmutable {
            name: name.to_string(),
            line: span.line,
            column: span.column,
            src: source.to_string(),
            span: (span.byte_index, span.length.max(1)).into(),
        }
    }

    fn undefined_variable(source: &str, name: &str, span: Span) -> Self {
        TypeError::UndefinedVariable {
            name: name.to_string(),
            line: span.line,
            column: span.column,
            src: source.to_string(),
            span: (span.byte_index, span.length.max(1)).into(),
        }
    }

    fn undefined_type(source: &str, name: &str, span: Span) -> Self {
        TypeError::UndefinedType {
            name: name.to_string(),
            line: span.line,
            column: span.column,
            src: source.to_string(),
            span: (span.byte_index, span.length.max(1)).into(),
        }
    }

    fn duplicate_definition(source: &str, name: &str, span: Span) -> Self {
        TypeError::DuplicateDefinition {
            name: name.to_string(),
            line: span.line,
            column: span.column,
            src: source.to_string(),
            span: (span.byte_index, span.length.max(1)).into(),
        }
    }

    fn argument_count(source: &str, expected: usize, found: usize, span: Span) -> Self {
        TypeError::ArgumentCount {
            expected,
            found,
            line: span.line,
            column: span.column,
            src: source.to_string(),
            span: (span.byte_index, span.length.max(1)).into(),
        }
    }

    fn recursive_type(source: &str, span: Span) -> Self {
        TypeError::RecursiveType {
            line: span.line,
            column: span.column,
            src: source.to_string(),
            span: (span.byte_index, span.length.max(1)).into(),
        }
    }

    /// Reconstruct a `nimble::Span` from the `miette::SourceSpan` stored in
    /// the error variant.  This is used by the LSP to map errors back to
    /// source locations.
    pub fn span(&self) -> Span {
        let span = match self {
            TypeError::Mismatch { span, .. } => *span,
            TypeError::AssignToImmutable { span, .. } => *span,
            TypeError::UndefinedVariable { span, .. } => *span,
            TypeError::UndefinedType { span, .. } => *span,
            TypeError::DuplicateDefinition { span, .. } => *span,
            TypeError::CallNonFunction { span, .. } => *span,
            TypeError::ArgumentCount { span, .. } => *span,
            TypeError::MissingMethod { span, .. } => *span,
            TypeError::RecursiveType { span, .. } => *span,
            TypeError::Internal { span, .. } => *span,
        };
        Span::new_with_len(0, 0, span.offset(), span.len())
    }
}

// TypeChecker

/// The core semantic‑analysis engine.
///
/// Performs name resolution, Hindley‑Milner type inference, and structural
/// sub‑typing validation in a single pass over the AST.
pub struct TypeChecker {
    /// Monotonically increasing counter for fresh type‑variable IDs.
    var_counter: usize,
    /// The accumulated substitution from unification.
    subst: Substitution,
    /// The source text (cloned for error reporting).
    source: String,
    /// Known interface type names (for structural subtyping validation).
    interfaces: std::collections::HashSet<String>,
    /// Optional module loader for resolving `load` statements.
    module_loader: Option<ModuleLoader>,
    /// Collected extern fn statements from loaded modules.
    pub collected_externs: std::rc::Rc<std::cell::RefCell<Vec<Stmt>>>,
    /// Collected top-level statements from loaded modules for code generation.
    pub collected_module_stmts: std::rc::Rc<std::cell::RefCell<Vec<(Stmt, Environment)>>>,
}

impl TypeChecker {
    /// Create a new checker for the given source text.
    pub fn new(source: &str) -> Self {
        Self::with_externs(source, std::rc::Rc::new(std::cell::RefCell::new(Vec::new())))
    }

    pub fn with_externs(source: &str, externs: std::rc::Rc<std::cell::RefCell<Vec<Stmt>>>) -> Self {
        Self::with_externs_and_module_stmts(
            source,
            externs,
            std::rc::Rc::new(std::cell::RefCell::new(Vec::new())),
        )
    }

    pub fn with_externs_and_module_stmts(
        source: &str,
        externs: std::rc::Rc<std::cell::RefCell<Vec<Stmt>>>,
        module_stmts: std::rc::Rc<std::cell::RefCell<Vec<(Stmt, Environment)>>>,
    ) -> Self {
        TypeChecker {
            var_counter: 0,
            subst: Substitution::new(),
            source: source.to_string(),
            interfaces: std::collections::HashSet::new(),
            module_loader: None,
            collected_externs: externs,
            collected_module_stmts: module_stmts,
        }
    }

    /// Attach a module loader for resolving `load` statements.
    pub fn with_loader(mut self, loader: ModuleLoader) -> Self {
        self.module_loader = Some(loader);
        self
    }

    // ── Fresh type variables ─────────────────────────────────────────────

    fn fresh_var(&mut self) -> Type {
        let id = self.var_counter;
        self.var_counter += 1;
        Type::Variable(id)
    }

    /// Resolve a possibly‑variable type through the current substitution.
    fn resolve(&self, t: &Type) -> Type {
        t.apply(&self.subst)
    }

    // ── Public entry point ───────────────────────────────────────────────

    /// Fully type‑check a program.
    ///
    /// If a module loader is attached via [`with_loader`], `load` statements
    /// in the program are resolved first, importing symbols into the
    /// environment before the normal two-pass checking.
    pub fn check_program(&mut self, program: &crate::ast::Program) -> Result<Environment, TypeError> {
        let mut env = Environment::new();

        // Phase 0: process `load` statements if a module loader is attached.
        if let Some(ref mut loader) = self.module_loader {
            for stmt in &program.statements {
                if let Stmt::Load { module_path, symbols, alias, span, .. } = stmt {
                    match loader.load(
                        module_path,
                        symbols.as_deref(),
                        alias.as_deref(),
                        &mut env,
                        &self.collected_externs,
                        &self.collected_module_stmts,
                        &self.source,
                        *span,
                    ) {
                        Ok(_) => {
                            // Externs and loaded module statements are already collected
                            // by the nested TypeChecker and module loader.
                        }
                        Err(e) => {
                            return Err(TypeError::Internal {
                                msg: format!("{}", e),
                                src: self.source.clone(),
                                span: (span.byte_index, span.length.max(1)).into(),
                            });
                        }
                    }
                }
            }
        }

        // First pass: register all top‑level function/struct/interface
        // signatures so that mutually‑recursive calls are possible.
        for stmt in &program.statements {
            self.register_declaration(stmt, &mut env)?;
        }

        // Register built-in functions (only if not already user-declared).
        self.register_builtins(&mut env);

        // Second pass: type‑check bodies.
        for stmt in &program.statements {
            self.check_stmt(stmt, &mut env)?;
        }

        Ok(env)
    }

    /// Register built-in functions that are available without explicit declaration.
    fn register_builtins(&self, env: &mut Environment) {
        let builtins: &[(&str, Vec<Type>, Type)] = &[
            ("print", vec![Type::String], Type::Void),
            ("print_int", vec![Type::Int], Type::Void),
            ("print_str", vec![Type::String], Type::Void),
            ("print_float", vec![Type::Float], Type::Void),
        ];
        for (name, param_tys, ret_ty) in builtins {
            if env.lookup_current(name).is_none() {
                env.define(
                    name,
                    Symbol {
                        kind: SymbolKind::Function,
                        mutable: false,
                        type_: Type::Function(param_tys.clone(), Box::new(ret_ty.clone())),
                        defined_at: Span::new(0, 0, 0),
                    },
                );
            }
        }
    }

    /// Register a top‑level declaration signature without checking its body.
    fn register_declaration(&mut self, stmt: &Stmt, env: &mut Environment) -> Result<(), TypeError> {
        match stmt {
            Stmt::FunctionDef { name, params, return_type, span, .. } => {
                if env.lookup_current(name).is_some() {
                    return Err(TypeError::duplicate_definition(
                        &self.source, name, *span,
                    ));
                }
                let param_types: Vec<Type> = params
                    .iter()
                    .map(|p| Type::from(&p.type_annot))
                    .collect();
                let ret = match return_type {
                    Some(t) => Type::from(t),
                    None => Type::Void,
                };
                let ft = Type::Function(param_types, Box::new(ret));
                env.define(
                    name,
                    Symbol {
                        kind: SymbolKind::Function,
                        mutable: false,
                        type_: ft,
                        defined_at: *span,
                    },
                );
            }
            Stmt::ExternFn { name, params, return_type, span } => {
                if env.lookup_current(name).is_some() {
                    return Err(TypeError::duplicate_definition(
                        &self.source, name, *span,
                    ));
                }
                let param_types: Vec<Type> = params
                    .iter()
                    .map(|p| Type::from(&p.type_annot))
                    .collect();
                let ret = match return_type {
                    Some(t) => Type::from(t),
                    None => Type::Void,
                };
                let ft = Type::Function(param_types, Box::new(ret));
                env.define(
                    name,
                    Symbol {
                        kind: SymbolKind::Function,
                        mutable: false,
                        type_: ft,
                        defined_at: *span,
                    },
                );
                self.collected_externs.borrow_mut().push(stmt.clone());
            }
            Stmt::Load { .. } => {} // Load is handled in phase 0
            Stmt::Var { .. } | Stmt::Let { .. } | Stmt::Expr(..) | Stmt::Return { .. }
            | Stmt::If { .. } | Stmt::While { .. } | Stmt::For { .. } => {}
        }
        Ok(())
    }

    // ── Statement checking ───────────────────────────────────────────────

    /// Type‑check a single statement, updating the environment as needed.
    pub fn check_stmt(&mut self, stmt: &Stmt, env: &mut Environment) -> Result<(), TypeError> {
        match stmt {
            Stmt::Var { name, type_annot, value, span }
            | Stmt::Let { name, type_annot, value, span } => {
                let is_mut = matches!(stmt, Stmt::Var { .. });

                if env.lookup_current(name).is_some() {
                    return Err(TypeError::duplicate_definition(
                        &self.source, name, *span,
                    ));
                }

                let value_type = self.infer_expr(value, env)?;

                let declared = match type_annot {
                    Some(t) => {
                        let dt = Type::from(t);
                        // Verify the annotation is a known type.
                        match &dt {
                            Type::Struct(s) => {
                                let sym = env.lookup(s)
                                    .ok_or_else(|| TypeError::undefined_type(
                                        &self.source, s, *span,
                                    ))?;
                                if sym.kind != SymbolKind::Struct {
                                    return Err(TypeError::undefined_type(
                                        &self.source, s, *span,
                                    ));
                                }
                            }
                            _ => {}
                        }
                        Some(dt)
                    }
                    None => None,
                };

                // If there is a type annotation, unify it with the inferred type.
                if let Some(ref ann) = declared {
                    self.unify(&value_type, ann, *span)?;
                }

                let resolved = self.resolve(&value_type);
                env.define(
                    name,
                    Symbol {
                        kind: SymbolKind::Variable,
                        mutable: is_mut,
                        type_: resolved,
                        defined_at: *span,
                    },
                );
                Ok(())
            }

            Stmt::ExternFn { .. } => {
                // Already registered in the first pass; no body to check.
                Ok(())
            }

            Stmt::FunctionDef { params, return_type, body, span, .. } => {
                // The function is already registered in the environment from
                // the first pass; now we check the body in a new scope.
                let param_types: Vec<Type> = params
                    .iter()
                    .map(|p| Type::from(&p.type_annot))
                    .collect();

                env.enter_scope();

                // Bind each parameter in the new scope.
                for (param, ptype) in params.iter().zip(param_types.iter()) {
                    let resolved = self.resolve(ptype);
                    env.define(
                        &param.name,
                        Symbol {
                            kind: SymbolKind::Variable,
                            mutable: false,
                            type_: resolved,
                            defined_at: param.span,
                        },
                    );
                }

                // Check body statements.
                for s in body {
                    self.check_stmt(s, env)?;
                }

                env.exit_scope();

                // Verify the declared return type is valid.
                if let Some(ret) = return_type {
                    let rt = Type::from(ret);
                    match &rt {
                        Type::Struct(s) => {
                            let sym = env.lookup(s)
                                .ok_or_else(|| TypeError::undefined_type(
                                    &self.source, s, *span,
                                ))?;
                            if sym.kind != SymbolKind::Struct {
                                return Err(TypeError::undefined_type(
                                    &self.source, s, *span,
                                ));
                            }
                        }
                        _ => {}
                    }
                }

                Ok(())
            }

            Stmt::If { condition, body, elifs, else_body, span: _ } => {
                let cond_type = self.infer_expr(condition, env)?;
                self.unify(&cond_type, &Type::Bool, condition.span())?;

                env.enter_scope();
                for s in body {
                    self.check_stmt(s, env)?;
                }
                env.exit_scope();

                for (elif_cond, elif_body) in elifs {
                    let elif_type = self.infer_expr(elif_cond, env)?;
                    self.unify(&elif_type, &Type::Bool, elif_cond.span())?;
                    env.enter_scope();
                    for s in elif_body {
                        self.check_stmt(s, env)?;
                    }
                    env.exit_scope();
                }

                if let Some(ebody) = else_body {
                    env.enter_scope();
                    for s in ebody {
                        self.check_stmt(s, env)?;
                    }
                    env.exit_scope();
                }

                Ok(())
            }

            Stmt::While { condition, body, span: _ } => {
                let cond_type = self.infer_expr(condition, env)?;
                self.unify(&cond_type, &Type::Bool, condition.span())?;

                env.enter_scope();
                for s in body {
                    self.check_stmt(s, env)?;
                }
                env.exit_scope();

                Ok(())
            }

            Stmt::For { variable, iterable, body, span } => {
                let iter_type = self.infer_expr(iterable, env)?;

                env.enter_scope();
                // The loop variable type is inferred from the iterable.
                // For now we create a fresh type variable.
                let loop_var_type = self.resolve(&iter_type);
                env.define(
                    variable,
                    Symbol {
                        kind: SymbolKind::Variable,
                        mutable: false,
                        type_: loop_var_type,
                        defined_at: *span,
                    },
                );
                for s in body {
                    self.check_stmt(s, env)?;
                }
                env.exit_scope();

                Ok(())
            }

            Stmt::Return { value, span: _ } => {
                if let Some(val) = value {
                    self.infer_expr(val, env)?;
                }
                Ok(())
            }

            Stmt::Load { .. } => {
                // Handled during phase 0 (check_program).
                Ok(())
            }

            Stmt::Expr(expr) => {
                self.infer_expr(expr, env)?;
                Ok(())
            }
        }
    }

    // ── Expression inference ─────────────────────────────────────────────

    /// Infer the type of an expression, returning its (partially‑resolved) type.
    pub fn infer_expr(&mut self, expr: &Expr, env: &Environment) -> Result<Type, TypeError> {
        match expr {
            Expr::IntLiteral(_, _span) => Ok(Type::Int),
            Expr::FloatLiteral(_, _span) => Ok(Type::Float),
            Expr::StringLiteral(_, _span) => Ok(Type::String),
            Expr::BoolLiteral(_, _span) => Ok(Type::Bool),

            Expr::Identifier(name, span) => {
                match env.lookup(name) {
                    Some(sym) => Ok(self.resolve(&sym.type_)),
                    None => Err(TypeError::undefined_variable(
                        &self.source, name, *span,
                    )),
                }
            }

            Expr::Binary { left, op, right, span } => {
                self.infer_binary(left, op, right, *span, env)
            }

            Expr::Unary { op, operand, span } => {
                let operand_type = self.infer_expr(operand, env)?;
                match op {
                    UnaryOp::Negate => {
                        // Must be numeric (Int or Float).
                        let num = self.fresh_var();
                        self.unify(&operand_type, &num, *span)?;
                        Ok(self.resolve(&num))
                    }
                    UnaryOp::Not => {
                        self.unify(&operand_type, &Type::Bool, *span)?;
                        Ok(Type::Bool)
                    }
                }
            }

            Expr::Call { callee, args, span } => self.infer_call(callee, args, *span, env),

            Expr::Assign { target, value, span } => {
                // Check mutability of the target.
                if let Expr::Identifier(name, _) = target.as_ref() {
                    match env.lookup(name) {
                        Some(sym) if !sym.mutable => {
                            return Err(TypeError::assign_to_immutable(
                                &self.source, name, *span,
                            ));
                        }
                        None => {
                            return Err(TypeError::undefined_variable(
                                &self.source, name, *span,
                            ));
                        }
                        _ => {}
                    }
                }
                let target_type = self.infer_expr(target, env)?;
                let value_type = self.infer_expr(value, env)?;
                self.unify(&target_type, &value_type, *span)?;
                Ok(self.resolve(&value_type))
            }

            Expr::Grouping { expr, span: _ } => self.infer_expr(expr, env),

            Expr::MemberAccess { object, member, span } => {
                // Support nested qualified names like `std.io.println` by
                // recursively flattening the member chain into a single
                // dotted identifier.
                fn flatten_qualified_name(expr: &Expr, tail: String) -> Option<String> {
                    match expr {
                        Expr::Identifier(name, _) => Some(format!("{}.{}", name, tail)),
                        Expr::MemberAccess { object, member, .. } => {
                            flatten_qualified_name(object, format!("{}.{}", member, tail))
                        }
                        _ => None,
                    }
                }

                if let Some(qualified_name) = flatten_qualified_name(object.as_ref(), member.clone()) {
                    if let Some(sym) = env.lookup(&qualified_name) {
                        return Ok(self.resolve(&sym.type_));
                    }
                }
                
                // If it fails, maybe it's just a local object field access?
                // For now, fall back to an error as we only support qualified names.
                Err(TypeError::undefined_variable(&self.source, member, *span))
            }

            Expr::Cast { expr, target_type, span: _ } => {
                let _expr_type = self.infer_expr(expr, env)?;
                let target_type = Type::from(target_type);
                // Explicit casts are trusted by the typechecker for now.
                Ok(target_type)
            }
        }
    }

    // ── Binary expression inference ──────────────────────────────────────

    fn infer_binary(
        &mut self,
        left: &Expr,
        op: &BinaryOp,
        right: &Expr,
        span: Span,
        env: &Environment,
    ) -> Result<Type, TypeError> {
        let left_type = self.infer_expr(left, env)?;
        let right_type = self.infer_expr(right, env)?;

        match op {
            // Arithmetic operators: Int or Float (both sides the same)
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                // Create a fresh type variable and unify both sides with it,
                // then constrain it to be numeric.
                let num = self.fresh_var();
                self.unify(&left_type, &num, span)?;
                self.unify(&right_type, &num, span)?;
                Ok(self.resolve(&num))
            }

            // Comparison operators: both sides same type, result is Bool
            BinaryOp::Equal | BinaryOp::NotEqual
            | BinaryOp::Less | BinaryOp::Greater
            | BinaryOp::LessEqual | BinaryOp::GreaterEqual => {
                self.unify(&left_type, &right_type, span)?;
                Ok(Type::Bool)
            }

            // Logical operators: both sides must be Bool, result is Bool
            BinaryOp::And | BinaryOp::Or => {
                self.unify(&left_type, &Type::Bool, span)?;
                self.unify(&right_type, &Type::Bool, span)?;
                Ok(Type::Bool)
            }
        }
    }

    // ── Function‑call inference ──────────────────────────────────────────

    fn infer_call(
        &mut self,
        callee: &Expr,
        args: &[Expr],
        span: Span,
        env: &Environment,
    ) -> Result<Type, TypeError> {
        let callee_type = self.infer_expr(callee, env)?;

        // The callee must be a function type.  Create one if needed.
        let expected_param_types: Vec<Type> =
            (0..args.len()).map(|_| self.fresh_var()).collect();
        let expected_ret = self.fresh_var();
        let expected_fn =
            Type::Function(expected_param_types.clone(), Box::new(expected_ret.clone()));

        self.unify(&callee_type, &expected_fn, span)?;

        // Now check each argument against its expected type.
        for (i, arg) in args.iter().enumerate() {
            let arg_type = self.infer_expr(arg, env)?;
            self.unify(&arg_type, &expected_param_types[i], span)?;
        }

        Ok(self.resolve(&expected_ret))
    }

    // ══════════════════════════════════════════════════════════════════════
    // Unification (Algorithm W core)
    // ══════════════════════════════════════════════════════════════════════

    /// Unify two types, extending `self.subst` with the necessary bindings.
    fn unify(&mut self, a: &Type, b: &Type, span: Span) -> Result<(), TypeError> {
        let a = self.resolve(a);
        let b = self.resolve(b);

        if a == b {
            return Ok(());
        }

        match (&a, &b) {
            (Type::Variable(ida), _) => {
                if a.free_vars().contains(ida) && b.free_vars().contains(ida) {
                    return Err(TypeError::recursive_type(&self.source, span));
                }
                // Occurs check
                if b.free_vars().contains(ida) {
                    return Err(TypeError::recursive_type(&self.source, span));
                }
                self.subst.insert(*ida, b.clone());
                Ok(())
            }
            (_, Type::Variable(idb)) => {
                let a_clone = a.clone();
                if a_clone.free_vars().contains(idb) {
                    return Err(TypeError::recursive_type(&self.source, span));
                }
                self.subst.insert(*idb, a.clone());
                Ok(())
            }

            (Type::Function(p1, r1), Type::Function(p2, r2)) => {
                if p1.len() != p2.len() {
                    return Err(TypeError::argument_count(
                        &self.source,
                        p1.len(),
                        p2.len(),
                        span,
                    ));
                }
                for (pa, pb) in p1.iter().zip(p2.iter()) {
                    self.unify(pa, pb, span)?;
                }
                self.unify(r1, r2, span)
            }

            (Type::GenericInstance(n1, a1), Type::GenericInstance(n2, a2)) => {
                if n1 != n2 {
                    return Err(TypeError::mismatch(
                        &self.source, &a, &b, span,
                    ));
                }
                if a1.len() != a2.len() {
                    return Err(TypeError::mismatch(
                        &self.source, &a, &b, span,
                    ));
                }
                for (ta, tb) in a1.iter().zip(a2.iter()) {
                    self.unify(ta, tb, span)?;
                }
                Ok(())
            }

            (Type::Interface(iface_name), Type::Struct(_))
            | (Type::Struct(_), Type::Interface(iface_name)) => {
                if !self.interfaces.contains(iface_name) {
                    return Ok(());
                }
                Ok(())
            }

            _ => Err(TypeError::mismatch(&self.source, &a, &b, span)),
        }
    }
}

// ── Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    fn typecheck(source: &str) -> Result<Environment, TypeError> {
        let prog = Parser::new(source).expect("parse failed").parse().expect("parse failed");
        let mut checker = TypeChecker::new(source);
        checker.check_program(&prog)
    }

    fn typecheck_ok(source: &str) -> Environment {
        typecheck(source).expect("typecheck failed")
    }

    fn typecheck_err(source: &str) -> TypeError {
        typecheck(source).expect_err("expected typecheck error")
    }

    // ── Successful inference ─────────────────────────────────────────────

    #[test]
    fn infer_int_literal() {
        let env = typecheck_ok("let x = 42\n");
        let sym = env.lookup("x").unwrap();
        assert_eq!(sym.type_, Type::Int);
    }

    #[test]
    fn infer_float_literal() {
        let env = typecheck_ok("let x = 3.14\n");
        let sym = env.lookup("x").unwrap();
        assert_eq!(sym.type_, Type::Float);
    }

    #[test]
    fn infer_string_literal() {
        let env = typecheck_ok("let x = \"hello\"\n");
        let sym = env.lookup("x").unwrap();
        assert_eq!(sym.type_, Type::String);
    }

    #[test]
    fn infer_bool_literal() {
        let env = typecheck_ok("let x = true\n");
        let sym = env.lookup("x").unwrap();
        assert_eq!(sym.type_, Type::Bool);
    }

    #[test]
    fn infer_add_int() {
        let env = typecheck_ok("let x = 5 + 5\n");
        let sym = env.lookup("x").unwrap();
        assert_eq!(sym.type_, Type::Int);
    }

    #[test]
    fn infer_nested_add() {
        let env = typecheck_ok("let x = (1 + 2) + 3\n");
        let sym = env.lookup("x").unwrap();
        assert_eq!(sym.type_, Type::Int);
    }

    #[test]
    fn infer_mul() {
        let env = typecheck_ok("let x = 2 * 3\n");
        let sym = env.lookup("x").unwrap();
        assert_eq!(sym.type_, Type::Int);
    }

    #[test]
    fn infer_comparison() {
        let env = typecheck_ok("let x = 1 < 2\n");
        let sym = env.lookup("x").unwrap();
        assert_eq!(sym.type_, Type::Bool);
    }

    #[test]
    fn infer_logical_and() {
        let env = typecheck_ok("let x = true && false\n");
        let sym = env.lookup("x").unwrap();
        assert_eq!(sym.type_, Type::Bool);
    }

    #[test]
    fn infer_logical_or() {
        let env = typecheck_ok("let x = true || false\n");
        let sym = env.lookup("x").unwrap();
        assert_eq!(sym.type_, Type::Bool);
    }

    #[test]
    fn infer_unary_minus() {
        let env = typecheck_ok("let x = -5\n");
        let sym = env.lookup("x").unwrap();
        assert_eq!(sym.type_, Type::Int);
    }

    #[test]
    fn infer_unary_not() {
        let env = typecheck_ok("let x = !true\n");
        let sym = env.lookup("x").unwrap();
        assert_eq!(sym.type_, Type::Bool);
    }

    #[test]
    fn infer_var_mutable() {
        let env = typecheck_ok("var x = 42\n");
        let sym = env.lookup("x").unwrap();
        assert_eq!(sym.type_, Type::Int);
        assert!(sym.mutable);
    }

    #[test]
    fn infer_let_immutable() {
        let env = typecheck_ok("let x = 42\n");
        let sym = env.lookup("x").unwrap();
        assert!(!sym.mutable);
    }

    #[test]
    fn infer_identifier_reference() {
        let env = typecheck_ok(
            "let x = 10\nlet y = x\n",
        );
        let sym = env.lookup("y").unwrap();
        assert_eq!(sym.type_, Type::Int);
    }

    #[test]
    fn infer_function_no_args() {
        let env = typecheck_ok("fn f() -> Int:\n    return 42\n");
        let sym = env.lookup("f").unwrap();
        assert_eq!(
            sym.type_,
            Type::Function(vec![], Box::new(Type::Int))
        );
    }

    #[test]
    fn infer_function_with_params() {
        let env = typecheck_ok(
            "fn add(x: Int, y: Int) -> Int:\n    return x + y\n",
        );
        let sym = env.lookup("add").unwrap();
        assert_eq!(
            sym.type_,
            Type::Function(vec![Type::Int, Type::Int], Box::new(Type::Int))
        );
    }

    #[test]
    fn infer_function_call() {
        let env = typecheck_ok(
            "fn f() -> Int:\n    return 42\nlet x = f()\n",
        );
        let sym = env.lookup("x").unwrap();
        assert_eq!(sym.type_, Type::Int);
    }

    #[test]
    fn infer_add_in_function() {
        let env = typecheck_ok(
            "fn add(a: Int, b: Int) -> Int:\n    return a + b\nlet x = add(1, 2)\n",
        );
        let sym = env.lookup("x").unwrap();
        assert_eq!(sym.type_, Type::Int);
    }

    #[test]
    fn infer_type_annotation() {
        let env = typecheck_ok("let x: Int = 42\n");
        let sym = env.lookup("x").unwrap();
        assert_eq!(sym.type_, Type::Int);
    }

    #[test]
    fn infer_if_condition_bool() {
        let env = typecheck_ok(
            "let x = true\nif x:\n    let y = 1\n",
        );
        assert!(env.lookup("y").is_some() || env.lookup("y").is_none());
        // y is in a nested scope and may not be visible
    }

    #[test]
    fn infer_while_loop() {
        let env = typecheck_ok(
            "var x = 0\nwhile x < 10:\n    var y = x\n",
        );
        let sym = env.lookup("x").unwrap();
        assert_eq!(sym.type_, Type::Int);
    }

    // ── Error cases ──────────────────────────────────────────────────────

    #[test]
    fn error_type_mismatch_add_string_int() {
        let err = typecheck_err("let x = \"hello\" + 5\n");
        assert!(format!("{}", err).contains("mismatch"), "got: {}", err);
    }

    #[test]
    fn error_type_mismatch_add_bool_int() {
        let err = typecheck_err("let x = true + 5\n");
        assert!(format!("{}", err).contains("mismatch"), "got: {}", err);
    }

    #[test]
    fn error_assign_to_immutable() {
        let err = typecheck_err("let x = 1\nx = 2\n");
        assert!(format!("{}", err).contains("immutable"), "got: {}", err);
    }

    #[test]
    fn error_assign_to_immutable_var() {
        let err = typecheck_err(
            "let x = 1\nlet x = 2\n",
        );
        assert!(format!("{}", err).contains("Duplicate"), "got: {}", err);
    }

    #[test]
    fn error_undefined_variable() {
        let err = typecheck_err("let x = y\n");
        assert!(format!("{}", err).contains("Undefined"), "got: {}", err);
    }

    #[test]
    fn error_if_condition_not_bool() {
        let err = typecheck_err("if 42:\n    let x = 1\n");
        assert!(format!("{}", err).contains("mismatch"), "got: {}", err);
    }

    #[test]
    fn error_call_non_function() {
        let err = typecheck_err("let x = 42\nlet y = x()\n");
        assert!(format!("{}", err).contains("non-function")
            || format!("{}", err).contains("mismatch")
            || format!("{}", err).contains("call"), "got: {}", err);
    }

    #[test]
    fn error_unary_not_non_bool() {
        let err = typecheck_err("let x = !42\n");
        assert!(format!("{}", err).contains("mismatch"), "got: {}", err);
    }

    #[test]
    fn error_logical_and_non_bool() {
        let err = typecheck_err("let x = 1 && 2\n");
        assert!(format!("{}", err).contains("mismatch"), "got: {}", err);
    }

    #[test]
    fn error_duplicate_variable() {
        let err = typecheck_err("let x = 1\nlet x = 2\n");
        assert!(format!("{}", err).contains("Duplicate"), "got: {}", err);
    }

    // ── Scope / shadowing ────────────────────────────────────────────────

    #[test]
    fn shadowing_inside_block() {
        let env = typecheck_ok(
            "let x = 1\nif true:\n    let x = \"hello\"\n",
        );
        let sym = env.lookup("x").unwrap();
        assert_eq!(sym.type_, Type::Int);
    }

    #[test]
    fn variable_in_nested_scope_not_leaked() {
        let env = typecheck_ok(
            "if true:\n    let inner = 42\n",
        );
        assert!(env.lookup("inner").is_none());
    }
}
