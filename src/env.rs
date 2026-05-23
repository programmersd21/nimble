use std::collections::HashMap;

use crate::lexer::Span;
use crate::types::Type;

/// What kind of binding a symbol represents.
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    /// A plain variable (let or var).
    Variable,
    /// A function definition.
    Function,
    /// A user-defined struct type.
    Struct,
    /// A user-defined interface type.
    Interface,
}

/// A single entry in the symbol table.
#[derive(Debug, Clone)]
pub struct Symbol {
    pub kind: SymbolKind,
    /// Whether this binding can be reassigned (`var` vs `let`).
    pub mutable: bool,
    /// The resolved (or inferred) type of the symbol.
    pub type_: Type,
    /// Source location where this symbol was defined.
    pub defined_at: Span,
}

/// A lexically-scoped symbol table built as a stack of frames.
///
/// Entering a new block pushes a fresh frame; leaving it pops that frame.
/// Name lookups search from the innermost frame outward, implementing
/// standard lexical scoping with shadowing.
#[derive(Debug, Clone)]
pub struct Environment {
    pub scopes: Vec<HashMap<String, Symbol>>,
}

impl Environment {
    /// Create a new, empty global environment (one top-level frame).
    pub fn new() -> Self {
        Environment {
            scopes: vec![HashMap::new()],
        }
    }

    /// Push a new scope frame.
    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Pop the innermost scope frame.
    pub fn exit_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    /// Define a new symbol in the innermost scope.
    pub fn define(&mut self, name: &str, symbol: Symbol) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), symbol);
        }
    }

    /// Look up a symbol by name, searching from the innermost scope outward.
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(sym) = scope.get(name) {
                return Some(sym);
            }
        }
        None
    }

    /// Check if a name is already defined in the *current* (innermost) scope.
    /// Used to detect duplicate definitions in the same block.
    pub fn lookup_current(&self, name: &str) -> Option<&Symbol> {
        self.scopes.last()?.get(name)
    }

    /// Access the global scope for function/type definitions that persist
    /// across block boundaries.
    pub fn lookup_global(&self, name: &str) -> Option<&Symbol> {
        self.scopes.first()?.get(name)
    }

    /// Return all symbols in the global (outermost) scope.
    pub fn get_globals(&self) -> &HashMap<String, Symbol> {
        self.scopes.first().expect("Environment must have at least one scope")
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Span;

    fn dummy_sym(type_: Type, mutable: bool) -> Symbol {
        Symbol {
            kind: SymbolKind::Variable,
            mutable,
            type_,
            defined_at: Span::new(1, 1, 0),
        }
    }

    #[test]
    fn define_and_lookup_global() {
        let mut env = Environment::new();
        env.define("x", dummy_sym(Type::Int, false));
        let sym = env.lookup("x");
        assert!(sym.is_some());
        assert_eq!(sym.unwrap().type_, Type::Int);
    }

    #[test]
    fn scope_enter_exit() {
        let mut env = Environment::new();
        env.define("x", dummy_sym(Type::Int, false));
        env.enter_scope();
        env.define("y", dummy_sym(Type::Bool, false));
        assert!(env.lookup("x").is_some());
        assert!(env.lookup("y").is_some());
        env.exit_scope();
        assert!(env.lookup("x").is_some());
        assert!(env.lookup("y").is_none());
    }

    #[test]
    fn shadowing_works() {
        let mut env = Environment::new();
        env.define("x", dummy_sym(Type::Int, false));
        env.enter_scope();
        env.define("x", dummy_sym(Type::String, false));
        let sym = env.lookup("x");
        assert_eq!(sym.unwrap().type_, Type::String);
        env.exit_scope();
        let sym = env.lookup("x");
        assert_eq!(sym.unwrap().type_, Type::Int);
    }

    #[test]
    fn lookup_current_only_innermost() {
        let mut env = Environment::new();
        env.define("x", dummy_sym(Type::Int, false));
        assert!(env.lookup_current("x").is_some());
        env.enter_scope();
        assert!(env.lookup_current("x").is_none());
        env.exit_scope();
    }

    #[test]
    fn lookup_global_only_top() {
        let mut env = Environment::new();
        env.define("g", dummy_sym(Type::Int, false));
        env.enter_scope();
        env.define("g", dummy_sym(Type::Bool, false));
        assert_eq!(
            env.lookup_global("g").unwrap().type_,
            Type::Int
        );
    }

    #[test]
    fn mutable_flag() {
        let mut env = Environment::new();
        env.define("mut_x", dummy_sym(Type::Int, true));
        env.define("immut_x", dummy_sym(Type::Int, false));
        assert!(env.lookup("mut_x").unwrap().mutable);
        assert!(!env.lookup("immut_x").unwrap().mutable);
    }

    #[test]
    fn undefined_name_returns_none() {
        let env = Environment::new();
        assert!(env.lookup("undefined").is_none());
    }
}
