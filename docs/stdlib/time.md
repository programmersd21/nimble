# time Module

Time helpers.

## Functions
- `now() -> int`: Returns Unix epoch time in milliseconds.
- `sleep(ms int)`: Sleeps for `ms` milliseconds.

## Example
```nimble
load time
start = time.now()
time.sleep(100)
out(time.now() - start)
```
