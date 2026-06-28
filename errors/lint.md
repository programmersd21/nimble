# Lint Errors

This page documents all error codes in the **Lint** category.

## Summary

Total codes: 61
- 61 warning code(s)

## Error Codes

---

### N5001: Unused variable

- **Severity:** Warning
- **Code:** `N5001`

A variable is declared but never used. Consider removing it or prefixing the name with an underscore to suppress this warning.

---

### N5002: Unused import

- **Severity:** Warning
- **Code:** `N5002`

An import is not used anywhere in the file. Remove the unused import to keep the code clean.

---

### N5003: Unused assignment

- **Severity:** Warning
- **Code:** `N5003`

A value is assigned to a variable but the variable is never read after the assignment.

---

### N5004: Unused function

- **Severity:** Warning
- **Code:** `N5004`

A function is defined but never called anywhere in the program.

---

### N5005: Unused struct

- **Severity:** Warning
- **Code:** `N5005`

A struct type is defined but never used.

---

### N5006: Unused type

- **Severity:** Warning
- **Code:** `N5006`

A type alias or enum is defined but never used.

---

### N5007: Dead code

- **Severity:** Warning
- **Code:** `N5007`

Code exists that will never execute due to control flow guarantees. Remove it.

---

### N5008: Unreachable code

- **Severity:** Warning
- **Code:** `N5008`

Code after a return, break, continue, or infinite loop will never execute.

---

### N5009: Empty loop body

- **Severity:** Warning
- **Code:** `N5009`

A loop has an empty body. Either add code to the body or remove the loop.

---

### N5010: Suspicious assignment in conditional

- **Severity:** Warning
- **Code:** `N5010`

An assignment expression is used in a conditional context. It may be a typo for equality comparison (`==`).

---

### N5011: Lossy implicit conversion

- **Severity:** Warning
- **Code:** `N5011`

An implicit type conversion that may lose precision or information. Use an explicit cast to make the conversion clear.

---

### N5012: Deprecated item

- **Severity:** Warning
- **Code:** `N5012`

The code uses a deprecated function, type, or construct. Check the documentation for the recommended replacement.

---

### N5013: Missing documentation

- **Severity:** Warning
- **Code:** `N5013`

A public API item is missing documentation. Add a doc comment (`///`) to describe it.

---

### N5014: Non-standard naming

- **Severity:** Warning
- **Code:** `N5014`

An identifier does not follow Nimble naming conventions. Use snake_case for variables and functions, PascalCase for types.

---

### N5015: Name shadows outer

- **Severity:** Warning
- **Code:** `N5015`

A local name shadows a name from an outer scope, which can cause confusion.

---

### N5016: Unnecessary closure

- **Severity:** Warning
- **Code:** `N5016`

A closure that simply wraps another function call can be replaced with a direct reference to the function.

---

### N5017: Redundant pattern

- **Severity:** Warning
- **Code:** `N5017`

A pattern in a match or let binding is unnecessarily complex. Simplify it.

---

### N5018: Missing else branch

- **Severity:** Warning
- **Code:** `N5018`

An if expression without an else branch may not return a value. Add an else branch if needed.

---

### N5019: Deep nesting

- **Severity:** Warning
- **Code:** `N5019`

Code is nested more than the recommended depth. Refactor to reduce nesting.

---

### N5020: Complex expression

- **Severity:** Warning
- **Code:** `N5020`

An expression is overly complex. Break it into simpler sub-expressions with intermediate variables.

---

### N5021: High cognitive complexity

- **Severity:** Warning
- **Code:** `N5021`

The cognitive complexity of a function exceeds the recommended limit. Consider breaking the function into smaller pieces.

---

### N5022: High cyclomatic complexity

- **Severity:** Warning
- **Code:** `N5022`

The cyclomatic complexity of a function is too high. Refactor to reduce the number of independent paths.

---

### N5023: Too many parameters

- **Severity:** Warning
- **Code:** `N5023`

A function has more parameters than the recommended maximum. Consider using a struct to group related parameters.

---

### N5024: Too many return types

- **Severity:** Warning
- **Code:** `N5024`

A type has more return type variants than recommended. Simplify the type hierarchy.

---

### N5025: Function too long

- **Severity:** Warning
- **Code:** `N5025`

A function body exceeds the recommended maximum number of lines. Extract helper functions.

---

### N5026: File too long

- **Severity:** Warning
- **Code:** `N5026`

The source file exceeds the recommended maximum line count. Split into multiple files.

---

### N5027: Line too long

- **Severity:** Warning
- **Code:** `N5027`

A source line exceeds the maximum allowed column width. Wrap the line.

---

### N5028: Inconsistent naming style

- **Severity:** Warning
- **Code:** `N5028`

Identifier naming is inconsistent within the codebase. Follow the established naming convention.

---

### N5029: Non-canonical ordering

- **Severity:** Warning
- **Code:** `N5029`

Items are not ordered according to the project's convention (e.g., imports before definitions, public before private).

---

### N5030: Unsafe block used

- **Severity:** Warning
- **Code:** `N5030`

An unsafe block is used. Ensure that all invariants are manually verified.

---

### N5031: Unsafe function

- **Severity:** Warning
- **Code:** `N5031`

A function is declared as unsafe. Only use unsafe when absolutely necessary and document safety requirements.

---

### N5032: Unnecessary unsafe

- **Severity:** Warning
- **Code:** `N5032`

An unsafe block or function is unnecessary because it contains no actual unsafe operations.

---

### N5033: Comparing boolean literal

- **Severity:** Warning
- **Code:** `N5033`

A boolean value is compared to `true` or `false` directly. Use the value directly or use the `!` operator for negation.

---

### N5034: Assigning boolean in conditional

- **Severity:** Warning
- **Code:** `N5034`

An assignment using `=` appears in a condition. This is likely a typo for `==`.

---

### N5035: Negating boolean literal

- **Severity:** Warning
- **Code:** `N5035`

A boolean literal is negated with `!`. Use the opposite literal directly.

---

### N5036: Nested conditional

- **Severity:** Warning
- **Code:** `N5036`

A conditional expression is nested inside another conditional in a way that could be simplified.

---

### N5037: Constant condition

- **Severity:** Warning
- **Code:** `N5037`

A condition in an if or while is always true or always false, making the branch redundant.

---

### N5038: Redundant cast

- **Severity:** Warning
- **Code:** `N5038`

A type cast is unnecessary because the source and target types are the same or the conversion is implicit.

---

### N5039: Suspicious comparison

- **Severity:** Warning
- **Code:** `N5039`

A comparison is likely always true or always false (e.g., x == x).

---

### N5040: Infinite loop detected

- **Severity:** Warning
- **Code:** `N5040`

A loop condition is always true and there is no break statement, making the loop potentially infinite.

---

### N5041: Missing break in loop

- **Severity:** Warning
- **Code:** `N5041`

A loop with a constant true condition has no break statement. Add a condition check and break.

---

### N5042: Uninitialized variable

- **Severity:** Warning
- **Code:** `N5042`

A variable is used before being assigned a value. Initialize it before use.

---

### N5043: Possibly uninitialized variable

- **Severity:** Warning
- **Code:** `N5043`

A variable may be used without being initialized in all code paths.

---

### N5044: Fallthrough in match

- **Severity:** Warning
- **Code:** `N5044`

A match arm falls through to the next arm. Use an explicit `continue` or handle all cases.

---

### N5045: Missing case in match

- **Severity:** Warning
- **Code:** `N5045`

A match on an enum or union type does not cover all variants. Add the missing cases.

---

### N5046: Redundant default case

- **Severity:** Warning
- **Code:** `N5046`

A match has a default case that can never be reached because all variants are already covered.

---

### N5047: Unnecessary else-if

- **Severity:** Warning
- **Code:** `N5047`

An else-if chain can be simplified. Use a match expression instead.

---

### N5048: Redundant else branch

- **Severity:** Warning
- **Code:** `N5048`

The else branch of an if-else is empty or redundant. Remove it.

---

### N5049: Empty else branch

- **Severity:** Warning
- **Code:** `N5049`

An else branch is empty. Either add code or remove the else.

---

### N5050: Unnecessary parentheses

- **Severity:** Warning
- **Code:** `N5050`

Parentheses around an expression are not needed and reduce readability. Remove them.

---

### N5051: Unnecessary return

- **Severity:** Warning
- **Code:** `N5051`

A return statement at the end of a function is redundant. Remove the return keyword.

---

### N5052: Unnecessary semicolon

- **Severity:** Warning
- **Code:** `N5052`

A semicolon appears in a position where it is not needed. Remove it.

---

### N5053: Empty statement

- **Severity:** Warning
- **Code:** `N5053`

An empty statement (bare semicolon or empty line) can be removed.

---

### N5054: Statement with no effect

- **Severity:** Warning
- **Code:** `N5054`

An expression statement has no side effects and its result is discarded. It can be removed.

---

### N5055: Variable assigned but not used

- **Severity:** Warning
- **Code:** `N5055`

A variable is assigned a value but is never read afterward. Consider removing the assignment.

---

### N5056: Function argument reassigned

- **Severity:** Warning
- **Code:** `N5056`

A function parameter is reassigned inside the function body. Use a local variable instead.

---

### N5057: Mutable variable could be immutable

- **Severity:** Warning
- **Code:** `N5057`

A variable declared with `var` is never mutated. Use `let` instead.

---

### N5058: Redundant field name in struct literal

- **Severity:** Warning
- **Code:** `N5058`

A struct literal uses the field name even though the value is the same name. The shorthand syntax can be used.

---

### N5059: Unnecessary qualification

- **Severity:** Warning
- **Code:** `N5059`

A module path qualification is unnecessary because the symbol is already in scope.

---

### N5060: Module naming convention violation

- **Severity:** Warning
- **Code:** `N5060`

A module name does not follow the project's naming conventions for modules.

---

### N5061: Non-idiomatic code pattern

- **Severity:** Warning
- **Code:** `N5061`

The code uses a pattern that is not idiomatic for Nimble. Consider using a more standard approach.

---

