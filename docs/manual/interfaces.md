# Interfaces

Interfaces declare a set of required method signatures. Conformance is structural: a type satisfies an interface when free functions with the required names exist and accept the concrete type as their first parameter.

## Declaration

```nimble
interface Shape:
    fn area(self: Shape) -> Int
```

Each method entry is a `fn` signature. No body is provided.

## Conformance

A struct conforms to an interface when, for every method declared in the interface, a top-level function exists with:

1. The same name.
2. A first parameter whose type is the concrete struct.
3. A compatible return type.

```nimble
struct Square:
    let side: Int = 0

fn area(self: Square) -> Int:
    return self.side * self.side
```

`Square` now conforms to `Shape` because `area(self: Square) -> Int` satisfies `fn area(self: Shape) -> Int`.

## Using Interface Types

Annotate a variable or parameter with the interface type to accept any conforming struct:

```nimble
let s: Shape = Square{side: 6}

fn print_area(s: Shape) -> Void:
    print_int(area(s))
```

The type checker verifies conformance at the assignment or call site.

## Multiple Interfaces

A struct can conform to multiple interfaces by providing the required functions for each.

## Current Limitations

- No dynamic dispatch / vtable. Interface variables are checked statically at the assignment site.
- No default method implementations.
- No interface composition (`interface A: B`).

## Example

See [`examples/interfaces.nbl`](../../examples/interfaces.nbl).
