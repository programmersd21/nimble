# process Module

Shell command helpers.

## Functions
- `run(cmd str) -> str | error`: Runs a shell command and returns stdout. Errors on non-zero exit.

## Example
```nimble
load process
out(process.run("echo hello")?)
```
