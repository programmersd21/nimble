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
```

Unary minus applies to `Int` or `Float`. NOT applies only to `Bool`.

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

## Grouping

```
(expression)
```

Parentheses override default precedence.

```
let r = (1 + 2) * 3    # 9, not 7
```
