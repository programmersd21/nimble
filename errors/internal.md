# Internal Errors

This page documents all error codes in the **Internal** category.

## Summary

Total codes: 25
- 25 bug code(s)

## Error Codes

---

### N9001: Internal compiler error

- **Severity:** Bug
- **Code:** `N9001`

The compiler encountered an unexpected internal error. This is a compiler bug. Please report it to the Nimble development team with a minimal reproduction of the source code.

---

### N9002: Internal bug - please report

- **Severity:** Bug
- **Code:** `N9002`

The compiler detected a condition that should never occur during correct compilation. This is a bug. Please file a bug report.

---

### N9003: Unreachable code path in compiler

- **Severity:** Bug
- **Code:** `N9003`

The compiler reached a code path that should be unreachable. This indicates a bug in the compiler's logic.

---

### N9004: Unimplemented compiler feature

- **Severity:** Bug
- **Code:** `N9004`

The compiler encountered a construct that is not yet implemented. This feature may be added in a future release.

---

### N9005: Compiler assertion failure

- **Severity:** Bug
- **Code:** `N9005`

An internal compiler assertion failed. This indicates a bug in the compiler that should be reported.

---

### N9006: Compiler invariant violation

- **Severity:** Bug
- **Code:** `N9006`

An internal compiler invariant was violated. This indicates a bug in the compiler that should be reported.

---

### N9007: Type checker invariant failure

- **Severity:** Bug
- **Code:** `N9007`

The type checker encountered an inconsistent internal state. This is a compiler bug. Report it with a reproduction case.

---

### N9008: Name resolution invariant failure

- **Severity:** Bug
- **Code:** `N9008`

The name resolution pass encountered an inconsistent internal state. This is a compiler bug.

---

### N9009: Codegen invariant failure

- **Severity:** Bug
- **Code:** `N9009`

The code generator encountered an inconsistent internal state. This is a compiler bug.

---

### N9010: Compiler data structure corruption

- **Severity:** Bug
- **Code:** `N9010`

An internal compiler data structure has been corrupted. This is a serious compiler bug.

---

### N9011: Missing compiler pass

- **Severity:** Bug
- **Code:** `N9011`

A required compiler analysis or transformation pass is missing from the compilation pipeline.

---

### N9012: Compiler pass cycle

- **Severity:** Bug
- **Code:** `N9012`

Compiler passes form a dependency cycle. This is a compiler architecture bug.

---

### N9013: Compiler query cycle

- **Severity:** Bug
- **Code:** `N9013`

Compiler queries formed a dependency cycle during incremental compilation.

---

### N9014: Incremental cache mismatch

- **Severity:** Bug
- **Code:** `N9014`

The incremental compilation cache is inconsistent with the current source code. A clean rebuild may be needed.

---

### N9015: Incremental fingerprint conflict

- **Severity:** Bug
- **Code:** `N9015`

Two source files had the same fingerprint, causing a cache conflict. This is a compiler bug.

---

### N9016: AST validation failure

- **Severity:** Bug
- **Code:** `N9016`

The Abstract Syntax Tree (AST) failed validation checks. This indicates a bug in the parser or tree construction.

---

### N9017: HIR validation failure

- **Severity:** Bug
- **Code:** `N9017`

The High-level Intermediate Representation (HIR) failed validation. This indicates a bug in lowering or analysis passes.

---

### N9018: MIR validation failure

- **Severity:** Bug
- **Code:** `N9018`

The Mid-level Intermediate Representation (MIR) failed validation. This indicates a bug in a transformation or optimization pass.

---

### N9019: LLVM / backend error

- **Severity:** Bug
- **Code:** `N9019`

The LLVM backend or alternative backend returned an unexpected error. This may be a compiler compatibility issue.

---

### N9020: Compiler memory allocation failure

- **Severity:** Bug
- **Code:** `N9020`

The compiler could not allocate required memory. Try building with more RAM or reducing parallel compilation jobs.

---

### N9021: Thread panic in compiler worker

- **Severity:** Bug
- **Code:** `N9021`

A compiler worker thread panicked. This is a compiler bug. Try re-running the compilation.

---

### N9022: Compiler resource limit exceeded

- **Severity:** Bug
- **Code:** `N9022`

The compiler exceeded a resource limit (e.g., too many types, too many files). Try splitting the project.

---

### N9023: Compiler I/O error

- **Severity:** Bug
- **Code:** `N9023`

The compiler encountered an I/O error while reading or writing files. Check disk space, permissions, and path validity.

---

### N9024: Compiler timeout

- **Severity:** Bug
- **Code:** `N9024`

The compilation exceeded the maximum allowed time. The code may be too large or the compiler may be stuck in a loop.

---

### N9025: Invalid incremental compilation state

- **Severity:** Bug
- **Code:** `N9025`

The incremental compilation cache was in an invalid state. Perform a clean build to resolve this.

---

