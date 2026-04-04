use crate::compiler::bytecode::FunctionChunk;
use crate::compiler::Compiler;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct ModuleResolver {
    stdlib_path: PathBuf,
}

impl ModuleResolver {
    pub fn new() -> Self {
        Self {
            stdlib_path: PathBuf::from("stdlib"),
        }
    }

    pub fn resolve(
        &self,
        source: &str,
        current_dir: &Path,
    ) -> Result<(Arc<FunctionChunk>, PathBuf), String> {
        let path = if source.starts_with('.') {
            // Local resolution
            let mut p = current_dir.to_path_buf();
            p.push(source);
            if p.is_dir() {
                p.push("mod.nmb");
            } else {
                p.set_extension("nmb");
            }
            p
        } else {
            // Stdlib resolution
            let mut p = self.stdlib_path.clone();
            p.push(source);
            p.push("mod.nmb");
            p
        };

        if !path.exists() {
            return Err(format!("Module not found: {}", path.display()));
        }

        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let chunk = self.compile_module(&content, source)?;
        Ok((chunk, path))
    }

    fn compile_module(&self, content: &str, name: &str) -> Result<Arc<FunctionChunk>, String> {
        let mut lexer = Lexer::new(content);
        let tokens = lexer.tokenize().map_err(|e| {
            format!(
                "Lexer error at {}:{}: {}",
                e.span.line, e.span.col, e.message
            )
        })?;
        let mut parser = Parser::new(tokens);
        let stmts = match parser.parse() {
            Ok(s) => s,
            Err(errs) => {
                let first = errs.into_iter().next().unwrap();
                return Err(format!(
                    "Parser error at {}:{}: {}",
                    first.span.line, first.span.col, first.message
                ));
            }
        };
        let mut compiler = Compiler::new(name.to_string());
        let chunk = compiler.compile_stmts(&stmts);
        Ok(Arc::new(chunk))
    }
}
