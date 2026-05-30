# std.crypto

Cryptographic and random number utilities.

## Functions

### `random_int() -> Int`
Returns a pseudo-random integer.

### `seed(seed: Int) -> Void`
Seeds the pseudo-random number generator with the given value.

## Examples

```nimble
load std.crypto

crypto.seed(42)
let r = crypto.random_int()
```
