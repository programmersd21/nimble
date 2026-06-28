# std.random

Pseudo-random number generation using the Xoshiro256** algorithm.

All functions are backed by the Rust runtime.

## Functions

### `random_int() -> Int`
Returns a pseudo-random 64-bit integer.

### `random_float() -> Float`
Returns a pseudo-random float in the range [0.0, 1.0).

### `random_range(lo: Int, hi: Int) -> Int`
Returns a pseudo-random integer in the range [lo, hi) (inclusive of lo, exclusive of hi).

### `seed(seed_val: Int) -> Void`
Seeds the pseudo-random number generator with the given value.

## Examples

```nimble
load std.random

random.seed(12345)
let r1 = random.random_int()
let r2 = random.random_float()
let r3 = random.random_range(1, 100)
```
