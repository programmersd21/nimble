# path Module

Path helpers.

## Functions
- `join(parts [str]) -> str`: Joins path segments with the platform separator.

## Example
```nimble
load path
p = path.join(["a", "b", "c.txt"])
out(p)
```
