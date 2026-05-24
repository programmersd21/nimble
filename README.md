# Nimble

A systems programming language with Python-like syntax and LLVM-powered performance.

## Features at a Glance

- **Pythonic syntax** - indentation-based blocks, no curly braces or semicolons
- **Static type system** with Hindley-Milner type inference and explicit casting (`as`)
- **LLVM codegen** - emits textual LLVM IR, assembles via `clang -c`
- **Cross-platform linker** - auto-discovers `cc`, `clang`, `gcc`, or `link.exe`
- **Built-in print** - `print("text")`, `print_int(42)`, `print_str("text")` backed by the ember runtime
- **Standard library** - `load std.<module>` and `load std` for the unified stdlib namespace
- **C FFI** - `extern fn printf(fmt: String) -> Int` for binding native libraries
- **Rich diagnostics** - parse and type errors rendered with source context via miette
- **Auto-run** - `smelt file.nbl -r` compiles and runs in one step
- **Built-in tooling** - compiler (`smelt`), project manager (`anvil`), formatter (`chisel`), LSP (`lantern`), REPL (`forge`), runtime (`ember`)

## Quick Start

```sh
# Install LLVM / clang 18+
# Ubuntu/Debian: sudo apt install llvm-18 clang
# macOS:         brew install llvm
# Windows:       https://llvm.org/builds/

# Build the toolchain
cargo build --release

# Write and compile a program
cat > hello.nbl << 'EOF'
fn main() -> Int:
    print("hello, world")
    return 0
EOF

./target/release/smelt hello.nbl -r
```

## Language

```nimble
fn fib(n: Int) -> Int:
    if n <= 1:
        return n
    else:
        return fib(n - 1) + fib(n - 2)

fn main() -> Int:
    print_str("fibonacci(")
    print_int(fib(10))
    print_str(")\n")
    return 0
```

### Types

| Type     | LLVM repr | Description          |
|----------|-----------|----------------------|
| `Int`    | `i64`     | 64-bit signed integer |
| `Float`  | `double`  | 64-bit IEEE float    |
| `Bool`   | `i1`      | Boolean              |
| `String` | `ptr`     | Null-terminated C string |
| `Void`   | `void`    | No return value      |

### Built-in Functions

| Function | Signature | Runtime symbol |
|----------|-----------|----------------|
| `print`  | `(String) -> Void` | `nimble_print` (with newline) |
| `print_int` | `(Int) -> Void` | `nimble_print_i64` |
| `print_str` | `(String) -> Void` | `nimble_print_str` (no newline) |

### Standard Library

The standard library lives under `std/` and can be imported with:

- `load std` - root `std` aggregator module
- `load std.io` - I/O helpers
- `load std.math` - math functions
- `load std.core` - common utility functions
- `load std.alloc` - allocation helpers
- `load std.log` - logging helpers
- `load std.testing` - assertion helpers

See `docs/stdlib.md` for the full stdlib overview and API reference.

## Toolchain

| Crate | Name     | Role                              |
|-------|----------|-----------------------------------|
| -     | `nimble` | Core compiler library (lexer, parser, typechecker, codegen) |
| `smelt`| Compiler | Compile `.nbl` → executable. Flags: `-o <file>`, `--emit-llvm`, `-r` / `--run` |
| `anvil`| Project  | `init`, `build`, `run`. Build accepts `-r` to auto-run after compile |
| `nim`| Package Manager | `fetch`, `pkg install`, `install`. Decentralized URI-driven dependency and binary management |
| `ember`| Runtime  | Static library linked into every executable (print, alloc, string ops) |
| `lantern`| LSP    | Language server protocol implementation |
| `chisel`| Formatter| Canonical source code formatting |
| `forge` | REPL    | Interactive REPL with JIT compilation |

## Pipeline

```
source.nbl  →  lexer  →  parser  →  typechecker  →  codegen  →  .ll  →  clang -c  →  .obj
                                                                                         │
                                                                                   linker (clang -o)
                                                                                         │
                                                                                     a.exe  ←  ember.lib
```

## Recent Changes

- **Replaced `llc` with `clang -c`** for assembly - no LLVM opt/llc dependency needed
- **Named LLVM parameters** - function params use `%name` instead of unnamed `%0`/`%1`, fixing register numbering conflicts with string literal globals
- **String literals at module level** - pre-pass collects all string constants and emits them before function bodies
- **Register type tracking** - new `register_types` HashMap replaces fragile IR-text-scanning heuristic for type resolution
- **Terminator-aware codegen** - `if`/`elif`/`else` and `while` only emit branch instructions when the block isn't already terminated by `ret`
- **Built-in `print`/`print_int`/`print_str`** - registered by the typechecker, mapped to ember runtime symbols in codegen
- **`nimble_print_str`** - new runtime function for string output without trailing newline (unlike `nimble_print` which uses `writeln!`)
- **`nimble_print_i64`** - changed from `writeln!` to `write!` for piecewise output construction
- **Rich error diagnostics** - miette with `fancy` feature renders `ParseError`/`TypeError` with source snippets, colors, and labels
- **`--run` / `-r` flag** - on both `smelt` and `anvil build` for compile-and-run in one step
- **Runtime discovery** - smelt walks up from its own executable path to find `ember/Cargo.toml`, with `NIMBLE_RUNTIME` env var override
- **Windows linker libs** - added `ws2_32`, `ntdll`, `bcrypt`, `ole32`, `oleaut32`, `userenv`, `msi`, `cfgmgr32` for MinGW Rust std linking
- **chisel writes files** - formatter output goes back to the source file, not stdout
- **anvil init template** - generates current-syntax source (`fn main() -> Int: ...`)

## Build Options

```sh
cargo build --release --workspace
```

The release profile enables LTO, single codegen unit, panic=abort, and symbol stripping.
