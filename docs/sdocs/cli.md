# std.cli

Command-line interface utilities - argument parsing, terminal detection, and terminal size queries.

All functions are backed by the Rust runtime.

## Functions

### `arg_count() -> Int`
Returns the number of command-line arguments passed to the program (including the program name).

### `arg_get(index: Int) -> String`
Returns the command-line argument at `index`. Index 0 is the program name.

### `is_terminal() -> Bool`
Returns `true` if stdout is connected to a terminal (TTY).

### `terminal_width() -> Int`
Returns the width (in columns) of the terminal. Returns 0 if not a terminal.

### `terminal_height() -> Int`
Returns the height (in rows) of the terminal. Returns 0 if not a terminal.

## Examples

```nimble
load std.cli

fn main() -> Int:
    let argc = cli.arg_count()
    if argc > 1:
        let name = cli.arg_get(0)
        let arg1 = cli.arg_get(1)
    if cli.is_terminal():
        let w = cli.terminal_width()
        let h = cli.terminal_height()
    return 0
```
