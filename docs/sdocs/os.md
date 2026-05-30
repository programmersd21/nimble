# std.os

Operating system interface for environment and command execution.

## Functions

### `get_env(name: String) -> String`
Retrieves the value of the environment variable `name`. Returns an empty string if not set.

### `execute(command: String) -> Int`
Executes `command` via the system shell. Returns the exit status.

## Examples

```nimble
load std.os

let path = os.get_env("PATH")
let code = os.execute("echo hello")
```
