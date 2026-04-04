# Getting Started with Nimble

Nimble is a register‑based bytecode language with optional type annotations and a small but pragmatic standard library. JIT scaffolding exists but is not yet wired into execution.

## Installation

Build from source using Cargo:

```bash
cargo build --release
```

## Your First Program

Create `hello.nmb`:

```nimble
fn main():
    out("Hello, Nimble!")

main()
```

Run it:

```bash
nimble run hello.nmb
```

## Tooling Workflow

- `nimble run <file>` – compiles and runs a script from the current directory, automatically checking types before executing.
- `nimble check <file>` – performs parsing and inference only, but still prints the same colorful diagnostics the runtime emits.
- `nimble repl` – drops into the REPL with `:globals`, diagnostics, and the runtime hook enabled so you can experiment safely.

Browse `examples/README.md` for the curated basic and standard-library samples referenced from the docs tree.

## Variables and Types

```nimble
x = 10
name = "Soumalya"
pi float = 3.14
active bool = true
```

## Functions

```nimble
fn add(a int, b int) -> int:
    return a + b

fn square(x int) -> int = x * x
```

## User Input

```nimble
name = in("Enter your name: ")
out("Hello {name}!")
```

## Range Loops

```nimble
for i in 0..10:
    out(i)

for i in 0..10 step 2:
    out(i)
```

## Modules

```nimble
load math
out(math.add(2, 3))
```
