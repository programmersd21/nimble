# std.encoding

Encoding and decoding utilities — base64, hex, and UTF-8 operations.

The module re-exports three sub-modules:

- `std.encoding.base64` — Base64 encoding/decoding (standard and URL-safe)
- `std.encoding.hex` — Hexadecimal encoding/decoding (upper and lower case)
- `std.encoding.utf8` — UTF-8 validation

All operations are backed by the Rust runtime.

## Sub-modules

### base64

#### `encode(data: String) -> String`
Encodes `data` to standard Base64.

#### `encode_url_safe(data: String) -> String`
Encodes `data` to URL-safe Base64 (uses `-` and `_` instead of `+` and `/`).

#### `decode(data: String) -> String`
Decodes a Base64-encoded string back to the original data.

```nimble
load std.encoding.base64

let enc = base64.encode("hello world")
let dec = base64.decode(enc)
```

### hex

#### `encode(data: String) -> String`
Encodes `data` to a lowercase hexadecimal string.

#### `encode_upper(data: String) -> String`
Encodes `data` to an uppercase hexadecimal string.

#### `decode(hex: String) -> String`
Decodes a hexadecimal string back to the original data.

```nimble
load std.encoding.hex

let enc = hex.encode("hello")
let upp = hex.encode_upper("hello")
let dec = hex.decode(enc)
```

### utf8

#### `validate(data: String) -> Bool`
Returns `true` if `data` is valid UTF-8.

#### `len(s: String) -> Int`
Returns the byte length of string `s`.

```nimble
load std.encoding.utf8

let ok = utf8.validate("hello")
let n = utf8.len("hello")
```
