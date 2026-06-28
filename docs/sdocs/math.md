# std.math

Standard mathematical functions and constants, backed by the Rust runtime.

## Constants

### `PI: Float`
Archimedes' constant (π ≈ 3.141592653589793).

### `E: Float`
Euler's number (e ≈ 2.718281828459045).

### `TAU: Float`
Tau constant (τ ≈ 6.283185307179586).

### `INF: Float`
Positive infinity (≈ 1e308).

## Trigonometric Functions

### `sin(x: Float) -> Float`
Sine of `x` (radians).

### `cos(x: Float) -> Float`
Cosine of `x` (radians).

### `tan(x: Float) -> Float`
Tangent of `x` (radians).

### `asin(x: Float) -> Float`
Arc sine of `x` (result in radians).

### `acos(x: Float) -> Float`
Arc cosine of `x` (result in radians).

### `atan(x: Float) -> Float`
Arc tangent of `x` (result in radians).

### `atan2(y: Float, x: Float) -> Float`
Arc tangent of `y/x` (result in radians).

### `sinh(x: Float) -> Float`
Hyperbolic sine of `x`.

### `cosh(x: Float) -> Float`
Hyperbolic cosine of `x`.

### `tanh(x: Float) -> Float`
Hyperbolic tangent of `x`.

## Power and Root Functions

### `sqrt(x: Float) -> Float`
Square root of `x`.

### `cbrt(x: Float) -> Float`
Cube root of `x`.

### `pow(base: Float, exp: Float) -> Float`
`base` raised to the power of `exp`.

### `fma(a: Float, b: Float, c: Float) -> Float`
Fused multiply-add: `a * b + c` (single rounding).

### `hypot(a: Float, b: Float) -> Float`
Euclidean norm: `sqrt(a² + b²)`.

### `fmod(a: Float, b: Float) -> Float`
Floating-point remainder of `a / b`.

## Rounding Functions

### `floor(x: Float) -> Float`
Largest integer ≤ `x`.

### `ceil(x: Float) -> Float`
Smallest integer ≥ `x`.

### `round(x: Float) -> Float`
Nearest integer to `x`, rounding half away from zero.

### `trunc(x: Float) -> Float`
Truncates the fractional part of `x`.

## Exponential and Logarithmic Functions

### `ln(x: Float) -> Float`
Natural logarithm of `x`.

### `log2(x: Float) -> Float`
Base-2 logarithm of `x`.

### `log10(x: Float) -> Float`
Base-10 logarithm of `x`.

### `exp(x: Float) -> Float`
`e` raised to the power of `x`.

## Absolute Value and IEEE 754 Checks

### `abs_float(x: Float) -> Float`
Absolute value of `x`.

### `is_nan(x: Float) -> Bool`
Returns `true` if `x` is NaN.

### `is_infinite(x: Float) -> Bool`
Returns `true` if `x` is ±infinity.

### `is_finite(x: Float) -> Bool`
Returns `true` if `x` is finite (not NaN and not infinite).

### `next_after(x: Float, y: Float) -> Float`
Returns the next representable float after `x` in the direction of `y`.

## Utility Functions

### `degrees_to_radians(degrees: Float) -> Float`
Converts degrees to radians: `degrees * PI / 180.0`.

### `radians_to_degrees(radians: Float) -> Float`
Converts radians to degrees: `radians * 180.0 / PI`.

### `lerp(a: Float, b: Float, t: Float) -> Float`
Linear interpolation: `a + (b - a) * clamp(t, 0, 1)`.

## Examples

```nimble
load std.math as m

let s = m.sin(m.PI / 2.0)
let r = m.sqrt(16.0)
let l = m.ln(m.E)
let deg = m.radians_to_degrees(m.PI)
let n = m.is_nan(0.0 / 0.0)
```
