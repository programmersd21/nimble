# Type System Errors

This page documents all error codes in the **Type System** category.

## Summary

Total codes: 97
- 93 error code(s)
- 4 warning code(s)

## Error Codes

---

### N3001: Type mismatch

- **Severity:** Error
- **Code:** `N3001`

An expression has a type that does not match the expected type in this context. This can happen in assignments, function arguments, return statements, and binary operations. Ensure the expression produces the correct type.

---

### N3002: Assign to immutable variable

- **Severity:** Error
- **Code:** `N3002`

An attempt is made to assign a new value to a variable declared with `let`. Use `var` instead of `let` if the variable needs to be reassigned.

---

### N3003: Undefined type

- **Severity:** Error
- **Code:** `N3003`

A type name used in an annotation or expression is not defined in any accessible scope. Check for typos and ensure the type is imported.

---

### N3004: Call of non-function value

- **Severity:** Error
- **Code:** `N3004`

A call expression attempts to invoke a value that is not a function. Only functions, closures, and callable objects can be called.

---

### N3005: Argument count mismatch

- **Severity:** Error
- **Code:** `N3005`

A function call provides a different number of arguments than the function signature expects. Check both the count and the presence of default arguments.

---

### N3006: Missing required method

- **Severity:** Error
- **Code:** `N3006`

A type is used where an interface or trait is expected, but the type does not implement all required methods.

---

### N3007: Recursive type without indirection

- **Severity:** Error
- **Code:** `N3007`

A type definition is recursive without using indirection (e.g., Box, reference). Direct recursion in value types leads to infinite size.

---

### N3008: Infinite type

- **Severity:** Error
- **Code:** `N3008`

Type inference produces an infinitely recursive type. This usually happens when a variable is used in a way that creates a cycle in its type.

---

### N3009: Cannot infer type

- **Severity:** Error
- **Code:** `N3009`

The type of an expression cannot be inferred from context. Add an explicit type annotation.

---

### N3010: Type annotation required

- **Severity:** Error
- **Code:** `N3010`

A construct requires an explicit type annotation but none was provided. This is often needed for function parameters and certain variable definitions where inference is ambiguous.

---

### N3011: Expected concrete type, found abstract

- **Severity:** Error
- **Code:** `N3011`

A concrete type was expected but an abstract type (trait, interface, or type variable) was provided. Use a specific concrete type.

---

### N3012: Trait bound not satisfied

- **Severity:** Error
- **Code:** `N3012`

A type is used in a context that requires it to implement a specific trait or interface, but the type does not satisfy the bound.

---

### N3013: Associated type not specified

- **Severity:** Error
- **Code:** `N3013`

A trait or interface has an associated type that must be specified but has not been provided. Use the `type Assoc = ConcreteType;` syntax.

---

### N3014: Wrong number of type arguments

- **Severity:** Error
- **Code:** `N3014`

A generic type or function is provided with a different number of type arguments than it expects.

---

### N3015: Type argument out of bounds

- **Severity:** Error
- **Code:** `N3015`

A type argument does not satisfy the bounds declared for the corresponding type parameter.

---

### N3016: Conflicting type arguments

- **Severity:** Error
- **Code:** `N3016`

Two type arguments to the same generic are in conflict with each other.

---

### N3017: Cross-module type violation

- **Severity:** Error
- **Code:** `N3017`

A type from one module is used in a way that violates the type's invariants across module boundaries.

---

### N3018: Borrow of immutable as mutable

- **Severity:** Error
- **Code:** `N3018`

An immutable reference is used where a mutable reference is required. Use `mut` on the binding or pass a mutable reference explicitly.

---

### N3019: Borrow of moved value

- **Severity:** Error
- **Code:** `N3019`

A value that has been moved to another owner is being used. Use a reference instead of moving, or reorder operations.

---

### N3020: Use after move

- **Severity:** Error
- **Code:** `N3020`

A value is used after it has been moved. The ownership was transferred and the original binding is no longer valid.

---

### N3021: Multiple mutable borrows

- **Severity:** Error
- **Code:** `N3021`

A value is mutably borrowed more than once at the same time. Only one mutable borrow is allowed at a time.

---

### N3022: Lifetime mismatch

- **Severity:** Error
- **Code:** `N3022`

The lifetimes of two references are incompatible. The borrowed value does not live long enough to satisfy the lifetime constraint.

---

### N3023: Lifetime elision failure

- **Severity:** Error
- **Code:** `N3023`

The compiler cannot infer the lifetime of a reference using the default elision rules. Add explicit lifetime annotations.

---

### N3024: Lifetime bound not satisfied

- **Severity:** Error
- **Code:** `N3024`

A reference's lifetime does not satisfy the bounds required by the context. The data must live at least as long as the constraint requires.

---

### N3025: Lifetime constraint violation

- **Severity:** Error
- **Code:** `N3025`

A lifetime constraint specified in the code is violated by the actual usage. Ensure all borrows respect the declared lifetimes.

---

### N3026: Missing lifetime annotation

- **Severity:** Error
- **Code:** `N3026`

A function or type signature uses references but is missing required lifetime annotations. Add lifetime parameters like `'a`.

---

### N3027: Invalid lifetime name

- **Severity:** Error
- **Code:** `N3027`

A lifetime name is not valid. Lifetime names must start with a tick (`'`) followed by an identifier.

---

### N3028: Mismatched mutability in reference

- **Severity:** Error
- **Code:** `N3028`

The mutability of a reference does not match what is required. An immutable reference cannot be used where a mutable reference is needed, and vice versa.

---

### N3029: Dangling reference

- **Severity:** Error
- **Code:** `N3029`

A reference outlives the data it points to. The referenced value has been dropped while the reference still exists.

---

### N3030: Drop of type with move semantics

- **Severity:** Error
- **Code:** `N3030`

An attempt is made to drop a value that has move semantics in a context that does not support it.

---

### N3031: Borrow of constant value

- **Severity:** Error
- **Code:** `N3031`

A constant value is being borrowed mutably. Constants are immutable and cannot be borrowed as mutable.

---

### N3032: Numeric overflow in constant expression

- **Severity:** Error
- **Code:** `N3032`

A constant expression produces a numeric value that overflows the target type. Use a smaller value or a larger type.

---

### N3033: Division by zero in constant expression

- **Severity:** Error
- **Code:** `N3033`

A constant expression contains a division by zero. Ensure all divisors are non-zero at compile time.

---

### N3034: Remainder by zero in constant expression

- **Severity:** Error
- **Code:** `N3034`

A constant expression contains a remainder operation with a zero divisor.

---

### N3035: Negation of unsigned integer

- **Severity:** Error
- **Code:** `N3035`

The unary negation operator (`-`) is applied to an unsigned integer type. Unsigned types cannot represent negative values.

---

### N3036: Shift exceeds bit width

- **Severity:** Error
- **Code:** `N3036`

A bit shift operation uses a shift amount that is greater than or equal to the bit width of the type.

---

### N3037: Operator not applicable to types

- **Severity:** Error
- **Code:** `N3037`

An operator is used with operand types that do not support that operation. Check the operator and types are compatible.

---

### N3038: Comparison of unordered values

- **Severity:** Error
- **Code:** `N3038`

A comparison operator is applied to values that cannot be ordered (e.g., floats with NaN, or types that do not implement comparison traits).

---

### N3039: Invalid unary operator for type

- **Severity:** Error
- **Code:** `N3039`

A unary operator (like `-`, `!`, `~`) is applied to a type that does not support it.

---

### N3040: Invalid binary operator for types

- **Severity:** Error
- **Code:** `N3040`

A binary operator (like `+`, `-`, `*`, `/`, `==`) is applied to types that do not support it. Check the operand types.

---

### N3041: No common operator overload

- **Severity:** Error
- **Code:** `N3041`

The operator overload resolution cannot find a suitable implementation for the given operand types.

---

### N3042: Ambiguous operator application

- **Severity:** Error
- **Code:** `N3042`

More than one operator overload matches the operand types. Use an explicit method call to disambiguate.

---

### N3043: Wrong number of generic type parameters

- **Severity:** Error
- **Code:** `N3043`

A generic type or function is instantiated with the wrong number of type parameters.

---

### N3044: Generic bound not satisfied

- **Severity:** Error
- **Code:** `N3044`

A type argument does not satisfy the bounds declared on the generic parameter.

---

### N3045: Missing generic type annotation

- **Severity:** Error
- **Code:** `N3045`

A generic construct requires a type annotation that cannot be inferred. Provide the type parameters explicitly.

---

### N3046: Generic parameter not used

- **Severity:** Warning
- **Code:** `N3046`

A generic type parameter is declared but never used in the function signature or body. Consider removing it.

---

### N3047: Concrete type in abstract context

- **Severity:** Error
- **Code:** `N3047`

A concrete type is used where an abstract type (interface/trait) was expected. The concrete type does not implement the required abstraction.

---

### N3048: Abstract type in concrete context

- **Severity:** Error
- **Code:** `N3048`

An abstract type is used where a concrete type is required. Abstract types cannot be instantiated or used in value contexts.

---

### N3049: Non-constant in const context

- **Severity:** Error
- **Code:** `N3049`

A non-constant expression is used where a compile-time constant is required. Only literal values and const functions can be used in const contexts.

---

### N3050: Non-const call in const context

- **Severity:** Error
- **Code:** `N3050`

A function that is not declared as `const` is called in a const context. Only const functions can be called at compile time.

---

### N3051: Mutable reference in const context

- **Severity:** Error
- **Code:** `N3051`

A mutable reference is used in a const context. Const contexts do not allow mutation.

---

### N3052: If condition must be boolean

- **Severity:** Error
- **Code:** `N3052`

The condition in an `if` expression must be of type Bool. The provided expression has a non-boolean type.

---

### N3053: While condition must be boolean

- **Severity:** Error
- **Code:** `N3053`

The condition in a `while` loop must be of type Bool. The provided expression has a non-boolean type.

---

### N3054: For-each binding type mismatch

- **Severity:** Error
- **Code:** `N3054`

The binding variable type in a for loop does not match the element type of the iterable expression.

---

### N3055: Return type mismatch

- **Severity:** Error
- **Code:** `N3055`

A return expression's type does not match the declared return type of the function. Ensure the returned value has the correct type.

---

### N3056: Missing return value

- **Severity:** Error
- **Code:** `N3056`

A function with a non-void return type has a code path that does not return a value. Add a return statement to all code paths.

---

### N3057: Extra return value from void function

- **Severity:** Error
- **Code:** `N3057`

A function declared as returning Void contains a return expression with a value. Remove the value or change the return type.

---

### N3058: Return not allowed here

- **Severity:** Error
- **Code:** `N3058`

A return statement appears in a context where it is not valid, such as inside a closure that does not capture the function's return path.

---

### N3059: Missing async annotation

- **Severity:** Error
- **Code:** `N3059`

A function that uses `await` is not marked as `async`. Add the async keyword to the function signature.

---

### N3060: Cannot await non-future

- **Severity:** Error
- **Code:** `N3060`

The `await` expression is used on a value that is not a Future. Only values of type Future can be awaited.

---

### N3061: Incompatible implicit conversion

- **Severity:** Warning
- **Code:** `N3061`

An implicit type conversion might lose information or is not allowed. Consider an explicit cast.

---

### N3062: Forward declaration type mismatch

- **Severity:** Error
- **Code:** `N3062`

The type of a forward-declared item does not match the full definition. The types must be consistent.

---

### N3063: Missing forward declaration

- **Severity:** Error
- **Code:** `N3063`

An item that requires a forward declaration (e.g., mutually recursive functions) does not have one.

---

### N3064: Field type mismatch in struct literal

- **Severity:** Error
- **Code:** `N3064`

The type of a value in a struct literal does not match the declared field type.

---

### N3065: Missing field in struct literal

- **Severity:** Error
- **Code:** `N3065`

A struct literal does not provide a value for all required fields.

---

### N3066: Extra field in struct literal

- **Severity:** Error
- **Code:** `N3066`

A struct literal contains a field name that does not exist in the struct definition.

---

### N3067: Ambiguous field in struct literal

- **Severity:** Error
- **Code:** `N3067`

A field name in a struct literal could refer to multiple fields (e.g., due to inheritance or mixins).

---

### N3068: Cyclic struct definition

- **Severity:** Error
- **Code:** `N3068`

A struct definition refers to itself in a way that creates an infinite-sized type. Use a pointer or Box for recursive references.

---

### N3069: Tuple index out of bounds

- **Severity:** Error
- **Code:** `N3069`

A tuple index accesses an element that does not exist. Tuple indices start at 0 and must be less than the tuple's arity.

---

### N3070: Array index out of bounds

- **Severity:** Error
- **Code:** `N3070`

A constant array index expression has a value outside the array's defined bounds.

---

### N3071: Non-comptime array index

- **Severity:** Error
- **Code:** `N3071`

An array index must be a compile-time constant expression but a variable expression was used.

---

### N3072: Mismatched array length in type

- **Severity:** Error
- **Code:** `N3072`

An array type annotation specifies a length that does not match the actual array literal.

---

### N3073: Type alias cycle

- **Severity:** Error
- **Code:** `N3073`

A type alias directly or indirectly refers to itself, creating a cycle.

---

### N3074: Type alias uses non-existent type

- **Severity:** Error
- **Code:** `N3074`

A type alias refers to a type that does not exist or is not in scope.

---

### N3075: Unsized type in field

- **Severity:** Error
- **Code:** `N3075`

An unsized type (type whose size is unknown at compile time) is used as a struct field. Use a pointer or Box.

---

### N3076: Unsized type in local variable

- **Severity:** Error
- **Code:** `N3076`

An unsized type is used as a local variable. Local variables must have a known size at compile time.

---

### N3077: Unsized type in parameter

- **Severity:** Error
- **Code:** `N3077`

An unsized type is used as a function parameter. Use a reference or pointer parameter instead.

---

### N3078: Unsized type in return position

- **Severity:** Error
- **Code:** `N3078`

An unsized type is used as a return type. Return types must have a known size.

---

### N3079: Unsized type in struct field

- **Severity:** Error
- **Code:** `N3079`

An unsized type appears as a struct field. Wrap it in a pointer type like `Box<T>`.

---

### N3080: Unexpected type parameter

- **Severity:** Error
- **Code:** `N3080`

A type parameter is provided in a context that does not expect one.

---

### N3081: Expected type parameter

- **Severity:** Error
- **Code:** `N3081`

A generic type or function expects a type parameter but none was supplied.

---

### N3082: Invalid enum discriminant type

- **Severity:** Error
- **Code:** `N3082`

The underlying type specified for an enum's discriminant is not valid. Only integer types can be used as discriminant types.

---

### N3083: Duplicate enum discriminant value

- **Severity:** Error
- **Code:** `N3083`

Two or more enum variants have the same discriminant value. Discriminant values must be unique within an enum.

---

### N3084: Enum discriminant overflow

- **Severity:** Error
- **Code:** `N3084`

An enum discriminant value exceeds the range of the underlying integer type.

---

### N3085: Non-exhaustive enum match

- **Severity:** Error
- **Code:** `N3085`

A match on an enum does not cover all variants. Add arms for missing variants or a catch-all pattern.

---

### N3086: Unreachable match arm

- **Severity:** Warning
- **Code:** `N3086`

A match arm can never execute because a previous arm already matches all values it would match.

---

### N3087: Overlapping match patterns

- **Severity:** Warning
- **Code:** `N3087`

Two or more patterns in a match can match the same value. Reorder or refine the patterns.

---

### N3088: Invalid ref pattern

- **Severity:** Error
- **Code:** `N3088`

A `ref` pattern modifier is used in an invalid position or on a type that does not support it.

---

### N3089: Invalid mut pattern

- **Severity:** Error
- **Code:** `N3089`

A `mut` pattern modifier is used in an invalid position or on a type that is not mutable.

---

### N3090: Pattern requires unit type

- **Severity:** Error
- **Code:** `N3090`

A pattern that expects a unit type (empty tuple) is used with a non-unit value.

---

### N3091: Closure without closure context

- **Severity:** Error
- **Code:** `N3091`

A closure is defined in a context that does not support closures. Closures require the ability to capture variables from the enclosing scope.

---

### N3092: Closure captures disjoint variables

- **Severity:** Error
- **Code:** `N3092`

A closure captures variables that have disjoint lifetimes, making it impossible to infer a single combined lifetime.

---

### N3093: Non-copy type in closure by copy

- **Severity:** Error
- **Code:** `N3093`

A closure attempts to capture a non-Copy type by value (copy) in a `Fn` closure context. The type must implement Copy.

---

### N3094: Mismatched async closure

- **Severity:** Error
- **Code:** `N3094`

An async closure is used in a context that expects a sync closure, or vice versa.

---

### N3095: Generator resume type mismatch

- **Severity:** Error
- **Code:** `N3095`

A generator's resume argument type does not match what the generator expects.

---

### N3096: Generator yield type mismatch

- **Severity:** Error
- **Code:** `N3096`

A generator's yielded value type does not match the expected yield type.

---

### N3097: Generator return type mismatch

- **Severity:** Error
- **Code:** `N3097`

A generator's return type does not match the declared or inferred return type.

---

