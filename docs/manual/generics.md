# Generics

Nimble supports generics for both structs and functions. Type parameters are written in square brackets.

## Generic Functions

Functions can declare type parameters before the parameter list:

```nimble
fn identity[T](x: T) -> T:
    return x

fn pair[T, U](a: T, b: U) -> T:
    return a
```

### Monomorphization

Each call to a generic function with distinct type arguments generates a fresh monomorphized copy. The type arguments are substituted into the function body, and the resulting concrete function is compiled independently.

```nimble
let a = identity[Int](42)     # monomorphizes identity with T=Int
let b = identity[Float](3.14) # monomorphizes identity with T=Float
```

## Generic Structs

Structs can be declared with generic parameters in their type annotation at the use site:

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

## Generic Enums

Enums also support type parameters:

```nimble
enum Option[T]:
    Some(T), None

enum Result[T, E]:
    Ok(T), Err(E)

let x: Option[Int] = Option.Some(42)
let y: Result[String, String] = Ok("ok")
```

## Generic Collections

The standard library uses generics for collections:

```nimble
let vec: Vec[Int] = new_vec()
let map: HashMap[String, Int] = new_map()
```

## Unification Rules

When the type checker encounters `T[A1..An]`:

1. It looks up the base type `T`.
2. It unifies each type argument `Ai` with the corresponding field type.
3. A generic instance also unifies with the bare type name `T` (the annotation is advisory).

## Current Limitations

- No variance annotations
- No bounds / constraints on type parameters
- No trait bounds on generic parameters
