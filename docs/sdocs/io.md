# std.io

Basic input/output operations.

## Functions

### `print(text: String)`
Prints a string followed by a newline.

### `print_int(val: Int)`
Prints an integer.

### `print_str(text: String)`
Prints a string without a newline.

### `println(s: String)`
Alias for `print`.

### `print_no_newline(s: String)`
Alias for `print_str`.

### `print_int_val(i: Int)`
Alias for `print_int`.

### `read_file(path: String) -> Result[String, String]`
Reads the contents of a file at the given path. Returns `Ok(content)` on success, `Err(message)` if the file is not found or empty.

### `write_file(path: String, content: String) -> Result[Int, String]`
Writes `content` to the file at `path`. Returns `Ok(result)` on success, `Err(message)` on failure.

### `read_line() -> Result[String, String]`
Reads a line from standard input. Returns `Ok(line)` on success, `Err(message)` if no input is available.

## Examples

```nimble
load std.io

io.println("hello, world")
io.print_no_newline("count: ")
io.print_int_val(42)

let content = io.read_file("data.txt")?
let ok = io.write_file("out.txt", "data")?
let line = io.read_line()?
```
