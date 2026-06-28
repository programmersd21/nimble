# Module System Errors

This page documents all error codes in the **Module System** category.

## Summary

Total codes: 27
- 26 error code(s)
- 1 warning code(s)

## Error Codes

---

### N4001: Module not found

- **Severity:** Error
- **Code:** `N4001`

A module referenced in a use/import declaration does not exist in the module search paths. Check the module name and ensure it is properly installed.

---

### N4002: File not found

- **Severity:** Error
- **Code:** `N4002`

A file referenced in a module path does not exist at the expected location.

---

### N4003: Circular module dependency

- **Severity:** Error
- **Code:** `N4003`

Modules form a circular dependency chain. Nimble requires acyclic module dependencies.

---

### N4004: Symbol not exported

- **Severity:** Error
- **Code:** `N4004`

A symbol imported from a module is not publicly exported by that module. Check the module's public API.

---

### N4005: Import cycle detected

- **Severity:** Error
- **Code:** `N4005`

An import graph cycle was detected. Modules must form a directed acyclic graph.

---

### N4006: Ambiguous import

- **Severity:** Error
- **Code:** `N4006`

An imported symbol resolves to multiple definitions across different imported modules. Use a qualified path to disambiguate.

---

### N4007: Shadowed import

- **Severity:** Error
- **Code:** `N4007`

An imported name conflicts with another import. Use an alias or rename one of the imports.

---

### N4008: Wildcard import naming conflict

- **Severity:** Warning
- **Code:** `N4008`

A wildcard import introduces names that conflict with existing definitions or other imports.

---

### N4009: Relative import beyond root

- **Severity:** Error
- **Code:** `N4009`

A relative import (using `super` or `../`) goes beyond the package root. Relative imports cannot escape the package boundary.

---

### N4010: Invalid module name

- **Severity:** Error
- **Code:** `N4010`

A module name contains invalid characters or does not follow naming conventions. Module names must be valid identifiers.

---

### N4011: Module not in search path

- **Severity:** Error
- **Code:** `N4011`

A module file could not be found in any of the configured import search paths.

---

### N4012: Module parse error

- **Severity:** Error
- **Code:** `N4012`

A module file could not be parsed due to syntax errors. The module must contain valid Nimble source code.

---

### N4013: Module type error

- **Severity:** Error
- **Code:** `N4013`

A module file contains type errors. The module must be type-correct before it can be imported.

---

### N4014: Dependency not found

- **Severity:** Error
- **Code:** `N4014`

A package dependency specified in the manifest cannot be resolved. Check that the dependency exists and is accessible.

---

### N4015: Dependency cycle

- **Severity:** Error
- **Code:** `N4015`

Two or more packages depend on each other, creating a cycle. All dependency graphs must be acyclic.

---

### N4016: Dependency version conflict

- **Severity:** Error
- **Code:** `N4016`

Multiple packages require different incompatible versions of the same dependency.

---

### N4017: Broken package structure

- **Severity:** Error
- **Code:** `N4017`

The package directory structure does not follow the expected convention.

---

### N4018: Missing manifest

- **Severity:** Error
- **Code:** `N4018`

The expected manifest file (e.g., nimble.toml) is missing from the package root.

---

### N4019: Invalid manifest format

- **Severity:** Error
- **Code:** `N4019`

The manifest file has an invalid format or structure and cannot be parsed.

---

### N4020: Manifest syntax error

- **Severity:** Error
- **Code:** `N4020`

The manifest file contains a syntax error. Check the manifest syntax.

---

### N4021: Manifest missing required field

- **Severity:** Error
- **Code:** `N4021`

The manifest file is missing a required field. Add the required field.

---

### N4022: Manifest duplicate entry

- **Severity:** Error
- **Code:** `N4022`

The manifest file contains duplicate entries for the same field or dependency.

---

### N4023: Module compiled with different version

- **Severity:** Error
- **Code:** `N4023`

A pre-compiled module was compiled with a different version of the compiler and is incompatible.

---

### N4024: Module compiled with incompatible flags

- **Severity:** Error
- **Code:** `N4024`

A pre-compiled module was compiled with different compiler flags that affect ABI compatibility.

---

### N4025: Module interface mismatch

- **Severity:** Error
- **Code:** `N4025`

The public interface of a compiled module does not match what is expected from its source.

---

### N4026: Recursive module loading

- **Severity:** Error
- **Code:** `N4026`

A module directly or indirectly triggers loading itself, creating infinite recursion.

---

### N4027: Module path too deep

- **Severity:** Error
- **Code:** `N4027`

A module path exceeds the maximum allowed nesting depth. Flatten your module hierarchy.

---

