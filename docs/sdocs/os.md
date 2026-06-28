# std.os

Operating system interaction — environment variables, hostname, OS information, and CPU count.

All functions are backed by the Rust runtime.

## Functions

### `get_env(name: String) -> String`
Retrieves the value of the environment variable `name`. Returns an empty string if not set.

### `set_env(name: String, value: String) -> Bool`
Sets the environment variable `name` to `value`. Returns `true` on success.

### `hostname() -> String`
Returns the system hostname.

### `os_name() -> String`
Returns the operating system name (e.g., "linux", "macos", "windows").

### `cpu_count() -> Int`
Returns the number of logical CPUs available on the system.

## Examples

```nimble
load std.os

let path = os.get_env("PATH")
let ok = os.set_env("MY_VAR", "value")
let host = os.hostname()
let osn = os.os_name()
let cpus = os.cpu_count()
```
