# std.str

String manipulation operations - length, concatenation, search, trimming, case conversion, replacement, splitting, and parsing.

All operations are backed by the Rust runtime.

## Functions

### `length(s: String) -> Int`
Returns the byte length of string `s`.

### `concat(a: String, b: String) -> String`
Returns a new string that concatenates `a` and `b`.

### `eq(a: String, b: String) -> Bool`
Returns `true` if `a` and `b` are equal.

### `substring(s: String, start: Int, end: Int) -> String`
Returns the substring from byte index `start` (inclusive) to `end` (exclusive).

### `find(s: String, needle: String) -> Int`
Returns the byte index of the first occurrence of `needle` in `s`, or -1 if not found.

### `trim(s: String) -> String`
Returns `s` with leading and trailing whitespace removed.

### `to_upper(s: String) -> String`
Returns `s` converted to uppercase.

### `to_lower(s: String) -> String`
Returns `s` converted to lowercase.

### `starts_with(s: String, prefix: String) -> Bool`
Returns `true` if `s` starts with `prefix`.

### `ends_with(s: String, suffix: String) -> Bool`
Returns `true` if `s` ends with `suffix`.

### `replace(s: String, from: String, to: String) -> String`
Replaces all occurrences of `from` with `to` in `s`.

### `repeat(s: String, count: Int) -> String`
Returns `s` repeated `count` times.

### `to_int(s: String) -> Int`
Parses `s` as a signed decimal integer, returning 0 on failure.

### `to_float(s: String) -> Float`
Parses `s` as a floating-point number, returning 0.0 on failure.

### `is_empty(s: String) -> Bool`
Returns `true` if `s` has zero length.

## Examples

```nimble
load std.str

let s = "Hello, Nimble!"
let n = str.length(s)             # 14
let c = str.concat(s, " World")   # "Hello, Nimble! World"
let sub = str.substring(s, 0, 5)  # "Hello"
let pos = str.find(s, "Nimble")   # 7
let t = str.trim("  spaced  ")    # "spaced"
let u = str.to_upper(s)           # "HELLO, NIMBLE!"
let r = str.replace(s, "!", "?")  # "Hello, Nimble?"
let rep = str.repeat("ha", 3)     # "hahaha"
let i = str.to_int("42")          # 42
```
