use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::{
    codegen::Codegen, module_loader::ModuleLoader, parser::Parser, typechecker::TypeChecker,
};
use miette::Report;

#[derive(Debug, Clone)]
pub struct CompileOptions {
    /// Path to the final executable.
    pub output_path: String,
    /// Optional source file path used to resolve relative and crate-relative loads.
    pub source_path: Option<String>,
    /// Emit LLVM IR only (do not link).
    pub emit_llvm: bool,
    /// Optional custom path to the C compiler / linker driver.
    pub linker: Option<String>,
    /// Directory where the ember runtime static library lives.
    pub runtime_dir: Option<PathBuf>,
    /// Keep temporary object / IR files.
    pub keep_temps: bool,
    /// Whether to run the compiled executable immediately.
    pub run_after: bool,
}

impl Default for CompileOptions {
    fn default() -> Self {
        CompileOptions {
            output_path: "output.exe".to_string(),
            source_path: None,
            emit_llvm: false,
            linker: None,
            runtime_dir: None,
            keep_temps: false,
            run_after: false,
        }
    }
}

/// Checks (in order): cc, clang, gcc, link.exe (MSVC).
fn find_linker() -> Result<String, String> {
    let candidates = ["cc", "clang", "gcc", "cl.exe", "link.exe"];
    for name in &candidates {
        if Command::new(name)
            .arg("--version")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .is_ok()
        {
            return Ok(name.to_string());
        }
    }
    // fallback - let the OS resolve it
    Ok("cc".to_string())
}

/// Walk up from start looking for `src/ember/mod.rs`.
fn find_workspace_root(start: &Path) -> Option<PathBuf> {
    let mut dir = Some(start.to_path_buf());
    for _ in 0..8 {
        if let Some(ref d) = dir
            && d.join("src").join("ember").join("mod.rs").exists() {
                return Some(d.clone());
            }
        }
        dir = dir?.parent().map(|p| p.to_path_buf());
    }
    None
}

/// Build the ember static library and return the path to the `.lib` / `.a`.

fn user_home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("USERPROFILE").map(PathBuf::from))
}

fn find_stdlib_dirs() -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    if let Ok(dir) = std::env::var("NIMBLE_STDLIB") {
        candidates.push(PathBuf::from(dir));
    }
    if let Some(home) = user_home_dir() {
        candidates.push(home.join("nimble").join("std"));
    }
    candidates.push(PathBuf::from("src/std"));
    if let Ok(dir) = std::env::var("CARGO_MANIFEST_DIR") {
        candidates.push(PathBuf::from(dir).join("src").join("std"));
    }

    let mut dirs: Vec<PathBuf> = candidates.into_iter().filter(|d| d.exists()).collect();
    dirs.sort();
    dirs.dedup();
    if dirs.is_empty() {
        dirs.push(PathBuf::from("src/std"));
    }
    dirs
}

fn build_runtime_lib(runtime_dir: Option<&PathBuf>) -> Result<PathBuf, String> {
    // Priority 1: NIMBLE_RUNTIME env var pointing directly to a static lib.
    if let Ok(path) = std::env::var("NIMBLE_RUNTIME") {
        let p = PathBuf::from(&path);
        if p.exists() {
            return Ok(p);
        }
        // If it's a directory, look for the lib inside it.
        if p.is_dir() {
            let lib_name = if cfg!(windows) {
                "ember.lib"
            } else {
                "libember.a"
            };
            let lib_path = p.join(lib_name);
            if lib_path.exists() {
                return Ok(lib_path);
            }
        }
    }

    // Priority 2: explicit runtime_dir from the API.
    if let Some(d) = runtime_dir {
        let lib_name = if cfg!(windows) {
            "ember.lib"
        } else {
            "libember.a"
        };
        let lib_path = d.join(lib_name);
        if lib_path.exists() {
            return Ok(lib_path);
        }
        // Treat as source directory
        if d.join("Cargo.toml").exists() {
            let workspace_root = d.parent().unwrap_or(d);
            let profile = if cfg!(debug_assertions) {
                "debug"
            } else {
                "release"
            };
            let target_lib = workspace_root.join("target").join(profile).join(lib_name);
            if target_lib.exists() {
                return Ok(target_lib);
            }
            return build_ember(workspace_root);
        }
    }

    // Priority 3: relative to the smelt executable.
    if let Ok(exe) = std::env::current_exe()
        && let Some(exe_dir) = exe.parent() {
            let lib_name = if cfg!(windows) {
                "ember.lib"
            } else {
                "libember.a"
            };
            // Check for a pre-built lib next to the binary.
            let bundled = exe_dir.join(lib_name);
            if bundled.exists() {
                return Ok(bundled);
            }
            // Walk up to find the workspace root.
            if let Some(root) = find_workspace_root(exe_dir) {
                return build_ember(&root);
            }
        }
    }

    // Priority 4: CARGO_MANIFEST_DIR (set only when running via cargo).
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let start = Path::new(&manifest_dir);
        if let Some(root) = find_workspace_root(start) {
            return build_ember(&root);
        }
    }

    // Priority 5: search from the current working directory.
    if let Ok(cwd) = std::env::current_dir()
        && let Some(root) = find_workspace_root(&cwd) {
            return build_ember(&root);
        }
    }

    Err("ember runtime not found. Set NIMBLE_RUNTIME to the path of ember.lib/libember.a, or run from the Nimble workspace.".to_string())
}

fn build_ember(workspace_root: &Path) -> Result<PathBuf, String> {
    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    let lib_name = if cfg!(windows) {
        "ember.lib"
    } else {
        "libember.a"
    };
    let target_dir = workspace_root.join("target").join(profile);
    let lib_path = target_dir.join(lib_name);

    // If already built, return immediately.
    if lib_path.exists() {
        return Ok(lib_path);
    }

    eprintln!("smelt: building ember runtime ...");

    std::fs::create_dir_all(&target_dir).map_err(|e| {
        format!(
            "failed to create target directory {}: {}",
            target_dir.display(),
            e
        )
    })?;

    let ember_src = workspace_root.join("src").join("ember").join("mod.rs");

    let mut cmd = Command::new("rustc");
    cmd.args([
        "--crate-type",
        "staticlib",
        ember_src.to_str().ok_or("invalid ember source path")?,
        "-o",
        lib_path.to_str().ok_or("invalid library output path")?,
    ]);

    if profile == "release" {
        cmd.args(["-C", "opt-level=3"]);
    }

    let output = cmd
        .output()
        .map_err(|e| format!("failed to invoke rustc: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("rustc build failed: {}", stderr));
    }

    if !lib_path.exists() {
        return Err(format!(
            "ember static library not found at {} after building",
            lib_path.display()
        ));
    }

    Ok(lib_path)
}

pub fn compile(source: &str, options: &CompileOptions) -> Result<(), String> {
    let prog = Parser::new(source)
        .map_err(|e| format!("{:?}", Report::new(e)))?
        .parse()
        .map_err(|e| format!("{:?}", Report::new(e)))?;

    let stdlib_dirs = find_stdlib_dirs();
    let source_path = options.source_path.as_deref();
    let source_dir = source_path.and_then(|p| Path::new(p).parent().map(|p| p.to_path_buf()));
    let loader = ModuleLoader::new(stdlib_dirs, source_dir);

    let externs_rc = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let module_fns_rc: std::rc::Rc<
        std::cell::RefCell<Vec<(crate::ast::Stmt, crate::env::Environment)>>,
    > = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let mut checker = TypeChecker::with_externs_and_module_stmts(
        source,
        externs_rc.clone(),
        module_fns_rc.clone(),
    )
    .with_loader(loader);

    let env = checker
        .check_program(&prog)
        .map_err(|e| format!("{:?}", Report::new(e)))?;

    let ir = {
        let mut cg = Codegen::new();
        cg.generate_with_externs_and_module_fns(&prog, &env, &externs_rc, &module_fns_rc)
            .map_err(|e| format!("codegen error: {}", e))?;
        cg.into_ir()
    };

    if options.emit_llvm {
        let out_path = Path::new(&options.output_path);
        let ll_path = if out_path.extension().is_some_and(|e| e == "ll") {
            out_path.to_path_buf()
        } else {
            out_path.with_extension("ll")
        };
        let mut f = std::fs::File::create(&ll_path)
            .map_err(|e| format!("failed to create {}: {}", ll_path.display(), e))?;
        f.write_all(ir.as_bytes())
            .map_err(|e| format!("failed to write IR: {}", e))?;
        eprintln!("smelt: wrote LLVM IR to {}", ll_path.display());
        return Ok(());
    }

    let tmp_dir = std::env::temp_dir().join(format!("smelt_{}", std::process::id()));
    std::fs::create_dir_all(&tmp_dir).map_err(|e| format!("failed to create temp dir: {}", e))?;

    let ir_path = tmp_dir.join("program.ll");
    let obj_path = tmp_dir.join("program.obj");

    {
        let mut f = std::fs::File::create(&ir_path)
            .map_err(|e| format!("failed to create {}: {}", ir_path.display(), e))?;
        f.write_all(ir.as_bytes())
            .map_err(|e| format!("failed to write IR: {}", e))?;
    }

    let clang_status = Command::new("clang")
        .arg("-c")
        .arg("-o")
        .arg(&obj_path)
        .arg(&ir_path)
        .status()
        .map_err(|e| format!("failed to invoke clang: {} (is LLVM/clang installed?)", e))?;

    if !clang_status.success() {
        return Err("clang assembly failed".to_string());
    }

    let linker = options
        .linker
        .clone()
        .unwrap_or_else(|| find_linker().unwrap_or_else(|_| "cc".to_string()));
    let runtime_lib = build_runtime_lib(options.runtime_dir.as_ref())?;
    let out_path = Path::new(&options.output_path);

    eprintln!("smelt: linking with `{}` → {}", linker, out_path.display());

    let mut cmd = Command::new(&linker);

    if linker == "link.exe" || linker == "cl.exe" {
        // MSVC-style linker
        cmd.arg(&obj_path)
            .arg(&runtime_lib)
            .args(["/OUT:", &options.output_path])
            .arg("/NOLOGO");
    } else {
        // Unix-style linker (cc, gcc, clang, etc.)
        cmd.arg(&obj_path)
            .arg(&runtime_lib)
            .arg("-o")
            .arg(&options.output_path);

        // On Windows, link against system libs required by the ember runtime.
        if cfg!(windows) {
            cmd.args([
                "-luser32",
                "-lkernel32",
                "-ladvapi32",
                "-lshell32",
                "-lws2_32",
                "-lntdll",
                "-lbcrypt",
                "-lole32",
                "-loleaut32",
                "-luserenv",
                "-lmsi",
                "-lcfgmgr32",
            ]);
        }
        // On Unix, link pthread and dl (commonly needed)
        if !cfg!(windows) {
            cmd.args(["-lpthread", "-ldl", "-lm"]);
        }
    }

    let link_status = cmd
        .status()
        .map_err(|e| format!("linker invocation failed: {}", e))?;

    if !link_status.success() {
        return Err("linking failed".to_string());
    }

    if !options.keep_temps {
        let _ = std::fs::remove_dir_all(&tmp_dir);
    }

    eprintln!("smelt: produced executable: {}", out_path.display());

    if options.run_after {
        eprintln!("smelt: running ...");
        let run_path = std::fs::canonicalize(out_path).unwrap_or_else(|_| out_path.to_path_buf());
        let status = Command::new(&run_path)
            .status()
            .map_err(|e| format!("failed to run executable: {}", e))?;
        if !status.success()
            && let Some(code) = status.code() {
                std::process::exit(code);
            }
        }
    }

    Ok(())
}
