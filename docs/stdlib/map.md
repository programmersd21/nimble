# map Module

Map helpers.

## Functions
- `has(m {str: T}, key str) -> bool`: Returns true if `key` exists.
- `keys(m {str: T}) -> [str]`: Returns list of keys.
- `values(m {str: T}) -> [T]`: Returns list of values.

## Example
```nimble
load map
cfg = {"env": "dev"}
if map.has(cfg, "env"):
    out(cfg["env"])
```
