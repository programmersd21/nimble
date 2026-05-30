# std.async

Asynchronous and concurrency primitives.

## Functions

### `sleep(ms: Int) -> Void`
Suspends execution for `ms` milliseconds.

### `spawn(entry: String) -> Int`
Spawns a new thread executing the function named `entry`. Returns a thread identifier.

### `join(thread: Int) -> Void`
Waits for the thread identified by `thread` to finish execution.

## Examples

```nimble
load std.async

async.sleep(1000)
let t = async.spawn("worker_function")
async.join(t)
```
