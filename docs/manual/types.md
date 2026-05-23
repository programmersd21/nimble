# Type System

## Primitive Types

| Type | LLVM Repr | Description |
|------|-----------|-------------|
| `Int` | `i64` | 64-bit signed integer |
| `Float` | `double` | 64-bit IEEE 754 floating-point |
| `String` | `ptr` (i8*) | Pointer to null-terminated byte array |
| `Bool` | `i1` | Boolean (`true` / `false`) |
| `Void` | `void` | Unit type (no value) |

Type names are case-insensitive. All of `Int`, `int`, `INT` resolve to the `Int` type.

## Explicit Type Annotations

Variable declarations and function parameters accept optional type annotations.

```
let x: Int = 42
fn add(a: Int, b: Int) -> Int:
    return a + b
```

## Type Casting

Nimble supports explicit type casting using the `as` keyword. This allows converting between primitive types when the conversion is safe or explicitly desired.

```
let x: Int = 42
let y: Float = x as Float

let z: Int = 3.14 as Int  # truncates to 3
let b: Bool = 1 as Bool   # true
let i: Int = true as Int  # 1
```

Supported casts:
- `Int as Float` / `Float as Int`
- `Int as Bool` / `Bool as Int`
- `Int as ptr` / `ptr as Int` (FFI)
- `Type as Type` (noop if same)

## Type Inference

The type checker uses Hindley-Milner inference (Algorithm W). Type annotations are optional; the checker infers types from literal values and usage.

```
let x = 42          # x : Int
let y = 3.14        # y : Float
let z = true        # z : Bool
let s = "hello"     # s : String
```

## Function Types

Functions have the type `fn(ParamTypes...) -> ReturnType`. Function types are inferred by the checker and used for call-site validation.

```
fn identity(x: Int) -> Int:
    return x
# identity : fn(Int) -> Int
```

## User-Defined Types

### Structs (Nominal)

User-defined struct types are tracked by name. Struct fields are reserved.

```
let p: Point = ...
```

### Interfaces (Structural)

Interface types define structural constraints. Any struct that provides the required methods satisfies the interface.

```
let w: Writer = ...
```

## Generic Instances

Parameterized types are represented as generic instances.

```
let arr: Array[Int] = ...
```

## Type Variables

During inference, fresh type variables are generated. The checker resolves them through unification. Unresolved variables appear as `?N` in error messages.

## Unification Rules

1. **Identity**: `a` unifies with `a` trivially.
2. **Variable binding**: `?N` unifies with any `T` by substituting `?N → T`.
3. **Occurs check**: `?N` cannot unify with a type containing `?N` (recursive types rejected).
4. **Function**: `fn(A1..An) -> R` unifies with `fn(B1..Bn) -> R'` by unifying each `Ai` with `Bi` and `R` with `R'`. Arity must match.
5. **Generic**: `T[A1..An]` unifies with `T[B1..Bn]` by unifying each `Ai` with `Bi`. Name and arity must match.
6. **Interface**: `Interface(I)` may unify with `Struct(S)` if `I` is a recognized interface name (structural conformance check).
7. **Mismatch**: Any other pair produces a `TypeError::Mismatch`.

## Type Errors

| Error | Cause |
|-------|-------|
| `Mismatch` | Expected type `A`, found type `B` |
| `AssignToImmutable` | Reassignment via `=` to a `let` binding |
| `UndefinedVariable` | Reference to undeclared name |
| `UndefinedType` | Reference to undeclared type name |
| `DuplicateDefinition` | Redeclaration in the same scope |
| `CallNonFunction` | Call expression on a non-function value |
| `ArgumentCount` | Wrong number of arguments to a function |
| `MissingMethod` | Interface requires a method the target lacks |
| `RecursiveType` | Occurs check failure |
