# std.thread

Low-level thread creation and management.

## Functions

### `create(entry: String) -> Int`
Creates a new thread that executes the function named `entry`. Returns a thread identifier.

### `join(thread: Int) -> Void`
Waits for the given thread to finish execution.

## Examples

```nimble
load std.thread

let t = thread.create("worker")
thread.join(t)
```
