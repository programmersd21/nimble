# Nimble

![logo](logo.jpeg)

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
- **Standard library** with 28 modules covering I/O, math, collections, encoding, hashing, strings, networking, filesystem, CLI, threading, synchronization, and more — all backed by a Rust runtime library
- **Rich runtime library** (`ember`) — 120+ C-ABI functions implementing string ops, math, random, hashing, base64/hex encoding, filesystem, networking, CLI, time, atomics, mutex, channels
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

The standard library lives under `std/` and provides **28 modules** — all backed by the Rust runtime:

`load std` — root aggregator (loads all modules)
- **`std.io`** — print, read/write/append file, file metadata, type-to-string conversion
- **`std.str`** — string manipulation: length, concat, find, trim, case, replace, repeat, split
- **`std.math`** — 30+ math functions: trig, log, pow, rounding, IEEE 754 checks, constants (PI, E, TAU, INF)
- **`std.collections`** — Vec[T], HashMap[K,V] generic data structures
- **`std.core`** — abs, min, max, clamp, popcount, bit ops, checked/saturating/wrapping arithmetic
- **`std.testing`** — assert_eq, assert_true, assert_ok, run_test
- **`std.async`** — Future, Channel, Mutex, async/await primitives
- **`std.sync`** — AtomicInt (load/store/add/sub/swap/CAS), Mutex, Arc
- **`std.thread`** — Thread, spawn, join, sleep, available_parallelism
- **`std.fmt`** — formatting utilities
- **`std.alloc`** — allocation helpers
- **`std.log`** — logging helpers (info, warn, error)
- **`std.fs`** — filesystem: read/write/append/exists/size/delete/rename, directory ops
- **`std.net`** — TCP connect/send/recv/close, DNS resolution
- **`std.json`** — JSON parsing and stringification
- **`std.crypto`** — random number generation
- **`std.random`** — random int/float/range with Xoshiro256** PRNG
- **`std.hash`** — FNV-1a, SipHash, xxHash3 hashing
- **`std.encoding`** — base64 (standard + URL-safe), hex (upper/lower), UTF-8 validation
- **`std.cli`** — command-line args, terminal detection, terminal size
- **`std.text`** — text pattern matching (contains, replace_all)
- **`std.os`** — environment variables (get/set), hostname, OS name, CPU count
- **`std.process`** — process management (exit, abort)
- **`std.time`** — epoch seconds, nanoseconds, monotonic clock, sleep, time formatting
- **`std.mem`** — memory helpers
- **`std.ffi`** — foreign function interface
- **`std.reflect`** — reflection utilities
- **`std.builtin`** — built-in type definitions

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
| 7 | **Linking** | `smelt` driver | Auto-discovers system linker and links with `ember` runtime library. |

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

## Runtime Library (ember)

The Nimble runtime library (`src/ember/mod.rs`) is a Rust static library (`ember.lib` / `libember.a`) that provides all stdlib backing functions via C ABI. It implements **120+ `extern "C"` functions** organized into these categories:

| Category | Functions |
|----------|-----------|
| **Memory** | alloc, free, realloc, string_new, string_free |
| **String** | length, concat, eq, substring, find, trim, to_upper, to_lower, starts_with, ends_with, replace, repeat, split, split_nth, to_int, to_float |
| **Print** | print, print_str, print_i64, print_f64, print_bool, flush, read_line |
| **I/O** | read_file, write_file, append_file, file_exists, file_size, delete_file |
| **Math** | sin, cos, tan, asin, acos, atan, atan2, sinh, cosh, tanh, sqrt, cbrt, pow, floor, ceil, round, trunc, fabs, log, log2, log10, exp, fma, hypot, fmod, clamp_f64, is_nan, is_infinite, is_finite, next_after |
| **Integer** | abs_i64, min_i64, max_i64, clamp_i64, popcount, clz, ctz, byte_swap, rotl, rotr, checked_add/sub/mul/div, saturating_add/sub, wrapping_add/sub/mul |
| **Random** | Xoshiro256** PRNG: rand_i64, rand_f64, rand_range, srand |
| **Hashing** | FNV-1a, SipHash (1-3), xxHash3 |
| **Encoding** | base64_encode (standard + URL-safe), base64_decode, hex_encode (upper + lower), hex_decode, utf8_validate |
| **Filesystem** | create_dir, remove_dir, list_dir, current_dir, rename_file, delete_file |
| **Networking** | TCP connect/send/recv/close via handle map, DNS resolve |
| **CLI** | args_count, args_get, is_terminal, terminal_width, terminal_height, getenv, setenv, hostname, os_name, cpu_count |
| **Time** | clock_monotonic, time_nanos, time_seconds, time_format, sleep_ms |
| **Sync** | atomic_load/store/add/sub/swap, mutex_create/lock/unlock |
| **JSON** | parse, stringify, get_field |
| **Conversion** | int_to_string, float_to_string, bool_to_string |

The runtime library is automatically built by the smelt driver (using `rustc`) and linked into every compiled Nimble program via `clang` or the system linker.

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
- Standard library with 28 modules, all runtime-backed

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

**Runtime Library (ember):**
- 120+ C-ABI functions covering memory, strings, print, I/O, filesystem, math, integer operations, random, hashing, encoding, networking, CLI, time, synchronization, JSON
- Xoshiro256** PRNG, FNV-1a / SipHash / xxHash3 hashing
- Base64 (standard + URL-safe), hex (upper + lower), UTF-8 validation
- TCP networking with handle map, DNS resolution
- Atomics (load/store/add/sub/swap/CAS), mutex, Arc

### Still Early

- Full ownership / borrow checker (scaffolding present)
- Package registry protocol (registry-less by design)

## Test Suite

The compiler has **261 unit tests** across all subsystems:

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
| Ember runtime | 12 | Memory, strings, math, random, hashing, encoding, file ops, atomics |
| Other | 66 | Env, types, query system, anvil, chisel, selfhost, etc. |

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

## Examples

21 sample programs are provided in `examples/`, covering every stdlib module:

| Example | Demonstrates |
|---------|-------------|
| `hello_world.nbl` | Basic program structure |
| `fibonacci.nbl` | Recursive functions |
| `fizzbuzz.nbl` | Loops and conditionals |
| `types.nbl` | Primitive types (Int, Float, Bool, String) |
| `structs.nbl` | Struct definition, field access, area/perimeter |
| `variables.nbl` | Immutable `let`, mutable `var`, compound assignment |
| `booleans.nbl` | Boolean logic (and, or, not) |
| `control_flow.nbl` | if/elif/else, while, break, continue |
| `functions.nbl` | Functions, recursion, composition |
| `generics.nbl` | Generic struct annotations |
| `interfaces.nbl` | Structural typing via interfaces |
| `operators.nbl` | Arithmetic, comparison, logical operators |
| `ffi_example.nbl` | C FFI via `extern fn` |
| `stdlib.nbl` | Standard library composition demo |
| `strings.nbl` | String operations (length, concat, find, trim, case, replace) |
| `math.nbl` | Math library (trig, sqrt, pow, rounding, abs, min/max) |
| `encoding_demo.nbl` | Base64 and hex encoding/decoding |
| `hash_demo.nbl` | FNV-1a, SipHash, xxHash3 hashing |
| `random_demo.nbl` | Random number generation |
| `time_demo.nbl` | Time functions (timestamp, sleep) |
| `test_print.nbl` | Minimal I/O verification |

Run all examples with:

```sh
python run_examples.py
```

## Project Structure

```
nimble/
├── benches/               Criterion benchmarks
├── docs/
│   ├── manual/            Language specification (types, expressions, etc.)
│   └── sdocs/             Standard library documentation (per-module)
├── examples/              21 sample .nbl programs
├── src/
│   ├── nim/               Package manager (cache, git, resolve, manifest, commands)
│   ├── anvil/             Build system
│   ├── chisel/            Code formatter
│   ├── diagnostics/       Structured error codes, pretty printing, LSP integration
│   ├── ember/             Runtime library (C static lib, 120+ functions)
│   ├── forge/             REPL (JIT and simple modes)
│   ├── lantern/           LSP server
│   ├── smelt/             Compiler driver (linking, runtime build)
│   ├── lexer.rs, parser.rs, ast.rs, hir.rs, resolver.rs, typechecker.rs, codegen.rs
│   ├── query.rs           Compiler database (query-based, memoized, cached)
│   ├── lib.rs             Crate root
│   └── main.rs            CLI entry point
└── std/                   Standard library (.nbl source, 28 modules)
    ├── io/, math/, str/, collections/, core/, testing/, async/, sync/
    ├── thread/, fmt/, alloc/, log/, fs/, net/, json/, crypto/
    ├── random/, hash/, encoding/, cli/, text/, os/, process/
    ├── time/, mem/, ffi/, reflect/, builtin/
    └── mod.nbl            Root aggregator module
```

## Platform Support

| Platform | Status |
|----------|--------|
| Linux (x86_64, aarch64) | ✅ Primary target |
| macOS (x86_64, arm64) | ✅ Tested |
| Windows (x86_64) | ✅ Tested (MSVC/MinGW) |
