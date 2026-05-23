# Nimble Language Overview

Nimble is a statically-typed, compiled systems programming language. It combines Python-style indentation-based syntax with LLVM code generation and Hindley-Milner type inference.

## Design Goals

- **C-like performance** via LLVM backend with textual IR codegen
- **Python-like readability** via significant indentation and minimal punctuation
- **Hindley-Milner type inference** with optional explicit type annotations
- **C ABI compatibility** through `extern fn` foreign function declarations
- **Incremental compilation** via pipeline optimization configuration

## Toolchain

| Component | Crate | Role |
|-----------|-------|------|
| `nimble` | root | Core library: lexer, parser, typechecker, codegen, pipeline |
| `smelt` | smelt | Compiler driver: source to object file via LLVM |
| `anvil` | anvil | Build system: project init, manifest, build, run |
| `lantern` | lantern | LSP server (tower-lsp + tokio) |
| `chisel` | chisel | AST formatter / pretty-printer |
| `forge` | forge | REPL with optional JIT (inkwell) |
| `ember` | ember | Runtime library (staticlib for linked binaries) |

## Compilation Pipeline

```
Source (.nbl) -> Lexer -> Parser -> AST -> TypeChecker -> Codegen -> LLVM IR (.ll) -> clang -c -> Object (.o/.obj)
```

The driver (`smelt`) auto-discovers the host linker (`cc`, `clang`, `gcc`, or `link.exe`) and builds the `ember` runtime before linking the final binary.

## Key Properties

- **Safe by default**: variables are immutable unless declared `var`. Type checker prevents reassignment to `let` bindings.
- **No GC**: memory management TBD (runtime `ember` provides alloc/free primitives).
- **Zero-cost FFI**: `extern fn` declarations produce LLVM `declare` with C calling convention.
- **Standard library**: `load std.<module>` imports std modules, and `load std` loads the root standard library aggregator.
- **Textual IR**: codegen emits human-readable `.ll` files. No LLVM dependency at compile time (except JIT REPL).
