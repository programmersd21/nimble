# std.builtin

Built-in utility functions for assertions and debugging.

## Functions

### `assert(condition: Bool, message: String) -> Void`
Evaluates `condition`. If false, prints the assertion failure message.

### `debug(message: String) -> Void`
Prints a debug-prefixed message to standard output.

## Examples

```nimble
load std.builtin

builtin.assert(true, "This will pass")
builtin.debug("Debugging info")
```
