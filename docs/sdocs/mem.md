# std.mem

Higher-level memory management utilities built on top of `std.alloc`.

## Functions

### `alloc_bytes(size: Int) -> String`
Allocates `size` bytes of memory. Delegates to `std.alloc.alloc`.

### `free_bytes(ptr: String, size: Int) -> Void`
Frees a memory region at `ptr` of the given `size`. Delegates to `std.alloc.free`.

### `resize_bytes(ptr: String, old_size: Int, new_size: Int) -> String`
Resizes a memory region from `old_size` to `new_size`. Delegates to `std.alloc.realloc`.

## Examples

```nimble
load std.mem

let buf = mem.alloc_bytes(256)
mem.free_bytes(buf, 256)
```
