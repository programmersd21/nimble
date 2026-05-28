# Nimble Language Overview

Nimble is a statically typed, compiled language with Python-style indentation and an integrated compiler/toolchain.

## Design Goals

- **Readable syntax** via significant indentation and minimal punctuation
- **Predictable compilation** via a direct lexer -> parser -> typechecker -> codegen pipeline
- **C ABI interoperability** through `extern fn`
- **Pragmatic tooling** with CLI, formatter, REPL, LSP, and package management entrypoints
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
| `nimble install` | Package Manager | Install standalone binaries from remote URIs |
| `nimble pkg` | Package Manager | Manage library dependencies |

## Compilation Pipeline

```
Source (`.nbl`) -> Lexer -> Parser -> AST -> TypeChecker -> Codegen -> LLVM IR (`.ll`) -> `clang -c` -> Object (`.o` / `.obj`)
```

The compiler driver (`smelt`) is now exposed through the library crate and is used by `nimble compile` and `nimble build`. It auto-discovers the host linker (`cc`, `clang`, `gcc`, or `link.exe`) and builds the `ember` runtime before linking the final binary.

## Key Properties

- **Immutable by default**: `let` bindings cannot be reassigned.
- **Mutable bindings**: `var` bindings can be reassigned.
- **Basic inference**: local values are inferred from usage and literals, while function signatures remain explicit.
- **Zero-cost FFI**: `extern fn` declarations produce LLVM declarations with C calling convention.
- **Standard library loading**: `load std.<module>` imports std modules, and `load std` loads the root standard library aggregator.
- **Textual IR**: codegen emits human-readable `.ll` files. No LLVM dependency is needed at compile time for normal compilation.

## Current Limitations

The implementation is still early and does not yet include:

- Ownership or borrow checking
- Generic function monomorphization
- Method dispatch syntax
- A stable module/package registry protocol
- An async runtime or concurrency primitives

See [`roadmap.md`](roadmap.md) for a concrete extension plan.
