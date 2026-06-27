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
- **Query-based compilation & caching** - central compiler database with dynamic dependency tracking and persistent disk cache
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

## Pipeline & Query System

The Nimble compiler adopts a **query-based, demand-driven compiler architecture** with an expanded multi-phase pipeline:
```
                      ┌─────────────────────────┐
                      │    Compiler Database    │
                      └────────────┼────────────┘
                                   │ (queries)
                                   ▼
[Source] ──► [Lex] ──► [Parse] ──► [HIR Lower] ──► [Resolve] ──► [Typecheck] ──► [Codegen] ──► [Link]
                                       │             │
                                       ▼             ▼
                                  [HirProgram]  [ResolvedProgram]
                                       │             │
                                       └─── [DefId map, scope chain]
                                   ▲
                                   │ (fingerprints, memoization, deps)
                      ┌────────────┴────────────┐
                      │    Persistent Cache     │
                      └─────────────────────────┘
```

Every compilation unit (file read, lexing, parsing, HIR lowering, name resolution, type-checking, and code generation) is represented as a memoized query managed by the compiler `Database` ([query.rs](src/query.rs)).
- **Dynamic Dependency Tracking**: Queries automatically register dependencies on other queries they invoke (e.g. `typecheck` of a file depends on `resolve`, `parse`, and `typecheck` of its imported modules).
- **Stable Hashing**: Computes stable content-hash fingerprints. If a source file or its dependency graph is unchanged, Nimble avoids re-lexing, re-parsing, re-typechecking, and re-codegen.
- **Persistent Cache**: Serializes query nodes to `target/.nimble_cache` to survive compiler restarts for near-instant rebuilds.

### Compiler Phases

| # | Phase | Module | Description |
|:-:|-------|--------|-------------|
| 1 | **Lexer** | `src/lexer.rs` | Tokenizes source with full UTF-8 Unicode support (XID_Start/XID_Continue identifiers). Structured `LexError` recovery via `drain_errors()`. Dedicated fuzz testing. |
| 2 | **Parser** | `src/parser.rs` | Pratt-style precedence climbing. Panic-mode error recovery with sync tokens (`Newline`, `Dedent`, `Eof`). `drain_parse_errors()` collects all errors without aborting. |
| 3 | **HIR Lowering** | `src/hir.rs` | Desugars AST into HIR by stripping transparent wrappers (e.g. `Grouping`). Preserves all `Span` info for diagnostics. |
| 4 | **Name Resolution** | `src/resolver.rs` | Two-pass resolver: Pass 1 collects definitions with lexical scoping; Pass 2 resolves identifier references to `DefId`s. Detects undefined variables and duplicate definitions. |
| 5 | **Type Checking** | `src/typechecker.rs` | Hindley-Milner inference with unification, generics, closures, method desugaring, interface conformance, ownership scaffold. |
| 6 | **Code Generation** | `src/codegen.rs` | Emits textual LLVM IR with debug info, defer stacks, lambda trampolines, enum tagged unions. |
| 7 | **Linking** | `smelt` driver | Auto-discovers system linker (`cc`, `clang`, `gcc`, `link.exe`) and links with `ember` runtime. |

### Diagnostics System

The diagnostics subsystem (`src/diagnostics/`) provides **Rust-quality error reporting**:

- **~300 stable error codes** (N0001–N9025) across all subsystems — lexer, parser, resolver, typechecker, module system, lint, codegen, runtime, config, and ICE
- **Structured diagnostics**: primary/secondary spans, multi-line spans, labels, notes, help messages
- **Typo suggestions**: Levenshtein/Damerau distance for misspelled identifiers with ranked candidates
- **Suggestions with applicability**: `MachineApplicable`, `MaybeIncorrect`, `HasPlaceholders`, `Unspecified`
- **Cascading error suppression**: `RecoveryState` tracks known-broken variables to avoid avalanche errors
- **Pretty printing**: Unicode box rendering with color theme, ANSI/ASCII fallback, source context with carets
- **Machine-readable output**: JSON emitter and LSP diagnostic conversion for IDE integration
- **Diagnostic deduplication**: `DiagnosticCache` prevents identical error reports
- **Error explanations**: `nimble explain N2001` shows long-form documentation with examples and root-cause analysis

```
error[N2001]: Undefined variable `my_val`
  --> src/main.nbl:3:15
   |
 3 |     println(my_val)
   |             ^^^^^^ `my_val` is not defined in this scope
   |
help: did you mean `my_var`?
  |
 3 |     println(my_var)
   |             ~~~~~~
   |
note: variables must be declared with `let` or `var` before use
```

The codegen optionally emits LLVM debug info metadata (`DILocation`, `DISubprogram`, `DICompileUnit`) for source-level debugging.

## Current Status

### Implemented

**Core Compiler:**
- Lexer, parser, AST, HIR lowering, name resolver, type checker, code generator
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
- **`defer` statements** for resource cleanup
- **Compile-time macros**
- **Async/await** with channel, mutex, atomic primitives
- **Reference types** (`&T`, `&mut T`)
- Standard library with 23 modules

**HIR Lowering** ([src/hir.rs](src/hir.rs)):
- Desugars AST into `HirProgram` with `HirStmt` and `HirExpr`
- Strips semantically transparent wrappers (Grouping) while preserving all `Span` info
- 5 unit tests covering lowering correctness

**Name Resolution** ([src/resolver.rs](src/resolver.rs)):
- Two-pass resolver: Pass 1 collects all definitions with lexical scoping; Pass 2 resolves identifier references to unique `DefId`s
- Block-level scoping for `if`/`while`/`for`/`defer`/function bodies
- `ResolvedProgram` with `lookup()`, `lookup_by_span()`, `get_def()` APIs
- Detects `UndefinedVariable` and `DuplicateDefinition` errors
- 12 unit tests

**Diagnostics System** ([src/diagnostics/](src/diagnostics/)):
- **~300 stable error codes** (N0001–N9025) covering every possible compiler error
- Structured diagnostics with primary/secondary spans, labels, notes, suggestions
- Pretty printer with Unicode box rendering, color themes, ASCII fallback
- JSON emitter and LSP diagnostic conversion for IDE integration
- Typo suggestions using Damerau-Levenshtein distance with ranked candidates
- Cascading error suppression via `RecoveryState`
- Diagnostic deduplication via `DiagnosticCache`
- Error explanations via `nimble explain <code>`
- Macro expansion awareness in span reporting

**Parser Error Recovery:**
- Panic-mode recovery with sync tokens (`Newline`, `Dedent`, `Eof`)
- Collects multiple parse errors in a single pass via `drain_parse_errors()`
- 6 dedicated recovery tests

**Lexer Improvements:**
- Full UTF-8 Unicode identifiers (XID_Start/XID_Continue) — CJK, Greek, Cyrillic, mixed scripts
- Structured `LexError` enum with miette diagnostics
- Dedicated fuzz testing for random byte sequences

**Tooling:**
- REPL, formatter, LSP (hover, goto-def, autocomplete), docgen, linter
- Project tooling (`init`, `build`, `run`, `pkg`, `install`, `fetch`)
- Profiling, fuzzing, and self-hosting header generation
- LLVM debug info emission

### Still Early

- Full ownership / borrow checker (scaffolding present)
- Package registry protocol

## Test Suite

The compiler has **223+ unit tests** across all subsystems:
- 47 lexer tests — including Unicode identifiers, error recovery, fuzz safety
- 41 parser tests — including error recovery, full AST coverage
- 6 HIR tests — lowering correctness
- 12 resolver tests — name resolution, scoping, error detection
- 32 typechecker tests — type inference, errors, generics, interfaces
- 5 codegen tests — full compilation examples (fibonacci, fizzbuzz, etc.)
- 5 lint tests — dead code detection, unused variable detection
- 5 diagnostics tests — pretty printing, span rendering, JSON output

## Build Options

```sh
cargo build --release
```

The release profile enables LTO, single codegen unit, panic=abort, and symbol stripping.
