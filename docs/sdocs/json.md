# std.json

JSON parsing and serialization.

## Functions

### `parse(json: String) -> String`
Parses a JSON string and returns an internal representation.

### `stringify(value: String) -> String`
Serializes an internal representation back to a JSON string.

## Examples

```nimble
load std.json

let data = json.parse("{\"key\": \"value\"}")
let out = json.stringify(data)
```
