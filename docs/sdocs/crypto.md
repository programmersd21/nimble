# std.crypto

Cryptographic and random number utilities — backed by the Rust runtime.

## Functions

### `random_int() -> Int`
Returns a pseudo-random 64-bit integer.

### `random_float() -> Float`
Returns a pseudo-random float in the range [0.0, 1.0).

### `random_range(lo: Int, hi: Int) -> Int`
Returns a pseudo-random integer in [lo, hi).

### `seed(seed_val: Int) -> Void`
Seeds the pseudo-random number generator.

## Examples

```nimble
load std.crypto

crypto.seed(42)
let r = crypto.random_int()
let rf = crypto.random_float()
let rr = crypto.random_range(1, 100)
```
