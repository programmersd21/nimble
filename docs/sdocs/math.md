# std.math

The `math` module provides standard mathematical functions and constants, backed by the system math library (libm).

## Constants

### `PI: Float`
Archimedes' constant (π ≈ 3.141592653589793).

### `E: Float`
Euler's number (e ≈ 2.718281828459045).

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

### `floor(x: Float) -> Float`
Returns the largest integer less than or equal to `x`.

### `ceil(x: Float) -> Float`
Returns the smallest integer greater than or equal to `x`.

### `round(x: Float) -> Float`
Returns the nearest integer to `x`, rounding half away from zero.

### `fabs(x: Float) -> Float`
Returns the absolute value of `x`.

### `log(x: Float) -> Float`
Returns the natural logarithm of `x`.

### `log10(x: Float) -> Float`
Returns the base-10 logarithm of `x`.

### `exp(x: Float) -> Float`
Returns e raised to the power of `x`.

### `degrees_to_radians(degrees: Float) -> Float`
Converts degrees to radians.

### `radians_to_degrees(radians: Float) -> Float`
Converts radians to degrees.

## Examples

```nimble
load std.math as m

let s = m.sin(m.PI / 2.0)
let r = m.sqrt(16.0)
let l = m.log(m.E)
let deg = m.radians_to_degrees(m.PI)  # 180.0
```
