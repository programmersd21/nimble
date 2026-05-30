# std.testing

Assertion helpers and test runner for unit testing.

## Functions

### `assert_eq[T](expected: T, actual: T) -> Void`
Asserts that `expected` equals `actual`. Prints a dot on success, or a failure message and panics on mismatch.

### `assert_true(condition: Bool, message: String) -> Void`
Asserts that `condition` is true. Prints a dot on success, or `message` and panics on failure.

### `assert_ok[T, E](result: Result[T, E]) -> T`
Asserts that `result` is `Ok(val)`, returning the unwrapped value. Prints a dot on success, or a failure message and panics on `Err`.

### `run_test(name: String, test_fn: fn() -> Void) -> Void`
Runs a named test. Prints `Testing <name>: ` before execution and ` OK` after successful completion.

## Examples

```nimble
load std.testing

fn test_addition():
    assert_eq(4, 2 + 2)
    assert_true(1 < 2, "one should be less than two")

fn main() -> Int:
    run_test("addition", test_addition)
    return 0
```
