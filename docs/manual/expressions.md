# Expression Reference

## Literals

```
42           # Int
3.14         # Float
"hello"      # String
true         # Bool
false        # Bool
```

## Identifiers

Reference a variable or function by name.

```
let x = 42
x            # evaluates to 42
```

## Binary Operators

### Arithmetic

```
1 + 2        # Int addition
3.0 - 1.0    # Float subtraction
4 * 5        # Int multiplication
10 / 3       # Int division (sdiv)
10 % 3       # Int modulo (srem)
```

Operand types must match. Both must be `Int` or both must be `Float`.

### Comparison

Results are always `Bool`.

```
a == b       # equality
a != b       # inequality
a < b        # less than
a > b        # greater than
a <= b       # less than or equal
a >= b       # greater than or equal
```

### Logical

Operands must be `Bool`.

```
a && b       # AND (select-based)
a || b       # OR (select-based)
```

## Unary Operators

```
-x           # numeric negation
!flag        # logical NOT
&x           # reference
```

Unary minus applies to `Int` or `Float`. NOT applies only to `Bool`. `&` creates a reference.

## Function Calls

```
callee(arg1, arg2, ...)
```

Arguments are evaluated left-to-right. The callee must be a function type.

```
fn twice(x: Int) -> Int:
    return x * 2

let r = twice(21)     # r = 42
```

Calls to undeclared names are treated as external function calls (FFI fallback).

```
print("hello")         # calls extern void @print(...)
printf(fmt, 42)        # calls extern i32 @printf(...)
```

## Method Calls

```
value.method(args...)
```

Desugars to `method(value, args...)` at the type-checker level. The first argument of the function must accept the value's type.

```
fn describe(self: String) -> Void:
    print(self)

"hello".describe()     # prints "hello"
```

## Lambda Expressions

```
fn(params): body
fn(params): expr
```

Anonymous function literals. The body can be a single expression (implicit return) or a block with statements.

```
let double = fn(x: Int): x * 2
let apply = fn(f: fn(Int) -> Int, x: Int): f(x)
let noop = fn(): print("noop")
```

Non-capturing closures compile to function pointers. Capturing closures
(calling outer variables) compile to a trampoline struct.

## Match Expressions

```
match value:
    pattern1:
        result1
    pattern2:
        result2
```

Evaluate value and execute the matching arm's body. Every arm returns the same type.

```
let description = match code:
    200:
        "OK"
    404:
        "Not Found"
    _:
        "Unknown"
```

## Assignment

```
target = value
target += value
target -= value
target *= value
target /= value
target %= value
```

Target must be a mutable variable (`var`). Compound assignment operators perform the operation and store the result.

```
var x = 10
x = x + 5       # x = 15
x += 5          # x = 20
x -= 3          # x = 17
```

## Error Propagation (`?`)

```
expr?
```

The postfix `?` operator checks the tag of a `Result[T, E]` or `Option[T]`:
- For `Result`: if `Ok(val)`, evaluates to `val`; if `Err(e)`, early-returns `Err(e)`.
- For `Option`: if `Some(val)`, evaluates to `val`; if `None`, early-returns `None`.

Must appear inside a function whose return type matches the error variant.

```
fn read_config(path: String) -> Result[String, String]:
    let content = read_file(path)?
    return Ok(content)
```

## Struct Literals

```
TypeName{field1: value1, field2: value2}
```

All fields must be provided. Field order does not need to match declaration order.

```
let p = Point{x: 3, y: 4}
```

## Enum Variant Construction

```
EnumName.Variant(args...)
EnumName.Variant
```

Construct an enum value by qualifying the variant name with the enum type.

```
let x = Option.Some(42)
let y: Option[Int] = None
let c = Color.Red
let rgb = Color.Blue(255)
```

## Macro Invocation

```
macro_name(args...)
```

Invoke a previously defined compile-time macro. The macro body is substituted with the arguments bound.

## Grouping

```
(expression)
```

Parentheses override default precedence.

```
let r = (1 + 2) * 3    # 9, not 7
```

## Casting

```
value as TargetType
```

Explicit type conversion between compatible types.

```
let x: Float = 42 as Float
let y: Int = 3.14 as Int
```
