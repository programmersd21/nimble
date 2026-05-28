// chisel - Nimble source code formatter
//
// Reads a `.nbl` file, parses it, and emits canonically formatted code with
// 4‑space indentation, strict Python‑style block layout, and no original
// whitespace preserved.

use crate::chisel::fmt;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: chisel <file.nbl>");
        std::process::exit(1);
    }

    let path = &args[1];
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("chisel: cannot read {}: {}", path, e);
            std::process::exit(1);
        }
    };

    let prog = match crate::Parser::new(&source) {
        Ok(mut p) => match p.parse() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("chisel: parse error: {}", e);
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("chisel: lex error: {}", e);
            std::process::exit(1);
        }
    };

    let formatted = fmt::format_program(&prog);
    if let Err(e) = fs::write(path, &formatted) {
        eprintln!("chisel: failed to write {}: {}", path, e);
        std::process::exit(1);
    }
    eprintln!("chisel: formatted {}", path);
}
