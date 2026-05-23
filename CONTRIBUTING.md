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
.github/workflows/   CI configuration
anvil/               Build system (`anvil` binary)
benches/             Criterion benchmarks
chisel/              Code formatter (`chisel` binary)
docs/                Language specification
ember/               Runtime library (`ember` staticlib)
examples/            Sample .nbl programs
forge/               REPL (`forge` binary, requires `--features jit`)
lantern/             LSP server (`lantern` binary)
smelt/               Compiler driver (`smelt` binary)
src/                 Core compiler library
```
