# Nimble

Nimble is a register-based bytecode interpreter and scripting language implemented in Rust. It prioritizes readability, clear diagnostics, and a pragmatic standard library.

## Key Features

- **Register-Based VM:** Efficient execution using a register-file model per call frame.
- **Indentation-Sensitive:** Python-inspired syntax for clean, readable code.
- **Optional Typing:** Type annotations for clarity and basic static analysis.
- **Modern Error Handling:** Errors as first-class values with `?` propagation.
- **Extensible FFI:** Call C-ABI symbols directly from shared libraries.

## Architecture Overview

Nimble follows a standard compiler pipeline:
1. **Lexer:** Hand-written, handles indentation and string interpolation.
2. **Parser:** Recursive-descent parser producing a typed AST.
3. **Compiler:** Translates AST to register-based bytecode.
4. **VM:** Register-based virtual machine with stack-based frame management.

## Quick Start

### Build
```bash
cargo build --release
```

### Run an Example
```bash
cargo run --release -- run examples/basic/functions/defs.nmb
```

### Start the REPL
```bash
cargo run --release -- repl
```

## Standard Library

The Nimble standard library includes modules for common tasks:
- `io`: File and console I/O.
- `ffi`: Foreign Function Interface for calling C libraries.
- `json`: JSON parsing and serialization.
- `math`, `string`, `list`, `map`, `regex`, `time`, `os`, `path`, `process`.

## Verification

Run the test suite:
```bash
cargo test
```

## License

MIT. See [LICENSE](LICENSE).
