#[cfg(feature = "jit")]
mod repl_jit;

#[cfg(not(feature = "jit"))]
mod repl_simple;

fn main() {
    #[cfg(feature = "jit")]
    {
        match repl_jit::run_repl() {
            Ok(()) => {}
            Err(e) => {
                eprintln!("REPL error: {}", e);
                std::process::exit(1);
            }
        }
    }

    #[cfg(not(feature = "jit"))]
    {
        eprintln!("Nimble REPL (IR preview mode - compile with `--features jit` for JIT execution)");
        match repl_simple::run_repl() {
            Ok(()) => {}
            Err(e) => {
                eprintln!("REPL error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
