# std.reflect

Runtime type reflection utilities.

## Functions

### `type_name(value: String) -> String`
Returns the type name of the given value as a string.

### `size_of(type_name: String) -> Int`
Returns the size in bytes of the type identified by `type_name`.

## Examples

```nimble
load std.reflect

let t = reflect.type_name("hello")
let s = reflect.size_of("Int")
```
