# std.random

Pseudo-random number generation.

## Functions

### `random_int() -> Int`
Returns a pseudo-random integer.

### `seed(seed: Int) -> Void`
Seeds the pseudo-random number generator with the given value.

## Examples

```nimble
load std.random

random.seed(12345)
let r = random.random_int()
```
