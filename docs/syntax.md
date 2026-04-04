# Nimble Syntax Reference

Nimble is indentation‑sensitive with a minimal, Python‑like syntax.

## Comments

```nimble
# This is a single‑line comment. No block comments.
```

## Variables

Variables are mutable by default. Type annotation is optional.

```nimble
x = 10                  # Inferred int
name = "Nimble"        # Inferred str
pi float = 3.14         # Explicit float
active bool = true      # Explicit bool
```

## Types

Primitive types:

- `int` (64‑bit)
- `float` (64‑bit)
- `str`
- `bool`
- `null`

Compound types:

- Lists: `[T]`
- Maps: `{K: V}`
- Unions: `A | B`

Example:

```nimble
items [int] = [1, 2, 3]
meta {str: str} = {"id": "123"}
result int | error = safe_divide(10, 2)?
```

## Strings & Interpolation

Use `{expr}` inside strings. To render literal braces, use `{{` and `}}`.

```nimble
name = "Nimble"
out("Hello, {name}!")
```

## Control Flow

### If / Elif / Else

```nimble
if x > 10:
    out("Greater")
elif x == 10:
    out("Equal")
else:
    out("Smaller")
```

### Ternary Expression

```nimble
label = "adult" if age >= 18 else "minor"
```

## Loops

### Range Loop

```nimble
for i in 0..10:
    out(i)
```

### Range Loop with Step

```nimble
for i in 0..10 step 2:
    out(i)
```

### Collection Loop

```nimble
for item in [1, 2, 3]:
    out(item)

for key, val in {"a": 1}:
    out("{key}: {val}")
```

### While Loop

```nimble
while x > 0:
    x -= 1
```

## Functions

Defined with the `fn` keyword.

```nimble
fn add(a int, b int) -> int:
    return a + b

# Short syntax
fn square(x int) -> int = x * x
```

### Lambdas

```nimble
spawn fn():
    out("Async task")
```

## Classes (Structs)

`cls` defines a data structure. Fields require types.

```nimble
cls User:
    name str
    age  int

user = User(name="Alice", age=30)
out(user.name)
```

## Error Handling

Errors are values. Use `| error` for fallible return types and `?` for propagation.

```nimble
fn check(n int) -> int | error:
    if n < 0: return error("Negative")
    return n

val = check(-1)?
```

## Modules

Use `load` to import modules and `export` to make symbols public.

```nimble
load math
load utils from "./local_utils"

export fn add(a int, b int) -> int:
    return a + b
```
