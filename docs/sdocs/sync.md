# std.sync

Synchronization primitives — atomic operations, mutex, and reference counting.

All functions are backed by the Rust runtime.

## AtomicInt

An integer that can be safely read and modified from multiple threads.

#### `atomic_int(initial: Int) -> AtomicInt`
Creates a new atomic integer with the given initial value.

#### `atomic_load(self: AtomicInt) -> Int`
Atomically loads and returns the current value.

#### `atomic_store(self: AtomicInt, val: Int) -> Void`
Atomically stores `val`.

#### `atomic_add(self: AtomicInt, val: Int) -> Int`
Atomically adds `val` and returns the previous value.

#### `atomic_sub(self: AtomicInt, val: Int) -> Int`
Atomically subtracts `val` and returns the previous value.

#### `atomic_swap(self: AtomicInt, val: Int) -> Int`
Atomically swaps the value with `val` and returns the previous value.

## Mutex

A mutual exclusion primitive for protecting shared data.

#### `new_mutex() -> Mutex`
Creates a new mutex.

#### `mutex_lock(self: Mutex) -> Void`
Locks the mutex. Blocks if already locked by another thread.

#### `mutex_unlock(self: Mutex) -> Void`
Unlocks the mutex.

## Arc (Atomic Reference Counted)

A thread-safe reference-counted pointer for sharing data across threads.

#### `arc_new(value: String) -> Arc`
Creates a new Arc with the given initial value and reference count 1.

#### `arc_clone(self: Arc) -> Arc`
Increments the reference count and returns a new handle to the same data.

#### `arc_drop(self: Arc) -> Void`
Decrements the reference count. Frees the data when the count reaches zero.

## Examples

```nimble
load std.sync

# Atomic integer
let counter = sync.atomic_int(0)
let prev = sync.atomic_add(counter, 1)

# Mutex
let m = sync.new_mutex()
sync.mutex_lock(m)
# critical section
sync.mutex_unlock(m)

# Arc
let shared = sync.arc_new("data")
let c2 = sync.arc_clone(shared)
sync.arc_drop(shared)
sync.arc_drop(c2)
```
