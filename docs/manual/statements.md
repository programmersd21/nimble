# Statement Reference

## Variable Declarations

### `let` - immutable binding

```
let name = value
let name: Type = value
```

Once bound, the variable cannot be reassigned.

```
let x = 42
x = 10    # error: AssignToImmutable
```

### `var` - mutable binding

```
var name = value
var name: Type = value
```

Mutable bindings can be reassigned with `=` or compound assignment operators.

```
var x = 0
x = 10
x += 5
```

## Function Definitions

```
fn name(params) [-> ReturnType]:
    body
```

Parameters are comma-separated `name: Type` pairs. Return type is optional (defaults to `Void`).

```
fn add(a: Int, b: Int) -> Int:
    return a + b
```

## `return`

```
return [value]
```

Exit the current function. The value type must match the declared return type. Bare `return` is valid in `Void` functions.

```
fn nothing():
    return
```

## Extern Function Declarations

```
extern fn name(params) [-> ReturnType]
```

Declare a foreign function with C calling convention. No body is provided. The codegen emits an LLVM `declare`.

```
extern fn printf(fmt: String) -> Int
```

## `if` / `elif` / `else`

```
if condition:
    body
elif condition:
    body
else:
    body
```

Condition must evaluate to `Bool`. Multiple `elif` branches are supported. The `else` branch is optional.

```
if x > 0:
    let sign = 1
elif x < 0:
    let sign = -1
else:
    let sign = 0
```

## `while`

```
while condition:
    body
```

Repeatedly execute body while condition is `true`.

```
var i = 0
while i < 10:
    i = i + 1
```

## `break` / `continue`

```
while condition:
    if done:
        break
    continue
```

`break` exits the nearest enclosing loop. `continue` jumps to the next loop iteration.

## `for`

```
for variable in iterable:
    body
```

Iterates over an expression. Currently iterates once over the value (simple assignment semantics). Extended iteration is reserved.

```
for i in range:
    print(i)
```

## `load`

```
load module.path
load module.path as alias
load module.path::{symbol1, symbol2}
```

Imports a module into the current scope. This is the primary module-loading mechanism for the standard library and project code.

## `pub`

```
pub load module.path
```

Marks a `load` as public. The parser and type checker recognize it, but visibility semantics are still lightweight.

## Expression Statements

Any expression can be used as a statement. The result is discarded.

```
print("hello")
x + y
fizzbuzz(42)
```

## Current Gaps

- No `match` statement
- No `try` / `except`-style error handling
- No `defer` or `using` statement
