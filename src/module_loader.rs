use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use crate::ast::Stmt;
use crate::env::{Environment, Symbol, SymbolKind};
use crate::lexer::Span;
use crate::parser::Parser;
use crate::typechecker::TypeChecker;
use crate::types::Type;

/// A loaded and type-checked module.
#[derive(Debug, Clone)]
struct LoadedModule {
    stmts: Vec<Stmt>,
    env: Environment,
}

#[derive(Debug, Clone)]
struct ModuleLoaderState {
    loaded: HashMap<String, LoadedModule>,
    loading: Vec<String>,
}

/// Error during module resolution or loading.
#[derive(Debug)]
pub enum ModuleError {
    NotFound { name: String },
    SymbolNotFound { module: String, symbol: String },
    Parse(String),
    TypeError(String),
    Cyclic(String),
}

impl std::fmt::Display for ModuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleError::NotFound { name } => write!(f, "module `{}` not found", name),
            ModuleError::SymbolNotFound { module, symbol } => {
                write!(f, "symbol `{}` not found in module `{}`", symbol, module)
            }
            ModuleError::Parse(msg) => write!(f, "module parse error: {}", msg),
            ModuleError::TypeError(msg) => write!(f, "module type error: {}", msg),
            ModuleError::Cyclic(name) => write!(f, "cyclic module dependency: `{}`", name),
        }
    }
}

/// Resolves and loads Nimble modules for the `load` statement.
///
/// Supports:
/// - `load std.xxx` - stdlib modules resolved from `stdlib_dirs`
/// - `load ./path` - relative to the source file
/// - `load c.xxx` - C FFI import (extern function declaration)
#[derive(Clone)]
pub struct ModuleLoader {
    stdlib_dirs: Vec<PathBuf>,
    source_dir: Option<PathBuf>,
    state: Rc<RefCell<ModuleLoaderState>>,
}

impl ModuleLoader {
    pub fn new(stdlib_dirs: Vec<PathBuf>, source_dir: Option<PathBuf>) -> Self {
        ModuleLoader {
            stdlib_dirs,
            source_dir,
            state: Rc::new(RefCell::new(ModuleLoaderState {
                loaded: HashMap::new(),
                loading: Vec::new(),
            })),
        }
    }

    /// Load a module and import its symbols into `target_env`.
    ///
    /// * `module_path` - the dotted path, e.g. `["std", "io"]`
    /// * `symbols` - `Some(&[...])` for selective import (`::{}`), `None` for namespace import
    /// * `alias` - optional `as` alias
    /// * `target_env` - the environment to import symbols into
    /// * `source` - source text for error reporting
    /// * `span` - span of the `load` statement for error reporting
    ///
    /// Returns the list of `extern fn` statements from the module (for codegen).
    pub fn load(
        &mut self,
        module_path: &[String],
        symbols: Option<&[String]>,
        alias: Option<&str>,
        target_env: &mut Environment,
        collected_externs: &std::rc::Rc<std::cell::RefCell<Vec<Stmt>>>,
        collected_module_stmts: &std::rc::Rc<std::cell::RefCell<Vec<(Stmt, Environment)>>>,
        _source: &str,
        span: Span,
    ) -> Result<Vec<Stmt>, ModuleError> {
        let module_key = module_path.join(".");

        // C FFI import: `load c.printf`
        if module_path.len() >= 2 && module_path[0] == "c" {
            let fn_name = module_path[1..].join(".");
            // Register as an external function in the environment.
            // We use a dummy Type::Function for now. 
            // In a real implementation, we might want to lookup the signature.
            target_env.define(&fn_name, Symbol {
                kind: SymbolKind::Function,
                mutable: false,
                type_: Type::Function(
                    vec![], // Variadic or unknown
                    Box::new(Type::Int),
                ),
                defined_at: span,
            });
            return Ok(Vec::new());
        }

        // Resolve the file path.
        let file_path = if module_path[0].starts_with('.') {
            // Relative import: `load ./lexer` or `load ../parser.ast`
            let rel: String = module_path.join("/");
            let rel = rel.trim_start_matches("./").to_string();
            let dir = self.source_dir.as_ref().ok_or_else(|| ModuleError::NotFound {
                name: module_key.clone(),
            })?;
            dir.join(&rel).with_extension("nbl")
        } else if module_path[0] == "crate" {
            // Crate-relative import: `load crate.parser`
            let rel = if module_path.len() > 1 {
                module_path[1..].join("/")
            } else {
                return Err(ModuleError::NotFound {
                    name: module_key.clone(),
                });
            };
            let dir = self.source_dir.as_ref().ok_or_else(|| ModuleError::NotFound {
                name: module_key.clone(),
            })?;
            dir.join(&rel).with_extension("nbl")
        } else {
            // Stdlib import: look in stdlib_dirs.
            // We support both `stdlib_dir = <workspace>/std` and `stdlib_dir = <workspace>`.
            let stdlib_path_base: String = module_path.join("/");
            let mut found = None;
            for dir in &self.stdlib_dirs {
                let candidates = if module_path[0] == "std" {
                    let stripped = module_path[1..].join("/");
                    if stripped.is_empty() {
                        vec![
                            dir.join("mod").with_extension("nbl"),
                            dir.join("std").join("mod").with_extension("nbl"),
                        ]
                    } else {
                        vec![
                            dir.join(&stripped).with_extension("nbl"),
                            dir.join(&stripped).join("mod").with_extension("nbl"),
                            dir.join("std").join(&stripped).with_extension("nbl"),
                            dir.join("std").join(&stripped).join("mod").with_extension("nbl"),
                        ]
                    }
                } else {
                    vec![
                        dir.join(&stdlib_path_base).with_extension("nbl"),
                        dir.join(&stdlib_path_base).join("mod").with_extension("nbl"),
                    ]
                };

                for fp in candidates {
                    if fp.exists() {
                        found = Some(fp);
                        break;
                    }
                }
                if found.is_some() {
                    break;
                }
            }
            found.ok_or_else(|| ModuleError::NotFound {
                name: module_key.clone(),
            })?
        };

        let module_dir = file_path.parent().map(|p| p.to_path_buf());

        // Check for cycles.
        if self.state.borrow().loading.contains(&module_key) {
            return Err(ModuleError::Cyclic(module_key));
        }

        // Load from cache if already loaded.
        if let Some(loaded) = self.state.borrow().loaded.get(&module_key) {
            let pairs: Vec<(Stmt, Environment)> = loaded.stmts.iter()
                .filter(|s| matches!(s, Stmt::FunctionDef { .. }))
                .map(|s| (s.clone(), loaded.env.clone()))
                .collect();
            collected_module_stmts.borrow_mut().extend(pairs);
            Self::import_from_env(&loaded.env, target_env, symbols, alias, &module_path, span)?;
            return Ok(loaded.stmts.iter().filter(|s| matches!(s, Stmt::ExternFn { .. })).cloned().collect());
        }

        // Read and parse the file.
        self.state.borrow_mut().loading.push(module_key.clone());
        let src = std::fs::read_to_string(&file_path)
            .map_err(|_| ModuleError::NotFound { name: module_key.clone() })?;

        // ...
        let prog = Parser::new(&src)
            .map_err(|e| ModuleError::Parse(format!("{:?}", e)))?
            .parse()
            .map_err(|e| ModuleError::Parse(format!("{:?}", e)))?;

        let externs: Vec<Stmt> = prog.statements.iter().filter(|s| matches!(s, Stmt::ExternFn { .. })).cloned().collect();
        let fn_defs: Vec<Stmt> = prog.statements.iter().filter(|s| matches!(s, Stmt::FunctionDef { .. })).cloned().collect();

        let mut nested_loader = self.clone();
        nested_loader.source_dir = module_dir.clone();
        
        let env = TypeChecker::with_externs(&src, collected_externs.clone())
            .with_loader(nested_loader)
            .check_program(&prog)
            .map_err(|e| ModuleError::TypeError(format!("{:?}", e)))?;

        let mut state = self.state.borrow_mut();
        state.loading.retain(|n| n != &module_key);
        state.loaded.insert(module_key.clone(), LoadedModule {
            stmts: prog.statements.clone(),
            env: env.clone(),
        });

        // Store the loaded module statements for later code generation.
        collected_externs.borrow_mut().extend(externs.clone());
        let pairs: Vec<(Stmt, Environment)> = fn_defs.into_iter().map(|s| (s, env.clone())).collect();
        collected_module_stmts.borrow_mut().extend(pairs);

        // Import symbols into the target environment.
        Self::import_from_env(&env, target_env, symbols, alias, &module_path, span)?;

        // Return extern fn statements for codegen.
        Ok(externs)
    }

    /// Import symbols from a module's environment into the target.
    fn import_from_env(
        module_env: &Environment,
        target_env: &mut Environment,
        symbols: Option<&[String]>,
        alias: Option<&str>,
        module_path: &[String],
        _span: Span,
    ) -> Result<(), ModuleError> {
        if let Some(selective_syms) = symbols {
            // Selective import: `load std.io::{print_int}`
            for sym_name in selective_syms {
                if let Some(sym) = module_env.lookup_global(sym_name) {
                    target_env.define(sym_name, sym.clone());
                } else {
                    return Err(ModuleError::SymbolNotFound {
                        module: module_path.join("."),
                        symbol: sym_name.clone(),
                    });
                }
            }
        } else {
            // Namespace import: `load std.io` or `load std.io as myio`
            // Prefix all symbols with the module path or alias.
            let prefix = alias.map(|s| s.to_string()).unwrap_or_else(|| module_path.join("."));

            // To ensure we get all symbols defined in the module, iterate over all
            // scopes in the module's environment and import them into the target.
            for scope in &module_env.scopes {
                for (name, sym) in scope {
                    // Only import functions and public symbols if applicable.
                    // For now, import everything defined in the module as a qualified name.
                    let prefixed_name = format!("{}.{}", prefix, name);
                    target_env.define(&prefixed_name, sym.clone());
                }
            }
        }
        Ok(())
    }
}
