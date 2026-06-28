# Name Resolution Errors

This page documents all error codes in the **Name Resolution** category.

## Summary

Total codes: 49
- 41 error code(s)
- 8 warning code(s)

## Error Codes

---

### N2001: Undefined variable

- **Severity:** Error
- **Code:** `N2001`

A variable name is used that has not been defined in the current scope. Check the spelling and ensure the variable is declared with `let` or `var` before use.

---

### N2002: Duplicate definition

- **Severity:** Error
- **Code:** `N2002`

A name is defined more than once in the same scope. Every variable, function, type, and module name must be unique within its scope.

---

### N2003: Undefined function

- **Severity:** Error
- **Code:** `N2003`

A function call references a function name that has not been defined in any accessible scope.

---

### N2004: Undefined struct

- **Severity:** Error
- **Code:** `N2004`

A struct type name is used but no matching struct definition is found.

---

### N2005: Undefined interface

- **Severity:** Error
- **Code:** `N2005`

An interface name is used but no matching interface definition is found.

---

### N2006: Undefined module

- **Severity:** Error
- **Code:** `N2006`

A module name in an import path cannot be found in the module search path.

---

### N2007: Undefined type

- **Severity:** Error
- **Code:** `N2007`

A type name is used that has not been defined. This includes built-in types and user-defined types.

---

### N2008: Undefined macro

- **Severity:** Error
- **Code:** `N2008`

A macro invocation references a macro name that has not been defined.

---

### N2009: Access to private item

- **Severity:** Error
- **Code:** `N2009`

A private item (function, type, field) from another module is being accessed outside of its defining module.

---

### N2010: Cyclic module dependency

- **Severity:** Error
- **Code:** `N2010`

Two or more modules depend on each other directly or indirectly, forming a cycle. Nimble does not support circular dependencies.

---

### N2011: Cyclic type definition

- **Severity:** Error
- **Code:** `N2011`

A type definition refers to itself directly or indirectly in a way that creates an infinite type.

---

### N2012: Ambiguous name

- **Severity:** Error
- **Code:** `N2012`

A name resolves to more than one definition in the current scope. Use a qualified path to disambiguate.

---

### N2013: Invalid visibility qualifier

- **Severity:** Error
- **Code:** `N2013`

A visibility qualifier is used in an invalid context or has incorrect syntax.

---

### N2014: Module not found

- **Severity:** Error
- **Code:** `N2014`

The module specified in an import statement cannot be found in any of the configured module search paths.

---

### N2015: Symbol not exported

- **Severity:** Error
- **Code:** `N2015`

An import statement tries to import a symbol that exists in the target module but is not publicly exported.

---

### N2016: Name conflicts with builtin

- **Severity:** Error
- **Code:** `N2016`

A user-defined name conflicts with a built-in name. Choose a different name.

---

### N2017: Name shadows builtin

- **Severity:** Warning
- **Code:** `N2017`

A user-defined name shadows a built-in name. This is allowed but can lead to confusion.

---

### N2018: Unused import

- **Severity:** Warning
- **Code:** `N2018`

An import statement introduces a name that is never used in the current file.

---

### N2019: Wildcard import leaks names

- **Severity:** Warning
- **Code:** `N2019`

A wildcard import (`use module::*`) brings names into scope that may conflict with other imports.

---

### N2020: `self` used outside of method

- **Severity:** Error
- **Code:** `N2020`

The `self` keyword is used outside of a method body. `self` is only valid as a method parameter or within method bodies.

---

### N2021: `super` used outside of class context

- **Severity:** Error
- **Code:** `N2021`

The `super` keyword is used outside of a class or type context. super is only valid for accessing parent type members.

---

### N2022: Invalid self parameter

- **Severity:** Error
- **Code:** `N2022`

A method's self parameter has an invalid type or position. Self must be the first parameter.

---

### N2023: Method without self parameter

- **Severity:** Error
- **Code:** `N2023`

A method in an implementation block has no self parameter. Methods must take `self` (or `&self`, `mut self`) as the first parameter.

---

### N2024: Return type mismatch in method impl

- **Severity:** Error
- **Code:** `N2024`

A method implementation's return type does not match the return type declared in the interface or parent definition.

---

### N2025: Missing required interface method

- **Severity:** Error
- **Code:** `N2025`

A type that claims to implement an interface does not provide an implementation for all required methods.

---

### N2026: Extra method not in interface

- **Severity:** Error
- **Code:** `N2026`

A type implementing an interface defines additional methods that are not part of the interface. This is not allowed.

---

### N2027: Invalid method override

- **Severity:** Error
- **Code:** `N2027`

A method override does not match the signature of the method it is overriding. Parameter types and return types must match.

---

### N2028: Override without base definition

- **Severity:** Error
- **Code:** `N2028`

A method is marked as override but no matching method exists in any parent type or interface.

---

### N2029: Inconsistent associated type binding

- **Severity:** Error
- **Code:** `N2029`

An associated type in a trait or interface implementation is bound to a type that conflicts with other constraints.

---

### N2030: Circular trait bound

- **Severity:** Error
- **Code:** `N2030`

Trait bounds form a cycle (e.g. A: B and B: A). Circular bounds are not allowed.

---

### N2031: Unused variable

- **Severity:** Warning
- **Code:** `N2031`

A variable is declared but never used. Consider removing it or prefixing with an underscore.

---

### N2032: Unused assignment

- **Severity:** Warning
- **Code:** `N2032`

A value is assigned to a variable but the variable is never read afterward.

---

### N2033: Variable shadows outer variable

- **Severity:** Warning
- **Code:** `N2033`

A variable declaration shadows a variable from an outer scope. Consider renaming to avoid confusion.

---

### N2034: Unreachable pattern

- **Severity:** Warning
- **Code:** `N2034`

A pattern in a match expression can never be reached because a previous pattern already covers all matching values.

---

### N2035: Non-exhaustive patterns

- **Severity:** Error
- **Code:** `N2035`

A match expression does not cover all possible values of the matched type. Add a catch-all pattern (`_ => ...`) or handle all variants.

---

### N2036: Pattern binding conflict

- **Severity:** Error
- **Code:** `N2036`

A pattern binds the same name more than once. Each binding in a pattern must have a unique name.

---

### N2037: Illegal binding mode in pattern

- **Severity:** Error
- **Code:** `N2037`

A pattern uses an unsupported binding mode (e.g., ref, mut) in a context where it is not allowed.

---

### N2038: Invalid pattern syntax

- **Severity:** Error
- **Code:** `N2038`

The pattern syntax is invalid. Patterns must follow the allowed grammar (literals, identifiers, destructuring, etc.).

---

### N2039: Unresolved import

- **Severity:** Error
- **Code:** `N2039`

An import path or symbol cannot be resolved. Check the module path and symbol name for typos.

---

### N2040: Unresolved re-export

- **Severity:** Error
- **Code:** `N2040`

A re-export (`pub use`) refers to a symbol that cannot be found.

---

### N2041: Private re-export

- **Severity:** Error
- **Code:** `N2041`

A re-export attempts to publicly expose a private symbol from another module.

---

### N2042: Conflicting re-export

- **Severity:** Error
- **Code:** `N2042`

Two or more re-exports attempt to expose different items with the same name.

---

### N2043: Re-export of non-existent symbol

- **Severity:** Error
- **Code:** `N2043`

A re-export refers to a symbol that does not exist in the source module.

---

### N2044: Self-import

- **Severity:** Error
- **Code:** `N2044`

A module attempts to import itself. Remove the self-import.

---

### N2045: Invalid use of `Self` type alias

- **Severity:** Error
- **Code:** `N2045`

The `Self` type alias is used in an invalid context. `Self` is only valid inside trait or interface definitions.

---

### N2046: External crate not found

- **Severity:** Error
- **Code:** `N2046`

An external crate dependency specified in an import cannot be found. Ensure the crate is listed in the manifest and installed.

---

### N2047: External crate version conflict

- **Severity:** Error
- **Code:** `N2047`

Two dependencies require different versions of the same external crate, creating a conflict.

---

### N2048: External crate feature not found

- **Severity:** Error
- **Code:** `N2048`

A feature of an external crate requested in an import or config does not exist.

---

### N2049: Unused extern crate

- **Severity:** Warning
- **Code:** `N2049`

An external crate is declared as a dependency but never imported or used.

---

