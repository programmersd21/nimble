# json Module

JSON parsing and serialization for maps.

## Functions
- `parse(s str) -> {str: str} | error`: Parses a JSON object into a map. Non-string values are stringified.
- `stringify(data {str: T}) -> str | error`: Serializes a map to JSON.
- `pretty(data {str: T}) -> str | error`: Pretty-prints JSON.

## Example
```nimble
load json
cfg = json.parse('{"env":"dev","port":8080}')?
out(cfg["env"])
```
