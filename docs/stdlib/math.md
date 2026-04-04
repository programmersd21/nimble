# math Module

Common math functions and constants.

## Functions
- `add(a float, b float) -> float`: Returns `a + b`.
- `sub(a float, b float) -> float`: Returns `a - b`.
- `mul(a float, b float) -> float`: Returns `a * b`.
- `div(a float, b float) -> float | error`: Returns `a / b`. Errors on `b == 0.0`.
- `pow(base float, exp float) -> float`: Returns `base^exp`.
- `sqrt(x float) -> float | error`: Returns square root of `x`.
- `abs(x float) -> float`: Absolute value.
- `floor(x float) -> float`: Floors `x`.
- `ceil(x float) -> float`: Ceils `x`.
- `round(x float) -> float`: Rounds `x`.
- `min(a float, b float) -> float`: Minimum.
- `max(a float, b float) -> float`: Maximum.
- `clamp(x float, lo float, hi float) -> float`: Clamps `x` in `[lo, hi]`.
- `log(x float) -> float | error`: Natural log.
- `log2(x float) -> float | error`: Base-2 log.
- `sin(x float) -> float`: Sine (radians).
- `cos(x float) -> float`: Cosine (radians).
- `tan(x float) -> float`: Tangent (radians).
- `random() -> float`: Random `0.0 <= x < 1.0`.
- `rand_int(lo int, hi int) -> int | error`: Random integer in `[lo, hi]`.

## Constants
- `PI float`: 3.141592653589793.
- `E float`: 2.718281828459045.

## Example
```nimble
load math
out(math.sqrt(16.0)?)
```
