# regex Module

Regular expression helpers.

## Functions
- `matches(pattern str, s str) -> bool`: Returns true if `pattern` matches `s`.
- `find(pattern str, s str) -> str | error`: Returns first match.
- `find_all(pattern str, s str) -> [str] | error`: Returns all matches.
- `replace(pattern str, s str, replacement str) -> str | error`: Replaces first match.
- `replace_all(pattern str, s str, replacement str) -> str | error`: Replaces all matches.
- `split(pattern str, s str) -> [str] | error`: Splits `s` by regex.

## Example
```nimble
load regex
out(regex.matches("[a-z]+", "abc"))
```
