# std.fs

File system operations — file and directory manipulation.

All functions are backed by the Rust runtime.

## Functions

### `read(path: String) -> String`
Reads the entire content of a file. Returns an empty string on error.

### `write(path: String, content: String) -> Bool`
Writes `content` to `path`. Returns `true` on success.

### `append(path: String, content: String) -> Bool`
Appends `content` to the file at `path`. Returns `true` on success.

### `exists(path: String) -> Bool`
Returns `true` if the file or directory at `path` exists.

### `size(path: String) -> Int`
Returns the size of the file in bytes. Returns -1 on error.

### `delete(path: String) -> Bool`
Deletes the file at `path`. Returns `true` on success.

### `rename(old: String, new: String) -> Bool`
Renames/moves a file from `old` to `new`. Returns `true` on success.

### `create_dir(path: String) -> Bool`
Creates a directory at `path` (all parent directories are created). Returns `true` on success.

### `remove_dir(path: String) -> Bool`
Removes an empty directory at `path`. Returns `true` on success.

### `list_dir(path: String) -> String`
Lists the contents of a directory. Returns newline-separated entries.

### `current_dir() -> String`
Returns the current working directory path.

## Examples

```nimble
load std.fs

let content = fs.read("data.txt")
let ok = fs.write("out.txt", "data")
let exists = fs.exists("data.txt")
let entries = fs.list_dir(".")
let cwd = fs.current_dir()
```
