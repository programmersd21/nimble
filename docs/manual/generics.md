# Generics

Nimble supports generic type syntax in annotations and unification. Type parameters are written in square brackets.

## Generic Type Annotations

Use `Type[Arg]` syntax in variable annotations:

```nimble
let b: Box[Int] = Box{value: 42}
```

The type checker unifies `Box[Int]` with the concrete struct `Box` by matching the base name and checking that the argument is compatible with the field types.

## Generic Structs

Declare a struct normally. The generic parameter appears only in the annotation at the use site:

```nimble
struct Box:
    let value: Int = 0

let b: Box[Int] = Box{value: 42}
```

Multi-parameter generics follow the same pattern:

```nimble
struct Pair:
    let first: Int = 0
    let second: Int = 0

let p: Pair[Int, Int] = Pair{first: 1, second: 2}
```

## Unification Rules

When the type checker encounters `T[A1..An]`:

1. It looks up the base struct `T`.
2. It unifies each type argument `Ai` with the corresponding field type.
3. A generic instance also unifies with the bare struct name `T` (the annotation is advisory).

## Current Limitations

- **No generic function monomorphization.** Functions cannot declare type parameters (`fn identity[T](x: T) -> T` is not yet supported). Generic parameters only appear in type annotations on variables.
- No variance annotations.
- No bounds / constraints on type parameters.

## Example

See [`examples/generics.nbl`](../../examples/generics.nbl).
