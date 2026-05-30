# Standard Library

Nimble provides a standard library under the `std` namespace. Standard modules are loaded using the `load` keyword:

- `load std.io` - basic I/O helpers (print, read_file, write_file, read_line)
- `load std.math` - math functions backed by the host math library (sin, cos, sqrt, log, PI, E, ...)
- `load std.collections` - generic Vec[T] and HashMap[K, V] data structures
- `load std.alloc` - allocation and reallocation helpers
- `load std.core` - core utilities: Result, Option, max_i, min_i, clamp, panic, expect
- `load std.testing` - assertion framework with assert_eq, assert_true, assert_ok, run_test
- `load std.fmt` - formatting utilities (print_label, format_int)
- `load std.async` - async primitives: Future, Channel, Mutex, spawn, join, sleep
- `load std.sync` - synchronization: AtomicInt, Arc
- `load std.thread` - threading: Thread, spawn, join
- `load std.log` - simple logging helpers (info, warn, error)
- `load std.fs` - file system operations (open, close)
- `load std.net` - networking (connect, send, recv)
- `load std.json` - JSON parsing (parse, stringify)
- `load std.crypto` - random numbers (rand, srand)
- `load std.os` - OS interaction (get_env, execute)
- `load std.process` - process management (terminate)
- `load std.time` - time utilities (now)
- `load std.mem` - memory management (alloc_bytes, free_bytes)
- `load std.ffi` - foreign function interface helpers (printf)
- `load std.reflect` - reflection utilities (type_name, size_of)
- `load std` - root `std` module aggregator that loads every available stdlib module

## Root `std` Module

The root `std` module is defined in `std/mod.nbl` and imports available standard library submodules. Use it when you want one import point for the standard library:

```nimble
load std

fn main() -> Int:
    std.io.println("Hello from std")
    std.log.info("root std module loaded")
    std.testing.assert_eq(10, std.core.max_i(3, 10))
    return 0
```

## Fine-grained imports

Use selective imports or aliasing for a smaller namespace:

```nimble
load std.io::{println, print_int_val}
load std.math as m

fn main() -> Int:
    println("sqrt(16) =")
    print_int_val(4)
    return 0
```

## Example: stdlib composition

```nimble
load std

fn main() -> Int:
    std.io.println("Nimble standard library demo")
    std.log.info("Testing stdlib helpers")
    std.testing.assert_eq(7, std.core.max_i(3, 7))
    std.testing.assert_eq(3, std.core.min_i(3, 7))
    std.fmt.print_label("clamp(10,0,5) = ", std.core.clamp(10, 0, 5))

    # File I/O with Result propagation
    let content = std.io.read_file("test.txt")?
    std.io.println(content)

    # Math with constants
    let half_pi = std.math.PI / 2.0
    let s = std.math.sin(half_pi)

    # Collections
    let vec = std.collections.new_vec[Int]()
    let vec = std.collections.push(vec, 42)

    # Async
    let future = std.async.spawn(fn(): 42)
    let result = std.async.await(future)?

    return 0
```

## Available standard modules

For detailed information on each module, see the [stdlib documentation](../sdocs/):

- [std.io](../sdocs/io.md) - print, read_file, write_file, read_line
- [std.math](../sdocs/math.md) - sin, cos, sqrt, log, PI, E, floor, ceil, round
- [std.collections](../sdocs/collections.md) - Vec[T], HashMap[K,V]
- [std.testing](../sdocs/testing.md) - assert_eq, assert_true, assert_ok, run_test
- [std.fmt](../sdocs/fmt.md) - print_label, format_int
- [std.async](../sdocs/async.md) - Future, Channel, Mutex
- [std.sync](../sdocs/sync.md) - AtomicInt, Arc
- [std.thread](../sdocs/thread.md) - Thread, spawn, join
- [std.core](../sdocs/core.md) - Result, Option, panic, expect, unwrap
- [std.alloc](../sdocs/alloc.md) - alloc, free, realloc
- [std.log](../sdocs/log.md) - info, warn, error
- [std.fs](../sdocs/fs.md) - open, close
- [std.net](../sdocs/net.md) - connect, send, recv
- [std.json](../sdocs/json.md) - parse, stringify
- [std.ffi](../sdocs/ffi.md) - printf helpers

## math module example

The `std.math` module exposes floating-point math functions from the host platform.

```nimble
load std.math as m
load std.io
load std.log

fn main() -> Int:
    std.log.info("math module demo")

    let zero = m.sin(0.0)
    let root = m.sqrt(16.0)
    let power = m.pow(2.0, 10.0)

    if zero == 0.0 && root == 4.0 && power == 1024.0:
        std.log.info("std.math functions are working")
        return 0
    std.log.error("std.math validation failed")
    return 1
```
