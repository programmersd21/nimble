# string Module

String manipulation utilities.

## Functions
- `split(s str, delim str) -> [str]`: Splits `s` by `delim`.
- `join(parts [str], sep str) -> str`: Joins `parts` with `sep`.
- `trim(s str) -> str`: Removes leading/trailing whitespace.
- `trim_start(s str) -> str`: Removes leading whitespace.
- `trim_end(s str) -> str`: Removes trailing whitespace.
- `upper(s str) -> str`: Converts to uppercase.
- `lower(s str) -> str`: Converts to lowercase.
- `contains(s str, sub str) -> bool`: Returns true if `sub` is in `s`.
- `starts_with(s str, prefix str) -> bool`: Returns true if `s` starts with `prefix`.
- `ends_with(s str, suffix str) -> bool`: Returns true if `s` ends with `suffix`.
- `replace(s str, old str, new str) -> str`: Replaces first occurrence.
- `replace_all(s str, old str, new str) -> str`: Replaces all occurrences.
- `count(s str, sub str) -> int`: Number of occurrences.
- `index_of(s str, sub str) -> int`: Index of `sub` or `-1`.
- `slice(s str, start int, end int) -> str`: Returns substring.
- `repeat(s str, n int) -> str`: Repeats `s` `n` times.
- `pad_left(s str, width int, char str) -> str`: Left pad to `width`.
- `pad_right(s str, width int, char str) -> str`: Right pad to `width`.
- `to_int(s str) -> int | error`: Parses `s` to int.
- `to_float(s str) -> float | error`: Parses `s` to float.
- `from_int(n int) -> str`: Converts int to string.
- `from_float(f float) -> str`: Converts float to string.
- `chars(s str) -> [str]`: List of characters.
- `len(s str) -> int`: String length.
- `is_empty(s str) -> bool`: Returns true if empty.
- `is_numeric(s str) -> bool`: Returns true if all digits.
- `is_alpha(s str) -> bool`: Returns true if all letters.
- `format(template str, args [str]) -> str`: Replaces `{}` placeholders with args.

## Example
```nimble
load string
out(string.split("a,b,c", ","))
```
