# std.process

Process control and exit handling — backed by the Rust runtime.

## Functions

### `terminate(code: Int) -> Void`
Terminates the current process with the given exit `code`.

### `exit_success() -> Void`
Terminates the current process with exit code 0 (success).

### `exit_failure() -> Void`
Terminates the current process with exit code 1 (failure).

## Examples

```nimble
load std.process

process.exit_success()
```
