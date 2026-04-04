# io Module

Standard input/output and file utilities.

## Functions
- `read_file(path str) -> str | error`: Reads entire file content.
- `write_file(path str, content str) -> error | null`: Overwrites file with content.
- `append_file(path str, content str) -> error | null`: Appends content to file.
- `delete_file(path str) -> error | null`: Deletes a file.
- `read_lines(path str) -> [str] | error`: Reads file into a list of lines.
- `write_lines(path str, lines [str]) -> error | null`: Writes lines to a file, joined by `\n`.
- `read_bytes(path str) -> [int] | error`: Reads file bytes as integers `0..255`.
- `write_bytes(path str, data [int]) -> error | null`: Writes bytes from integers `0..255`.
- `copy_file(src str, dst str) -> error | null`: Copies a file.
- `exists(path str) -> bool`: Returns true if the path exists.
- `stdin() -> str`: Reads a line from stdin.
- `stdout(data str)`: Writes to stdout with newline.
- `stderr(data str)`: Writes to stderr with newline.

## Example
```nimble
load io
if io.exists("data.txt"):
    out(io.read_file("data.txt")?)
```
