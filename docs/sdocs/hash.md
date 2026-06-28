# std.hash

Hash function implementations - FNV-1a, SipHash, and xxHash3.

All operations are backed by the Rust runtime.

## Functions

### `fnv1a(data: String) -> Int`
Computes the 64-bit FNV-1a hash of `data`.

### `sip(data: String) -> Int`
Computes the 64-bit SipHash (SipHash-1-3) of `data`.

### `xxhash3(data: String) -> Int`
Computes the 64-bit xxHash3 hash of `data`.

## Examples

```nimble
load std.hash

let data = "hello world"
let h1 = hash.fnv1a(data)
let h2 = hash.sip(data)
let h3 = hash.xxhash3(data)
```
