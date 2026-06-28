# std.io

Basic input/output operations — console output, file I/O, and type-to-string conversion.

All functions are backed by the Rust runtime (ember).

## Functions

### `print(s: String) -> Void`
Prints a string followed by a newline.

### `print_no_newline(s: String) -> Void`
Prints a string without a trailing newline.

### `print_int(i: Int) -> Void`
Prints an integer.

### `print_float(f: Float) -> Void`
Prints a floating-point number.

### `print_bool(b: Bool) -> Void`
Prints `true` or `false`.

### `flush() -> Void`
Flushes the stdout buffer.

### `read_line() -> String`
Reads a line from standard input. Returns an empty string on EOF.

### `read_file(path: String) -> String`
Reads the entire contents of a file as a string. Returns an empty string if the file is not found or empty.

### `write_file(path: String, content: String) -> Int`
Writes `content` to `path`. Returns 0 on success, non-zero on failure.

### `append_file(path: String, content: String) -> Int`
Appends `content` to the file at `path`. Returns 0 on success, non-zero on failure.

### `file_exists(path: String) -> Bool`
Returns `true` if the file at `path` exists.

### `file_size(path: String) -> Int`
Returns the size of the file at `path` in bytes. Returns -1 on error.

### `delete_file(path: String) -> Bool`
Deletes the file at `path`. Returns `true` on success.

### `rename_file(old: String, new: String) -> Bool`
Renames/moves a file from `old` to `new`. Returns `true` on success.

### `to_string(val: Int) -> String`
Converts an integer to its string representation.

### `float_to_string(val: Float) -> String`
Converts a float to its string representation.

### `bool_to_string(val: Bool) -> String`
Converts a boolean to `"true"` or `"false"`.

## Examples

```nimble
load std.io

io.print("hello, world")
io.print_no_newline("count: ")
io.print_int(42)
io.print_float(3.14)
io.print_bool(true)

let content = io.read_file("data.txt")
let ok = io.write_file("out.txt", "data")
let line = io.read_line()
let s = io.to_string(42)
```
