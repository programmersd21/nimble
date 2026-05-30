# std.alloc

Low-level memory allocation primitives backed by the Nimble runtime.

## Functions

### `alloc(size: Int) -> String`
Allocates a memory region of `size` bytes and returns a pointer as a string.

### `free(ptr: String, size: Int) -> Void`
Frees a previously allocated memory region at `ptr` of the given `size`.

### `realloc(ptr: String, old_size: Int, new_size: Int) -> String`
Resizes an allocated memory region from `old_size` to `new_size` bytes. Returns a new pointer.

## Examples

```nimble
load std.alloc

let buf = alloc.alloc(1024)
alloc.free(buf, 1024)
```
