# std.ffi

Foreign function interface — direct bindings to C standard library functions.

## Functions

### `print_format(fmt: String) -> Int`
Calls C's `printf` with the given format string. Returns the number of characters printed.

### `write_line(s: String) -> Int`
Calls C's `puts` to write a string followed by a newline. Returns a non-negative value on success.

## Examples

```nimble
load std.ffi

ffi.print_format("Hello %s\n", "world")
ffi.write_line("Hello, world!")
```
