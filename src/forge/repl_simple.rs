use rustyline::DefaultEditor;

use crate::codegen::Codegen;
use crate::parser::Parser;
use crate::typechecker::TypeChecker;

pub fn run_repl() -> Result<(), String> {
    let mut rl =
        DefaultEditor::new().map_err(|e| format!("failed to initialise rustyline: {}", e))?;

    println!("Nimble REPL (IR preview mode). Type `:quit` to exit.");

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
            Ok(mut parser) => match parser.parse() {
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
            Ok(_) => {
                let ir = cg.into_ir();
                println!("{}", ir);
            }
            Err(e) => {
                eprintln!("Codegen error: {}", e);
            }
        }

        source_accumulator.clear();
    }

    Ok(())
}
