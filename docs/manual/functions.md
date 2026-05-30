# Functions

## Function Definitions

```
fn name(param1: Type1, param2: Type2, ...) -> ReturnType:
    body
```

- Parameters are positional and typed.
- Return type is declared after `->`. Omitting it defaults to `Void`.
- The body is an indented block of statements.
- Multiple functions can be defined at module level. Recursive and mutually-recursive calls are supported via two-pass registration (signatures registered first, bodies checked second).

```
fn fib(n: Int) -> Int:
    if n <= 1:
        return n
    return fib(n - 1) + fib(n - 2)
```

## Generic Functions

Functions can declare type parameters in square brackets before the parameter list:

```
fn identity[T](x: T) -> T:
    return x

fn swap[T](a: T, b: T) -> T:
    return b

fn first[T, U](a: T, b: U) -> T:
    return a
```

Generic functions are monomorphized: a separate copy is generated for each unique combination of type arguments at each call site.

## Closures / Lambdas

Anonymous functions are written with `fn`:

```
let double = fn(x: Int): x * 2
let apply = fn(f: fn(Int) -> Int, x: Int): f(x)
```

Lambdas can capture variables from the enclosing scope. Capturing closures compile to a trampoline struct `{fn_ptr, capture_data}`. Non-capturing closures compile to plain function pointers.

```
let scale = 2
let doubler = fn(x: Int): x * scale   # captures `scale`
```

## Parameter Passing

Parameters are passed by value (copied into the callee's stack frame). Parameters are immutable bindings inside the function body.

```
fn swap(a: Int, b: Int) -> (Int, Int):
    return b               # only one return value supported
```

Nimble currently supports exactly one return value. Tuple returns are not implemented.

## Return Values

Use `return` to exit a function. The returned expression must match the declared return type.

```
fn zero() -> Int:
    return 0               # returns Int

fn greet(name: String):
    print("Hello, " + name)  # returns Void
    return
```

## Method Call Syntax

Functions whose first parameter is named `self` can be called with method syntax:

```
fn describe(self: String) -> Void:
    print(self)

"hello".describe()     # same as describe("hello")
```

This is desugared at the type-checker level, so no special codegen support is needed.

## Foreign Function Interface

`extern fn` declares a function with C calling convention. No body is provided. The codegen emits an LLVM `declare`, making the symbol available to the linker.

```
extern fn printf(fmt: String, ...) -> Int
extern fn malloc(size: Int) -> String
extern fn free(ptr: String) -> Void
```

Calling an `extern fn` generates a standard `call` instruction. The caller is responsible for passing arguments matching the declared types.

The compiler driver (`smelt`) links the resulting object file against the `ember` runtime and the system C library via the auto-discovered host linker.

## Variable Scope

- Each function body creates a new scope.
- Parameters are scoped to the function body.
- Nested blocks (`if`, `while`, `for`) create inner scopes.
- Inner scopes can shadow outer bindings.
- Variables declared in inner scopes are not visible outside that scope.

```
fn example(x: Int):
    let a = 1          # visible in entire function
    if true:
        let b = 2      # visible only inside if block
        a = 3          # error: a is immutable
    # b is not visible here
```

## Current Limitations

- No keyword-only, default, or variadic parameters
- No multiple return values
- No async functions (use the async/await primitives from std.async instead)
