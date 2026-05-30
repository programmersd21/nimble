# std.sync

Mutex-based synchronization primitives for thread safety.

## Functions

### `mutex_new() -> Int`
Creates a new mutex and returns its handle.

### `mutex_lock(mutex: Int) -> Void`
Locks the mutex. Blocks if the mutex is already locked by another thread.

### `mutex_unlock(mutex: Int) -> Void`
Unlocks the mutex.

## Examples

```nimble
load std.sync

let m = sync.mutex_new()
sync.mutex_lock(m)
# critical section
sync.mutex_unlock(m)
```
