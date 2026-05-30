# Structs

Structs are nominal record types with named, typed fields.

## Declaration

```nimble
struct Point:
    let x: Int = 0
    let y: Int = 0
```

Each field is declared with `let name: Type = default`. The default value is required in the declaration syntax.

## Struct Literals

Construct a struct by naming the type and providing field values:

```nimble
let p = Point{x: 3, y: 4}
```

All fields must be provided. Field order in the literal does not need to match the declaration order.

## Field Access

Access fields with `.`:

```nimble
let x = p.x
let y = p.y
```

Field access is also valid as the target of an assignment on a `var` binding:

```nimble
var p = Point{x: 0, y: 0}
p.x = 10
```

## Structs as Function Parameters

Pass structs by value to functions:

```nimble
fn area(r: Rectangle) -> Int:
    return r.width * r.height
```

## Method Syntax

Functions whose first parameter is the struct type can be called with method syntax (`obj.method(args)`). This is desugared at the type-checker level.

```nimble
fn describe(self: Point) -> Void:
    print("point")

let p = Point{x: 1, y: 2}
p.describe()     # same as describe(p)
```

## Type Annotations

Struct types can be used anywhere a type is expected:

```nimble
let rect: Rectangle = Rectangle{width: 10, height: 5}
```

## Conformance to Interfaces

A struct satisfies an interface when, for every method in the interface, a free function exists with the matching name and the struct as the first parameter type:

```nimble
interface Shape:
    fn area(self: Shape) -> Int

struct Square:
    let side: Int = 0

fn area(self: Square) -> Int:
    return self.side * self.side

let s: Shape = Square{side: 6}
```

## Codegen

Each struct is lowered to an LLVM named struct type. Fields are laid out in declaration order. Struct literals emit `insertvalue` sequences. Field access emits `extractvalue`.

## Current Limitations

- No inheritance or embedding
- No visibility modifiers on fields

## Example

See [`examples/structs.nbl`](../../examples/structs.nbl).
