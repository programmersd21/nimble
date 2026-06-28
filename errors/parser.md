# Parser Errors

This page documents all error codes in the **Parser** category.

## Summary

Total codes: 84
- 84 error code(s)

## Error Codes

---

### N1001: Expected specific token

- **Severity:** Error
- **Code:** `N1001`

The parser expected a specific token (e.g. ';', ':', '=') at the current position but found something else. This usually means a syntax error, missing punctuation, or a misplaced construct near the indicated location.

---

### N1002: Unexpected token

- **Severity:** Error
- **Code:** `N1002`

The parser encountered a token that does not make sense at the current position in the grammar. This is a generic syntax error that indicates a structural problem with the code around the indicated location.

---

### N1003: Expected expression

- **Severity:** Error
- **Code:** `N1003`

The parser expected an expression (a value-producing construct) but found something else. This can happen after operators, after `=`, after `return`, or in other expression contexts.

---

### N1004: Unclosed delimiter

- **Severity:** Error
- **Code:** `N1004`

A grouping delimiter (parenthesis, bracket, or brace) was opened but not closed by the expected token. Check the matching delimiter at the reported location.

---

### N1005: Expected indented block

- **Severity:** Error
- **Code:** `N1005`

A colon (`:`) was followed by an end-of-line without an indented block on the next line. Nimble requires an indented body after function definitions, if/else, while, for, and other colon-terminated constructs.

---

### N1006: Unexpected indentation

- **Severity:** Error
- **Code:** `N1006`

An indentation change occurs at an unexpected location, typically because a line is indented more than expected relative to surrounding blocks.

---

### N1007: Expected identifier

- **Severity:** Error
- **Code:** `N1007`

The parser expected an identifier (a name) but found something else. Identifiers are required for variable names, function names, type names, and field access.

---

### N1008: Expected type name

- **Severity:** Error
- **Code:** `N1008`

A type annotation requires a type name but found something else. Type names come after colons in variable declarations and function signatures.

---

### N1009: Expected parameter name

- **Severity:** Error
- **Code:** `N1009`

A function or method parameter list contains a construct that is not a valid parameter name.

---

### N1010: Expected colon

- **Severity:** Error
- **Code:** `N1010`

A colon (`:`) was expected in a type annotation, label, or block-introducing construct but was not found.

---

### N1011: Expected semicolon

- **Severity:** Error
- **Code:** `N1011`

A semicolon (`;`) was expected but not found. In Nimble, semicolons are optional line separators but required in certain contexts like separating statements on the same line.

---

### N1012: Expected equals sign

- **Severity:** Error
- **Code:** `N1012`

An equals sign (`=`) was expected in an assignment, variable initializer, or default value but was not found.

---

### N1013: Expected arrow

- **Severity:** Error
- **Code:** `N1013`

A function arrow (`->`) was expected in a return type annotation or lambda expression but was not found.

---

### N1014: Expected comma

- **Severity:** Error
- **Code:** `N1014`

A comma (`,`) was expected in a list (parameters, arguments, struct fields) but was not found.

---

### N1015: Expected dot

- **Severity:** Error
- **Code:** `N1015`

A dot (`.`) was expected for member access or qualified path access but was not found.

---

### N1016: Missing function body

- **Severity:** Error
- **Code:** `N1016`

A function declaration is missing its body. Functions must have either a body block or be declared as extern.

---

### N1017: Missing return type or return expression

- **Severity:** Error
- **Code:** `N1017`

A function with a declared return type is missing a return expression in its body, or a non-void function is missing its return type annotation.

---

### N1018: Invalid function parameter

- **Severity:** Error
- **Code:** `N1018`

A function parameter list contains an invalid parameter. Parameters must have the form `name: Type` or `name`.

---

### N1019: Too many function parameters

- **Severity:** Error
- **Code:** `N1019`

A function declaration exceeds the maximum number of allowed parameters defined by the compiler limit.

---

### N1020: Too few arguments in call

- **Severity:** Error
- **Code:** `N1020`

A function call provides fewer arguments than the function signature expects.

---

### N1021: Expected statement

- **Severity:** Error
- **Code:** `N1021`

The parser expected a statement (declaration, assignment, expression statement, etc.) at the current position.

---

### N1022: Expected binding

- **Severity:** Error
- **Code:** `N1022`

Expected a `let` or `var` binding declaration at this position. In Nimble, variables must be explicitly declared with `let` (immutable) or `var` (mutable).

---

### N1023: Expected keyword

- **Severity:** Error
- **Code:** `N1023`

A specific keyword (`if`, `else`, `while`, `for`, `return`, `fn`, etc.) was expected at this position.

---

### N1024: Invalid left-hand side of assignment

- **Severity:** Error
- **Code:** `N1024`

The left-hand side of an assignment operator (`=`, `+=`, etc.) is not a valid assignment target. Only variables, mutable fields, and indexed expressions can be assigned to.

---

### N1025: Nested function without closure context

- **Severity:** Error
- **Code:** `N1025`

A function is nested inside another function but the context does not support closures. Nested functions require closure support.

---

### N1026: Duplicate parameter name

- **Severity:** Error
- **Code:** `N1026`

A function has two parameters with the same name. Parameter names must be unique within a function signature.

---

### N1027: Default value before non-default parameter

- **Severity:** Error
- **Code:** `N1027`

A parameter with a default value appears before a parameter without one. All parameters with defaults must come after parameters without defaults.

---

### N1028: Expected module path

- **Severity:** Error
- **Code:** `N1028`

An import or use statement expects a module path (a sequence of identifiers separated by `::` or dots) but found something else.

---

### N1029: Expected import symbol

- **Severity:** Error
- **Code:** `N1029`

An import statement expects a symbol name to import from a module.

---

### N1030: Circular import

- **Severity:** Error
- **Code:** `N1030`

Two or more modules directly or indirectly import each other, creating a cycle. Nimble does not allow circular imports.

---

### N1031: Break outside of loop

- **Severity:** Error
- **Code:** `N1031`

A `break` statement appears outside of any enclosing loop (while, for). break is only valid within loop bodies.

---

### N1032: Continue outside of loop

- **Severity:** Error
- **Code:** `N1032`

A `continue` statement appears outside of any enclosing loop. continue is only valid within loop bodies.

---

### N1033: Return outside of function

- **Severity:** Error
- **Code:** `N1033`

A `return` statement appears outside of any function body. return is only valid inside functions.

---

### N1034: Yield outside of generator

- **Severity:** Error
- **Code:** `N1034`

A `yield` expression appears outside of a generator function. yield is only valid inside generators.

---

### N1035: Invalid for-loop binding

- **Severity:** Error
- **Code:** `N1035`

The binding pattern in a for-loop is invalid. For-loops require an identifier or destructuring pattern followed by `in` and an iterable expression.

---

### N1036: Expected `in` in for-loop

- **Severity:** Error
- **Code:** `N1036`

A for-loop is missing the `in` keyword between the binding and the iterable expression.

---

### N1037: Empty struct body

- **Severity:** Error
- **Code:** `N1037`

A struct declaration has no fields. Structs must have at least one field.

---

### N1038: Empty interface body

- **Severity:** Error
- **Code:** `N1038`

An interface declaration has no methods. Interfaces must have at least one method signature.

---

### N1039: Method declaration outside of interface

- **Severity:** Error
- **Code:** `N1039`

A method-like declaration appears outside of an interface or struct body.

---

### N1040: Duplicate field name in struct

- **Severity:** Error
- **Code:** `N1040`

A struct has two or more fields with the same name. Field names must be unique within a struct.

---

### N1041: Unnamed field in struct literal

- **Severity:** Error
- **Code:** `N1041`

A struct literal uses a positional (unnamed) value but the struct expects named fields. Use `field: value` syntax.

---

### N1042: Expected struct expression

- **Severity:** Error
- **Code:** `N1042`

A struct literal expression is expected but something else was found. Struct literals use `Name { field: value, ... }` syntax.

---

### N1043: Missing colon in type annotation

- **Severity:** Error
- **Code:** `N1043`

A type annotation is missing the colon separator between the name and its type. Use `name: Type` syntax.

---

### N1044: Unexpected trailing comma

- **Severity:** Error
- **Code:** `N1044`

A trailing comma appears where it is not syntactically allowed.

---

### N1045: Malformed string interpolation expression

- **Severity:** Error
- **Code:** `N1045`

A string interpolation expression within a string has invalid syntax. Interpolation expressions must be valid expressions enclosed in the proper delimiters.

---

### N1046: Unterminated block comment

- **Severity:** Error
- **Code:** `N1046`

A block comment (/*) was started but the closing (*/) was not found before end of file.

---

### N1047: Expected attribute

- **Severity:** Error
- **Code:** `N1047`

An attribute annotation is expected at this position. Attributes use `#[name]` syntax.

---

### N1048: Invalid attribute target

- **Severity:** Error
- **Code:** `N1048`

An attribute is applied to a construct that does not support attributes.

---

### N1049: Duplicate attribute

- **Severity:** Error
- **Code:** `N1049`

The same attribute is applied more than once to the same construct.

---

### N1050: Unknown attribute

- **Severity:** Error
- **Code:** `N1050`

An unrecognized attribute name was used. Check the spelling of the attribute.

---

### N1051: Expected type arguments

- **Severity:** Error
- **Code:** `N1051`

A generic type is missing its type argument list. Use `Type<T, U>` syntax.

---

### N1052: Unclosed type argument list

- **Severity:** Error
- **Code:** `N1052`

A type argument list (`<`) was opened but not closed (`>`) before the expected position.

---

### N1053: Type argument count mismatch

- **Severity:** Error
- **Code:** `N1053`

The number of type arguments provided does not match the number of type parameters defined by the generic.

---

### N1054: Expected `<` for generics

- **Severity:** Error
- **Code:** `N1054`

A `<` token was expected to open a generic or type argument list.

---

### N1055: Expected `>` for generics

- **Severity:** Error
- **Code:** `N1055`

A `>` token was expected to close a generic or type argument list.

---

### N1056: Unterminated lambda body

- **Severity:** Error
- **Code:** `N1056`

A lambda expression has no body or the body is incomplete. Lambdas use `|params| expr` syntax.

---

### N1057: Expected binding in for-loop

- **Severity:** Error
- **Code:** `N1057`

A for-loop is missing its binding variable. Use `for var in iter: body`.

---

### N1058: Expected path expression

- **Severity:** Error
- **Code:** `N1058`

A qualified path expression was expected. Paths use `module::name` or `object.field` syntax.

---

### N1059: Expected literal

- **Severity:** Error
- **Code:** `N1059`

A literal value was expected but something else was found.

---

### N1060: Expected pattern

- **Severity:** Error
- **Code:** `N1060`

A pattern (used in match arms, destructuring, or binding) was expected but something else was found.

---

### N1061: Expected guard expression

- **Severity:** Error
- **Code:** `N1061`

A match arm with a guard (`if` clause) is missing the guard expression after `if`.

---

### N1062: Expected where clause

- **Severity:** Error
- **Code:** `N1062`

Expected a `where` clause for specifying trait bounds or constraints.

---

### N1063: Expected semicolon or newline after statement

- **Severity:** Error
- **Code:** `N1063`

A statement must be terminated by either a semicolon or a newline. Two statements cannot appear on the same line without a separator.

---

### N1064: Expected operator

- **Severity:** Error
- **Code:** `N1064`

An operator (such as `+`, `-`, `*`, `/`, `==`, etc.) was expected at this position, typically in an expression context.

---

### N1065: Invalid prefix operator

- **Severity:** Error
- **Code:** `N1065`

The prefix operator used is not valid for the type of expression it precedes.

---

### N1066: Invalid postfix operator

- **Severity:** Error
- **Code:** `N1066`

The postfix operator used is not valid for the type of expression it follows.

---

### N1067: Operator precedence ambiguity

- **Severity:** Error
- **Code:** `N1067`

The combination of operators in an expression is ambiguous and requires explicit grouping with parentheses.

---

### N1068: Incompatible comparison chaining

- **Severity:** Error
- **Code:** `N1068`

Comparisons of different types or incompatible operators cannot be chained together. Use parentheses to group comparisons explicitly.

---

### N1069: Expected tuple element

- **Severity:** Error
- **Code:** `N1069`

A tuple expression or type is missing an element. Use `(a, b, c)` syntax with commas between elements.

---

### N1070: Expected array element

- **Severity:** Error
- **Code:** `N1070`

An array literal is missing an element. Use `[a, b, c]` syntax with commas between elements.

---

### N1071: Expected struct field

- **Severity:** Error
- **Code:** `N1071`

A struct literal is missing a field. Use `Struct { field: value }` syntax.

---

### N1072: Expected enum variant

- **Severity:** Error
- **Code:** `N1072`

An enum variant pattern or constructor was expected but something else was found.

---

### N1073: Expected match arm

- **Severity:** Error
- **Code:** `N1073`

A match expression must have at least one arm (pattern => expression). Add at least one match arm.

---

### N1074: Expected `=>` in match arm

- **Severity:** Error
- **Code:** `N1074`

A match arm pattern is not followed by `=>`. Use `pattern => expression` syntax.

---

### N1075: Expected pattern guard

- **Severity:** Error
- **Code:** `N1075`

A match arm guard (if clause) is missing its condition expression.

---

### N1076: Invalid doc comment placement

- **Severity:** Error
- **Code:** `N1076`

A doc comment (`///` or `//!`) appears in a position where documentation is not allowed.

---

### N1077: Expected doc comment

- **Severity:** Error
- **Code:** `N1077`

A documentation comment was expected, typically before a public API item.

---

### N1078: Invalid visibility modifier

- **Severity:** Error
- **Code:** `N1078`

A visibility modifier (pub, pub(crate), etc.) is being used in an invalid position or with an incorrect syntax.

---

### N1079: Expected item

- **Severity:** Error
- **Code:** `N1079`

A top-level or nested item (function, struct, interface, constant, etc.) was expected at this position.

---

### N1080: Nested function without body

- **Severity:** Error
- **Code:** `N1080`

A nested function declaration has no body. Nested functions must have an inline body.

---

### N1081: Unterminated generic list

- **Severity:** Error
- **Code:** `N1081`

A generic parameter list (`<`) was started but not closed (`>`) before a structural end was reached.

---

### N1082: Expected lifetime parameter

- **Severity:** Error
- **Code:** `N1082`

A lifetime parameter (using `'a` syntax) was expected but not found.

---

### N1083: Expected const parameter

- **Severity:** Error
- **Code:** `N1083`

A const generic parameter was expected but not found. Const parameters use `const NAME: Type` syntax.

---

### N1084: Ambiguous literal suffix

- **Severity:** Error
- **Code:** `N1084`

A numeric literal has a suffix that could refer to multiple types or is not a recognized suffix. Add an explicit type annotation or use a standard suffix.

---

