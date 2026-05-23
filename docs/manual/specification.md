# Nimble Language Specification

## Types

| Type      | Description          | LLVM mapping |
|-----------|----------------------|--------------|
| `int`     | 64-bit signed integer| `i64`        |
| `float`   | 64-bit IEEE-754      | `double`     |
| `string`  | UTF-8 byte array     | `i8*`        |
| `bool`    | Boolean              | `i1`         |
| `void`    | No value             | `void`       |

## Variables

```nimble
let x = 42           // immutable, type inferred
var y: float = 3.14  // mutable with explicit type
var z = 1            // mutable, type inferred
z = 2
```

- `let` declares an immutable binding
- `var` declares a mutable binding
- Type annotations are optional when the type can be inferred

## Control Flow

Blocks are defined by indentation (4 spaces per level). No curly braces.

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
fn add(a: int, b: int) -> int:
    return a + b

fn greet(name: string) -> void:
    print("hello, ", name)
```

- Parameters require explicit type annotations
- Return type is optional (defaults to `void`)
- Functions must have a `return` statement if the return type is not `void`

## FFI

External C functions are declared with the `extern` keyword:

```nimble
extern fn printf(fmt: string) -> int
```

Nimble links against the `ember` runtime which provides:
- `nimble_alloc`, `nimble_free`, `nimble_realloc`
- `nimble_print_i64`, `nimble_print_f64`, `nimble_print_string`, `nimble_print_bool`
- `nimble_panic`

## Compilation

```sh
smelt source.nbl                 # produces source.exe
smelt source.nbl --emit-llvm     # produces source.ll (IR only)
anvil init my_project            # scaffold a new project
anvil build                      # compile project
anvil run                        # run project
```
