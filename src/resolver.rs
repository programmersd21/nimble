use std::collections::HashMap;
use std::fmt;

use crate::ast::*;
use crate::errors::ResolveError;
use crate::lexer::Span;

/// A unique identifier for every definition in a program.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DefId(pub usize);

impl fmt::Display for DefId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DefId({})", self.0)
    }
}

/// What kind of definition a `DefId` refers to.
#[derive(Debug, Clone, PartialEq)]
pub enum DefKind {
    Variable,
    Function,
    Struct,
    Interface,
    Param,
    Builtin,
}

/// Full information about a definition.
#[derive(Debug, Clone)]
pub struct Def {
    pub id: DefId,
    pub name: String,
    pub kind: DefKind,
    pub span: Span,
    pub mutable: bool,
}

/// The result of name resolution: a mapping from every name occurrence
/// to the `DefId` it refers to, plus a table of all definitions.
#[derive(Debug, Clone)]
pub struct ResolvedProgram {
    pub program: Program,
    /// Maps each identifier's span to the definition it resolves to.
    pub resolved: HashMap<usize, DefId>,
    /// All definitions in the program.
    pub definitions: Vec<Def>,
    /// Map from name to the most recent DefId in each scope chain.
    pub name_map: HashMap<String, Vec<DefId>>,
}

impl ResolvedProgram {
    pub fn lookup(&self, name: &str) -> Option<DefId> {
        self.definitions
            .iter()
            .find(|d| d.name == name)
            .map(|d| d.id)
    }

    pub fn lookup_by_span(&self, span: &Span) -> Option<&DefId> {
        self.resolved.get(&span.byte_index)
    }

    pub fn get_def(&self, id: DefId) -> Option<&Def> {
        self.definitions.get(id.0)
    }
}

/// Multi-pass name resolver.
///
/// Pass 1: Walk the AST, collect all definitions (function names, variable
/// declarations, struct/interface names, parameters) and assign each a unique
/// `DefId`. Build scope chains.
///
/// Pass 2: Walk all identifier references and resolve them to their `DefId`.
/// Report undefined variables and duplicate definitions as `ResolveError`.
pub struct Resolver {
    next_id: usize,
    /// Scopes stack: each scope is a set of (name, DefId) pairs.
    scopes: Vec<HashMap<String, DefId>>,
    /// All definitions collected.
    definitions: Vec<Def>,
    /// Resolved identifier byte_index → DefId.
    resolved: HashMap<usize, DefId>,
    /// Errors collected during resolution.
    errors: Vec<ResolveError>,
}

impl Default for Resolver {
    fn default() -> Self {
        Self::new()
    }
}

impl Resolver {
    pub fn new() -> Self {
        let mut r = Resolver {
            next_id: 0,
            scopes: vec![HashMap::new()],
            definitions: Vec::new(),
            resolved: HashMap::new(),
            errors: Vec::new(),
        };
        // Register builtins
        r.define_builtin("print");
        r.define_builtin("print_int");
        r.define_builtin("print_str");
        r.define_builtin("print_float");
        r.define_builtin("Int");
        r.define_builtin("Float");
        r.define_builtin("String");
        r.define_builtin("Bool");
        r.define_builtin("Void");
        r
    }

    pub fn resolve(&mut self, program: &Program) -> ResolvedProgram {
        // Pass 1: collect definitions
        self.collect_definitions(program);

        // Pass 2: resolve references
        self.resolve_program(program);

        ResolvedProgram {
            program: program.clone(),
            resolved: std::mem::take(&mut self.resolved),
            definitions: std::mem::take(&mut self.definitions),
            name_map: HashMap::new(),
        }
    }

    pub fn drain_errors(&mut self) -> Vec<ResolveError> {
        std::mem::take(&mut self.errors)
    }

    fn fresh_id(&mut self) -> DefId {
        let id = DefId(self.next_id);
        self.next_id += 1;
        id
    }

    fn define_builtin(&mut self, name: &str) {
        let id = self.fresh_id();
        self.definitions.push(Def {
            id,
            name: name.to_string(),
            kind: DefKind::Builtin,
            span: Span::new(0, 0, 0),
            mutable: false,
        });
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), id);
        }
    }

    fn define(&mut self, name: &str, kind: DefKind, span: Span, mutable: bool) -> DefId {
        let id = self.fresh_id();
        self.definitions.push(Def {
            id,
            name: name.to_string(),
            kind,
            span,
            mutable,
        });
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(name) {
                self.errors.push(ResolveError::DuplicateDefinition {
                    name: name.to_string(),
                    existing_span: span,
                    new_span: span,
                    src: String::new(),
                    span: (span.byte_index, span.length.max(1)).into(),
                });
            }
            scope.insert(name.to_string(), id);
        }
        id
    }

    /// Insert a name into the current scope without creating a new DefId.
    /// Used during pass 2 to rebuild scopes for reference resolution.
    fn bind_name(&mut self, name: &str, id: DefId, _span: Span) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), id);
        }
    }

    /// Look up an existing definition by name (most recent first) for pass 2.
    fn find_def_id(&self, name: &str) -> Option<DefId> {
        self.definitions
            .iter()
            .rev()
            .find(|d| d.name == name)
            .map(|d| d.id)
    }

    fn lookup_scope(&self, name: &str) -> Option<DefId> {
        for scope in self.scopes.iter().rev() {
            if let Some(&id) = scope.get(name) {
                return Some(id);
            }
        }
        None
    }

    fn find_closest_name(&self, name: &str) -> Option<String> {
        let mut candidates: Vec<&str> = Vec::new();
        for scope in self.scopes.iter() {
            for key in scope.keys() {
                if !candidates.contains(&key.as_str()) {
                    candidates.push(key.as_str());
                }
            }
        }
        let suggestions =
            crate::diagnostics::suggestions::get_spelling_suggestions(name, &candidates, 1);
        suggestions.first().map(|(n, _)| n.clone())
    }

    fn resolve_ident(&mut self, name: &str, span: &Span) {
        if let Some(id) = self.lookup_scope(name) {
            self.resolved.insert(span.byte_index, id);
        } else {
            let src = String::new();
            let suggestion = self.find_closest_name(name);
            self.errors.push(ResolveError::UndefinedVariable {
                name: name.to_string(),
                line: span.line,
                column: span.column,
                src,
                span: (span.byte_index, span.length.max(1)).into(),
                suggestion,
            });
        }
    }

    // ── Pass 1: collect definitions ───────────────────────────────────────

    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn collect_definitions(&mut self, program: &Program) {
        for stmt in &program.statements {
            self.collect_stmt_defs(stmt);
        }
    }

    fn collect_stmt_defs(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::FunctionDef {
                name,
                params,
                body,
                span,
                ..
            } => {
                self.define(name, DefKind::Function, *span, false);
                self.enter_scope();
                for param in params {
                    self.define(&param.name, DefKind::Param, param.span, false);
                }
                for s in body {
                    self.collect_stmt_defs(s);
                }
                self.exit_scope();
            }
            Stmt::StructDef {
                name, fields, span, ..
            } => {
                self.define(name, DefKind::Struct, *span, false);
                self.enter_scope();
                for f in fields {
                    self.define(&f.name, DefKind::Param, f.span, false);
                }
                self.exit_scope();
            }
            Stmt::InterfaceDef { name, span, .. } => {
                self.define(name, DefKind::Interface, *span, false);
            }
            Stmt::Let {
                name, value, span, ..
            } => {
                self.define(name, DefKind::Variable, *span, false);
                self.collect_expr_defs(value);
            }
            Stmt::Var {
                name, value, span, ..
            } => {
                self.define(name, DefKind::Variable, *span, true);
                self.collect_expr_defs(value);
            }
            Stmt::If {
                condition,
                body,
                elifs,
                else_body,
                ..
            } => {
                self.collect_expr_defs(condition);
                self.enter_scope();
                for s in body {
                    self.collect_stmt_defs(s);
                }
                self.exit_scope();
                for (c, b) in elifs {
                    self.collect_expr_defs(c);
                    self.enter_scope();
                    for s in b {
                        self.collect_stmt_defs(s);
                    }
                    self.exit_scope();
                }
                if let Some(b) = else_body {
                    self.enter_scope();
                    for s in b {
                        self.collect_stmt_defs(s);
                    }
                    self.exit_scope();
                }
            }
            Stmt::While {
                condition, body, ..
            } => {
                self.collect_expr_defs(condition);
                self.enter_scope();
                for s in body {
                    self.collect_stmt_defs(s);
                }
                self.exit_scope();
            }
            Stmt::For {
                variable,
                iterable,
                body,
                span,
            } => {
                self.collect_expr_defs(iterable);
                self.define(variable, DefKind::Variable, *span, false);
                self.enter_scope();
                for s in body {
                    self.collect_stmt_defs(s);
                }
                self.exit_scope();
            }
            Stmt::Defer { body, .. } => {
                self.enter_scope();
                for s in body {
                    self.collect_stmt_defs(s);
                }
                self.exit_scope();
            }
            Stmt::MacroDef {
                name,
                params,
                body,
                span,
                ..
            } => {
                self.define(name, DefKind::Function, *span, false);
                self.enter_scope();
                for p in params {
                    self.define(p, DefKind::Param, *span, false);
                }
                for s in body {
                    self.collect_stmt_defs(s);
                }
                self.exit_scope();
            }
            Stmt::Return { value, .. } => {
                if let Some(e) = value {
                    self.collect_expr_defs(e);
                }
            }
            Stmt::ExternFn { name, span, .. } => {
                self.define(name, DefKind::Function, *span, false);
            }
            Stmt::Expr(e) => {
                self.collect_expr_defs(e);
            }
            Stmt::Load { .. } | Stmt::Break { .. } | Stmt::Continue { .. } => {}
        }
    }

    fn collect_expr_defs(&mut self, expr: &Expr) {
        match expr {
            Expr::Lambda { params, body, .. } => {
                self.enter_scope();
                for p in params {
                    self.define(&p.name, DefKind::Param, p.span, false);
                }
                for s in body {
                    self.collect_stmt_defs(s);
                }
                self.exit_scope();
            }
            Expr::Binary { left, right, .. } => {
                self.collect_expr_defs(left);
                self.collect_expr_defs(right);
            }
            Expr::Unary { operand, .. } => self.collect_expr_defs(operand),
            Expr::Call { callee, args, .. } => {
                self.collect_expr_defs(callee);
                for a in args {
                    self.collect_expr_defs(a);
                }
            }
            Expr::Assign { target, value, .. } => {
                self.collect_expr_defs(target);
                self.collect_expr_defs(value);
            }
            Expr::Grouping { expr: inner, .. } => self.collect_expr_defs(inner),
            Expr::MemberAccess { object, .. } => self.collect_expr_defs(object),
            Expr::StructLiteral { fields, .. } => {
                for (_, e) in fields {
                    self.collect_expr_defs(e);
                }
            }
            Expr::Cast { expr: inner, .. } => self.collect_expr_defs(inner),
            Expr::MacroInvocation { args, .. } => {
                for a in args {
                    self.collect_expr_defs(a);
                }
            }
            Expr::IntLiteral(..)
            | Expr::FloatLiteral(..)
            | Expr::StringLiteral(..)
            | Expr::BoolLiteral(..)
            | Expr::Identifier(..) => {}
        }
    }

    // ── Pass 2: resolve references ────────────────────────────────────────

    fn resolve_program(&mut self, program: &Program) {
        for stmt in &program.statements {
            self.resolve_stmt(stmt);
        }
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::FunctionDef {
                name,
                params,
                body,
                span,
                ..
            } => {
                if let Some(id) = self.find_def_id(name) {
                    self.bind_name(name, id, *span);
                }
                self.enter_scope();
                for p in params {
                    if let Some(id) = self.find_def_id(&p.name) {
                        self.bind_name(&p.name, id, p.span);
                    }
                }
                for s in body {
                    self.resolve_stmt(s);
                }
                self.exit_scope();
            }
            Stmt::StructDef {
                name, fields, span, ..
            } => {
                if let Some(id) = self.find_def_id(name) {
                    self.bind_name(name, id, *span);
                }
                self.enter_scope();
                for f in fields {
                    if let Some(id) = self.find_def_id(&f.name) {
                        self.bind_name(&f.name, id, f.span);
                    }
                }
                self.exit_scope();
            }
            Stmt::Let {
                name, value, span, ..
            } => {
                if let Some(id) = self.find_def_id(name) {
                    self.bind_name(name, id, *span);
                }
                self.resolve_expr(value);
            }
            Stmt::Var {
                name, value, span, ..
            } => {
                if let Some(id) = self.find_def_id(name) {
                    self.bind_name(name, id, *span);
                }
                self.resolve_expr(value);
            }
            Stmt::If {
                condition,
                body,
                elifs,
                else_body,
                ..
            } => {
                self.resolve_expr(condition);
                self.enter_scope();
                for s in body {
                    self.resolve_stmt(s);
                }
                self.exit_scope();
                for (c, b) in elifs {
                    self.resolve_expr(c);
                    self.enter_scope();
                    for s in b {
                        self.resolve_stmt(s);
                    }
                    self.exit_scope();
                }
                if let Some(b) = else_body {
                    self.enter_scope();
                    for s in b {
                        self.resolve_stmt(s);
                    }
                    self.exit_scope();
                }
            }
            Stmt::While {
                condition, body, ..
            } => {
                self.resolve_expr(condition);
                self.enter_scope();
                for s in body {
                    self.resolve_stmt(s);
                }
                self.exit_scope();
            }
            Stmt::For {
                variable,
                iterable,
                body,
                span,
                ..
            } => {
                self.resolve_expr(iterable);
                if let Some(id) = self.find_def_id(variable) {
                    self.bind_name(variable, id, *span);
                }
                self.enter_scope();
                for s in body {
                    self.resolve_stmt(s);
                }
                self.exit_scope();
            }
            Stmt::Return { value, .. } => {
                if let Some(e) = value {
                    self.resolve_expr(e);
                }
            }
            Stmt::Expr(e) => {
                self.resolve_expr(e);
            }
            Stmt::Defer { body, .. } => {
                self.enter_scope();
                for s in body {
                    self.resolve_stmt(s);
                }
                self.exit_scope();
            }
            Stmt::MacroDef {
                name,
                params,
                body,
                span,
                ..
            } => {
                if let Some(id) = self.find_def_id(name) {
                    self.bind_name(name, id, *span);
                }
                self.enter_scope();
                for p in params {
                    if let Some(id) = self.find_def_id(p) {
                        self.bind_name(p, id, *span);
                    }
                }
                for s in body {
                    self.resolve_stmt(s);
                }
                self.exit_scope();
            }
            Stmt::Load { .. }
            | Stmt::Break { .. }
            | Stmt::Continue { .. }
            | Stmt::ExternFn { .. }
            | Stmt::InterfaceDef { .. } => {}
        }
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Identifier(name, span) => {
                self.resolve_ident(name, span);
            }
            Expr::Lambda { params, body, .. } => {
                self.enter_scope();
                for p in params {
                    if let Some(id) = self.find_def_id(&p.name) {
                        self.bind_name(&p.name, id, p.span);
                    }
                }
                for s in body {
                    self.resolve_stmt(s);
                }
                self.exit_scope();
            }
            Expr::Binary { left, right, .. } => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            }
            Expr::Unary { operand, .. } => self.resolve_expr(operand),
            Expr::Call { callee, args, .. } => {
                self.resolve_expr(callee);
                for a in args {
                    self.resolve_expr(a);
                }
            }
            Expr::Assign { target, value, .. } => {
                self.resolve_expr(target);
                self.resolve_expr(value);
            }
            Expr::Grouping { expr: inner, .. } => self.resolve_expr(inner),
            Expr::MemberAccess { object, .. } => self.resolve_expr(object),
            Expr::StructLiteral { fields, .. } => {
                for (_, e) in fields {
                    self.resolve_expr(e);
                }
            }
            Expr::Cast { expr: inner, .. } => self.resolve_expr(inner),
            Expr::MacroInvocation { args, .. } => {
                for a in args {
                    self.resolve_expr(a);
                }
            }
            Expr::IntLiteral(..)
            | Expr::FloatLiteral(..)
            | Expr::StringLiteral(..)
            | Expr::BoolLiteral(..) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    fn resolve_prog(src: &str) -> (ResolvedProgram, Vec<ResolveError>) {
        let mut parser = Parser::new(src).expect("parser creation failed");
        let prog = parser.parse().expect("parse failed");
        let mut resolver = Resolver::new();
        let resolved = resolver.resolve(&prog);
        let errors = resolver.drain_errors();
        (resolved, errors)
    }

    #[test]
    fn resolve_builtins() {
        let (resolved, errors) = resolve_prog("let x = print\n");
        assert!(errors.is_empty());
        // print is a builtin
        let _ = resolved.lookup("print").expect("print should be defined");
    }

    #[test]
    fn resolve_user_defined_variable() {
        let (resolved, errors) = resolve_prog("let x = 42\n");
        assert!(errors.is_empty());
        let id = resolved.lookup("x").expect("x should be defined");
        let def = resolved.get_def(id).expect("def should exist");
        assert_eq!(def.name, "x");
        assert_eq!(def.kind, DefKind::Variable);
    }

    #[test]
    fn resolve_mutable_variable() {
        let (resolved, errors) = resolve_prog("var x = 1\n");
        assert!(errors.is_empty());
        let id = resolved.lookup("x").expect("x should be defined");
        let def = resolved.get_def(id).expect("def should exist");
        assert!(def.mutable);
    }

    #[test]
    fn resolve_undefined_variable_error() {
        let (_, errors) = resolve_prog("let x = y\n");
        assert_eq!(errors.len(), 1);
        match &errors[0] {
            ResolveError::UndefinedVariable { name, .. } => {
                assert_eq!(name, "y");
            }
            _ => panic!("expected UndefinedVariable"),
        }
    }

    #[test]
    fn resolve_duplicate_definition_error() {
        let (_, errors) = resolve_prog("let x = 1\nlet x = 2\n");
        // The first `x` is defined, the second is a duplicate.
        // There may also be an UndefinedVariable for the first `x`'s initializer if it uses something undefined
        let dup_errors: Vec<_> = errors
            .iter()
            .filter(|e| matches!(e, ResolveError::DuplicateDefinition { .. }))
            .collect();
        assert!(
            !dup_errors.is_empty(),
            "expected at least one DuplicateDefinition error"
        );
    }

    #[test]
    fn resolve_function_definition() {
        let (resolved, errors) = resolve_prog("fn foo(x: Int) -> Int:\n    return x\n");
        assert!(errors.is_empty(), "unexpected errors: {:?}", errors);
        let id = resolved.lookup("foo").expect("foo should be defined");
        let def = resolved.get_def(id).expect("def should exist");
        assert_eq!(def.kind, DefKind::Function);
    }

    #[test]
    fn resolve_reference_to_function() {
        let (resolved, errors) = resolve_prog("fn foo() -> Int:\n    return 1\nlet a = foo()\n");
        assert!(errors.is_empty(), "unexpected errors: {:?}", errors);
        let id = resolved.lookup("foo").expect("foo should be defined");
        let def = resolved.get_def(id).expect("def should exist");
        assert_eq!(def.kind, DefKind::Function);
    }

    #[test]
    fn resolve_scoped_shadowing() {
        let (_, errors) = resolve_prog("let x = 1\nif true:\n    let x = 2\n    let y = x\n");
        assert!(errors.is_empty(), "unexpected errors: {:?}", errors);
    }

    #[test]
    fn resolve_for_loop_variable() {
        let (_, errors) = resolve_prog("let range = 5\nfor i in range:\n    print_int(i)\n");
        assert!(errors.is_empty(), "unexpected errors: {:?}", errors);
    }

    #[test]
    fn resolve_struct_definition() {
        let (resolved, errors) =
            resolve_prog("struct Point:\n    let x: Int = 0\n    let y: Int = 0\n");
        assert!(errors.is_empty(), "unexpected errors: {:?}", errors);
        let id = resolved.lookup("Point").expect("Point should be defined");
        let def = resolved.get_def(id).expect("def should exist");
        assert_eq!(def.kind, DefKind::Struct);
    }

    #[test]
    fn resolve_interface_definition() {
        let (resolved, errors) =
            resolve_prog("interface Printable:\n    fn print(self: Printable)\n");
        assert!(errors.is_empty(), "unexpected errors: {:?}", errors);
        let id = resolved
            .lookup("Printable")
            .expect("Printable should be defined");
        let def = resolved.get_def(id).expect("def should exist");
        assert_eq!(def.kind, DefKind::Interface);
    }

    #[test]
    fn resolve_member_access_does_not_report_undefined() {
        let (_, errors) = resolve_prog("let a = b.c\n");
        // b is undefined, but .c is a member access not an identifier reference
        let undefined: Vec<_> = errors
            .iter()
            .filter(|e| matches!(e, ResolveError::UndefinedVariable { .. }))
            .collect();
        assert_eq!(undefined.len(), 1);
        match &undefined[0] {
            ResolveError::UndefinedVariable { name, .. } => {
                assert_eq!(name, "b");
            }
            _ => unreachable!(),
        }
    }
}
