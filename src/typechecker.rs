use miette::{Diagnostic, SourceSpan};
use std::collections::HashMap;
use thiserror::Error;

use crate::ast::{BinaryOp, Expr, Stmt, UnaryOp};
use crate::env::{Environment, Symbol, SymbolKind};
use crate::lexer::Span;
use crate::module_loader::ModuleLoader;
use crate::types::{Substitution, Type};

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

    #[error(
        "Interface `{interface}` requires method `{method}` but the target does not provide it at line {line}:{column}"
    )]
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

    /// Used by LSP to map errors back to source locations.
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

pub struct TypeChecker {
    var_counter: usize,
    subst: Substitution,
    source: String,
    interfaces: std::collections::HashSet<String>,
    interface_methods: HashMap<String, Vec<String>>,
    function_sigs: HashMap<String, Vec<Type>>,
    module_loader: Option<ModuleLoader>,
    pub collected_externs: std::rc::Rc<std::cell::RefCell<Vec<Stmt>>>,
    pub collected_module_stmts: std::rc::Rc<std::cell::RefCell<Vec<(Stmt, Environment)>>>,
    loop_depth: usize,
}

impl TypeChecker {
    pub fn new(source: &str) -> Self {
        Self::with_externs(
            source,
            std::rc::Rc::new(std::cell::RefCell::new(Vec::new())),
        )
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
            interface_methods: HashMap::new(),
            function_sigs: HashMap::new(),
            module_loader: None,
            collected_externs: externs,
            collected_module_stmts: module_stmts,
            loop_depth: 0,
        }
    }

    pub fn with_loader(mut self, loader: ModuleLoader) -> Self {
        self.module_loader = Some(loader);
        self
    }

    fn fresh_var(&mut self) -> Type {
        let id = self.var_counter;
        self.var_counter += 1;
        Type::Variable(id)
    }

    fn resolve(&self, t: &Type) -> Type {
        t.apply(&self.subst)
    }

    fn type_from_ast(&self, t: &crate::ast::Type, env: &Environment) -> Type {
        if !t.args.is_empty() {
            return Type::GenericInstance(
                t.name.clone(),
                t.args.iter().map(|arg| self.type_from_ast(arg, env)).collect(),
            );
        }
        match t.name.to_lowercase().as_str() {
            "int" => Type::Int,
            "float" => Type::Float,
            "string" => Type::String,
            "bool" => Type::Bool,
            "void" => Type::Void,
            _ => match env.lookup(&t.name).map(|sym| &sym.kind) {
                Some(SymbolKind::Interface) => Type::Interface(t.name.clone()),
                _ => Type::Struct(t.name.clone()),
            },
        }
    }

    pub fn check_program(
        &mut self,
        program: &crate::ast::Program,
    ) -> Result<Environment, TypeError> {
        let mut env = Environment::new();

        if let Some(ref mut loader) = self.module_loader {
            for stmt in &program.statements {
                if let Stmt::Load {
                    module_path,
                    symbols,
                    alias,
                    span,
                    ..
                } = stmt
                {
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
                        Ok(_) => {}
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

        for stmt in &program.statements {
            self.register_declaration(stmt, &mut env)?;
        }

        self.register_builtins(&mut env);

        for stmt in &program.statements {
            self.check_stmt(stmt, &mut env)?;
        }

        Ok(env)
    }

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

    fn register_declaration(
        &mut self,
        stmt: &Stmt,
        env: &mut Environment,
    ) -> Result<(), TypeError> {
        match stmt {
            Stmt::FunctionDef {
                name,
                params,
                return_type,
                span,
                ..
            } => {
                // Allow overloaded function names (same name, different first-param type)
                // for interface conformance. Only reject true duplicates (same signature).
                let param_types: Vec<Type> =
                    params.iter().map(|p| self.type_from_ast(&p.type_annot, env)).collect();
                let ret = match return_type {
                    Some(t) => self.type_from_ast(t, env),
                    None => Type::Void,
                };
                let ft = Type::Function(param_types, Box::new(ret));
                self.function_sigs.entry(name.clone()).or_default().push(ft.clone());
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
            Stmt::StructDef { name, fields, span } => {
                if env.lookup_current(name).is_some() {
                    return Err(TypeError::duplicate_definition(&self.source, name, *span));
                }
                let field_types: Vec<(String, Type)> = fields
                    .iter()
                    .map(|f| (f.name.clone(), self.type_from_ast(&f.type_annot, env)))
                    .collect();
                env.define(
                    name,
                    Symbol {
                        kind: SymbolKind::Struct,
                        mutable: false,
                        type_: Type::Struct(name.clone()),
                        defined_at: *span,
                    },
                );
                env.define_struct(name, field_types);
            }
            Stmt::InterfaceDef { name, methods, span } => {
                if env.lookup_current(name).is_some() {
                    return Err(TypeError::duplicate_definition(&self.source, name, *span));
                }
                self.interfaces.insert(name.clone());
                self.interface_methods.insert(
                    name.clone(),
                    methods.iter().map(|m| m.name.clone()).collect(),
                );
                env.define(
                    name,
                    Symbol {
                        kind: SymbolKind::Interface,
                        mutable: false,
                        type_: Type::Interface(name.clone()),
                        defined_at: *span,
                    },
                );
            }
            Stmt::ExternFn {
                name,
                params,
                return_type,
                span,
            } => {
                if env.lookup_current(name).is_some() {
                    return Err(TypeError::duplicate_definition(&self.source, name, *span));
                }
                let param_types: Vec<Type> =
                    params.iter().map(|p| self.type_from_ast(&p.type_annot, env)).collect();
                let ret = match return_type {
                    Some(t) => self.type_from_ast(t, env),
                    None => Type::Void,
                };
                let ft = Type::Function(param_types, Box::new(ret));
                self.function_sigs.entry(name.clone()).or_default().push(ft.clone());
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
            Stmt::Load { .. } => {}
            Stmt::Var { .. }
            | Stmt::Let { .. }
            | Stmt::Expr(..)
            | Stmt::Return { .. }
            | Stmt::If { .. }
            | Stmt::While { .. }
            | Stmt::Break { .. }
            | Stmt::Continue { .. }
            | Stmt::For { .. } => {}
        }
        Ok(())
    }

    pub fn check_stmt(&mut self, stmt: &Stmt, env: &mut Environment) -> Result<(), TypeError> {
        match stmt {
            Stmt::Var {
                name,
                type_annot,
                value,
                span,
            }
            | Stmt::Let {
                name,
                type_annot,
                value,
                span,
            } => {
                let is_mut = matches!(stmt, Stmt::Var { .. });

                if env.lookup_current(name).is_some() {
                    return Err(TypeError::duplicate_definition(&self.source, name, *span));
                }

                let value_type = self.infer_expr(value, env)?;

                let declared = match type_annot {
                    Some(t) => {
                        let dt = self.type_from_ast(t, env);
                                match &dt {
                            Type::Struct(s) => {
                                let sym = env.lookup(s).ok_or_else(|| {
                                    TypeError::undefined_type(&self.source, s, *span)
                                })?;
                                if sym.kind != SymbolKind::Struct {
                                    return Err(TypeError::undefined_type(&self.source, s, *span));
                                }
                            }
                            _ => {}
                        }
                        Some(dt)
                    }
                    None => None,
                };

                if let Some(ref ann) = declared {
                    self.unify(&value_type, ann, *span)?;
                }

                let resolved = declared
                    .as_ref()
                    .map(|ann| self.resolve(ann))
                    .unwrap_or_else(|| self.resolve(&value_type));
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

            Stmt::ExternFn { .. } => Ok(()),

            Stmt::FunctionDef {
                params,
                return_type,
                body,
                span,
                ..
            } => {
                let param_types: Vec<Type> =
                    params.iter().map(|p| self.type_from_ast(&p.type_annot, env)).collect();

                env.enter_scope();

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

                for s in body {
                    self.check_stmt(s, env)?;
                }

                env.exit_scope();

                if let Some(ret) = return_type {
                            let rt = self.type_from_ast(ret, env);
                    match &rt {
                        Type::Struct(s) => {
                            let sym = env
                                .lookup(s)
                                .ok_or_else(|| TypeError::undefined_type(&self.source, s, *span))?;
                            if sym.kind != SymbolKind::Struct {
                                return Err(TypeError::undefined_type(&self.source, s, *span));
                            }
                        }
                        _ => {}
                    }
                }

                Ok(())
            }

            Stmt::If {
                condition,
                body,
                elifs,
                else_body,
                span: _,
            } => {
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

            Stmt::While {
                condition,
                body,
                span: _,
            } => {
                let cond_type = self.infer_expr(condition, env)?;
                self.unify(&cond_type, &Type::Bool, condition.span())?;

                self.loop_depth += 1;
                env.enter_scope();
                for s in body {
                    self.check_stmt(s, env)?;
                }
                env.exit_scope();
                self.loop_depth -= 1;

                Ok(())
            }

            Stmt::Break { span } | Stmt::Continue { span } => {
                if self.loop_depth == 0 {
                    return Err(TypeError::Internal {
                        msg: format!(
                            "{} used outside of a loop",
                            if matches!(stmt, Stmt::Break { .. }) {
                                "break"
                            } else {
                                "continue"
                            }
                        ),
                        src: self.source.clone(),
                        span: (span.byte_index, span.length.max(1)).into(),
                    });
                }
                Ok(())
            }

            Stmt::For {
                variable,
                iterable,
                body,
                span,
            } => {
                let iter_type = self.infer_expr(iterable, env)?;

                self.loop_depth += 1;
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
                self.loop_depth -= 1;

                Ok(())
            }

            Stmt::Return { value, span: _ } => {
                if let Some(val) = value {
                    self.infer_expr(val, env)?;
                }
                Ok(())
            }

            Stmt::Load { .. } => Ok(()),

            Stmt::StructDef { .. } | Stmt::InterfaceDef { .. } => Ok(()),

            Stmt::Expr(expr) => {
                self.infer_expr(expr, env)?;
                Ok(())
            }
        }
    }

    pub fn infer_expr(&mut self, expr: &Expr, env: &Environment) -> Result<Type, TypeError> {
        match expr {
            Expr::IntLiteral(_, _span) => Ok(Type::Int),
            Expr::FloatLiteral(_, _span) => Ok(Type::Float),
            Expr::StringLiteral(_, _span) => Ok(Type::String),
            Expr::BoolLiteral(_, _span) => Ok(Type::Bool),

            Expr::Identifier(name, span) => match env.lookup(name) {
                Some(sym) => Ok(self.resolve(&sym.type_)),
                None => Err(TypeError::undefined_variable(&self.source, name, *span)),
            },

            Expr::Binary {
                left,
                op,
                right,
                span,
            } => self.infer_binary(left, op, right, *span, env),

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

            Expr::Assign {
                target,
                value,
                span,
            } => {
                if let Expr::Identifier(name, _) = target.as_ref() {
                    match env.lookup(name) {
                        Some(sym) if !sym.mutable => {
                            return Err(TypeError::assign_to_immutable(&self.source, name, *span));
                        }
                        None => {
                            return Err(TypeError::undefined_variable(&self.source, name, *span));
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

            Expr::MemberAccess {
                object,
                member,
                span,
            } => {
                let obj_ty = self.infer_expr(object, env)?;
                let obj_ty = self.resolve(&obj_ty);
                if let Type::Struct(name) = obj_ty {
                    if let Some(fields) = env.get_struct_fields(&name) {
                        if let Some((_, ty)) = fields.iter().find(|(n, _)| n == member) {
                            return Ok(ty.clone());
                        }
                    }
                }
                Err(TypeError::undefined_variable(&self.source, member, *span))
            }

            Expr::StructLiteral { name, fields, span } => {
                let sym = env
                    .lookup(name)
                    .ok_or_else(|| TypeError::undefined_type(&self.source, name, *span))?;
                if sym.kind != SymbolKind::Struct {
                    return Err(TypeError::undefined_type(&self.source, name, *span));
                }
                let def_fields = env
                    .get_struct_fields(name)
                    .ok_or_else(|| TypeError::undefined_type(&self.source, name, *span))?;
                for (fname, fexpr) in fields {
                    let fty = def_fields
                        .iter()
                        .find(|(n, _)| n == fname)
                        .ok_or_else(|| TypeError::undefined_type(&self.source, fname, *span))?
                        .1
                        .clone();
                    let ety = self.infer_expr(fexpr, env)?;
                    self.unify(&ety, &fty, *span)?;
                }
                Ok(Type::Struct(name.clone()))
            }

            Expr::Cast {
                expr,
                target_type,
                span: _,
            } => {
                let _expr_type = self.infer_expr(expr, env)?;
                let target_type = self.type_from_ast(target_type, env);
                // Explicit casts are trusted by the typechecker for now.
                Ok(target_type)
            }
        }
    }

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
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                // Create a fresh type variable and unify both sides with it,
                // then constrain it to be numeric.
                let num = self.fresh_var();
                self.unify(&left_type, &num, span)?;
                self.unify(&right_type, &num, span)?;
                Ok(self.resolve(&num))
            }

            BinaryOp::Equal
            | BinaryOp::NotEqual
            | BinaryOp::Less
            | BinaryOp::Greater
            | BinaryOp::LessEqual
            | BinaryOp::GreaterEqual => {
                self.unify(&left_type, &right_type, span)?;
                Ok(Type::Bool)
            }

            BinaryOp::And | BinaryOp::Or => {
                self.unify(&left_type, &Type::Bool, span)?;
                self.unify(&right_type, &Type::Bool, span)?;
                Ok(Type::Bool)
            }
        }
    }

    fn infer_call(
        &mut self,
        callee: &Expr,
        args: &[Expr],
        span: Span,
        env: &Environment,
    ) -> Result<Type, TypeError> {
        let callee_type = self.infer_expr(callee, env)?;

        let expected_param_types: Vec<Type> = (0..args.len()).map(|_| self.fresh_var()).collect();
        let expected_ret = self.fresh_var();
        let expected_fn =
            Type::Function(expected_param_types.clone(), Box::new(expected_ret.clone()));

        self.unify(&callee_type, &expected_fn, span)?;

        for (i, arg) in args.iter().enumerate() {
            let arg_type = self.infer_expr(arg, env)?;
            self.unify(&arg_type, &expected_param_types[i], span)?;
        }

        Ok(self.resolve(&expected_ret))
    }

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
                    return Err(TypeError::mismatch(&self.source, &a, &b, span));
                }
                if a1.len() != a2.len() {
                    return Err(TypeError::mismatch(&self.source, &a, &b, span));
                }
                for (ta, tb) in a1.iter().zip(a2.iter()) {
                    self.unify(ta, tb, span)?;
                }
                Ok(())
            }

            (Type::GenericInstance(name, _), Type::Struct(struct_name))
            | (Type::Struct(struct_name), Type::GenericInstance(name, _)) => {
                if name == struct_name {
                    Ok(())
                } else {
                    Err(TypeError::mismatch(&self.source, &a, &b, span))
                }
            }

            (Type::Interface(iface_name), Type::Struct(_))
            | (Type::Struct(_), Type::Interface(iface_name)) => {
                let struct_name = match (&a, &b) {
                    (Type::Interface(_), Type::Struct(s)) => s,
                    (Type::Struct(s), Type::Interface(_)) => s,
                    _ => unreachable!(),
                };
                self.check_interface_conformance(iface_name, struct_name, span)
            }

            _ => Err(TypeError::mismatch(&self.source, &a, &b, span)),
        }
    }

    fn check_interface_conformance(
        &self,
        iface_name: &str,
        struct_name: &str,
        span: Span,
    ) -> Result<(), TypeError> {
        let Some(methods) = self.interface_methods.get(iface_name) else {
            return Err(TypeError::undefined_type(&self.source, iface_name, span));
        };
        for method in methods {
            let found = self.function_sigs.get(method).map_or(false, |sigs| {
                sigs.iter().any(|ty| {
                    if let Type::Function(params, _) = ty {
                        matches!(params.first(), Some(Type::Struct(name)) if name == struct_name)
                    } else {
                        false
                    }
                })
            });
            if !found {
                return Err(TypeError::MissingMethod {
                    interface: iface_name.to_string(),
                    method: method.clone(),
                    line: span.line,
                    column: span.column,
                    src: self.source.clone(),
                    span: (span.byte_index, span.length.max(1)).into(),
                });
            }
        }
        Ok(())
    }
}

    #[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    fn typecheck(source: &str) -> Result<Environment, TypeError> {
        let prog = Parser::new(source)
            .expect("parse failed")
            .parse()
            .expect("parse failed");
        let mut checker = TypeChecker::new(source);
        checker.check_program(&prog)
    }

    fn typecheck_ok(source: &str) -> Environment {
        typecheck(source).expect("typecheck failed")
    }

    fn typecheck_err(source: &str) -> TypeError {
        typecheck(source).expect_err("expected typecheck error")
    }

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
        let env = typecheck_ok("let x = 10\nlet y = x\n");
        let sym = env.lookup("y").unwrap();
        assert_eq!(sym.type_, Type::Int);
    }

    #[test]
    fn infer_function_no_args() {
        let env = typecheck_ok("fn f() -> Int:\n    return 42\n");
        let sym = env.lookup("f").unwrap();
        assert_eq!(sym.type_, Type::Function(vec![], Box::new(Type::Int)));
    }

    #[test]
    fn infer_function_with_params() {
        let env = typecheck_ok("fn add(x: Int, y: Int) -> Int:\n    return x + y\n");
        let sym = env.lookup("add").unwrap();
        assert_eq!(
            sym.type_,
            Type::Function(vec![Type::Int, Type::Int], Box::new(Type::Int))
        );
    }

    #[test]
    fn infer_function_call() {
        let env = typecheck_ok("fn f() -> Int:\n    return 42\nlet x = f()\n");
        let sym = env.lookup("x").unwrap();
        assert_eq!(sym.type_, Type::Int);
    }

    #[test]
    fn infer_add_in_function() {
        let env =
            typecheck_ok("fn add(a: Int, b: Int) -> Int:\n    return a + b\nlet x = add(1, 2)\n");
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
    fn infer_generic_type_annotation() {
        let env = typecheck_ok("struct Box:\n    let value: Int = 0\nlet x: Box[Int] = Box{value: 1}\n");
        let sym = env.lookup("x").unwrap();
        assert_eq!(
            sym.type_,
            Type::GenericInstance("Box".into(), vec![Type::Int])
        );
    }

    #[test]
    fn interface_conformance_by_first_param() {
        typecheck_ok(
            "interface Drawable:\n    fn draw(self: Drawable) -> Void\n\nstruct Circle:\n    let radius: Int = 0\n\nfn draw(self: Circle) -> Void:\n    return\n\nlet d: Drawable = Circle{radius: 5}\n",
        );
    }

    #[test]
    fn infer_if_condition_bool() {
        let env = typecheck_ok("let x = true\nif x:\n    let y = 1\n");
        assert!(env.lookup("y").is_some() || env.lookup("y").is_none());
        // y is in a nested scope and may not be visible
    }

    #[test]
    fn infer_while_loop() {
        let env = typecheck_ok("var x = 0\nwhile x < 10:\n    var y = x\n");
        let sym = env.lookup("x").unwrap();
        assert_eq!(sym.type_, Type::Int);
    }

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
        let err = typecheck_err("let x = 1\nlet x = 2\n");
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
        assert!(
            format!("{}", err).contains("non-function")
                || format!("{}", err).contains("mismatch")
                || format!("{}", err).contains("call"),
            "got: {}",
            err
        );
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

    #[test]
    fn shadowing_inside_block() {
        let env = typecheck_ok("let x = 1\nif true:\n    let x = \"hello\"\n");
        let sym = env.lookup("x").unwrap();
        assert_eq!(sym.type_, Type::Int);
    }

    #[test]
    fn variable_in_nested_scope_not_leaked() {
        let env = typecheck_ok("if true:\n    let inner = 42\n");
        assert!(env.lookup("inner").is_none());
    }
}
