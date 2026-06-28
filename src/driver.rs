use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::codegen::Codegen;
use crate::module_loader::ModuleLoader;
use crate::parser::Parser;
use crate::typechecker::TypeChecker;

#[derive(Debug, Clone)]
pub struct CompileOptions {
    /// Path to write the output object file (`.o` / `.obj`).
    pub output_path: String,
    /// Optional source file path used to resolve relative and crate-relative loads.
    pub source_path: Option<String>,
    /// Path to `clang` executable.  Defaults to `"clang"` (must be on PATH).
    pub clang_path: String,
    /// Optimization level (0-3).  Default: 2.
    pub opt_level: u8,
    /// Emit textual LLVM IR instead of object code.
    pub emit_ir: bool,
}

impl Default for CompileOptions {
    fn default() -> Self {
        CompileOptions {
            output_path: "output.obj".to_string(),
            source_path: None,
            clang_path: "clang".to_string(),
            opt_level: 2,
            emit_ir: false,
        }
    }
}

/// Locate the user's home directory.
pub(crate) fn user_home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("USERPROFILE").map(PathBuf::from))
}

/// Find the stdlib directory by searching near the workspace root and user home.
pub(crate) fn find_stdlib_dirs() -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    // Explicit override via NIMBLE_STDLIB
    if let Ok(dir) = std::env::var("NIMBLE_STDLIB") {
        candidates.push(PathBuf::from(dir));
    }

    // User home stdlib path: ~/nimble/std or %USERPROFILE%\nimble\std
    if let Some(home) = user_home_dir() {
        candidates.push(home.join("nimble").join("std"));
    }

    // Relative to CWD (workspace root)
    candidates.push(PathBuf::from("src/std"));

    // Relative to CARGO_MANIFEST_DIR
    if let Ok(dir) = std::env::var("CARGO_MANIFEST_DIR") {
        candidates.push(PathBuf::from(dir).join("src").join("std"));
    }

    let mut dirs: Vec<PathBuf> = candidates.into_iter().filter(|d| d.exists()).collect();
    // Remove duplicates
    dirs.sort();
    dirs.dedup();
    if dirs.is_empty() {
        dirs.push(PathBuf::from("src/std")); // fallback
    }
    dirs
}

pub fn compile(source: &str, options: &CompileOptions) -> miette::Result<()> {
    let prog = Parser::new(source)?.parse()?;

    let stdlib_dirs = find_stdlib_dirs();
    let source_path = options
        .source_path
        .as_deref()
        .unwrap_or(&options.output_path);
    let source_dir = Path::new(source_path).parent().map(|p| p.to_path_buf());
    let loader = ModuleLoader::new(stdlib_dirs, source_dir);

    let externs_rc = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let module_stmts_rc = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let mut checker = TypeChecker::with_externs_and_module_stmts(
        source,
        externs_rc.clone(),
        module_stmts_rc.clone(),
    )
    .with_loader(loader);

    let env = checker.check_program(&prog)?;

    let mut cg = Codegen::new();
    cg.generate_with_externs_and_module_fns(&prog, &env, &externs_rc, &module_stmts_rc)
        .map_err(|e| miette::miette!("codegen error: {}", e))?;
    let ir = cg.into_ir();

    if options.emit_ir {
        let ll_path = Path::new(&options.output_path).with_extension("ll");
        let mut file = std::fs::File::create(&ll_path)
            .map_err(|e| miette::miette!("failed to create {}: {}", ll_path.display(), e))?;
        file.write_all(ir.as_bytes())
            .map_err(|e| miette::miette!("failed to write IR: {}", e))?;
        return Ok(());
    }

    let tmp_dir = std::env::temp_dir();
    let ir_path = tmp_dir.join("nimble_program.ll");
    let mut ir_file = std::fs::File::create(&ir_path)
        .map_err(|e| miette::miette!("failed to create temp IR: {}", e))?;
    ir_file
        .write_all(ir.as_bytes())
        .map_err(|e| miette::miette!("failed to write temp IR: {}", e))?;

    let output = Command::new(&options.clang_path)
        .arg("-c")
        .arg(format!("-O{}", options.opt_level))
        .arg("-o")
        .arg(&options.output_path)
        .arg(
            ir_path
                .to_str()
                .ok_or_else(|| miette::miette!("non-UTF-8 path: {}", ir_path.display()))?,
        )
        .output()
        .map_err(|e| {
            miette::miette!(
                "failed to invoke `{}`: {}. Is LLVM/clang installed?",
                options.clang_path,
                e
            )
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(miette::miette!("clang failed: {}", stderr));
    }

    Ok(())
}

pub fn compile_to_ir(source: &str) -> miette::Result<String> {
    let prog = Parser::new(source)?.parse()?;

    let stdlib_dirs = find_stdlib_dirs();
    let loader = ModuleLoader::new(stdlib_dirs, None);
    let mut checker = TypeChecker::new(source).with_loader(loader);

    let env = checker.check_program(&prog)?;

    let mut cg = Codegen::new();
    cg.generate(&prog, &env)
        .map_err(|e| miette::miette!("codegen error: {}", e))?;

    Ok(cg.into_ir())
}
