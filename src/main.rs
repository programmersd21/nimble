use clap::{Parser as ClapParser, Subcommand};
use colored::Colorize;
use miette::Report;
use nimble_core::compiler::Compiler;
use nimble_core::error::{install_diagnostic_hook, print_diagnostic, report_for_span, DiagnosticKind};
use nimble_core::lexer::Lexer;
use nimble_core::parser::{ast::Stmt, Parser};
use nimble_core::repl;
use nimble_core::types::infer::Inferencer;
use nimble_core::vm::VM;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(ClapParser)]
#[command(name = "nimble")]
#[command(about = "The Nimble Programming Language", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a .nmb file
    Run {
        file: PathBuf,
        #[arg(last = true)]
        args: Vec<String>,
    },
    /// Type check only
    Check { file: PathBuf },
    /// Start the interactive REPL
    Repl,
    /// Print version info
    Version,
}

fn main() {
    install_diagnostic_hook();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Run { file, args: _ }) => match fs::read_to_string(&file) {
            Ok(source) => run_source(&source, Some(&file)),
            Err(err) => print_diagnostic(
                Report::msg(format!("Failed to read {}: {}", file.display(), err)),
                DiagnosticKind::Error,
                Some("I/O"),
            ),
        },
        Some(Commands::Check { file }) => {
            check_file(&file);
        }
        Some(Commands::Repl) | None => {
            repl::repl::start();
        }
        Some(Commands::Version) => {
            println!("Nimble v0.1.0");
        }
    }
}

fn parse_source(name: &str, source: &str) -> Result<Vec<Stmt>, ()> {
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            let report = report_for_span(
                name,
                source,
                format!("Lexer error: {}", e.message),
                e.span,
                "here",
            );
            print_diagnostic(report, DiagnosticKind::Error, Some("lexical"));
            return Err(());
        }
    };

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(stmts) => Ok(stmts),
        Err(errs) => {
            for err in errs {
                let report = report_for_span(
                    name,
                    source,
                    format!("Parser error: {}", err.message),
                    err.span,
                    "here",
                );
                print_diagnostic(report, DiagnosticKind::Error, Some("parser"));
            }
            Err(())
        }
    }
}

fn check_file(path: &PathBuf) {
    println!(
        "{}",
        format!("🔍 Checking {} ...", path.display())
            .bright_blue()
            .bold()
    );
    let source = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(err) => {
            print_diagnostic(
                Report::msg(format!("Failed to read {}: {}", path.display(), err)),
                DiagnosticKind::Error,
                Some("check"),
            );
            return;
        }
    };
    let name = path.display().to_string();
    let stmts = match parse_source(&name, &source) {
        Ok(list) => list,
        Err(_) => return,
    };

    let mut inferencer = Inferencer::new();
    match inferencer.infer_stmts(&stmts) {
        Ok(_) => println!("{}", "✅ Type check passed.".green().bold()),
        Err(err) => print_diagnostic(
            Report::msg(format!("Type error: {}", err)),
            DiagnosticKind::Error,
            Some("type inference"),
        ),
    }
}

fn run_source(source: &str, path: Option<&PathBuf>) {
    let name = path
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "<stdin>".to_string());
    let stmts = match parse_source(&name, source) {
        Ok(list) => list,
        Err(_) => return,
    };

    let mut compiler = Compiler::new("main".into());
    let chunk = compiler.compile_stmts(&stmts);

    let mut vm = VM::new();
    if let Some(p) = path {
        let dir = p
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .to_path_buf();
        match vm.run_with_dir(Arc::new(chunk), dir) {
            Ok(_) => {}
            Err(e) => print_diagnostic(
                Report::msg(format!("Runtime error: {}", e)),
                DiagnosticKind::Error,
                Some("runtime"),
            ),
        }
        return;
    }
    match vm.run(Arc::new(chunk)) {
        Ok(_) => {}
        Err(e) => print_diagnostic(
            Report::msg(format!("Runtime error: {}", e)),
            DiagnosticKind::Error,
            Some("runtime"),
        ),
    }
}
