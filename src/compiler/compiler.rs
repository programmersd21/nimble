//! High-level compiler facade.

use crate::compiler::legacy::compiler as legacy_compiler;
use crate::parser::ast::Stmt;

/// A thin wrapper around the legacy AST compiler that will later be replaced with the
/// new register-based backend.
pub struct Compiler {
    inner: legacy_compiler::Compiler,
}

impl Compiler {
    /// Create a compiler that emits the given top-level function name.
    pub fn new(name: String) -> Self {
        Self {
            inner: legacy_compiler::Compiler::new(name),
        }
    }

    /// Compile AST statements into the existing bytecode chunk.
    pub fn compile_stmts(&mut self, stmts: &[Stmt]) -> bytecode::FunctionChunk {
        self.inner.compile_stmts(stmts)
    }
}
