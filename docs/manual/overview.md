# Nimble Language Overview

Nimble is a statically typed, compiled language with Python-style indentation and an integrated compiler/toolchain.

## Design Goals

- **Readable syntax** via significant indentation and minimal punctuation
- **Predictable compilation** via a direct lexer -> parser -> typechecker -> codegen pipeline
- **C ABI interoperability** through `extern fn`
- **Algebraic types** with enums, pattern matching, and Result/Option for robust error handling
- **Pragmatic tooling** with CLI, formatter, REPL, LSP, docgen, linter, profiler, and package management
- **Room for growth** toward stronger types, better diagnostics, and a richer standard library

## Toolchain

The entire Nimble toolchain is integrated into a single `nimble` binary.

| Command | Role | Description |
|---------|------|-------------|
| `nimble init` | Project Init | Create a new Nimble project with a default layout |
| `nimble build` | Build System | Build the current project using the manifest |
| `nimble run` | Project Runner | Run the current project |
| `nimble compile` | Compiler | Compile a single `.nbl` file to an executable |
| `nimble fmt` | Formatter | Format Nimble source code canonically |
| `nimble repl` | REPL | Start an interactive Nimble REPL |
| `nimble lsp` | LSP Server | Start the Language Server for IDE support |
| `nimble doc` | Doc Generator | Generate HTML API documentation |
| `nimble profile` | Profiler | Profile compilation and execution |
| `nimble fuzz` | Fuzzer | Fuzz-test the compiler for crashes |
| `nimble lint` | Linter | Lint Nimble source files for common issues |
| `nimble generate-header` | Self-hosting | Generate C runtime header |
| `nimble install` | Package Manager | Install standalone binaries from remote URIs |
| `nimble pkg` | Package Manager | Manage library dependencies |
| `nimble fetch` | Dependencies | Fetch manifest dependencies |

## Compilation Pipeline

```
Source (`.nbl`) -> Lexer -> Parser -> AST -> TypeChecker -> Codegen -> LLVM IR (`.ll`) -> `clang -c` -> Object (`.o` / `.obj`)
```

The compiler driver (`smelt`) is now exposed through the library crate and is used by `nimble compile` and `nimble build`. It auto-discovers the host linker (`cc`, `clang`, `gcc`, or `link.exe`) and builds the `ember` runtime before linking the final binary.

The codegen supports optional LLVM debug info emission (`DILocation`, `DISubprogram`, `DICompileUnit`) for source-level debugging.

## Key Properties

- **Immutable by default**: `let` bindings cannot be reassigned.
- **Mutable bindings**: `var` bindings can be reassigned.
- **Hindley-Milner inference**: local values are inferred from usage and literals, while function signatures remain explicit.
- **Enums and pattern matching**: sum types with exhaustive match checking and destructuring.
- **Generic functions**: `fn identity[T](x: T) -> T` with monomorphization.
- **Closures and lambdas**: anonymous functions with capture analysis; non-capturing closures compile to function pointers, capturing closures use a trampoline.
- **Zero-cost FFI**: `extern fn` declarations produce LLVM declarations with C calling convention.
- **Error propagation**: `?` operator on Result types unwraps or early-returns the error.
- **Resource cleanup**: `defer` statements run on scope exit.
- **Compile-time macros**: `macro name(params): body` with AST substitution.
- **Standard library loading**: `load std.<module>` imports std modules via qualified names (e.g., `std.io.print_int()`), and `load std` loads the root standard library aggregator. All 21 examples demonstrate this pattern.
- **Textual IR**: codegen emits human-readable `.ll` files. No LLVM dependency is needed at compile time for normal compilation.
- **Debug info**: optional `--emit-llvm` mode includes LLVM debug metadata.

## Current Limitations

The implementation is still maturing:

- Ownership and borrow checking is scaffolding-only (reference types parsed and tracked but no borrow checker passes yet)
- Incremental compilation is in-memory only (no disk cache)
- Package registry protocol documented but not yet deployed

See the full language manual for detailed documentation.
