# Standard Library

Nimble provides a standard library under the `std` namespace. Standard modules are loaded using the `load` keyword:

- `load std.io` - basic I/O helpers
- `load std.math` - math functions backed by the host math library
- `load std.alloc` - allocation and reallocation helpers
- `load std.core` - core utilities like `max_i`, `min_i`, and `clamp`
- `load std.log` - simple logging helpers
- `load std.fmt` - formatting utilities
- `load std.testing` - lightweight assertion helpers
- `load std` - root `std` module aggregator that loads every available stdlib module

## Root `std` Module

The root `std` module is defined in `std/mod.nbl` and imports every available standard library submodule. Use it when you want one import point for the standard library:

```nimble
load std
load std.math as m

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
load std.fmt

fn main() -> Int:
    std.io.println("Nimble standard library demo")
    std.log.info("Testing stdlib helpers")
    std.testing.assert_eq(7, std.core.max_i(3, 7))
    std.testing.assert_eq(3, std.core.min_i(3, 7))
    std.fmt.print_label("clamp(10,0,5) = ", std.core.clamp(10, 0, 5))
    let buffer = std.alloc.alloc(16)
    std.mem.free_bytes(buffer, 16)
    return 0
```

## Available standard modules

For detailed information on each module, see the [stdlib documentation](../sdocs/):

- [std.io](../sdocs/io.md) - `println`, `print_no_newline`, `print_int_val`
- [std.math](../sdocs/math.md) - `sin`, `cos`, `tan`, `sqrt`, `pow`
- [std.fs](../sdocs/fs.md) - `read_file`, `write_file`
- [std.testing](../sdocs/testing.md) - `assert_eq`
- [std.fmt](../sdocs/fmt.md) - `format`
- `std.alloc` - `alloc`, `free`, `realloc`
... (and so on)

Many modules already provide stable API surfaces and runtime-backed extern declarations. Some namespaces reserve future runtime support while keeping the `std` import model consistent across the language.

## Math module example

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
