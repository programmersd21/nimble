# Nimble Language Specification

This document describes the Nimble language as currently implemented.

## Types

| Type | Description | LLVM mapping |
|------|-------------|--------------|
| `Int` | 64-bit signed integer | `i64` |
| `Float` | 64-bit IEEE-754 float | `double` |
| `String` | UTF-8 byte array | `i8*` |
| `Bool` | Boolean | `i1` |
| `Void` | No value | `void` |
| `&T` | Immutable reference to T | `ptr` |
| `&mut T` | Mutable reference to T | `ptr` |
| `fn(A) -> B` | Function pointer / closure | `ptr` or `{ptr, ptr}` |

Custom types:
| `struct` | Nominal record with named fields | LLVM named struct |
| `enum` | Tagged union with variants | `{i64 tag, i64 payload}` |
| `interface` | Structural type (method set) | Monomorphized at use site |
| `Type[T, ...]` | Generic instance | Specialized per instantiation |

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

## Pattern Matching

```nimble
match value:
    Some(val):
        print(val)
    None:
        print("none")

match value:
    _:
        print("wildcard")

if let Some(val) = optional:
    print("has value")

while let Ok(line) = read_line():
    process(line)
```

## Functions

```nimble
fn add(a: Int, b: Int) -> Int:
    return a + b

fn greet(name: String) -> Void:
    print("hello, ", name)

# Generic function
fn identity[T](x: T) -> T:
    return x

# Lambda / closure
let double = fn(x: Int): x * 2

# Method call syntax
fn describe(self: String) -> Void:
    print(self)

"hello".describe()
```

- Parameters require explicit type annotations
- Return type is optional and defaults to `Void`
- Functions should return a value when the return type is not `Void`
- Generic type parameters are written in square brackets
- Methods are free functions whose first parameter is `self`; called with `obj.method()`

## Enums

```nimble
enum Option[T]:
    Some(T), None

enum Result[T, E]:
    Ok(T), Err(E)

let x = Option.Some(42)
```

## Error Propagation

```nimble
fn load_file(path: String) -> Result[String, String]:
    let content = read_file(path)?
    return Ok(content)
```

The `?` operator checks the tag of a Result, unwraps if Ok, or early-returns the Err.

## Defer

```nimble
fn work() -> Void:
    let handle = open("file.txt")
    defer close(handle)
    # handle is automatically closed on scope exit
```

## Macros

```nimble
macro assert_equal(a, b):
    if a != b:
        print("assertion failed")
        print_int(a)
        print_str(" != ")
        print_int(b)
        panic("")

assert_equal(1 + 1, 2)
```

## Async

```nimble
let future = async fetch_data()
let result = await future
```

## Reference Types

```nimble
let x = 42
let r: &Int = &x
let r2: &mut Int = &mut x
```

References provide shared/mutable access without ownership transfer.

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
nimble doc
nimble profile program.nbl
nimble fuzz --iterations 5000
nimble lint source.nbl
nimble generate-header nimble_runtime.h
```
