# Standard Library

Nimble provides a standard library under the `std` namespace backed by a Rust runtime library (`ember`) that exposes ~120 C-ABI functions. Standard modules are loaded using the `load` keyword.

## All Standard Modules

| Module | Path | Description |
|--------|------|-------------|
| **Root** | `std` | Aggregator - loads every stdlib module |
| **IO** | `std.io` | Print, read/write/append file, file metadata, type-to-string conversion |
| **String** | `std.str` | String ops: length, concat, find, trim, case, replace, repeat, split, to_int, to_float |
| **Math** | `std.math` | 30+ functions: trig, log, pow, rounding, IEEE 754 checks, constants (PI, E, TAU, INF) |
| **Collections** | `std.collections` | Vec[T], HashMap[K,V] generic data structures |
| **Core** | `std.core` | abs, min, max, clamp, popcount, bit ops, checked/saturating/wrapping arithmetic |
| **Testing** | `std.testing` | assert_eq, assert_true, assert_ok, run_test |
| **Async** | `std.async` | Future, Channel, Mutex, async/await primitives |
| **Sync** | `std.sync` | AtomicInt (load/store/add/sub/swap/CAS), Mutex, Arc |
| **Thread** | `std.thread` | Thread, spawn, join, sleep, available_parallelism |
| **Format** | `std.fmt` | Formatting utilities (print_label, format_int) |
| **Alloc** | `std.alloc` | Memory allocation helpers (alloc, free, realloc) |
| **Log** | `std.log` | Logging helpers (info, warn, error) |
| **Filesystem** | `std.fs` | Filesystem: read/write/append/exists/size/delete/rename, directory ops |
| **Networking** | `std.net` | TCP connect/send/recv/close, DNS resolution |
| **JSON** | `std.json` | JSON parsing and stringification |
| **Crypto** | `std.crypto` | Random number generation |
| **Random** | `std.random` | Random int/float/range with Xoshiro256** PRNG |
| **Hashing** | `std.hash` | FNV-1a, SipHash, xxHash3 hashing |
| **Encoding** | `std.encoding` | Base64 (standard + URL-safe), hex (upper/lower), UTF-8 validation |
| **CLI** | `std.cli` | Command-line args, terminal detection, terminal size |
| **Text** | `std.text` | Text pattern matching (contains, replace_all) |
| **OS** | `std.os` | Environment variables (get/set), hostname, OS name, CPU count |
| **Process** | `std.process` | Process management (exit, abort) |
| **Time** | `std.time` | Epoch seconds, nanoseconds, monotonic clock, sleep, time formatting |
| **Memory** | `std.mem` | Higher-level memory management helpers |
| **FFI** | `std.ffi` | Foreign function interface helpers (printf) |
| **Reflection** | `std.reflect` | Reflection utilities (type_name, size_of) |
| **Builtins** | `std.builtin` | Built-in type definitions |

All modules are backed by the Rust runtime (`src/ember/mod.rs`) and linked as a static library during compilation - no stubs or unimplemented functions.

## Root `std` Module

The root `std` module is defined in `std/mod.nbl` and imports every available standard library submodule. Use it for a single import point:

```nimble
load std

fn main() -> Int:
    std.io.print("Hello from std")
    std.log.info("root std module loaded")
    std.testing.assert_eq(10, std.core.max(3, 10))
    return 0
```

## Fine-grained imports

Use selective imports or aliasing for a smaller namespace:

```nimble
load std.io::{print, print_int}
load std.math as m

fn main() -> Int:
    print("sqrt(16) =")
    print_int(4)
    return 0
```

## Example: stdlib composition

```nimble
load std

fn main() -> Int:
    std.io.print("Nimble standard library demo")

    # Core utilities
    std.testing.assert_eq(7, std.core.max(3, 7))
    std.testing.assert_eq(3, std.core.min(3, 7))

    # Math
    let half_pi = std.math.PI / 2.0
    let s = std.math.sin(half_pi)

    # Collections
    let vec = std.collections.new_vec[Int]()
    let vec = std.collections.push(vec, 42)

    # String operations
    let h = std.str.concat("hello", " world")

    # Encoding
    let b64 = std.encoding.base64.encode("hello")

    # Hashing
    let h1 = std.hash.fnv1a("data")

    # Random
    let r = std.random.random_range(1, 100)

    # CLI
    let n = std.cli.arg_count()
    let t = std.cli.is_terminal()

    return 0
```

## Runtime Architecture

All stdlib functions delegate to the **ember** Rust runtime (`src/ember/mod.rs`), a C-ABI static library. When a Nimble program calls `std.str.length(s)`, the chain is:

1. Nimble codegen emits `call void @nimble_string_length(ptr %s)`
2. The LLVM IR is assembled and linked against `ember.lib` / `libember.a`
3. At runtime, the call resolves to the Rust `extern "C" fn nimble_string_length()` implementation

This architecture provides:
- **Performance**: Rust implementations for all stdlib operations
- **Correctness**: Well-tested algorithms (Xoshiro256**, FNV-1a, SipHash, etc.)
- **Portability**: Platform-specific logic (filesystem, networking, threading) handled by Rust's standard library

## Available documentation

For detailed information on each module, see the [stdlib documentation](../sdocs/):

- [std.io](../sdocs/io.md)
- [std.str](../sdocs/str.md)
- [std.math](../sdocs/math.md)
- [std.collections](../sdocs/collections.md)
- [std.core](../sdocs/core.md)
- [std.testing](../sdocs/testing.md)
- [std.sync](../sdocs/sync.md)
- [std.thread](../sdocs/thread.md)
- [std.fmt](../sdocs/fmt.md)
- [std.alloc](../sdocs/alloc.md)
- [std.log](../sdocs/log.md)
- [std.fs](../sdocs/fs.md)
- [std.net](../sdocs/net.md)
- [std.json](../sdocs/json.md)
- [std.crypto](../sdocs/crypto.md)
- [std.random](../sdocs/random.md)
- [std.hash](../sdocs/hash.md)
- [std.encoding](../sdocs/encoding.md)
- [std.cli](../sdocs/cli.md)
- [std.text](../sdocs/text.md)
- [std.os](../sdocs/os.md)
- [std.process](../sdocs/process.md)
- [std.time](../sdocs/time.md)
- [std.mem](../sdocs/mem.md)
- [std.ffi](../sdocs/ffi.md)
- [std.reflect](../sdocs/reflect.md)
- [std.builtin](../sdocs/builtin.md)
