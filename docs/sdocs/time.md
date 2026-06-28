# std.time

Time and clock utilities - epoch timestamps, monotonic clock, nanosecond precision, sleep, and time formatting.

All functions are backed by the Rust runtime.

## Functions

### `now() -> Int`
Returns the current Unix epoch timestamp in seconds.

### `now_nanos() -> Int`
Returns the current Unix epoch timestamp in nanoseconds.

### `monotonic() -> Float`
Returns the value of the monotonic clock in seconds (useful for measuring elapsed time).

### `sleep(ms: Int) -> Void`
Sleeps (blocks) for `ms` milliseconds.

### `format(fmt: String) -> String`
Formats the current local time according to `fmt` (uses C `strftime` format specifiers).

## Examples

```nimble
load std.time

let t = time.now()
let ns = time.now_nanos()
let before = time.monotonic()
time.sleep(500)
let after = time.monotonic()
let elapsed = after - before
let formatted = time.format("%Y-%m-%d %H:%M:%S")
```
