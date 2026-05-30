# Nimble

A statically typed language with Python-style indentation, LLVM-based code generation, and an integrated toolchain.

## Features at a Glance

- **Pythonic syntax** - indentation-based blocks, no curly braces or semicolons
- **Static type system** with Hindley-Milner type inference, generics, enums, and interfaces
- **Enums (sum types)** with tagged union representation and pattern matching
- **Generic functions** with monomorphization (`fn identity[T](x: T) -> T`)
- **Closures and lambdas** with capture analysis and trampoline codegen
- **Pattern matching** with wildcards, bindings, variant patterns, and literals
- **Method call syntax** (`obj.method(args)`) desugared to `method(obj, args)`
- **`?` operator** for ergonomic error propagation with Result types
- **`defer` statements** for scope-exit cleanup
- **Compile-time macros** with AST substitution
- **Async/concurrency primitives** (future, channel, mutex, thread, atomic)
- **Option[T] / Result[T, E]** algebraic types in stdlib
- **LLVM codegen** with optional debug info emission (`!DILocation`, `DISubprogram`)
- **Cross-platform linker** - auto-discovers `cc`, `clang`, `gcc`, or `link.exe`
- **Standard library** with 23 modules covering I/O, math, collections, testing, async, sync, crypto, net, FFI, and more
- **C FFI** - `extern fn` for binding native libraries
- **Rich diagnostics** - parse and type errors rendered with source context via miette
- **Integrated LSP server** with hover info, goto-definition, and autocompletion
- **Doc generator** - `nimble doc` produces HTML API docs from source
- **Fuzzer** - `nimble fuzz` stress-tests the compiler with random programs
- **Formatter** - `nimble fmt` canonical source formatting
- **Profiler** - `nimble profile` measures compilation and execution timing
- **Self-hosting support** - `nimble generate-header` produces C runtime API headers
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
# Fibonacci
fn fib(n: Int) -> Int:
    if n <= 1:
        return n
    return fib(n - 1) + fib(n - 2)

# Generic function
fn identity[T](x: T) -> T:
    return x

# Enum with pattern matching
enum Option[T]: Some(T), None

fn unwrap_or[T](opt: Option[T], default: T) -> T:
    match opt:
        Some(val):
            return val
        None:
            return default

# Method call syntax
fn print_self(self: String) -> Void:
    print(self)

"hello".print_self()

# Error propagation with ?
fn load_file(path: String) -> Result[String, String]:
    let content = read_file(path)?
    return Ok(content)

# Closure
let doubler = fn(x: Int): x * 2

# Defer for cleanup
fn work() -> Void:
    let handle = open("file.txt")
    defer close(handle)
    # handle is closed on scope exit

# Async
let future = async fetch_data()
let result = await future
```

### Types

| Type | LLVM repr | Description |
|------|-----------|-------------|
| `Int` | `i64` | 64-bit signed integer |
| `Float` | `double` | 64-bit IEEE float |
| `Bool` | `i1` | Boolean |
| `String` | `ptr` | Null-terminated C string |
| `Void` | `void` | No return value |
| `&T` / `&mut T` | `ptr` | Reference / mutable reference |
| `fn(...) -> ...` | `ptr` or `{ptr, ptr}` | Function pointer / closure |

### Built-in Functions

| Function | Signature | Runtime symbol |
|----------|-----------|----------------|
| `print` | `(String) -> Void` | `nimble_print` (with newline) |
| `print_int` | `(Int) -> Void` | `nimble_print_i64` |
| `print_str` | `(String) -> Void` | `nimble_print_str` (no newline) |

### Standard Library

The standard library lives under `std/` and can be imported with `load`:

- `load std` - root aggregator module
- `load std.io` - I/O (print, read_file, write_file, read_line)
- `load std.math` - math functions (sin, cos, sqrt, log, PI, E, ...)
- `load std.collections` - Vec[T], HashMap[K,V]
- `load std.core` - Result, Option, common utilities
- `load std.testing` - assert_eq, assert_true, assert_ok, run_test
- `load std.async` - Future, Channel, Mutex, async/await primitives
- `load std.sync` - AtomicInt, Arc
- `load std.thread` - Thread, spawn, join
- `load std.fmt` - formatting utilities
- `load std.alloc` - allocation helpers
- `load std.log` - logging helpers
- `load std.fs` - file system
- `load std.net` - networking
- `load std.json` - JSON parsing
- `load std.crypto` - random numbers
- `load std.os` - OS interaction

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
| `nimble doc` | Doc Generator | Generate HTML documentation from source |
| `nimble profile` | Profiler | Profile compilation and execution |
| `nimble fuzz` | Fuzzer | Fuzz-test the compiler for crashes |
| `nimble lint` | Linter | Lint a Nimble source file for common issues |
| `nimble generate-header` | Self-hosting | Generate the C runtime header for a Nimble compiler |
| `nimble install` | Package Manager | Install standalone binaries from remote URIs |
| `nimble pkg` | Package Manager | Manage library dependencies |
| `nimble fetch` | Dependencies | Fetch manifest dependencies |

## Pipeline

```
source.nbl  →  lexer  →  parser  →  typechecker  →  codegen  →  .ll  →  clang -c  →  .obj
                                                                                         │
                                                                                   linker (clang -o)
                                                                                         │
                                                                                     a.exe  ←  ember.lib
```

The codegen optionally emits LLVM debug info metadata (`DILocation`, `DISubprogram`, `DICompileUnit`) for source-level debugging.

## Current Status

Implemented:

- Lexer, parser, AST, type checker, code generator
- `if` / `elif` / `else`, `while`, `for`, `break`, `continue`, `return`, `load`, `extern fn`
- Immutable `let` and mutable `var`
- Struct declarations, struct literals, and field access
- Interface declarations with structural conformance checks
- **Enums (sum types)** with pattern matching
- **Generic functions** with monomorphization
- **Generic structs and interfaces**
- **Closures and lambdas** with capture analysis
- **Method call syntax** (`obj.method(args)`)
- **`if let` / `while let`** pattern matching
- **`?` operator** for error propagation
- **`defer`** statements for resource cleanup
- **Compile-time macros**
- **Async/await** with channel, mutex, atomic primitives
- **Reference types** (`&T`, `&mut T`)
- **Standard library** with 23 modules
- REPL, formatter, LSP (hover, goto-def, autocomplete), docgen, linter
- Project tooling (`init`, `build`, `run`, `pkg`, `install`, `fetch`)
- Profiling, fuzzing, and self-hosting header generation
- LLVM debug info emission

Still early:

- Full ownership / borrow checker (scaffolding present)
- Package registry protocol
- Incremental compilation (in-memory caching present)

## Build Options

```sh
cargo build --release
```

The release profile enables LTO, single codegen unit, panic=abort, and symbol stripping.
