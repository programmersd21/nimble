# std.collections

Generic dynamic array (`Vec[T]`) and hash map (`HashMap[K, V]`) data structures.

## Types

### `Vec[T]`
A growable, heap-allocated array.

```nimble
struct Vec[T]:
    let data: String
    let len: Int
    let cap: Int
```

### `HashMap[K, V]`
A hash map with generic key and value types.

```nimble
struct HashMap[K, V]:
    let keys: String
    let values: String
    let len: Int
    let cap: Int
```

## Functions

### `new_vec[T]() -> Vec[T]`
Creates a new empty vector.

### `push[T](self: Vec[T], val: T) -> Vec[T]`
Appends `val` to the end of the vector. Automatically grows the capacity.

### `get[T](self: Vec[T], index: Int) -> Option[T]`
Retrieves the element at `index`. Returns `Some(val)` if the index is valid, `None` otherwise.

### `len[T](self: Vec[T]) -> Int`
Returns the number of elements in the vector.

### `is_empty[T](self: Vec[T]) -> Bool`
Returns `true` if the vector contains no elements.

### `new_map[K, V]() -> HashMap[K, V]`
Creates a new empty hash map.

## Examples

```nimble
load std.collections

let vec = collections.new_vec[Int]()
let vec = collections.push(vec, 42)
let val = collections.get(vec, 0)  # Some(42)

let map = collections.new_map[String, Int]()
```
