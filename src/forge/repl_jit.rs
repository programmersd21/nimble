use inkwell::OptimizationLevel;
use inkwell::context::Context;
use rustyline::DefaultEditor;

use crate::codegen::Codegen;
use crate::parser::Parser;
use crate::typechecker::TypeChecker;

pub fn run_repl() -> Result<(), String> {
    let mut rl =
        DefaultEditor::new().map_err(|e| format!("failed to initialise rustyline: {}", e))?;

    let context = Context::create();
    let module = context.create_module("nimble_repl");
    let engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| format!("failed to create JIT engine: {}", e))?;

    println!("Nimble REPL (JIT mode). Type `:quit` to exit.");

    let mut source_accumulator = String::new();

    loop {
        let prompt = if source_accumulator.is_empty() {
            ">>> "
        } else {
            "... "
        };

        let line = match rl.readline(prompt) {
            Ok(l) => l,
            Err(_) => break,
        };
        rl.add_history_entry(&line)
            .map_err(|e| format!("history error: {}", e))?;

        let trimmed = line.trim();
        if trimmed == ":quit" || trimmed == ":exit" {
            break;
        }
        if trimmed.is_empty() && source_accumulator.is_empty() {
            continue;
        }

        source_accumulator.push_str(trimmed);
        source_accumulator.push('\n');

        let prog = match Parser::new(&source_accumulator) {
            Ok(parser) => match parser.parse() {
                Ok(p) => p,
                Err(_) => continue,
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                source_accumulator.clear();
                continue;
            }
        };

        let env = match TypeChecker::new(&source_accumulator).check_program(&prog) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Type error: {}", e);
                source_accumulator.clear();
                continue;
            }
        };

        let mut cg = Codegen::new();
        match cg.generate(&prog, &env) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Codegen error: {}", e);
                source_accumulator.clear();
                continue;
            }
        }

        let ir_string = cg.into_ir();

        match context.create_module_from_ir(&ir_string) {
            Ok(repl_module) => {
                if let Some(main_fn) = repl_module.get_function("main") {
                    unsafe {
                        match engine.run_function(main_fn, &[]) {
                            Ok(result) => {
                                let val = result.as_int(true);
                                println!("=> {}", val);
                            }
                            Err(e) => {
                                eprintln!("Execution error: {:?}", e);
                            }
                        }
                    }
                } else {
                    println!("=> ok");
                }
            }
            Err(e) => {
                eprintln!("IR parse error: {:?}", e);
            }
        }

        source_accumulator.clear();
    }

    Ok(())
}
