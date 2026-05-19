use crate::compiler::Compiler;
use crate::error::{emit_report, NimbleError, NimbleResult, SourceFile};
use crate::lexer::Lexer;
use crate::parser::{ast::Stmt, Parser};
use crate::types::infer::Inferencer;
use crate::vm::Value;
use crate::vm::VM;
use colored::Colorize;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::sync::Arc;

pub fn start() -> NimbleResult<()> {
    let mut rl = DefaultEditor::new()
        .map_err(|error| miette::Report::msg(format!("failed to start the REPL: {error}")))?;
    let mut vm = VM::new();
    let mut inferencer = Inferencer::new();

    println!("Nimble v0.1.0");
    println!("Type :help for commands");

    loop {
        match rl.readline(">>> ") {
            Ok(line) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                let _ = rl.add_history_entry(line.as_str());

                if trimmed.starts_with(":globals") {
                    handle_globals(trimmed, &vm);
                    continue;
                }
                if trimmed.starts_with(':') {
                    match trimmed {
                        ":help" => println!("Commands: :quit, :q, :clear, :globals"),
                        ":quit" | ":q" => break,
                        ":clear" => {
                            let _ = rl.clear_history();
                        }
                        _ => println!("Unknown command: {}", trimmed),
                    }
                    continue;
                }

                if let Err(report) = execute_line(&line, &mut vm, &mut inferencer) {
                    emit_report(&report);
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(error) => {
                return Err(miette::Report::msg(format!("REPL input failed: {error}")));
            }
        }
    }

    Ok(())
}

fn parse_source(source_file: &SourceFile) -> NimbleResult<Vec<Stmt>> {
    let mut lexer = Lexer::new(source_file.source());
    let tokens = lexer
        .tokenize()
        .map_err(|error| miette::Report::new(NimbleError::from_lex(source_file, error)))?;

    let mut parser = Parser::new(tokens);
    parser.parse().map_err(|errors| {
        let mut diagnostics = errors
            .into_iter()
            .map(|error| NimbleError::from_parse(source_file, error))
            .collect::<Vec<_>>();

        if diagnostics.len() == 1 {
            miette::Report::new(diagnostics.remove(0))
        } else {
            miette::Report::new(NimbleError::multiple(
                source_file,
                "failed to parse REPL input",
                diagnostics,
            ))
        }
    })
}

fn type_check(source_file: &SourceFile, stmts: &[Stmt], inferencer: &mut Inferencer) -> NimbleResult<()> {
    inferencer
        .infer_stmts(stmts)
        .map_err(|error| miette::Report::new(NimbleError::from_semantic(source_file, error)))
}

fn execute_line(line: &str, vm: &mut VM, inferencer: &mut Inferencer) -> NimbleResult<()> {
    let source_file = SourceFile::new("<repl>", format!("{line}\n"));
    let stmts = parse_source(&source_file)?;
    type_check(&source_file, &stmts, inferencer)?;

    let mut compiler = Compiler::new("repl".into());
    let chunk = compiler.compile_stmts(&stmts);

    match vm.run(Arc::clone(&chunk)) {
        Ok(Value::Null) => {}
        Ok(value) => println!("{}", value.stringify().cyan()),
        Err(message) => {
            return Err(miette::Report::new(NimbleError::runtime(
                &source_file,
                message,
            )))
        }
    }

    Ok(())
}

fn handle_globals(cmd: &str, vm: &VM) {
    let filter = cmd.strip_prefix(":globals").unwrap_or("").trim();
    let entries = vm.global_entries();
    if entries.is_empty() {
        println!("(no globals)");
        return;
    }
    for (name, val) in entries {
        if filter.is_empty() || name.contains(filter) {
            println!("  {} = {}", name.yellow(), val.stringify());
        }
    }
}
