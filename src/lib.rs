pub mod ast;
pub mod codegen;
pub mod diagnostics;
pub mod driver;
pub mod env;
pub mod errors;
pub mod hir;
pub mod lexer;
pub mod module_loader;
pub mod parser;
pub mod pipeline;
pub mod query;
pub mod resolver;
pub mod typechecker;
pub mod types;

pub use query::{Database, TypecheckResult};

pub mod anvil;
pub mod chisel;
pub mod docgen;
pub mod ember;
pub mod forge;
pub mod fuzzer;
pub mod lantern;
pub mod lint;
pub mod nim;
pub mod profiler;
pub mod selfhost;
pub mod smelt;

pub use codegen::Codegen;
pub use lexer::Lexer;
pub use parser::Parser;
pub use pipeline::PipelineConfig;
pub use typechecker::TypeChecker;
