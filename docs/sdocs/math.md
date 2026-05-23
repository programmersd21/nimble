# std.math

The `math` module provides standard mathematical functions, backed by the system math library (libm).

## Functions

### `sin(x: Float) -> Float`
Returns the sine of `x` (in radians).

### `cos(x: Float) -> Float`
Returns the cosine of `x` (in radians).

### `tan(x: Float) -> Float`
Returns the tangent of `x` (in radians).

### `sqrt(x: Float) -> Float`
Returns the square root of `x`.

### `pow(base: Float, exp: Float) -> Float`
Returns `base` raised to the power of `exp`.

## Examples

```nimble
load std.math

let s = math.sin(3.14159 / 2.0)
let r = math.sqrt(16.0)
```
