# Nimble

A statically typed language with Python-style indentation, LLVM-based code generation, and an integrated toolchain.

## Features at a Glance

- **Pythonic syntax** - indentation-based blocks, no curly braces or semicolons
- **Static type system** with type inference for locals and explicit typing for functions/parameters
- **LLVM codegen** - emits textual LLVM IR, assembles via `clang -c`
- **Cross-platform linker** - auto-discovers `cc`, `clang`, `gcc`, or `link.exe`
- **Built-in print** - `print("text")`, `print_int(42)`, `print_str("text")` backed by the ember runtime
- **Standard library** - `load std.<module>` and `load std` for the unified stdlib namespace
- **C FFI** - `extern fn printf(fmt: String) -> Int` for binding native libraries
- **Rich diagnostics** - parse and type errors rendered with source context via miette
- **Auto-run** - `nimble compile file.nbl -r` compiles and runs in one step
- **Unified Toolchain** - everything integrated into a single `nimble` crate

## Quick Start

```sh
# Install LLVM / clang
# Ubuntu/Debian: sudo apt install llvm clang
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

./target/release/nimble compile hello.nbl -r
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

The standard library lives under `src/std/` and can be imported with:

- `load std` - root `std` aggregator module
- `load std.io` - I/O helpers
- `load std.math` - math functions
- `load std.core` - common utility functions
- `load std.alloc` - allocation helpers
- `load std.log` - logging helpers
- `load std.testing` - assertion helpers

See [`docs/manual/stdlib.md`](docs/manual/stdlib.md) for the full stdlib overview and API reference.

## Toolchain

The entire Nimble toolchain is integrated into a single `nimble` binary.

| Command | Role | Description |
|---------|------|-------------|
| `nimble init` | Project Init | Create a new Nimble project with a default layout |
| `nimble build` | Build System | Build the current project using the manifest |
| `nimble run` | Project Runner | Run the current project |
| `nimble compile` | Compiler | Compile a single `.nbl` file to an executable. Flags: `-o <file>`, `--emit-llvm`, `-r` / `--run` |
| `nimble fmt` | Formatter | Format Nimble source code canonically |
| `nimble repl` | REPL | Start an interactive Nimble REPL |
| `nimble lsp` | LSP Server | Start the Language Server for IDE support |
| `nimble install` | Package Manager | Install standalone binaries from remote URIs |
| `nimble pkg` | Package Manager | Manage library dependencies |
| `ember` | Runtime | Runtime primitives (now a module in `src/ember`) |

## Pipeline

```
source.nbl  →  lexer  →  parser  →  typechecker  →  codegen  →  .ll  →  clang -c  →  .obj
                                                                                         │
                                                                                   linker (clang -o)
                                                                                         │
                                                                                     a.exe  ←  ember.lib
```

## Current Status

Implemented:

- Lexer, parser, AST, type checker, code generator
- `if` / `elif` / `else`, `while`, `for`, `break`, `continue`, `return`, `load`, `extern fn`
- Immutable `let` and mutable `var`
- Struct declarations, struct literals, and field access
- Interface declarations with structural conformance checks
- Generic type syntax and generic instance unification
- Function definitions and basic module loading
- REPL, formatter, and LSP scaffolding
- Project tooling (`init`, `build`, `run`, `pkg`, `install`, `fetch`)

Still early:

- A documented package registry protocol
- Stable semantics for ownership, mutation, and concurrency
- A broader standard library

## Build Options

```sh
cargo build --release
```

The release profile enables LTO, single codegen unit, panic=abort, and symbol stripping.
