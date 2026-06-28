# std.core

Core utility functions for integer arithmetic - absolute value, min/max, clamping, bit manipulation, and checked/saturating/wrapping operations.

All functions are backed by the Rust runtime.

## Basic Numeric Helpers

### `abs(x: Int) -> Int`
Absolute value of `x`.

### `max(a: Int, b: Int) -> Int`
Larger of `a` and `b`.

### `min(a: Int, b: Int) -> Int`
Smaller of `a` and `b`.

### `clamp(value: Int, low: Int, high: Int) -> Int`
Clamps `value` to the range `[low, high]`.

## Bit Manipulation

### `count_ones(x: Int) -> Int`
Number of ones in the binary representation of `x` (popcount).

### `leading_zeros(x: Int) -> Int`
Number of leading zeros in the binary representation of `x`.

### `trailing_zeros(x: Int) -> Int`
Number of trailing zeros in the binary representation of `x`.

### `byteswap(x: Int) -> Int`
Reverses the byte order of `x`.

### `rotate_left(x: Int, n: Int) -> Int`
Rotates the bits of `x` left by `n` positions.

### `rotate_right(x: Int, n: Int) -> Int`
Rotates the bits of `x` right by `n` positions.

## Checked Arithmetic (returns 0 on overflow)

### `checked_add(a: Int, b: Int) -> Int`
`a + b`, returns 0 on overflow.

### `checked_sub(a: Int, b: Int) -> Int`
`a - b`, returns 0 on underflow.

### `checked_mul(a: Int, b: Int) -> Int`
`a * b`, returns 0 on overflow.

### `checked_div(a: Int, b: Int) -> Int`
`a / b`, returns 0 on division by zero.

## Saturating Arithmetic (clamps at bounds)

### `saturating_add(a: Int, b: Int) -> Int`
`a + b`, saturates at Int min/max.

### `saturating_sub(a: Int, b: Int) -> Int`
`a - b`, saturates at Int min/max.

## Wrapping Arithmetic (wraps around)

### `wrapping_add(a: Int, b: Int) -> Int`
`a + b`, wraps on overflow.

### `wrapping_sub(a: Int, b: Int) -> Int`
`a - b`, wraps on underflow.

### `wrapping_mul(a: Int, b: Int) -> Int`
`a * b`, wraps on overflow.

## Examples

```nimble
load std.core

let a = core.abs(-5)              # 5
let m = core.max(3, 7)            # 7
let c = core.clamp(15, 0, 10)     # 10
let n = core.count_ones(42)       # 3
let sa = core.saturating_add(9223372036854775807, 1)
let wa = core.wrapping_add(255, 1)
```
