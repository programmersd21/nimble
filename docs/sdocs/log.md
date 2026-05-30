# std.log

Logging utilities with severity levels.

## Functions

### `info(message: String) -> Void`
Prints an info-level log message prefixed with `[INFO]`.

### `warn(message: String) -> Void`
Prints a warning-level log message prefixed with `[WARN]`.

### `error(message: String) -> Void`
Prints an error-level log message prefixed with `[ERROR]`.

## Examples

```nimble
load std.log

log.info("System started")
log.warn("Low disk space")
log.error("Connection failed")
```
