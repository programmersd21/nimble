use crate::compiler::Compiler;
use crate::error::report_for_span;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::vm::Value;
use crate::vm::VM;
use colored::Colorize;
use miette::Report;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::sync::Arc;

pub fn start() {
    let mut rl = DefaultEditor::new().unwrap();
    let mut vm = VM::new();
    println!("Nimble v0.1.0");
    println!("Type :help for commands");

    loop {
        let readline = rl.readline(">>> ");
        match readline {
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

                let _ = execute_line(&line, &mut vm);
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => {
                eprintln!("REPL Error: {:?}", err);
                break;
            }
        }
    }
}

fn execute_line(line: &str, vm: &mut VM) -> Result<(), ()> {
    // We add a newline to satisfy the parser's requirement for statement termination
    let source = format!("{}\n", line);
    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            let report = report_for_span(
                "<repl>",
                &source,
                format!("Lexer error: {}", e.message),
                e.span,
                "here",
            );
            eprintln!("{report}");
            return Err(());
        }
    };

    let mut parser = Parser::new(tokens);
    let stmts = match parser.parse() {
        Ok(s) => s,
        Err(errs) => {
            for err in errs {
                let report = report_for_span(
                    "<repl>",
                    &source,
                    format!("Parser error: {}", err.message),
                    err.span,
                    "here",
                );
                eprintln!("{report}");
            }
            return Err(());
        }
    };

    let mut compiler = Compiler::new("repl".into());
    let chunk = compiler.compile_stmts(&stmts);

    match vm.run(Arc::new(chunk)) {
        Ok(Value::Null) => {}
        Ok(val) => println!("{:?}", val),
        Err(e) => {
            eprintln!("{}", Report::msg(format!("Runtime error: {}", e)));
            return Err(());
        }
    }
    Ok(())
}

fn handle_globals(command: &str, vm: &VM) {
    let mut tokens = command.split_whitespace().skip(1);
    let mut show_internal = false;
    let mut filter_term: Option<String> = None;

    if let Some(next) = tokens.next() {
        if next.eq_ignore_ascii_case("all") {
            show_internal = true;
            if let Some(term) = tokens.next() {
                filter_term = Some(term.to_lowercase());
            }
        } else {
            filter_term = Some(next.to_lowercase());
        }
    }

    let entries = vm.global_entries();
    if entries.is_empty() {
        println!("No globals defined yet.");
        return;
    }

    if let Some(term) = &filter_term {
        println!("Filtering globals by '{term}'");
    }

    let mut printed = 0;
    for (name, value) in entries.iter() {
        if !show_internal && name.starts_with("__") {
            continue;
        }
        if let Some(term) = &filter_term {
            if !name.to_lowercase().contains(term) {
                continue;
            }
        }

        if printed == 0 {
            println!(
                "{:<24} {:<10} {}",
                "Name".underline(),
                "Type".underline(),
                "Value".underline()
            );
        }

        let name_label = format!("{:<24}", name);
        let type_label = format!("{:<10}", value.type_name());

        let name_display = if name.starts_with("__") {
            name_label.dimmed()
        } else {
            name_label.green()
        };
        let type_display = type_label.cyan();

        println!("{name_display} {type_display} {}", value.stringify());
        printed += 1;
    }

    if printed == 0 {
        println!(
            "No globals match{}.",
            filter_term
                .as_ref()
                .map(|term| format!(" '{term}'", term = term))
                .unwrap_or_default()
        );
        println!("Try :globals all to include builtins.");
        return;
    }

    if !show_internal {
        let hidden = entries
            .iter()
            .filter(|(name, _)| name.starts_with("__"))
            .count();
        if hidden > 0 {
            println!(
                "{} internal globals hidden (use :globals all to show)",
                hidden
            );
        }
    }
}
