# std.text

Text processing utilities — pattern matching and search operations.

All functions are backed by the Rust runtime.

## Functions

### `contains(s: String, pattern: String) -> Bool`
Returns `true` if `s` contains the substring `pattern`.

### `replace_all(s: String, from: String, to: String) -> String`
Replaces all occurrences of `from` with `to` in `s`.

## Examples

```nimble
load std.text

let s = "The quick brown fox"
if text.contains(s, "fox"):
    let r = text.replace_all(s, "fox", "dog")
```
