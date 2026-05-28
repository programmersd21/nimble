# Nimble Language Specification

This document describes the intended language direction. For the current implementation gap, see [`roadmap.md`](roadmap.md).

## Types

| Type | Description | LLVM mapping |
|------|-------------|--------------|
| `Int` | 64-bit signed integer | `i64` |
| `Float` | 64-bit IEEE-754 float | `double` |
| `String` | UTF-8 byte array | `i8*` |
| `Bool` | Boolean | `i1` |
| `Void` | No value | `void` |

## Variables

```nimble
let x = 42
var y: Float = 3.14
var z = 1
z = 2
```

- `let` declares an immutable binding
- `var` declares a mutable binding
- Type annotations are optional when the type can be inferred

## Control Flow

Blocks are defined by indentation. No curly braces.

```nimble
if x > 0:
    print("positive")
elif x < 0:
    print("negative")
else:
    print("zero")

while i < 10:
    print(i)
    i = i + 1

for x in items:
    print(x)
```

## Functions

```nimble
fn add(a: Int, b: Int) -> Int:
    return a + b

fn greet(name: String) -> Void:
    print("hello, ", name)
```

- Parameters require explicit type annotations
- Return type is optional and defaults to `Void`
- Functions should return a value when the return type is not `Void`

## FFI

External C functions are declared with the `extern` keyword:

```nimble
extern fn printf(fmt: String) -> Int
```

Nimble links against the `ember` runtime, which provides runtime primitives used by the compiler and standard library integration.

## Compilation

```sh
nimble compile source.nbl
nimble compile source.nbl --emit-llvm
nimble init my_project
nimble build
nimble run
```
