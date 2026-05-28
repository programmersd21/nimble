use crate::smelt::driver;

use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: smelt <file.nimble> [-o <output>] [--emit-llvm] [--run] [-r] [--clean] [-c]");
        std::process::exit(1);
    }

    let source_path = &args[1];
    let mut output_path = None;
    let mut emit_llvm = false;
    let mut run_after = false;
    let mut clean_after = false;

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "-o" => {
                i += 1;
                output_path = Some(args[i].clone());
            }
            "--emit-llvm" => emit_llvm = true,
            "--run" | "-r" => run_after = true,
            "--clean" | "-c" => clean_after = true,
            other => {
                eprintln!("smelt: unknown flag `{}`", other);
                std::process::exit(1);
            }
        }
        i += 1;
    }

    let source = match std::fs::read_to_string(source_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("smelt: cannot read `{}`: {}", source_path, e);
            std::process::exit(1);
        }
    };

    let out_path = output_path.unwrap_or_else(|| {
        let p = Path::new(source_path).with_extension("exe");
        p.to_string_lossy().to_string()
    });

    let opts = driver::CompileOptions {
        output_path: out_path.clone(),
        source_path: Some(source_path.clone()),
        emit_llvm,
        run_after,
        ..Default::default()
    };

    if let Err(e) = driver::compile(&source, &opts) {
        eprintln!("smelt: error: {}", e);
        std::process::exit(1);
    }

    if clean_after && run_after {
        let _ = std::fs::remove_file(&out_path);
    }
}
