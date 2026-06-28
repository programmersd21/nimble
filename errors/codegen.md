# Codegen Errors

This page documents all error codes in the **Codegen** category.

## Summary

Total codes: 34
- 16 error code(s)
- 18 bug code(s)

## Error Codes

---

### N6001: Code generation failure

- **Severity:** Bug
- **Code:** `N6001`

The code generator encountered an internal error while generating the output. This is a compiler bug. Report it to the Nimble developers with the source code that triggered it.

---

### N6002: Unsupported feature for target

- **Severity:** Bug
- **Code:** `N6002`

A language feature or construct is not supported on the selected code generation target (e.g., architecture, OS).

---

### N6003: Linker error

- **Severity:** Bug
- **Code:** `N6003`

The linker returned an error while linking the compiled output. Check for missing symbols or library paths.

---

### N6004: Assembly error

- **Severity:** Bug
- **Code:** `N6004`

An error occurred during assembly generation. This typically indicates a compiler bug.

---

### N6005: Target not supported

- **Severity:** Bug
- **Code:** `N6005`

The specified compilation target is not supported by the code generator.

---

### N6006: Invalid optimization level

- **Severity:** Error
- **Code:** `N6006`

The specified optimization level is not recognized. Valid levels are 0-3, s, or z.

---

### N6007: Invalid debug info level

- **Severity:** Error
- **Code:** `N6007`

The specified debug information level is invalid.

---

### N6008: Inline assembly error

- **Severity:** Bug
- **Code:** `N6008`

An inline assembly block contains invalid syntax or constraints. Check the assembly template.

---

### N6009: Compiler intrinsic error

- **Severity:** Bug
- **Code:** `N6009`

A compiler intrinsic function was used incorrectly or has an invalid signature.

---

### N6010: Stack overflow during codegen

- **Severity:** Bug
- **Code:** `N6010`

The code generator's internal stack overflowed while processing a deeply nested construct. Try simplifying the code.

---

### N6011: Global offset overflow

- **Severity:** Bug
- **Code:** `N6011`

The global offset table for the generated code exceeds the maximum allowed size. Reduce the number of global variables or functions.

---

### N6012: Jump table overflow

- **Severity:** Bug
- **Code:** `N6012`

A generated jump table (for match/large switches) exceeds the maximum size. Use if-else chains instead.

---

### N6013: Too many static variables

- **Severity:** Bug
- **Code:** `N6013`

The number of static variables in the compilation unit exceeds the target limit. Reduce static variable usage.

---

### N6014: Too many functions

- **Severity:** Bug
- **Code:** `N6014`

The number of functions in the compilation unit exceeds the target module limit. Split the code into multiple files.

---

### N6015: Function too large

- **Severity:** Bug
- **Code:** `N6015`

A single function exceeds the maximum size that the code generator can handle. Split the function into smaller functions.

---

### N6016: External symbol not found

- **Severity:** Error
- **Code:** `N6016`

A symbol referenced as external (e.g., from a different compilation unit or library) was not found during linking.

---

### N6017: Duplicate symbol export

- **Severity:** Error
- **Code:** `N6017`

Two compilation units export the same symbol. Rename one of the symbols.

---

### N6018: Undefined symbol

- **Severity:** Error
- **Code:** `N6018`

A symbol referenced in the code has no definition anywhere in the compilation unit or its dependencies.

---

### N6019: Relocation overflow

- **Severity:** Bug
- **Code:** `N6019`

A relocation (address fixup) in the generated object code exceeds the maximum range for the target architecture.

---

### N6020: TLS not supported

- **Severity:** Bug
- **Code:** `N6020`

Thread-local storage is not available on the target platform. Remove the `thread_local` annotation.

---

### N6021: ABI mismatch

- **Severity:** Error
- **Code:** `N6021`

The calling convention or ABI of an external function does not match the expected ABI. Ensure consistent ABI declarations.

---

### N6022: CPU feature not available

- **Severity:** Error
- **Code:** `N6022`

A required CPU feature (e.g., AVX2, SSE4.1) is not available on the target platform.

---

### N6023: OS feature not available

- **Severity:** Error
- **Code:** `N6023`

A required operating system feature is not available on the target platform.

---

### N6024: Inline assembly constraint violation

- **Severity:** Error
- **Code:** `N6024`

An inline assembly constraint is invalid or incompatible with the operands.

---

### N6025: Intrinsic signature mismatch

- **Severity:** Bug
- **Code:** `N6025`

A compiler intrinsic is called with arguments that do not match its expected signature.

---

### N6026: Vector type not supported

- **Severity:** Error
- **Code:** `N6026`

The target does not support the required SIMD vector type. Use a scalar type or check target features.

---

### N6027: Atomic not supported

- **Severity:** Error
- **Code:** `N6027`

Atomic operations are not supported on the target platform for the specified data type.

---

### N6028: SIMD not supported

- **Severity:** Error
- **Code:** `N6028`

SIMD operations are not supported on the target platform. Disable SIMD features or use a different target.

---

### N6029: Codegen buffer overflow

- **Severity:** Bug
- **Code:** `N6029`

An internal code generator buffer overflowed. This is a compiler bug.

---

### N6030: Unsupported calling convention

- **Severity:** Error
- **Code:** `N6030`

The specified calling convention is not supported on the target platform.

---

### N6031: Too many locals

- **Severity:** Bug
- **Code:** `N6031`

A function has too many local variables for the code generator to handle. Split the function.

---

### N6032: Section attribute conflict

- **Severity:** Error
- **Code:** `N6032`

Two or more items in the same section have conflicting attributes.

---

### N6033: Link once group conflict

- **Severity:** Error
- **Code:** `N6033`

Two or more items in the same link_once group have conflicting definitions.

---

### N6034: Visibility attribute conflict

- **Severity:** Error
- **Code:** `N6034`

Two or more symbols with the same name but different visibility are defined.

---

