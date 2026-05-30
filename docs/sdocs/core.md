# std.core

Core utility functions for integer arithmetic.

## Functions

### `abs_i(x: Int) -> Int`
Returns the absolute value of `x`.

### `max_i(a: Int, b: Int) -> Int`
Returns the larger of `a` and `b`.

### `min_i(a: Int, b: Int) -> Int`
Returns the smaller of `a` and `b`.

### `clamp(value: Int, low: Int, high: Int) -> Int`
Clamps `value` to the range `[low, high]`.

## Examples

```nimble
load std.core

let a = core.abs_i(-5)     # 5
let m = core.max_i(3, 7)   # 7
let c = core.clamp(15, 0, 10)  # 10
```
