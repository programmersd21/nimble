# Nimble

A statically typed language with Python-style indentation, LLVM-based code generation, and an integrated toolchain.

## Features at a Glance

- **Pythonic syntax** — indentation-based blocks, no curly braces or semicolons
- **Static type system** with Hindley-Milner type inference, generics, enums, and interfaces
- **Enums (sum types)** with tagged union representation and pattern matching
- **Generic functions, structs, and interfaces** with monomorphization
- **Closures and lambdas** with capture analysis and trampoline codegen
- **Pattern matching** with wildcards, bindings, variant patterns, and literals
- **Method call syntax** (`obj.method(args)`) desugared to `method(obj, args)`
- **`?` operator** for ergonomic error propagation with Result types
- **`defer` statements** for scope-exit cleanup
- **Compile-time macros** with AST substitution
- **Async/concurrency primitives** (future, channel, mutex, thread, atomic)
- **Option[T] / Result[T, E]** algebraic types in stdlib
- **LLVM codegen** with optional debug info emission
- **Cross-platform linker** — auto-discovers `cc`, `clang`, `gcc`, or `link.exe`
- **Standard library** with 23 modules covering I/O, math, collections, testing, async, sync, crypto, net, FFI, and more
- **C FFI** — `extern fn` for binding native libraries
- **Rich diagnostics** — ~300 error codes with structured spans, suggestions, and pretty printing via miette
- **Integrated LSP server** with hover info, goto-definition, and autocompletion
- **Doc generator** — `nimble doc` produces HTML API docs from source
- **Fuzzer** — `nimble fuzz` stress-tests the compiler with random programs
- **Formatter** — `nimble fmt` canonical source formatting
- **Profiler** — `nimble profile` measures compilation and execution timing
- **Self-hosting support** — `nimble generate-header` produces C runtime API headers
- **Query-based compilation & caching** — central compiler database with dynamic dependency tracking and persistent disk cache
- **Registry-less package manager** — Git-native dependency resolution with semver, lockfiles, topological ordering, and parallel fetch
- **Unified toolchain** — everything integrated into a single `nimble` binary

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

- `load std` — root aggregator module
- `load std.io` — I/O (print, read_file, write_file, read_line)
- `load std.math` — math functions (sin, cos, sqrt, log, PI, E, ...)
- `load std.collections` — Vec[T], HashMap[K,V]
- `load std.core` — Result, Option, common utilities
- `load std.testing` — assert_eq, assert_true, assert_ok, run_test
- `load std.async` — Future, Channel, Mutex, async/await primitives
- `load std.sync` — AtomicInt, Arc
- `load std.thread` — Thread, spawn, join
- `load std.fmt` — formatting utilities
- `load std.alloc` — allocation helpers
- `load std.log` — logging helpers
- `load std.fs` — file system
- `load std.net` — networking
- `load std.json` — JSON parsing
- `load std.crypto` — random numbers
- `load std.os` — OS interaction
- `load std.process` — process management
- `load std.time` — time utilities
- `load std.mem` — memory helpers
- `load std.ffi` — foreign function interface
- `load std.reflect` — reflection utilities
- `load std.builtin` — built-in type definitions

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
| `nimble repl` | REPL | Start an interactive Nimble REPL (JIT with `--features jit`) |
| `nimble lsp` | LSP Server | Start the Language Server for IDE support |
| `nimble doc` | Doc Generator | Generate HTML documentation from source |
| `nimble profile` | Profiler | Profile compilation and execution |
| `nimble fuzz` | Fuzzer | Fuzz-test the compiler for crashes |
| `nimble lint` | Linter | Lint a Nimble source file for common issues |
| `nimble explain` | Error Help | Show long-form explanation for a compiler error code |
| `nimble generate-header` | Self-hosting | Generate the C runtime header for a Nimble compiler |
| `nimble install` | Package Manager | Install standalone binaries from remote Git URIs |
| `nimble uninstall` | Package Manager | Remove an installed binary |
| `nimble upgrade` | Package Manager | Re-install a binary at the latest tag |
| `nimble pkg` | Package Manager | Cache/remove library packages globally |
| `nimble fetch` | Dependencies | Clone and lock all manifest dependencies |

## Pipeline & Query System

The Nimble compiler adopts a **query-based, demand-driven compiler architecture**:

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

- **Dynamic Dependency Tracking**: Queries register dependencies on other queries automatically.
- **Stable Hashing**: Content-hash fingerprints skip re-compilation for unchanged files.
- **Persistent Cache**: Serialized to `target/.nimble_cache` for near-instant rebuilds.

### Compiler Phases

| # | Phase | Module | Description |
|:-:|-------|--------|-------------|
| 1 | **Lexer** | `src/lexer.rs` | Tokenizes source with full UTF-8 Unicode support (XID_Start/XID_Continue). Structured `LexError` recovery. Fuzz-tested. |
| 2 | **Parser** | `src/parser.rs` | Pratt-style precedence climbing with panic-mode error recovery. |
| 3 | **HIR Lowering** | `src/hir.rs` | Desugars AST into HIR, preserving `Span` info for diagnostics. |
| 4 | **Name Resolution** | `src/resolver.rs` | Two-pass resolver: collects definitions then resolves references to `DefId`s. |
| 5 | **Type Checking** | `src/typechecker.rs` | Hindley-Milner inference with unification, generics, closures, method desugaring, interface conformance. |
| 6 | **Code Generation** | `src/codegen.rs` | Emits textual LLVM IR with debug info, defer stacks, lambda trampolines, enum tagged unions. |
| 7 | **Linking** | `smelt` driver | Auto-discovers system linker and links with `ember` runtime. |

### Diagnostics System

The diagnostics subsystem (`src/diagnostics/`) provides **~300 stable error codes** (N0001–N9025) with:

- Structured diagnostics: primary/secondary spans, multi-line spans, labels, notes, help messages
- Typo suggestions via Damerau-Levenshtein distance with ranked candidates
- Cascading error suppression via `RecoveryState`
- Pretty printing with Unicode box rendering, color theme, ANSI/ASCII fallback
- JSON emitter and LSP diagnostic conversion
- Error explanations via `nimble explain <code>`

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

### Package Manager

The package manager (`src/nim/`) is a **registry-less, Git-native** dependency resolver:

- **Unified manifest** (`nimble.toml`) with `[project]`, `[dependencies]`, `[dev-dependencies]`, `[build-dependencies]`, `[features]`, `[profile]`
- **Git sources** with `tag`, `branch`, `rev` specifiers and path dependencies
- **Semver resolution** — matches version tags against constraints (caret `^`, tilde `~`, wildcard `*`)
- **Lockfile** (`nimble.lock`) with commit hashes and SHA-256 checksums
- **Cycle detection** via DFS visiting set
- **Topological ordering** (Kahn's algorithm) for correct build order
- **Dependency kind separation** — Normal, Dev, Build with independent resolution
- **Feature propagation** — dependency features tracked through the graph
- **Parallel fetch** — concurrent cloning/fetching via threads
- **Cache** at `~/.nimble/{bin, cache/repos, cache/pkgs}`
- **Compiler integration** — `install_binary` compiles via `smelt::driver::compile`

## Current Status

### Implemented

**Core Compiler:**
- Lexer, parser, AST, HIR lowering, name resolver, type checker, code generator
- `if` / `elif` / `else`, `while`, `for`, `break`, `continue`, `return`, `load`, `extern fn`
- Immutable `let` and mutable `var`
- Struct declarations, struct literals, and field access
- Interface declarations with structural conformance checks
- Enums (sum types) with pattern matching, `if let` / `while let`
- Generic functions, structs, and interfaces with monomorphization
- Closures and lambdas with capture analysis and trampoline codegen
- Method call syntax (`obj.method(args)`)
- `?` operator for error propagation
- `defer` statements for resource cleanup
- Compile-time macros with AST substitution
- Async/await with channel, mutex, atomic primitives
- Reference types (`&T`, `&mut T`)
- Standard library with 23 modules

**Diagnostics System:**
- ~300 stable error codes (N0001–N9025)
- Structured diagnostics with primary/secondary spans, labels, notes, suggestions
- Pretty printer with Unicode box rendering, color themes, ASCII fallback
- JSON emitter and LSP diagnostic conversion
- Typo suggestions using Damerau-Levenshtein distance
- Cascading error suppression via `RecoveryState`
- Diagnostic deduplication via `DiagnosticCache`
- Error explanations via `nimble explain <code>`

**Tooling:**
- REPL (JIT with `--features jit`), formatter, LSP (hover, goto-def, autocomplete), docgen, linter
- Project tooling (`init`, `build`, `run`, `test`)
- Package management (`install`, `uninstall`, `pkg`, `fetch`)
- Profiling, fuzzing, and self-hosting header generation
- LLVM debug info emission

**Package Manager (src/nim/):**
- Unified manifest parsing and generation
- Git repository cloning, fetching, checkout, tag listing, ref resolution
- Semver-aware dependency resolution with lockfile generation
- Transitive dependency collection with cycle detection and topological sort
- Parallel dependency fetching
- Binary installation and library caching
- Checked/statured/wrapping arithmetic, bit manipulation

### Still Early

- Full ownership / borrow checker (scaffolding present)
- Package registry protocol (registry-less by design)

## Test Suite

The compiler has **257 unit tests** across all subsystems:

| Subsystem | Test count | Coverage |
|-----------|-----------|----------|
| Lexer | 47 | Unicode identifiers, error recovery, fuzz safety |
| Parser | 41 | Error recovery, full AST coverage |
| HIR | 5 | Lowering correctness |
| Resolver | 12 | Name resolution, scoping, error detection |
| Typechecker | 32 | Type inference, errors, generics, interfaces |
| Codegen | 17 | Full compilation examples (fibonacci, fizzbuzz, booleans, etc.) |
| Diagnostics | 5 | Pretty printing, span rendering, JSON output |
| Lint | 3 | Dead code detection, unused variable detection |
| Nim module | 21 | Manifest, cache, git, resolve, topological sort, features/kind |
| Other | 74 | Env, types, ember, query system, anvil, chisel, selfhost, etc. |

## Build Options

```sh
cargo build --release
```

The release profile enables LTO, single codegen unit, panic=abort, and symbol stripping.

```sh
# Run all tests
cargo test

# Run benchmarks
cargo bench --bench compiler_perf

# Check code style
cargo clippy
cargo fmt --all --check
```

## Project Structure

```
nimble/
├── benches/               Criterion benchmarks
├── docs/
│   ├── manual/            Language specification (types, expressions, etc.)
│   └── sdocs/             Standard library documentation (per-module)
├── examples/              Sample .nbl programs
├── src/
│   ├── nim/               Package manager (cache, git, resolve, manifest, commands)
│   ├── anvil/             Build system
│   ├── chisel/            Code formatter
│   ├── diagnostics/       Structured error codes, pretty printing, LSP integration
│   ├── ember/             Runtime library (C static lib)
│   ├── forge/             REPL (JIT and simple modes)
│   ├── lantern/           LSP server
│   ├── smelt/             Compiler driver (linking, runtime build)
│   ├── lexer.rs, parser.rs, ast.rs, hir.rs, resolver.rs, typechecker.rs, codegen.rs
│   ├── query.rs           Compiler database (query-based, memoized, cached)
│   ├── lib.rs             Crate root
│   └── main.rs            CLI entry point
└── std/                   Standard library (.nbl source)
    ├── io/, math/, collections/, core/, testing/, async/, sync/
    ├── thread/, fmt/, alloc/, log/, fs/, net/, json/, crypto/
    ├── os/, process/, time/, mem/, ffi/, reflect/, builtin/
    └── mod.nbl            Root aggregator module
```

## Platform Support

| Platform | Status |
|----------|--------|
| Linux (x86_64, aarch64) | ✅ Primary target |
| macOS (x86_64, arm64) | ✅ Tested |
| Windows (x86_64) | ✅ Tested (MSVC/MinGW) |
