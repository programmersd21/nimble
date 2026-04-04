# os Module

Process-level helpers.

## Functions
- `args() -> [str]`: Returns command-line arguments (excluding the executable).
- `exit(code int)`: Exits the process with `code`.

## Example
```nimble
load os
if len(os.args()) == 0:
    out("missing arg")
    os.exit(1)
```
