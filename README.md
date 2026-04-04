# ⚡ Nimble

**"Reads like Python. Runs like a VM. Feels like freedom."**

Nimble is a register‑based bytecode language focused on fast startup, small binaries, and ergonomic scripting with optional static type annotations.

## Key Features

- **Register‑Based VM:** Efficient bytecode execution with predictable performance.
- **Simple, Expressive Syntax:** Python‑style indentation with modern features.
- **Optional Types:** Annotations are supported and parsed, with lightweight inference.
- **String Interpolation:** Format values inline with `{expr}`.
- **Modules & Stdlib:** Built‑in stdlib plus local module loading (IO, strings, regex, JSON, math, and more).
- **Concurrency (Spawn):** Lightweight background execution.
- **Diagnostics:** Colorful error reports with line and syntax highlighting.
- **JIT (Experimental):** Cranelift scaffolding is present but not wired into execution yet.

## Quick Start

### Build

```bash
cargo build --release
```

### Your First Program

Create `main.nmb`:

```nimble
fn main():
    name = in("What is your name? ")
    out("Hello, {name}!")

main()
```

Run it:

```bash
./target/release/nimble run main.nmb
```

## CLI and Workflow

- `nimble run <file>` – compile, type-check, and execute a script, inheriting the current working directory for module resolution.
- `nimble check <file>` – stops after parsing + inference, emitting the same fun, colorful diagnostics that the REPL and runtime use.
- `nimble repl` – interactive console with the new `:globals` command (lists all registered globals) and the diagnostic hook installed for inline feedback.

Use `cargo run --release -- <command>` when you want the fastest tooling loop during development.

## Examples & Learning Path

See [examples/README.md](examples/README.md) for the reorganized sample catalog (basic topics vs. stdlib-focused folders) and the exact commands used to run every snippet. The `docs/` tree references smaller, topic-based sections that align with the `basic/` exercises plus a directory per stdlib module.

## Documentation

- [Getting Started](docs/getting-started.md)
- [Syntax Reference](docs/syntax.md)
- [Standard Library](docs/stdlib/)

## License

MIT - see [LICENSE](LICENSE) for details.
