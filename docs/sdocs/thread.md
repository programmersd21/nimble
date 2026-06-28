# std.thread

Thread creation, management, and parallelism utilities.

All functions are backed by the Rust runtime.

## Types

### `Thread`
A handle to a spawned thread.

```nimble
struct Thread:
    let id: Int = 0
```

## Functions

### `spawn(f: fn() -> Void) -> Thread`
Spawns a new thread executing the given function. Returns a `Thread` handle.

### `thread_join(self: Thread) -> Void`
Waits for the thread to finish execution.

### `sleep(ms: Int) -> Void`
Suspends the current thread for `ms` milliseconds.

### `available_parallelism() -> Int`
Returns the number of logical CPUs available on the system.

## Examples

```nimble
load std.thread

fn worker() -> Void:
    thread.sleep(100)

fn main() -> Int:
    let t = thread.spawn(worker)
    thread.thread_join(t)
    let n = thread.available_parallelism()
    return 0
```
