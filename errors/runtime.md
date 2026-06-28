# Runtime Errors

This page documents all error codes in the **Runtime** category.

## Summary

Total codes: 29
- 1 note code(s)
- 28 bug code(s)

## Error Codes

---

### N7001: Program panicked

- **Severity:** Bug
- **Code:** `N7001`

The program encountered an unexpected condition and panicked. This is a runtime error that terminates the process. Check the panic message for details about what went wrong.

---

### N7002: Index out of bounds

- **Severity:** Bug
- **Code:** `N7002`

An array, list, or tuple index was outside the valid range. Ensure the index is less than the container's length and non-negative.

---

### N7003: Stack overflow

- **Severity:** Bug
- **Code:** `N7003`

The program's call stack exceeded the maximum allowed size. This usually indicates infinite recursion or excessive stack allocation.

---

### N7004: Arithmetic overflow

- **Severity:** Bug
- **Code:** `N7004`

An arithmetic operation overflowed the range of the integer type. Use checked arithmetic or larger types.

---

### N7005: Division by zero

- **Severity:** Bug
- **Code:** `N7005`

A division or modulo operation had a zero divisor at runtime. Ensure the divisor is non-zero before the operation.

---

### N7006: Null pointer dereference

- **Severity:** Bug
- **Code:** `N7006`

The program tried to dereference a null or invalid pointer. This is a serious bug.

---

### N7007: Unwrap of None value

- **Severity:** Bug
- **Code:** `N7007`

A `None` value was unwrapped, causing a panic. Use pattern matching or safe unwrapping methods to handle the None case.

---

### N7008: Unwrap of error result

- **Severity:** Bug
- **Code:** `N7008`

An `Err` result was unwrapped, causing a panic. Handle errors with pattern matching or propagate them properly.

---

### N7009: Out of memory

- **Severity:** Bug
- **Code:** `N7009`

The program could not allocate the required memory. This is typically a system resource exhaustion issue.

---

### N7010: Assertion failed

- **Severity:** Bug
- **Code:** `N7010`

A runtime assertion (`assert`) failed. The condition evaluated to false. Check the assertion condition and the program logic.

---

### N7011: Unreachable code executed

- **Severity:** Bug
- **Code:** `N7011`

Code marked as unreachable was executed, indicating a logic error in the program.

---

### N7012: TODO encountered at runtime

- **Severity:** Note
- **Code:** `N7012`

A TODO stub was executed at runtime. This indicates incomplete implementation.

---

### N7013: Unimplemented functionality

- **Severity:** Bug
- **Code:** `N7013`

The program reached a code path that is not yet implemented.

---

### N7014: Buffer overflow

- **Severity:** Bug
- **Code:** `N7014`

A buffer write operation exceeded the buffer's capacity. This can be a security vulnerability.

---

### N7015: Invalid UTF-8 sequence

- **Severity:** Bug
- **Code:** `N7015`

An operation encountered an invalid UTF-8 byte sequence. Ensure all string data is valid UTF-8.

---

### N7016: Integer conversion overflow

- **Severity:** Bug
- **Code:** `N7016`

An integer type conversion resulted in an overflow. Use checked conversion or ensure the value fits in the target type.

---

### N7017: Float conversion overflow

- **Severity:** Bug
- **Code:** `N7017`

A floating-point conversion resulted in an overflow or underflow.

---

### N7018: Negative index

- **Severity:** Bug
- **Code:** `N7018`

A negative index was used in a context that only accepts non-negative indices.

---

### N7019: Invalid enum discriminant

- **Severity:** Bug
- **Code:** `N7019`

An enum value has an invalid discriminant value, indicating memory corruption or unsafe code violation.

---

### N7020: Type cast error at runtime

- **Severity:** Bug
- **Code:** `N7020`

A runtime type cast failed because the value's actual type does not match the target type.

---

### N7021: Recursive call overflow

- **Severity:** Bug
- **Code:** `N7021`

The recursion depth exceeded the maximum allowed limit. Use iteration instead of recursion or increase the recursion limit.

---

### N7022: Invalid allocator state

- **Severity:** Bug
- **Code:** `N7022`

The memory allocator detected an inconsistent internal state. This is a serious bug.

---

### N7023: Double free detected

- **Severity:** Bug
- **Code:** `N7023`

A memory deallocation was attempted twice on the same allocation. This indicates a memory management bug.

---

### N7024: Use after free

- **Severity:** Bug
- **Code:** `N7024`

Memory was accessed after it was freed. This indicates a memory management bug.

---

### N7025: Mutex poison

- **Severity:** Bug
- **Code:** `N7025`

A mutex is in a poisoned state because a previous lock holder panicked while holding the lock.

---

### N7026: Channel closed

- **Severity:** Bug
- **Code:** `N7026`

A channel operation failed because the channel was closed. Check if the channel is still open before operating.

---

### N7027: Timeout

- **Severity:** Bug
- **Code:** `N7027`

An operation timed out before completing. Increase the timeout or optimize the operation.

---

### N7028: IO error

- **Severity:** Bug
- **Code:** `N7028`

An input/output operation failed. Check the file system, permissions, and device availability.

---

### N7029: Network error

- **Severity:** Bug
- **Code:** `N7029`

A network operation failed. Check network connectivity, address correctness, and firewall settings.

---

