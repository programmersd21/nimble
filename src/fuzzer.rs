/// Simple fuzzer for the Nimble compiler
/// Generates random programs and checks that the compiler doesn't crash
use rand::RngExt;
use rand::SeedableRng;
use rand::rngs::StdRng;
use std::path::PathBuf;

/// Configuration for lexer-targeted fuzzing.
pub struct LexerFuzzConfig {
    pub max_length: usize,
    pub include_null: bool,
    pub include_high_bytes: bool,
}

impl Default for LexerFuzzConfig {
    fn default() -> Self {
        LexerFuzzConfig {
            max_length: 64,
            include_null: true,
            include_high_bytes: true,
        }
    }
}

pub struct Fuzzer {
    seed: u64,
    iterations: u64,
    temp_dir: PathBuf,
}

impl Fuzzer {
    pub fn new(seed: u64, iterations: u64) -> Self {
        let temp_dir = std::env::temp_dir().join("nimble_fuzz");
        let _ = std::fs::create_dir_all(&temp_dir);
        Fuzzer {
            seed,
            iterations,
            temp_dir,
        }
    }

    /// Generate a random identifier
    fn random_ident(&self, rng: &mut impl RngExt) -> String {
        let len = rng.random_range(1..12);
        let first = rng.random_range(b'a'..=b'z') as char;
        let rest: String = (0..len)
            .map(|_| rng.random_range(b'a'..=b'z') as char)
            .collect();
        format!("{}{}", first, rest)
    }

    /// Generate a random integer literal
    fn random_int(&self, rng: &mut impl RngExt) -> i64 {
        rng.random_range(-1000..1000)
    }

    /// Generate a random expression
    fn random_expr(&self, rng: &mut impl RngExt, depth: u32) -> String {
        if depth > 3 {
            return self.random_int(rng).to_string();
        }
        match rng.random_range(0..10) {
            0 => self.random_int(rng).to_string(),
            1 => format!("\"{}\"", self.random_ident(rng)),
            2 => {
                if rng.random_bool(0.5) {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            3 => self.random_ident(rng),
            4 => format!(
                "({} + {})",
                self.random_expr(rng, depth + 1),
                self.random_expr(rng, depth + 1)
            ),
            5 => format!(
                "({} - {})",
                self.random_expr(rng, depth + 1),
                self.random_expr(rng, depth + 1)
            ),
            6 => format!(
                "({} * {})",
                self.random_expr(rng, depth + 1),
                self.random_expr(rng, depth + 1)
            ),
            7 => format!("!{}", self.random_expr(rng, depth + 1)),
            8 => format!(
                "{}({})",
                self.random_ident(rng),
                self.random_expr(rng, depth + 1)
            ),
            _ => self.random_int(rng).to_string(),
        }
    }

    /// Generate a random statement
    fn random_stmt(&self, rng: &mut impl RngExt, depth: u32) -> String {
        if depth > 5 {
            return String::new();
        }
        match rng.random_range(0..15) {
            0 => format!(
                "let {} = {}",
                self.random_ident(rng),
                self.random_expr(rng, depth + 1)
            ),
            1 => format!(
                "var {} = {}",
                self.random_ident(rng),
                self.random_expr(rng, depth + 1)
            ),
            2 => format!("return {}", self.random_expr(rng, depth + 1)),
            3 => format!(
                "if {}:\n    {}",
                self.random_expr(rng, depth + 1),
                self.random_stmt(rng, depth + 1)
            ),
            4 => format!(
                "while {}:\n    {}",
                self.random_expr(rng, depth + 1),
                self.random_stmt(rng, depth + 1)
            ),
            5 => "break".to_string(),
            6 => "continue".to_string(),
            7 => format!("fn {}() -> Int:\n    return 0", self.random_ident(rng)),
            8 => format!("extern fn {}() -> Void", self.random_ident(rng)),
            _ => format!("let _x = {}", self.random_expr(rng, depth + 1)),
        }
    }

    /// Generate a random program
    pub fn generate_program(&self) -> String {
        let mut rng = StdRng::seed_from_u64(self.seed);
        let mut program = String::new();
        program.push_str("fn main() -> Int:\n");

        let stmt_count = rng.random_range(1..8);
        for _ in 0..stmt_count {
            program.push_str("    ");
            program.push_str(&self.random_stmt(&mut rng, 0));
            program.push('\n');
        }

        program.push_str("    return 0\n");
        program
    }

    /// Generate a random byte sequence for lexer fuzzing.
    pub fn generate_lexer_input(&self, config: &LexerFuzzConfig) -> Vec<u8> {
        let mut rng = StdRng::seed_from_u64(self.seed ^ 0xABCD);
        let len = rng.random_range(0..=config.max_length);
        let mut input = Vec::with_capacity(len);
        for _ in 0..len {
            let byte = if config.include_high_bytes && rng.random_bool(0.3) {
                // Include multi-byte UTF-8 leading bytes
                rng.random_range(0xC0u8..=0xF7u8)
            } else if config.include_null && rng.random_bool(0.05) {
                0x00
            } else if rng.random_bool(0.2) {
                rng.random_range(b'\x01'..=b'\x7f')
            } else {
                rng.random_range(b'\x80'..=b'\xBF')
            };
            input.push(byte);
        }
        input
    }

    /// Fuzz the lexer with random byte sequences, returning crashes or panics.
    pub fn fuzz_lexer(&self, iterations: u64) -> Vec<String> {
        let mut crashes = Vec::new();
        let config = LexerFuzzConfig::default();
        for i in 0..iterations {
            let bytes = self.generate_lexer_input(&config);
            // Only test valid UTF-8 (the lexer requires &str)
            if let Ok(s) = String::from_utf8(bytes) {
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let mut lex = crate::lexer::Lexer::new(&s);
                    loop {
                        match lex.next_token() {
                            Ok(tok) if matches!(tok.kind, crate::lexer::TokenKind::Eof) => break,
                            Ok(_) => continue,
                            Err(_) => break,
                        }
                    }
                }));
                if result.is_err() {
                    crashes.push(format!(
                        "Fuzz lexer {}: PANIC/CRASH\nInput (len={}): {:?}",
                        i,
                        s.len(),
                        s
                    ));
                }
            }
        }
        crashes
    }

    /// Run the fuzzer
    pub fn run(&self) -> Result<Vec<String>, String> {
        let mut crashes = Vec::new();
        for i in 0..self.iterations {
            let program = self.generate_program();
            let file_path = self.temp_dir.join(format!("fuzz_{}.nbl", i));
            std::fs::write(&file_path, &program)
                .map_err(|e| format!("failed to write fuzz file: {}", e))?;

            // Try to parse and type-check
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut parser = match crate::parser::Parser::new(&program) {
                    Ok(p) => p,
                    Err(_) => return Ok(()),
                };
                let prog = match parser.parse() {
                    Ok(p) => p,
                    Err(_) => return Ok(()),
                };
                let _ = crate::typechecker::TypeChecker::new(&program).check_program(&prog);
                Ok::<_, String>(())
            }));

            match result {
                Ok(Ok(())) => {} // no crash
                Ok(Err(e)) => {
                    crashes.push(format!(
                        "Fuzz {}: compiler error: {}\nProgram:\n{}",
                        i, e, program
                    ));
                }
                Err(_) => {
                    crashes.push(format!("Fuzz {}: PANIC/CRASH\nProgram:\n{}", i, program));
                }
            }
        }
        Ok(crashes)
    }
}
