# Contributing

## Prerequisites

- Rust 1.85+ (edition 2024)
- LLVM / clang 18+ (`clang` must be on `$PATH`)

  - **Ubuntu/Debian**: `sudo apt install llvm-18`
  - **macOS**: `brew install llvm`
  - **Windows**: download from https://llvm.org/builds/

## Setup

```sh
git clone https://github.com/programmersd21/nimble
cd nimble
cargo build
```

## Testing

```sh
# Run all tests across the workspace (excludes forge which needs LLVM JIT)
cargo test --workspace --exclude forge

# Single crate
cargo test -p nimble

# Benchmark suite
cargo bench --bench compiler_perf
```

## Code Style

- Rust edition 2024, formatted with `cargo fmt`
- Zero clippy warnings on `cargo clippy`
- Comments only for non-obvious reasoning, never for trivial code
- Unsafe operations are wrapped in explicit `unsafe {}` blocks
- All FFI exports use `#[unsafe(no_mangle)]` and `extern "C"`

## Pull Request Process

1. Ensure tests pass: `cargo test --workspace --exclude forge`
2. Ensure no clippy warnings: `cargo clippy --workspace --exclude forge`
3. Ensure formatting: `cargo fmt --all --check`
4. Open a PR against `main` with a clear summary of changes

## Project Structure

```
.github/workflows/     CI configuration
benches/               Criterion benchmarks
docs/
  ├── manual/          Language specification (types, expressions, statements, etc.)
  └── sdocs/           Standard library documentation (per-module)
examples/              Sample .nbl programs
src/
  ├── anvil/           Build system (`init`, `build`, `run`)
  ├── chisel/          Code formatter (`fmt`)
  ├── diagnostics/     Structured error codes, pretty printing, LSP conversion
  ├── ember/           Runtime library C source
  ├── forge/           REPL (requires `--features jit` for JIT execution)
  ├── lantern/         LSP server
  ├── nim/             Package manager (cache, git, resolve, manifest)
  ├── smelt/           Compiler driver (linking, runtime build)
  ├── lib.rs           Crate root
  └── main.rs          CLI entry point (all tools in one binary)
std/                   Standard library source files (.nbl)
```
