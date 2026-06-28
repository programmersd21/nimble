/// Comprehensive error code taxonomy for the Nimble compiler.
///
/// Each error code is a stable, unique identifier grouped by subsystem:
///
/// | Range       | Subsystem                    |
/// |-------------|------------------------------|
/// | N0001-N0999 | Lexer                        |
/// | N1001-N1999 | Parser                       |
/// | N2001-N2999 | Name Resolution / Imports    |
/// | N3001-N3999 | Type System / Type Checking  |
/// | N4001-N4999 | Module / Import System       |
/// | N5001-N5999 | Lint / Warnings              |
/// | N6001-N6999 | Code Generation / Backend    |
/// | N7001-N7999 | Runtime / Panic              |
/// | N8001-N8999 | Configuration / Build System |
/// | N9001-N9999 | Internal Compiler Errors     |
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum ErrorCode {
    // ╔══════════════════════════════════════════════════════════════╗
    // ║  LEXER  (N0001–N0999)                                       ║
    // ╚══════════════════════════════════════════════════════════════╝
    N0001, // Illegal tab character
    N0002, // Unexpected character
    N0003, // Unmatched closing delimiter
    N0004, // Invalid float literal
    N0005, // Integer literal out of range
    N0006, // Unterminated string literal
    N0007, // Invalid escape sequence
    N0008, // Newline inside string literal
    N0009, // Indentation error
    N0010, // Empty character literal
    N0011, // Multi-byte character literal
    N0012, // Unicode escape in non-unicode context
    N0013, // Invalid numeric suffix
    N0014, // Leading zeros in decimal integer
    N0015, // Binary literal overflow
    N0016, // Hex literal overflow
    N0017, // Octal literal overflow
    N0018, // Invalid binary literal format
    N0019, // Invalid hex literal format
    N0020, // Invalid octal literal format
    N0021, // Unterminated block comment
    N0022, // Unrecognized token in string interpolation
    N0023, // Invalid unicode identifier start
    N0024, // Non-printable character in source
    N0025, // Byte order mark detected
    N0026, // Null character in source
    N0027, // String literal exceeds maximum length
    N0028, // Empty unicode escape sequence
    N0029, // Malformed unicode escape sequence
    N0030, // Unicode codepoint out of range
    N0031, // Reserved keyword used as identifier
    N0032, // Non-standard line ending
    N0033, // Mixed tabs and spaces
    N0034, // Digit separator at wrong position
    N0035, // Consecutive digit separators
    N0036, // Trailing digit separator
    N0037, // Leading digit separator
    N0038, // Invalid digit for radix
    N0039, // Unterminated raw string literal

    // ╔══════════════════════════════════════════════════════════════╗
    // ║  PARSER  (N1001–N1999)                                      ║
    // ╚══════════════════════════════════════════════════════════════╝
    N1001, // Expected specific token but found something else
    N1002, // Unexpected token at this position
    N1003, // Expected expression
    N1004, // Unclosed parenthesis or delimiter
    N1005, // Expected indented block after colon
    N1006, // Unexpected indentation
    N1007, // Expected identifier
    N1008, // Expected type name
    N1009, // Expected parameter name
    N1010, // Expected colon `:`
    N1011, // Expected semicolon `;`
    N1012, // Expected equals sign `=`
    N1013, // Expected arrow `->`
    N1014, // Expected comma `,`
    N1015, // Expected dot `.`
    N1016, // Missing function body
    N1017, // Missing return type or return expression
    N1018, // Invalid function parameter
    N1019, // Too many function parameters
    N1020, // Too few arguments in call
    N1021, // Expected statement
    N1022, // Expected binding (let/var)
    N1023, // Expected keyword
    N1024, // Invalid left-hand side of assignment
    N1025, // Nested function without proper closure context
    N1026, // Duplicate parameter name in function signature
    N1027, // Default parameter value before non-default
    N1028, // Expected module path
    N1029, // Expected import symbol
    N1030, // Circular import
    N1031, // Break outside of loop
    N1032, // Continue outside of loop
    N1033, // Return outside of function
    N1034, // Yield outside of generator
    N1035, // Invalid for-loop binding
    N1036, // Expected `in` keyword in for-loop
    N1037, // Empty struct body
    N1038, // Empty interface body
    N1039, // Method declaration outside of interface
    N1040, // Duplicate field name in struct
    N1041, // Unnamed field in struct literal
    N1042, // Expected struct expression
    N1043, // Missing colon in type annotation
    N1044, // Unexpected trailing comma
    N1045, // Malformed string interpolation expression
    N1046, // Unterminated block comment
    N1047, // Expected attribute
    N1048, // Invalid attribute target
    N1049, // Duplicate attribute
    N1050, // Unknown attribute
    N1051, // Expected type arguments
    N1052, // Unclosed type argument list
    N1053, // Type argument count mismatch
    N1054, // Expected `<` for generic arguments
    N1055, // Expected `>` for generic arguments
    N1056, // Unterminated lambda body
    N1057, // Expected binding in for-loop
    N1058, // Expected path expression
    N1059, // Expected literal
    N1060, // Expected pattern
    N1061, // Expected guard expression
    N1062, // Expected where clause
    N1063, // Expected semicolon or newline after statement
    N1064, // Expected operator
    N1065, // Invalid prefix operator
    N1066, // Invalid postfix operator
    N1067, // Operator precedence ambiguity
    N1068, // Comparison chaining with incompatible operators
    N1069, // Expected tuple element
    N1070, // Expected array element
    N1071, // Expected struct field
    N1072, // Expected enum variant
    N1073, // Expected match arm
    N1074, // Expected fat arrow `=>` in match arm
    N1075, // Expected pattern guard
    N1076, // Invalid doc comment placement
    N1077, // Expected doc comment
    N1078, // Invalid visibility modifier
    N1079, // Expected item (function, struct, etc.)
    N1080, // Nested function without body
    N1081, // Unterminated generic list
    N1082, // Expected lifetime parameter
    N1083, // Expected const parameter
    N1084, // Ambiguous literal suffix

    // ╔══════════════════════════════════════════════════════════════╗
    // ║  NAME RESOLUTION / IMPORTS  (N2001–N2999)                   ║
    // ╚══════════════════════════════════════════════════════════════╝
    N2001, // Undefined variable
    N2002, // Duplicate definition
    N2003, // Undefined function
    N2004, // Undefined struct
    N2005, // Undefined interface
    N2006, // Undefined module
    N2007, // Undefined type
    N2008, // Undefined macro
    N2009, // Access to private item
    N2010, // Cyclic module dependency detected
    N2011, // Cyclic type definition
    N2012, // Ambiguous name — multiple matching definitions
    N2013, // Invalid visibility qualifier
    N2014, // Module not found
    N2015, // Symbol is not exported from module
    N2016, // Name conflicts with builtin
    N2017, // Name shadows a builtin
    N2018, // Unused import
    N2019, // Wildcard import leaks names into scope
    N2020, // `self` used outside of method
    N2021, // `super` used outside of class context
    N2022, // Invalid self parameter type or position
    N2023, // Method without self parameter
    N2024, // Return type mismatch in method implementation
    N2025, // Missing required interface method implementation
    N2026, // Extra method not declared in interface
    N2027, // Invalid method override
    N2028, // Override without base definition
    N2029, // Inconsistent associated type binding
    N2030, // Circular trait bound
    N2031, // Unused variable (warning)
    N2032, // Unused assignment
    N2033, // Variable shadows outer variable
    N2034, // Unreachable pattern
    N2035, // Non-exhaustive patterns
    N2036, // Pattern binding conflict
    N2037, // Illegal binding mode in pattern
    N2038, // Invalid pattern syntax
    N2039, // Unresolved import
    N2040, // Unresolved re-export
    N2041, // Private re-export
    N2042, // Conflicting re-export
    N2043, // Re-export of non-existent symbol
    N2044, // Self-import (module imports itself)
    N2045, // Invalid use of `Self` type alias
    N2046, // External crate not found
    N2047, // External crate version conflict
    N2048, // External crate feature not found
    N2049, // Unused extern crate

    // ╔══════════════════════════════════════════════════════════════╗
    // ║  TYPE SYSTEM / TYPE CHECKING  (N3001–N3999)                 ║
    // ╚══════════════════════════════════════════════════════════════╝
    N3001, // Type mismatch
    N3002, // Assign to immutable variable
    N3003, // Undefined type
    N3004, // Call of non-function value
    N3005, // Argument count mismatch
    N3006, // Missing required method / interface unsatisfied
    N3007, // Recursive type without indirection
    N3008, // Infinite type / recursive unification
    N3009, // Cannot infer type — annotation needed
    N3010, // Type annotation required
    N3011, // Expected concrete type, found abstract type
    N3012, // Trait bound not satisfied
    N3013, // Associated type not specified
    N3014, // Wrong number of type arguments
    N3015, // Type argument out of bounds
    N3016, // Conflicting type arguments
    N3017, // Cross-module type violation
    N3018, // Attempted to borrow immutable variable as mutable
    N3019, // Borrow of moved value
    N3020, // Use after move
    N3021, // Double borrow / multiple mutable borrows
    N3022, // Lifetime mismatch
    N3023, // Lifetime elision failure — ambiguous
    N3024, // Lifetime bound not satisfied
    N3025, // Lifetime constraint violation
    N3026, // Missing lifetime annotation
    N3027, // Invalid lifetime name
    N3028, // Mismatched mutability in reference
    N3029, // Dangling reference
    N3030, // Drop of type with move semantics
    N3031, // Borrow of constant value
    N3032, // Numeric overflow in constant expression
    N3033, // Division by zero in constant expression
    N3034, // Remainder by zero in constant expression
    N3035, // Negation of unsigned integer
    N3036, // Shift exceeds bit width of type
    N3037, // Operator not applicable to given types
    N3038, // Comparison of unordered values
    N3039, // Invalid unary operator for type
    N3040, // Invalid binary operator for types
    N3041, // No common overload for operator
    N3042, // Ambiguous operator application
    N3043, // Wrong number of generic type parameters
    N3044, // Generic parameter bound not satisfied
    N3045, // Missing generic type annotation
    N3046, // Generic parameter not used in signature
    N3047, // Concrete type used where abstract expected
    N3048, // Abstract type used where concrete expected
    N3049, // Non-constant value in const context
    N3050, // Non-const function call in const context
    N3051, // Mutable reference in const context
    N3052, // If condition must be boolean
    N3053, // While condition must be boolean
    N3054, // For-each binding type mismatch
    N3055, // Return type mismatch with function signature
    N3056, // Missing return value
    N3057, // Extra return value (returning from void function)
    N3058, // Return not allowed in this context
    N3059, // Missing async annotation on function
    N3060, // Mismatched await — cannot await non-future
    N3061, // Incompatible implicit type conversion
    N3062, // Forward declaration type mismatch
    N3063, // Missing forward declaration
    N3064, // Field type mismatch in struct literal
    N3065, // Missing field in struct literal
    N3066, // Extra field in struct literal
    N3067, // Ambiguous field in struct literal
    N3068, // Cyclic struct definition
    N3069, // Tuple index out of bounds
    N3070, // Array index out of bounds (constant)
    N3071, // Non-comptime array index
    N3072, // Mismatched array length in type
    N3073, // Type alias cycle
    N3074, // Type alias uses non-existent type
    N3075, // Unsized type in field
    N3076, // Unsized type in local variable
    N3077, // Unsized type in function parameter
    N3078, // Unsized type in return position
    N3079, // Unsized type in struct field
    N3080, // Unexpected type parameter
    N3081, // Expected type parameter
    N3082, // Invalid discriminant type for enum
    N3083, // Duplicate enum discriminant value
    N3084, // Enum discriminant overflow
    N3085, // Non-exhaustive enum match
    N3086, // Unreachable match arm
    N3087, // Overlapping match patterns
    N3088, // Invalid ref pattern
    N3089, // Invalid mut pattern
    N3090, // Pattern requires unit type
    N3091, // Closure with non-closure context
    N3092, // Closure captures disjoint variables
    N3093, // Non-copy type in closure by copy
    N3094, // Mismatched async closure
    N3095, // Generator resume type mismatch
    N3096, // Generator yield type mismatch
    N3097, // Generator return type mismatch

    // ╔══════════════════════════════════════════════════════════════╗
    // ║  MODULE / IMPORT SYSTEM  (N4001–N4999)                      ║
    // ╚══════════════════════════════════════════════════════════════╝
    N4001, // Module not found
    N4002, // File not found
    N4003, // Circular module dependency
    N4004, // Symbol not exported
    N4005, // Import cycle detected
    N4006, // Ambiguous import — symbol found in multiple modules
    N4007, // Shadowed import — name conflicts with another import
    N4008, // Wildcard import naming conflict
    N4009, // Relative import beyond package root
    N4010, // Invalid module name
    N4011, // Module not found in search path
    N4012, // Module parse error
    N4013, // Module type checking error
    N4014, // Dependency not found
    N4015, // Dependency cycle
    N4016, // Dependency version conflict
    N4017, // Broken package structure
    N4018, // Missing manifest file
    N4019, // Invalid manifest format
    N4020, // Manifest syntax error
    N4021, // Manifest missing required field
    N4022, // Manifest contains duplicate entry
    N4023, // Module compiled with different compiler version
    N4024, // Module compiled with incompatible flags
    N4025, // Module does not contain expected interface
    N4026, // Recursive module loading
    N4027, // Module path exceeds maximum nesting depth

    // ╔══════════════════════════════════════════════════════════════╗
    // ║  LINT / WARNINGS  (N5001–N5999)                             ║
    // ╚══════════════════════════════════════════════════════════════╝
    N5001, // Unused variable
    N5002, // Unused import
    N5003, // Unused assignment
    N5004, // Unused function
    N5005, // Unused struct
    N5006, // Unused type
    N5007, // Dead code
    N5008, // Unreachable code
    N5009, // Empty loop body
    N5010, // Suspicious assignment in conditional
    N5011, // Lossy implicit type conversion
    N5012, // Deprecated item usage
    N5013, // Missing documentation
    N5014, // Non-standard naming convention
    N5015, // Name shadows outer item
    N5016, // Unnecessary closure
    N5017, // Redundant pattern
    N5018, // Missing else branch
    N5019, // Deep nesting
    N5020, // Overly complex expression
    N5021, // Cognitive complexity too high
    N5022, // Cyclomatic complexity too high
    N5023, // Too many function parameters
    N5024, // Too many return types
    N5025, // Function too long
    N5026, // Source file too long
    N5027, // Line exceeds maximum length
    N5028, // Inconsistent naming style
    N5029, // Non-canonical ordering
    N5030, // Unsafe block used
    N5031, // Unsafe function declaration
    N5032, // Unnecessary unsafe block
    N5033, // Comparing boolean literal
    N5034, // Assigning boolean literal in conditional
    N5035, // Negating boolean literal
    N5036, // Nested conditional
    N5037, // Constant condition in if/while
    N5038, // Redundant type cast
    N5039, // Suspicious comparison (e.g. `x == x`)
    N5040, // Infinite loop detected (constant condition)
    N5041, // Missing break in loop
    N5042, // Use of uninitialized variable
    N5043, // Possibly uninitialized variable
    N5044, // Fallthrough in switch/match
    N5045, // Missing case in match
    N5046, // Redundant default case
    N5047, // Unnecessary `else if` (use `else`)
    N5048, // Redundant else branch
    N5049, // Empty else branch
    N5050, // Unnecessary parentheses
    N5051, // Unnecessary return at end of function
    N5052, // Unnecessary semicolon
    N5053, // Empty statement
    N5054, // Statement with no effect
    N5055, // Variable assigned but not used
    N5056, // Function argument reassigned
    N5057, // Mutable variable could be immutable
    N5058, // Redundant field name in struct literal
    N5059, // Unnecessary qualification
    N5060, // Module naming convention violation
    N5061, // Non-idiomatic code pattern

    // ╔══════════════════════════════════════════════════════════════╗
    // ║  CODE GENERATION / BACKEND  (N6001–N6999)                   ║
    // ╚══════════════════════════════════════════════════════════════╝
    N6001, // Code generation failure
    N6002, // Unsupported feature for target
    N6003, // Linker error
    N6004, // Assembly generation error
    N6005, // Target platform not supported
    N6006, // Invalid optimization level
    N6007, // Invalid debug info level
    N6008, // Inline assembly error
    N6009, // Compiler intrinsic error
    N6010, // Stack overflow during codegen
    N6011, // Global offset overflow
    N6012, // Jump table overflow
    N6013, // Too many static variables
    N6014, // Too many functions in compilation unit
    N6015, // Function too large for codegen
    N6016, // External symbol not found
    N6017, // Duplicate symbol export
    N6018, // Undefined symbol in object code
    N6019, // Relocation overflow
    N6020, // Thread-local storage not supported on target
    N6021, // ABI mismatch with external function
    N6022, // Required CPU feature not available
    N6023, // Required OS feature not available
    N6024, // Inline assembly constraint violation
    N6025, // Intrinsic signature mismatch
    N6026, // Vector type not supported on target
    N6027, // Atomic operation not supported on target
    N6028, // SIMD operation not supported on target
    N6029, // Codegen buffer overflow
    N6030, // Unsupported calling convention
    N6031, // Too many locals in function
    N6032, // Section attribute conflict
    N6033, // Link once group conflict
    N6034, // Visibility attribute conflict

    // ╔══════════════════════════════════════════════════════════════╗
    // ║  RUNTIME / PANIC  (N7001–N7999)                             ║
    // ╚══════════════════════════════════════════════════════════════╝
    N7001, // Program panicked
    N7002, // Index out of bounds
    N7003, // Stack overflow
    N7004, // Arithmetic overflow
    N7005, // Division by zero
    N7006, // Null pointer dereference
    N7007, // Unwrap of None value
    N7008, // Unwrap of error result
    N7009, // Out of memory
    N7010, // Assertion failed
    N7011, // Unreachable code executed
    N7012, // TODO / stub encountered at runtime
    N7013, // Unimplemented functionality
    N7014, // Buffer overflow
    N7015, // Invalid UTF-8 sequence
    N7016, // Integer conversion overflow
    N7017, // Float conversion overflow
    N7018, // Negative index used
    N7019, // Invalid enum discriminant
    N7020, // Type cast error at runtime
    N7021, // Recursive call overflow
    N7022, // Invalid allocator state
    N7023, // Double free detected
    N7024, // Use after free
    N7025, // Mutex poison error
    N7026, // Channel closed unexpectedly
    N7027, // Timeout exceeded
    N7028, // IO error
    N7029, // Network error

    // ╔══════════════════════════════════════════════════════════════╗
    // ║  CONFIGURATION / BUILD SYSTEM  (N8001–N8999)                ║
    // ╚══════════════════════════════════════════════════════════════╝
    N8001, // Configuration parse error
    N8002, // Configuration missing required field
    N8003, // Configuration contains invalid value
    N8004, // Build target not found
    N8005, // Build script execution error
    N8006, // Missing build dependency
    N8007, // Invalid build profile
    N8008, // Invalid manifest file
    N8009, // Manifest missing package name
    N8010, // Manifest missing version
    N8011, // Manifest contains duplicate entry
    N8012, // Manifest contains invalid dependency
    N8013, // Workspace member not found
    N8014, // Workspace contains duplicate member
    N8015, // Invalid toolchain specification
    N8016, // Toolchain not installed
    N8017, // Invalid target triple
    N8018, // Test execution failed
    N8019, // Benchmark execution failed
    N8020, // Missing test configuration
    N8021, // Invalid compiler flag
    N8022, // Conflicting compiler flags
    N8023, // Unsupported compiler flag for target
    N8024, // Invalid linker flag
    N8025, // Missing linker
    N8026, // Missing assembler
    N8027, // Output path not writable
    N8028, // Cache directory not accessible
    N8029, // Concurrent build conflict
    N8030, // Build system internal error
    N8031, // Invalid package name
    N8032, // Package name contains invalid characters
    N8033, // Package version format invalid
    N8034, // Package license not recognized
    N8035, // Missing package license
    N8036, // Missing package description
    N8037, // Invalid edition / language version
    N8038, // Feature flag not recognized
    N8039, // Feature flag conflict

    // ╔══════════════════════════════════════════════════════════════╗
    // ║  INTERNAL COMPILER ERRORS  (N9001–N9999)                    ║
    // ╚══════════════════════════════════════════════════════════════╝
    N9001, // Internal compiler error (ICE)
    N9002, // Internal bug — please report
    N9003, // Unreachable code path in compiler
    N9004, // Unimplemented compiler feature
    N9005, // Compiler assertion failure
    N9006, // Compiler invariant violation
    N9007, // Type checker invariant failure
    N9008, // Name resolution invariant failure
    N9009, // Code generation invariant failure
    N9010, // Compiler data structure corruption
    N9011, // Missing compiler analysis pass
    N9012, // Compiler pass cycle
    N9013, // Compiler query cycle
    N9014, // Incremental compilation cache mismatch
    N9015, // Incremental compilation fingerprint conflict
    N9016, // AST validation failure
    N9017, // HIR validation failure
    N9018, // MIR validation failure
    N9019, // LLVM / backend error
    N9020, // Memory allocation failure in compiler
    N9021, // Thread panic in compiler worker
    N9022, // Compiler resource limit exceeded
    N9023, // Compiler input/output error
    N9024, // Compiler timeout
    N9025, // Invalid compiler state during incremental reuse
}

impl ErrorCode {
    /// Returns the string representation (e.g. "N0001").
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::N0001 => "N0001",
            Self::N0002 => "N0002",
            Self::N0003 => "N0003",
            Self::N0004 => "N0004",
            Self::N0005 => "N0005",
            Self::N0006 => "N0006",
            Self::N0007 => "N0007",
            Self::N0008 => "N0008",
            Self::N0009 => "N0009",
            Self::N0010 => "N0010",
            Self::N0011 => "N0011",
            Self::N0012 => "N0012",
            Self::N0013 => "N0013",
            Self::N0014 => "N0014",
            Self::N0015 => "N0015",
            Self::N0016 => "N0016",
            Self::N0017 => "N0017",
            Self::N0018 => "N0018",
            Self::N0019 => "N0019",
            Self::N0020 => "N0020",
            Self::N0021 => "N0021",
            Self::N0022 => "N0022",
            Self::N0023 => "N0023",
            Self::N0024 => "N0024",
            Self::N0025 => "N0025",
            Self::N0026 => "N0026",
            Self::N0027 => "N0027",
            Self::N0028 => "N0028",
            Self::N0029 => "N0029",
            Self::N0030 => "N0030",
            Self::N0031 => "N0031",
            Self::N0032 => "N0032",
            Self::N0033 => "N0033",
            Self::N0034 => "N0034",
            Self::N0035 => "N0035",
            Self::N0036 => "N0036",
            Self::N0037 => "N0037",
            Self::N0038 => "N0038",
            Self::N0039 => "N0039",
            Self::N1001 => "N1001",
            Self::N1002 => "N1002",
            Self::N1003 => "N1003",
            Self::N1004 => "N1004",
            Self::N1005 => "N1005",
            Self::N1006 => "N1006",
            Self::N1007 => "N1007",
            Self::N1008 => "N1008",
            Self::N1009 => "N1009",
            Self::N1010 => "N1010",
            Self::N1011 => "N1011",
            Self::N1012 => "N1012",
            Self::N1013 => "N1013",
            Self::N1014 => "N1014",
            Self::N1015 => "N1015",
            Self::N1016 => "N1016",
            Self::N1017 => "N1017",
            Self::N1018 => "N1018",
            Self::N1019 => "N1019",
            Self::N1020 => "N1020",
            Self::N1021 => "N1021",
            Self::N1022 => "N1022",
            Self::N1023 => "N1023",
            Self::N1024 => "N1024",
            Self::N1025 => "N1025",
            Self::N1026 => "N1026",
            Self::N1027 => "N1027",
            Self::N1028 => "N1028",
            Self::N1029 => "N1029",
            Self::N1030 => "N1030",
            Self::N1031 => "N1031",
            Self::N1032 => "N1032",
            Self::N1033 => "N1033",
            Self::N1034 => "N1034",
            Self::N1035 => "N1035",
            Self::N1036 => "N1036",
            Self::N1037 => "N1037",
            Self::N1038 => "N1038",
            Self::N1039 => "N1039",
            Self::N1040 => "N1040",
            Self::N1041 => "N1041",
            Self::N1042 => "N1042",
            Self::N1043 => "N1043",
            Self::N1044 => "N1044",
            Self::N1045 => "N1045",
            Self::N1046 => "N1046",
            Self::N1047 => "N1047",
            Self::N1048 => "N1048",
            Self::N1049 => "N1049",
            Self::N1050 => "N1050",
            Self::N1051 => "N1051",
            Self::N1052 => "N1052",
            Self::N1053 => "N1053",
            Self::N1054 => "N1054",
            Self::N1055 => "N1055",
            Self::N1056 => "N1056",
            Self::N1057 => "N1057",
            Self::N1058 => "N1058",
            Self::N1059 => "N1059",
            Self::N1060 => "N1060",
            Self::N1061 => "N1061",
            Self::N1062 => "N1062",
            Self::N1063 => "N1063",
            Self::N1064 => "N1064",
            Self::N1065 => "N1065",
            Self::N1066 => "N1066",
            Self::N1067 => "N1067",
            Self::N1068 => "N1068",
            Self::N1069 => "N1069",
            Self::N1070 => "N1070",
            Self::N1071 => "N1071",
            Self::N1072 => "N1072",
            Self::N1073 => "N1073",
            Self::N1074 => "N1074",
            Self::N1075 => "N1075",
            Self::N1076 => "N1076",
            Self::N1077 => "N1077",
            Self::N1078 => "N1078",
            Self::N1079 => "N1079",
            Self::N1080 => "N1080",
            Self::N1081 => "N1081",
            Self::N1082 => "N1082",
            Self::N1083 => "N1083",
            Self::N1084 => "N1084",
            Self::N2001 => "N2001",
            Self::N2002 => "N2002",
            Self::N2003 => "N2003",
            Self::N2004 => "N2004",
            Self::N2005 => "N2005",
            Self::N2006 => "N2006",
            Self::N2007 => "N2007",
            Self::N2008 => "N2008",
            Self::N2009 => "N2009",
            Self::N2010 => "N2010",
            Self::N2011 => "N2011",
            Self::N2012 => "N2012",
            Self::N2013 => "N2013",
            Self::N2014 => "N2014",
            Self::N2015 => "N2015",
            Self::N2016 => "N2016",
            Self::N2017 => "N2017",
            Self::N2018 => "N2018",
            Self::N2019 => "N2019",
            Self::N2020 => "N2020",
            Self::N2021 => "N2021",
            Self::N2022 => "N2022",
            Self::N2023 => "N2023",
            Self::N2024 => "N2024",
            Self::N2025 => "N2025",
            Self::N2026 => "N2026",
            Self::N2027 => "N2027",
            Self::N2028 => "N2028",
            Self::N2029 => "N2029",
            Self::N2030 => "N2030",
            Self::N2031 => "N2031",
            Self::N2032 => "N2032",
            Self::N2033 => "N2033",
            Self::N2034 => "N2034",
            Self::N2035 => "N2035",
            Self::N2036 => "N2036",
            Self::N2037 => "N2037",
            Self::N2038 => "N2038",
            Self::N2039 => "N2039",
            Self::N2040 => "N2040",
            Self::N2041 => "N2041",
            Self::N2042 => "N2042",
            Self::N2043 => "N2043",
            Self::N2044 => "N2044",
            Self::N2045 => "N2045",
            Self::N2046 => "N2046",
            Self::N2047 => "N2047",
            Self::N2048 => "N2048",
            Self::N2049 => "N2049",
            Self::N3001 => "N3001",
            Self::N3002 => "N3002",
            Self::N3003 => "N3003",
            Self::N3004 => "N3004",
            Self::N3005 => "N3005",
            Self::N3006 => "N3006",
            Self::N3007 => "N3007",
            Self::N3008 => "N3008",
            Self::N3009 => "N3009",
            Self::N3010 => "N3010",
            Self::N3011 => "N3011",
            Self::N3012 => "N3012",
            Self::N3013 => "N3013",
            Self::N3014 => "N3014",
            Self::N3015 => "N3015",
            Self::N3016 => "N3016",
            Self::N3017 => "N3017",
            Self::N3018 => "N3018",
            Self::N3019 => "N3019",
            Self::N3020 => "N3020",
            Self::N3021 => "N3021",
            Self::N3022 => "N3022",
            Self::N3023 => "N3023",
            Self::N3024 => "N3024",
            Self::N3025 => "N3025",
            Self::N3026 => "N3026",
            Self::N3027 => "N3027",
            Self::N3028 => "N3028",
            Self::N3029 => "N3029",
            Self::N3030 => "N3030",
            Self::N3031 => "N3031",
            Self::N3032 => "N3032",
            Self::N3033 => "N3033",
            Self::N3034 => "N3034",
            Self::N3035 => "N3035",
            Self::N3036 => "N3036",
            Self::N3037 => "N3037",
            Self::N3038 => "N3038",
            Self::N3039 => "N3039",
            Self::N3040 => "N3040",
            Self::N3041 => "N3041",
            Self::N3042 => "N3042",
            Self::N3043 => "N3043",
            Self::N3044 => "N3044",
            Self::N3045 => "N3045",
            Self::N3046 => "N3046",
            Self::N3047 => "N3047",
            Self::N3048 => "N3048",
            Self::N3049 => "N3049",
            Self::N3050 => "N3050",
            Self::N3051 => "N3051",
            Self::N3052 => "N3052",
            Self::N3053 => "N3053",
            Self::N3054 => "N3054",
            Self::N3055 => "N3055",
            Self::N3056 => "N3056",
            Self::N3057 => "N3057",
            Self::N3058 => "N3058",
            Self::N3059 => "N3059",
            Self::N3060 => "N3060",
            Self::N3061 => "N3061",
            Self::N3062 => "N3062",
            Self::N3063 => "N3063",
            Self::N3064 => "N3064",
            Self::N3065 => "N3065",
            Self::N3066 => "N3066",
            Self::N3067 => "N3067",
            Self::N3068 => "N3068",
            Self::N3069 => "N3069",
            Self::N3070 => "N3070",
            Self::N3071 => "N3071",
            Self::N3072 => "N3072",
            Self::N3073 => "N3073",
            Self::N3074 => "N3074",
            Self::N3075 => "N3075",
            Self::N3076 => "N3076",
            Self::N3077 => "N3077",
            Self::N3078 => "N3078",
            Self::N3079 => "N3079",
            Self::N3080 => "N3080",
            Self::N3081 => "N3081",
            Self::N3082 => "N3082",
            Self::N3083 => "N3083",
            Self::N3084 => "N3084",
            Self::N3085 => "N3085",
            Self::N3086 => "N3086",
            Self::N3087 => "N3087",
            Self::N3088 => "N3088",
            Self::N3089 => "N3089",
            Self::N3090 => "N3090",
            Self::N3091 => "N3091",
            Self::N3092 => "N3092",
            Self::N3093 => "N3093",
            Self::N3094 => "N3094",
            Self::N3095 => "N3095",
            Self::N3096 => "N3096",
            Self::N3097 => "N3097",
            Self::N4001 => "N4001",
            Self::N4002 => "N4002",
            Self::N4003 => "N4003",
            Self::N4004 => "N4004",
            Self::N4005 => "N4005",
            Self::N4006 => "N4006",
            Self::N4007 => "N4007",
            Self::N4008 => "N4008",
            Self::N4009 => "N4009",
            Self::N4010 => "N4010",
            Self::N4011 => "N4011",
            Self::N4012 => "N4012",
            Self::N4013 => "N4013",
            Self::N4014 => "N4014",
            Self::N4015 => "N4015",
            Self::N4016 => "N4016",
            Self::N4017 => "N4017",
            Self::N4018 => "N4018",
            Self::N4019 => "N4019",
            Self::N4020 => "N4020",
            Self::N4021 => "N4021",
            Self::N4022 => "N4022",
            Self::N4023 => "N4023",
            Self::N4024 => "N4024",
            Self::N4025 => "N4025",
            Self::N4026 => "N4026",
            Self::N4027 => "N4027",
            Self::N5001 => "N5001",
            Self::N5002 => "N5002",
            Self::N5003 => "N5003",
            Self::N5004 => "N5004",
            Self::N5005 => "N5005",
            Self::N5006 => "N5006",
            Self::N5007 => "N5007",
            Self::N5008 => "N5008",
            Self::N5009 => "N5009",
            Self::N5010 => "N5010",
            Self::N5011 => "N5011",
            Self::N5012 => "N5012",
            Self::N5013 => "N5013",
            Self::N5014 => "N5014",
            Self::N5015 => "N5015",
            Self::N5016 => "N5016",
            Self::N5017 => "N5017",
            Self::N5018 => "N5018",
            Self::N5019 => "N5019",
            Self::N5020 => "N5020",
            Self::N5021 => "N5021",
            Self::N5022 => "N5022",
            Self::N5023 => "N5023",
            Self::N5024 => "N5024",
            Self::N5025 => "N5025",
            Self::N5026 => "N5026",
            Self::N5027 => "N5027",
            Self::N5028 => "N5028",
            Self::N5029 => "N5029",
            Self::N5030 => "N5030",
            Self::N5031 => "N5031",
            Self::N5032 => "N5032",
            Self::N5033 => "N5033",
            Self::N5034 => "N5034",
            Self::N5035 => "N5035",
            Self::N5036 => "N5036",
            Self::N5037 => "N5037",
            Self::N5038 => "N5038",
            Self::N5039 => "N5039",
            Self::N5040 => "N5040",
            Self::N5041 => "N5041",
            Self::N5042 => "N5042",
            Self::N5043 => "N5043",
            Self::N5044 => "N5044",
            Self::N5045 => "N5045",
            Self::N5046 => "N5046",
            Self::N5047 => "N5047",
            Self::N5048 => "N5048",
            Self::N5049 => "N5049",
            Self::N5050 => "N5050",
            Self::N5051 => "N5051",
            Self::N5052 => "N5052",
            Self::N5053 => "N5053",
            Self::N5054 => "N5054",
            Self::N5055 => "N5055",
            Self::N5056 => "N5056",
            Self::N5057 => "N5057",
            Self::N5058 => "N5058",
            Self::N5059 => "N5059",
            Self::N5060 => "N5060",
            Self::N5061 => "N5061",
            Self::N6001 => "N6001",
            Self::N6002 => "N6002",
            Self::N6003 => "N6003",
            Self::N6004 => "N6004",
            Self::N6005 => "N6005",
            Self::N6006 => "N6006",
            Self::N6007 => "N6007",
            Self::N6008 => "N6008",
            Self::N6009 => "N6009",
            Self::N6010 => "N6010",
            Self::N6011 => "N6011",
            Self::N6012 => "N6012",
            Self::N6013 => "N6013",
            Self::N6014 => "N6014",
            Self::N6015 => "N6015",
            Self::N6016 => "N6016",
            Self::N6017 => "N6017",
            Self::N6018 => "N6018",
            Self::N6019 => "N6019",
            Self::N6020 => "N6020",
            Self::N6021 => "N6021",
            Self::N6022 => "N6022",
            Self::N6023 => "N6023",
            Self::N6024 => "N6024",
            Self::N6025 => "N6025",
            Self::N6026 => "N6026",
            Self::N6027 => "N6027",
            Self::N6028 => "N6028",
            Self::N6029 => "N6029",
            Self::N6030 => "N6030",
            Self::N6031 => "N6031",
            Self::N6032 => "N6032",
            Self::N6033 => "N6033",
            Self::N6034 => "N6034",
            Self::N7001 => "N7001",
            Self::N7002 => "N7002",
            Self::N7003 => "N7003",
            Self::N7004 => "N7004",
            Self::N7005 => "N7005",
            Self::N7006 => "N7006",
            Self::N7007 => "N7007",
            Self::N7008 => "N7008",
            Self::N7009 => "N7009",
            Self::N7010 => "N7010",
            Self::N7011 => "N7011",
            Self::N7012 => "N7012",
            Self::N7013 => "N7013",
            Self::N7014 => "N7014",
            Self::N7015 => "N7015",
            Self::N7016 => "N7016",
            Self::N7017 => "N7017",
            Self::N7018 => "N7018",
            Self::N7019 => "N7019",
            Self::N7020 => "N7020",
            Self::N7021 => "N7021",
            Self::N7022 => "N7022",
            Self::N7023 => "N7023",
            Self::N7024 => "N7024",
            Self::N7025 => "N7025",
            Self::N7026 => "N7026",
            Self::N7027 => "N7027",
            Self::N7028 => "N7028",
            Self::N7029 => "N7029",
            Self::N8001 => "N8001",
            Self::N8002 => "N8002",
            Self::N8003 => "N8003",
            Self::N8004 => "N8004",
            Self::N8005 => "N8005",
            Self::N8006 => "N8006",
            Self::N8007 => "N8007",
            Self::N8008 => "N8008",
            Self::N8009 => "N8009",
            Self::N8010 => "N8010",
            Self::N8011 => "N8011",
            Self::N8012 => "N8012",
            Self::N8013 => "N8013",
            Self::N8014 => "N8014",
            Self::N8015 => "N8015",
            Self::N8016 => "N8016",
            Self::N8017 => "N8017",
            Self::N8018 => "N8018",
            Self::N8019 => "N8019",
            Self::N8020 => "N8020",
            Self::N8021 => "N8021",
            Self::N8022 => "N8022",
            Self::N8023 => "N8023",
            Self::N8024 => "N8024",
            Self::N8025 => "N8025",
            Self::N8026 => "N8026",
            Self::N8027 => "N8027",
            Self::N8028 => "N8028",
            Self::N8029 => "N8029",
            Self::N8030 => "N8030",
            Self::N8031 => "N8031",
            Self::N8032 => "N8032",
            Self::N8033 => "N8033",
            Self::N8034 => "N8034",
            Self::N8035 => "N8035",
            Self::N8036 => "N8036",
            Self::N8037 => "N8037",
            Self::N8038 => "N8038",
            Self::N8039 => "N8039",
            Self::N9001 => "N9001",
            Self::N9002 => "N9002",
            Self::N9003 => "N9003",
            Self::N9004 => "N9004",
            Self::N9005 => "N9005",
            Self::N9006 => "N9006",
            Self::N9007 => "N9007",
            Self::N9008 => "N9008",
            Self::N9009 => "N9009",
            Self::N9010 => "N9010",
            Self::N9011 => "N9011",
            Self::N9012 => "N9012",
            Self::N9013 => "N9013",
            Self::N9014 => "N9014",
            Self::N9015 => "N9015",
            Self::N9016 => "N9016",
            Self::N9017 => "N9017",
            Self::N9018 => "N9018",
            Self::N9019 => "N9019",
            Self::N9020 => "N9020",
            Self::N9021 => "N9021",
            Self::N9022 => "N9022",
            Self::N9023 => "N9023",
            Self::N9024 => "N9024",
            Self::N9025 => "N9025",
        }
    }

    /// Parse an error code from its string representation.
    pub fn parse_str(s: &str) -> Option<Self> {
        // Use the as_str output as canonical repr
        let upper = s.to_uppercase();
        // Walk all variants via as_str
        // We define a static list for efficiency
        const ALL: &[ErrorCode] = &[
            ErrorCode::N0001,
            ErrorCode::N0002,
            ErrorCode::N0003,
            ErrorCode::N0004,
            ErrorCode::N0005,
            ErrorCode::N0006,
            ErrorCode::N0007,
            ErrorCode::N0008,
            ErrorCode::N0009,
            ErrorCode::N0010,
            ErrorCode::N0011,
            ErrorCode::N0012,
            ErrorCode::N0013,
            ErrorCode::N0014,
            ErrorCode::N0015,
            ErrorCode::N0016,
            ErrorCode::N0017,
            ErrorCode::N0018,
            ErrorCode::N0019,
            ErrorCode::N0020,
            ErrorCode::N0021,
            ErrorCode::N0022,
            ErrorCode::N0023,
            ErrorCode::N0024,
            ErrorCode::N0025,
            ErrorCode::N0026,
            ErrorCode::N0027,
            ErrorCode::N0028,
            ErrorCode::N0029,
            ErrorCode::N0030,
            ErrorCode::N0031,
            ErrorCode::N0032,
            ErrorCode::N0033,
            ErrorCode::N0034,
            ErrorCode::N0035,
            ErrorCode::N0036,
            ErrorCode::N0037,
            ErrorCode::N0038,
            ErrorCode::N0039,
            ErrorCode::N1001,
            ErrorCode::N1002,
            ErrorCode::N1003,
            ErrorCode::N1004,
            ErrorCode::N1005,
            ErrorCode::N1006,
            ErrorCode::N1007,
            ErrorCode::N1008,
            ErrorCode::N1009,
            ErrorCode::N1010,
            ErrorCode::N1011,
            ErrorCode::N1012,
            ErrorCode::N1013,
            ErrorCode::N1014,
            ErrorCode::N1015,
            ErrorCode::N1016,
            ErrorCode::N1017,
            ErrorCode::N1018,
            ErrorCode::N1019,
            ErrorCode::N1020,
            ErrorCode::N1021,
            ErrorCode::N1022,
            ErrorCode::N1023,
            ErrorCode::N1024,
            ErrorCode::N1025,
            ErrorCode::N1026,
            ErrorCode::N1027,
            ErrorCode::N1028,
            ErrorCode::N1029,
            ErrorCode::N1030,
            ErrorCode::N1031,
            ErrorCode::N1032,
            ErrorCode::N1033,
            ErrorCode::N1034,
            ErrorCode::N1035,
            ErrorCode::N1036,
            ErrorCode::N1037,
            ErrorCode::N1038,
            ErrorCode::N1039,
            ErrorCode::N1040,
            ErrorCode::N1041,
            ErrorCode::N1042,
            ErrorCode::N1043,
            ErrorCode::N1044,
            ErrorCode::N1045,
            ErrorCode::N1046,
            ErrorCode::N1047,
            ErrorCode::N1048,
            ErrorCode::N1049,
            ErrorCode::N1050,
            ErrorCode::N1051,
            ErrorCode::N1052,
            ErrorCode::N1053,
            ErrorCode::N1054,
            ErrorCode::N1055,
            ErrorCode::N1056,
            ErrorCode::N1057,
            ErrorCode::N1058,
            ErrorCode::N1059,
            ErrorCode::N1060,
            ErrorCode::N1061,
            ErrorCode::N1062,
            ErrorCode::N1063,
            ErrorCode::N1064,
            ErrorCode::N1065,
            ErrorCode::N1066,
            ErrorCode::N1067,
            ErrorCode::N1068,
            ErrorCode::N1069,
            ErrorCode::N1070,
            ErrorCode::N1071,
            ErrorCode::N1072,
            ErrorCode::N1073,
            ErrorCode::N1074,
            ErrorCode::N1075,
            ErrorCode::N1076,
            ErrorCode::N1077,
            ErrorCode::N1078,
            ErrorCode::N1079,
            ErrorCode::N1080,
            ErrorCode::N1081,
            ErrorCode::N1082,
            ErrorCode::N1083,
            ErrorCode::N1084,
            ErrorCode::N2001,
            ErrorCode::N2002,
            ErrorCode::N2003,
            ErrorCode::N2004,
            ErrorCode::N2005,
            ErrorCode::N2006,
            ErrorCode::N2007,
            ErrorCode::N2008,
            ErrorCode::N2009,
            ErrorCode::N2010,
            ErrorCode::N2011,
            ErrorCode::N2012,
            ErrorCode::N2013,
            ErrorCode::N2014,
            ErrorCode::N2015,
            ErrorCode::N2016,
            ErrorCode::N2017,
            ErrorCode::N2018,
            ErrorCode::N2019,
            ErrorCode::N2020,
            ErrorCode::N2021,
            ErrorCode::N2022,
            ErrorCode::N2023,
            ErrorCode::N2024,
            ErrorCode::N2025,
            ErrorCode::N2026,
            ErrorCode::N2027,
            ErrorCode::N2028,
            ErrorCode::N2029,
            ErrorCode::N2030,
            ErrorCode::N2031,
            ErrorCode::N2032,
            ErrorCode::N2033,
            ErrorCode::N2034,
            ErrorCode::N2035,
            ErrorCode::N2036,
            ErrorCode::N2037,
            ErrorCode::N2038,
            ErrorCode::N2039,
            ErrorCode::N2040,
            ErrorCode::N2041,
            ErrorCode::N2042,
            ErrorCode::N2043,
            ErrorCode::N2044,
            ErrorCode::N2045,
            ErrorCode::N2046,
            ErrorCode::N2047,
            ErrorCode::N2048,
            ErrorCode::N2049,
            ErrorCode::N3001,
            ErrorCode::N3002,
            ErrorCode::N3003,
            ErrorCode::N3004,
            ErrorCode::N3005,
            ErrorCode::N3006,
            ErrorCode::N3007,
            ErrorCode::N3008,
            ErrorCode::N3009,
            ErrorCode::N3010,
            ErrorCode::N3011,
            ErrorCode::N3012,
            ErrorCode::N3013,
            ErrorCode::N3014,
            ErrorCode::N3015,
            ErrorCode::N3016,
            ErrorCode::N3017,
            ErrorCode::N3018,
            ErrorCode::N3019,
            ErrorCode::N3020,
            ErrorCode::N3021,
            ErrorCode::N3022,
            ErrorCode::N3023,
            ErrorCode::N3024,
            ErrorCode::N3025,
            ErrorCode::N3026,
            ErrorCode::N3027,
            ErrorCode::N3028,
            ErrorCode::N3029,
            ErrorCode::N3030,
            ErrorCode::N3031,
            ErrorCode::N3032,
            ErrorCode::N3033,
            ErrorCode::N3034,
            ErrorCode::N3035,
            ErrorCode::N3036,
            ErrorCode::N3037,
            ErrorCode::N3038,
            ErrorCode::N3039,
            ErrorCode::N3040,
            ErrorCode::N3041,
            ErrorCode::N3042,
            ErrorCode::N3043,
            ErrorCode::N3044,
            ErrorCode::N3045,
            ErrorCode::N3046,
            ErrorCode::N3047,
            ErrorCode::N3048,
            ErrorCode::N3049,
            ErrorCode::N3050,
            ErrorCode::N3051,
            ErrorCode::N3052,
            ErrorCode::N3053,
            ErrorCode::N3054,
            ErrorCode::N3055,
            ErrorCode::N3056,
            ErrorCode::N3057,
            ErrorCode::N3058,
            ErrorCode::N3059,
            ErrorCode::N3060,
            ErrorCode::N3061,
            ErrorCode::N3062,
            ErrorCode::N3063,
            ErrorCode::N3064,
            ErrorCode::N3065,
            ErrorCode::N3066,
            ErrorCode::N3067,
            ErrorCode::N3068,
            ErrorCode::N3069,
            ErrorCode::N3070,
            ErrorCode::N3071,
            ErrorCode::N3072,
            ErrorCode::N3073,
            ErrorCode::N3074,
            ErrorCode::N3075,
            ErrorCode::N3076,
            ErrorCode::N3077,
            ErrorCode::N3078,
            ErrorCode::N3079,
            ErrorCode::N3080,
            ErrorCode::N3081,
            ErrorCode::N3082,
            ErrorCode::N3083,
            ErrorCode::N3084,
            ErrorCode::N3085,
            ErrorCode::N3086,
            ErrorCode::N3087,
            ErrorCode::N3088,
            ErrorCode::N3089,
            ErrorCode::N3090,
            ErrorCode::N3091,
            ErrorCode::N3092,
            ErrorCode::N3093,
            ErrorCode::N3094,
            ErrorCode::N3095,
            ErrorCode::N3096,
            ErrorCode::N3097,
            ErrorCode::N4001,
            ErrorCode::N4002,
            ErrorCode::N4003,
            ErrorCode::N4004,
            ErrorCode::N4005,
            ErrorCode::N4006,
            ErrorCode::N4007,
            ErrorCode::N4008,
            ErrorCode::N4009,
            ErrorCode::N4010,
            ErrorCode::N4011,
            ErrorCode::N4012,
            ErrorCode::N4013,
            ErrorCode::N4014,
            ErrorCode::N4015,
            ErrorCode::N4016,
            ErrorCode::N4017,
            ErrorCode::N4018,
            ErrorCode::N4019,
            ErrorCode::N4020,
            ErrorCode::N4021,
            ErrorCode::N4022,
            ErrorCode::N4023,
            ErrorCode::N4024,
            ErrorCode::N4025,
            ErrorCode::N4026,
            ErrorCode::N4027,
            ErrorCode::N5001,
            ErrorCode::N5002,
            ErrorCode::N5003,
            ErrorCode::N5004,
            ErrorCode::N5005,
            ErrorCode::N5006,
            ErrorCode::N5007,
            ErrorCode::N5008,
            ErrorCode::N5009,
            ErrorCode::N5010,
            ErrorCode::N5011,
            ErrorCode::N5012,
            ErrorCode::N5013,
            ErrorCode::N5014,
            ErrorCode::N5015,
            ErrorCode::N5016,
            ErrorCode::N5017,
            ErrorCode::N5018,
            ErrorCode::N5019,
            ErrorCode::N5020,
            ErrorCode::N5021,
            ErrorCode::N5022,
            ErrorCode::N5023,
            ErrorCode::N5024,
            ErrorCode::N5025,
            ErrorCode::N5026,
            ErrorCode::N5027,
            ErrorCode::N5028,
            ErrorCode::N5029,
            ErrorCode::N5030,
            ErrorCode::N5031,
            ErrorCode::N5032,
            ErrorCode::N5033,
            ErrorCode::N5034,
            ErrorCode::N5035,
            ErrorCode::N5036,
            ErrorCode::N5037,
            ErrorCode::N5038,
            ErrorCode::N5039,
            ErrorCode::N5040,
            ErrorCode::N5041,
            ErrorCode::N5042,
            ErrorCode::N5043,
            ErrorCode::N5044,
            ErrorCode::N5045,
            ErrorCode::N5046,
            ErrorCode::N5047,
            ErrorCode::N5048,
            ErrorCode::N5049,
            ErrorCode::N5050,
            ErrorCode::N5051,
            ErrorCode::N5052,
            ErrorCode::N5053,
            ErrorCode::N5054,
            ErrorCode::N5055,
            ErrorCode::N5056,
            ErrorCode::N5057,
            ErrorCode::N5058,
            ErrorCode::N5059,
            ErrorCode::N5060,
            ErrorCode::N5061,
            ErrorCode::N6001,
            ErrorCode::N6002,
            ErrorCode::N6003,
            ErrorCode::N6004,
            ErrorCode::N6005,
            ErrorCode::N6006,
            ErrorCode::N6007,
            ErrorCode::N6008,
            ErrorCode::N6009,
            ErrorCode::N6010,
            ErrorCode::N6011,
            ErrorCode::N6012,
            ErrorCode::N6013,
            ErrorCode::N6014,
            ErrorCode::N6015,
            ErrorCode::N6016,
            ErrorCode::N6017,
            ErrorCode::N6018,
            ErrorCode::N6019,
            ErrorCode::N6020,
            ErrorCode::N6021,
            ErrorCode::N6022,
            ErrorCode::N6023,
            ErrorCode::N6024,
            ErrorCode::N6025,
            ErrorCode::N6026,
            ErrorCode::N6027,
            ErrorCode::N6028,
            ErrorCode::N6029,
            ErrorCode::N6030,
            ErrorCode::N6031,
            ErrorCode::N6032,
            ErrorCode::N6033,
            ErrorCode::N6034,
            ErrorCode::N7001,
            ErrorCode::N7002,
            ErrorCode::N7003,
            ErrorCode::N7004,
            ErrorCode::N7005,
            ErrorCode::N7006,
            ErrorCode::N7007,
            ErrorCode::N7008,
            ErrorCode::N7009,
            ErrorCode::N7010,
            ErrorCode::N7011,
            ErrorCode::N7012,
            ErrorCode::N7013,
            ErrorCode::N7014,
            ErrorCode::N7015,
            ErrorCode::N7016,
            ErrorCode::N7017,
            ErrorCode::N7018,
            ErrorCode::N7019,
            ErrorCode::N7020,
            ErrorCode::N7021,
            ErrorCode::N7022,
            ErrorCode::N7023,
            ErrorCode::N7024,
            ErrorCode::N7025,
            ErrorCode::N7026,
            ErrorCode::N7027,
            ErrorCode::N7028,
            ErrorCode::N7029,
            ErrorCode::N8001,
            ErrorCode::N8002,
            ErrorCode::N8003,
            ErrorCode::N8004,
            ErrorCode::N8005,
            ErrorCode::N8006,
            ErrorCode::N8007,
            ErrorCode::N8008,
            ErrorCode::N8009,
            ErrorCode::N8010,
            ErrorCode::N8011,
            ErrorCode::N8012,
            ErrorCode::N8013,
            ErrorCode::N8014,
            ErrorCode::N8015,
            ErrorCode::N8016,
            ErrorCode::N8017,
            ErrorCode::N8018,
            ErrorCode::N8019,
            ErrorCode::N8020,
            ErrorCode::N8021,
            ErrorCode::N8022,
            ErrorCode::N8023,
            ErrorCode::N8024,
            ErrorCode::N8025,
            ErrorCode::N8026,
            ErrorCode::N8027,
            ErrorCode::N8028,
            ErrorCode::N8029,
            ErrorCode::N8030,
            ErrorCode::N8031,
            ErrorCode::N8032,
            ErrorCode::N8033,
            ErrorCode::N8034,
            ErrorCode::N8035,
            ErrorCode::N8036,
            ErrorCode::N8037,
            ErrorCode::N8038,
            ErrorCode::N8039,
            ErrorCode::N9001,
            ErrorCode::N9002,
            ErrorCode::N9003,
            ErrorCode::N9004,
            ErrorCode::N9005,
            ErrorCode::N9006,
            ErrorCode::N9007,
            ErrorCode::N9008,
            ErrorCode::N9009,
            ErrorCode::N9010,
            ErrorCode::N9011,
            ErrorCode::N9012,
            ErrorCode::N9013,
            ErrorCode::N9014,
            ErrorCode::N9015,
            ErrorCode::N9016,
            ErrorCode::N9017,
            ErrorCode::N9018,
            ErrorCode::N9019,
            ErrorCode::N9020,
            ErrorCode::N9021,
            ErrorCode::N9022,
            ErrorCode::N9023,
            ErrorCode::N9024,
            ErrorCode::N9025,
        ];
        for code in ALL {
            if code.as_str() == upper {
                return Some(*code);
            }
        }
        None
    }

    /// Short human-readable title for this error code.
    pub fn title(&self) -> &'static str {
        match self {
            Self::N0001 => "Illegal tab character",
            Self::N0002 => "Unexpected character",
            Self::N0003 => "Unmatched closing delimiter",
            Self::N0004 => "Invalid float literal",
            Self::N0005 => "Integer literal out of range",
            Self::N0006 => "Unterminated string literal",
            Self::N0007 => "Invalid escape sequence",
            Self::N0008 => "Newline inside string literal",
            Self::N0009 => "Indentation error",
            Self::N0010 => "Empty character literal",
            Self::N0011 => "Multi-byte character literal",
            Self::N0012 => "Unicode escape in non-unicode context",
            Self::N0013 => "Invalid numeric suffix",
            Self::N0014 => "Leading zeros in decimal integer",
            Self::N0015 => "Binary literal overflow",
            Self::N0016 => "Hex literal overflow",
            Self::N0017 => "Octal literal overflow",
            Self::N0018 => "Invalid binary literal format",
            Self::N0019 => "Invalid hex literal format",
            Self::N0020 => "Invalid octal literal format",
            Self::N0021 => "Unterminated block comment",
            Self::N0022 => "Unrecognized token in string interpolation",
            Self::N0023 => "Invalid unicode identifier start",
            Self::N0024 => "Non-printable character in source",
            Self::N0025 => "Byte order mark detected",
            Self::N0026 => "Null character in source",
            Self::N0027 => "String literal exceeds maximum length",
            Self::N0028 => "Empty unicode escape sequence",
            Self::N0029 => "Malformed unicode escape sequence",
            Self::N0030 => "Unicode codepoint out of range",
            Self::N0031 => "Reserved keyword used as identifier",
            Self::N0032 => "Non-standard line ending",
            Self::N0033 => "Mixed tabs and spaces",
            Self::N0034 => "Digit separator at wrong position",
            Self::N0035 => "Consecutive digit separators",
            Self::N0036 => "Trailing digit separator",
            Self::N0037 => "Leading digit separator",
            Self::N0038 => "Invalid digit for radix",
            Self::N0039 => "Unterminated raw string literal",
            Self::N1001 => "Expected token",
            Self::N1002 => "Unexpected token",
            Self::N1003 => "Expected expression",
            Self::N1004 => "Unclosed delimiter",
            Self::N1005 => "Expected indented block",
            Self::N1006 => "Unexpected indentation",
            Self::N1007 => "Expected identifier",
            Self::N1008 => "Expected type name",
            Self::N1009 => "Expected parameter name",
            Self::N1010 => "Expected colon",
            Self::N1011 => "Expected semicolon",
            Self::N1012 => "Expected equals sign",
            Self::N1013 => "Expected arrow",
            Self::N1014 => "Expected comma",
            Self::N1015 => "Expected dot",
            Self::N1016 => "Missing function body",
            Self::N1017 => "Missing return type or return expression",
            Self::N1018 => "Invalid function parameter",
            Self::N1019 => "Too many function parameters",
            Self::N1020 => "Too few arguments in call",
            Self::N1021 => "Expected statement",
            Self::N1022 => "Expected binding",
            Self::N1023 => "Expected keyword",
            Self::N1024 => "Invalid left-hand side of assignment",
            Self::N1025 => "Nested function without closure context",
            Self::N1026 => "Duplicate parameter name",
            Self::N1027 => "Default parameter value before non-default",
            Self::N1028 => "Expected module path",
            Self::N1029 => "Expected import symbol",
            Self::N1030 => "Circular import",
            Self::N1031 => "Break outside of loop",
            Self::N1032 => "Continue outside of loop",
            Self::N1033 => "Return outside of function",
            Self::N1034 => "Yield outside of generator",
            Self::N1035 => "Invalid for-loop binding",
            Self::N1036 => "Expected `in` in for-loop",
            Self::N1037 => "Empty struct body",
            Self::N1038 => "Empty interface body",
            Self::N1039 => "Method declaration outside of interface",
            Self::N1040 => "Duplicate field name in struct",
            Self::N1041 => "Unnamed field in struct literal",
            Self::N1042 => "Expected struct expression",
            Self::N1043 => "Missing colon in type annotation",
            Self::N1044 => "Unexpected trailing comma",
            Self::N1045 => "Malformed string interpolation",
            Self::N1046 => "Unterminated block comment",
            Self::N1047 => "Expected attribute",
            Self::N1048 => "Invalid attribute target",
            Self::N1049 => "Duplicate attribute",
            Self::N1050 => "Unknown attribute",
            Self::N1051 => "Expected type arguments",
            Self::N1052 => "Unclosed type argument list",
            Self::N1053 => "Type argument count mismatch",
            Self::N1054 => "Expected `<` for generics",
            Self::N1055 => "Expected `>` for generics",
            Self::N1056 => "Unterminated lambda body",
            Self::N1057 => "Expected binding in for-loop",
            Self::N1058 => "Expected path expression",
            Self::N1059 => "Expected literal",
            Self::N1060 => "Expected pattern",
            Self::N1061 => "Expected guard expression",
            Self::N1062 => "Expected where clause",
            Self::N1063 => "Expected semicolon or newline",
            Self::N1064 => "Expected operator",
            Self::N1065 => "Invalid prefix operator",
            Self::N1066 => "Invalid postfix operator",
            Self::N1067 => "Operator precedence ambiguity",
            Self::N1068 => "Comparison chaining with incompatible operators",
            Self::N1069 => "Expected tuple element",
            Self::N1070 => "Expected array element",
            Self::N1071 => "Expected struct field",
            Self::N1072 => "Expected enum variant",
            Self::N1073 => "Expected match arm",
            Self::N1074 => "Expected `=>` in match arm",
            Self::N1075 => "Expected pattern guard",
            Self::N1076 => "Invalid doc comment placement",
            Self::N1077 => "Expected doc comment",
            Self::N1078 => "Invalid visibility modifier",
            Self::N1079 => "Expected item",
            Self::N1080 => "Nested function without body",
            Self::N1081 => "Unterminated generic list",
            Self::N1082 => "Expected lifetime parameter",
            Self::N1083 => "Expected const parameter",
            Self::N1084 => "Ambiguous literal suffix",
            Self::N2001 => "Undefined variable",
            Self::N2002 => "Duplicate definition",
            Self::N2003 => "Undefined function",
            Self::N2004 => "Undefined struct",
            Self::N2005 => "Undefined interface",
            Self::N2006 => "Undefined module",
            Self::N2007 => "Undefined type",
            Self::N2008 => "Undefined macro",
            Self::N2009 => "Access to private item",
            Self::N2010 => "Cyclic module dependency",
            Self::N2011 => "Cyclic type definition",
            Self::N2012 => "Ambiguous name",
            Self::N2013 => "Invalid visibility qualifier",
            Self::N2014 => "Module not found",
            Self::N2015 => "Symbol not exported",
            Self::N2016 => "Name conflicts with builtin",
            Self::N2017 => "Name shadows builtin",
            Self::N2018 => "Unused import",
            Self::N2019 => "Wildcard import leaks names",
            Self::N2020 => "`self` used outside of method",
            Self::N2021 => "`super` used outside of class context",
            Self::N2022 => "Invalid self parameter",
            Self::N2023 => "Method without self parameter",
            Self::N2024 => "Return type mismatch in implementation",
            Self::N2025 => "Missing required interface method",
            Self::N2026 => "Extra method not in interface",
            Self::N2027 => "Invalid method override",
            Self::N2028 => "Override without base definition",
            Self::N2029 => "Inconsistent associated type binding",
            Self::N2030 => "Circular trait bound",
            Self::N2031 => "Unused variable",
            Self::N2032 => "Unused assignment",
            Self::N2033 => "Variable shadows outer variable",
            Self::N2034 => "Unreachable pattern",
            Self::N2035 => "Non-exhaustive patterns",
            Self::N2036 => "Pattern binding conflict",
            Self::N2037 => "Illegal binding mode in pattern",
            Self::N2038 => "Invalid pattern",
            Self::N2039 => "Unresolved import",
            Self::N2040 => "Unresolved re-export",
            Self::N2041 => "Private re-export",
            Self::N2042 => "Conflicting re-export",
            Self::N2043 => "Re-export of non-existent symbol",
            Self::N2044 => "Self-import",
            Self::N2045 => "Invalid use of `Self` type alias",
            Self::N2046 => "External crate not found",
            Self::N2047 => "External crate version conflict",
            Self::N2048 => "External crate feature not found",
            Self::N2049 => "Unused extern crate",
            Self::N3001 => "Type mismatch",
            Self::N3002 => "Assign to immutable variable",
            Self::N3003 => "Undefined type",
            Self::N3004 => "Call of non-function",
            Self::N3005 => "Argument count mismatch",
            Self::N3006 => "Missing method / unsatisfied interface",
            Self::N3007 => "Recursive type without indirection",
            Self::N3008 => "Infinite type",
            Self::N3009 => "Cannot infer type",
            Self::N3010 => "Type annotation required",
            Self::N3011 => "Expected concrete type, found abstract",
            Self::N3012 => "Trait bound not satisfied",
            Self::N3013 => "Associated type not specified",
            Self::N3014 => "Wrong number of type arguments",
            Self::N3015 => "Type argument out of bounds",
            Self::N3016 => "Conflicting type arguments",
            Self::N3017 => "Cross-module type violation",
            Self::N3018 => "Borrow of immutable as mutable",
            Self::N3019 => "Borrow of moved value",
            Self::N3020 => "Use after move",
            Self::N3021 => "Multiple mutable borrows",
            Self::N3022 => "Lifetime mismatch",
            Self::N3023 => "Lifetime elision failure",
            Self::N3024 => "Lifetime bound not satisfied",
            Self::N3025 => "Lifetime constraint violation",
            Self::N3026 => "Missing lifetime annotation",
            Self::N3027 => "Invalid lifetime name",
            Self::N3028 => "Mismatched mutability in reference",
            Self::N3029 => "Dangling reference",
            Self::N3030 => "Drop of move type",
            Self::N3031 => "Borrow of constant value",
            Self::N3032 => "Numeric overflow in constant",
            Self::N3033 => "Division by zero in constant",
            Self::N3034 => "Remainder by zero in constant",
            Self::N3035 => "Negation of unsigned integer",
            Self::N3036 => "Shift exceeds bit width",
            Self::N3037 => "Operator not applicable to types",
            Self::N3038 => "Comparison of unordered values",
            Self::N3039 => "Invalid unary operator for type",
            Self::N3040 => "Invalid binary operator for types",
            Self::N3041 => "No common operator overload",
            Self::N3042 => "Ambiguous operator application",
            Self::N3043 => "Wrong number of generic type params",
            Self::N3044 => "Generic bound not satisfied",
            Self::N3045 => "Missing generic type annotation",
            Self::N3046 => "Generic parameter not used",
            Self::N3047 => "Concrete type in abstract context",
            Self::N3048 => "Abstract type in concrete context",
            Self::N3049 => "Non-constant in const context",
            Self::N3050 => "Non-const call in const context",
            Self::N3051 => "Mutable reference in const context",
            Self::N3052 => "If condition must be boolean",
            Self::N3053 => "While condition must be boolean",
            Self::N3054 => "For-each binding type mismatch",
            Self::N3055 => "Return type mismatch",
            Self::N3056 => "Missing return value",
            Self::N3057 => "Extra return value from void function",
            Self::N3058 => "Return not allowed here",
            Self::N3059 => "Missing async annotation",
            Self::N3060 => "Cannot await non-future",
            Self::N3061 => "Incompatible implicit conversion",
            Self::N3062 => "Forward declaration type mismatch",
            Self::N3063 => "Missing forward declaration",
            Self::N3064 => "Field type mismatch in struct literal",
            Self::N3065 => "Missing field in struct literal",
            Self::N3066 => "Extra field in struct literal",
            Self::N3067 => "Ambiguous field in struct literal",
            Self::N3068 => "Cyclic struct definition",
            Self::N3069 => "Tuple index out of bounds",
            Self::N3070 => "Array index out of bounds",
            Self::N3071 => "Non-comptime array index",
            Self::N3072 => "Mismatched array length",
            Self::N3073 => "Type alias cycle",
            Self::N3074 => "Type alias uses non-existent type",
            Self::N3075 => "Unsized type in field",
            Self::N3076 => "Unsized type in local variable",
            Self::N3077 => "Unsized type in parameter",
            Self::N3078 => "Unsized type in return position",
            Self::N3079 => "Unsized type in struct field",
            Self::N3080 => "Unexpected type parameter",
            Self::N3081 => "Expected type parameter",
            Self::N3082 => "Invalid enum discriminant type",
            Self::N3083 => "Duplicate enum discriminant",
            Self::N3084 => "Enum discriminant overflow",
            Self::N3085 => "Non-exhaustive enum match",
            Self::N3086 => "Unreachable match arm",
            Self::N3087 => "Overlapping match patterns",
            Self::N3088 => "Invalid ref pattern",
            Self::N3089 => "Invalid mut pattern",
            Self::N3090 => "Pattern requires unit type",
            Self::N3091 => "Closure with non-closure context",
            Self::N3092 => "Closure captures disjoint variables",
            Self::N3093 => "Non-copy type in closure by copy",
            Self::N3094 => "Mismatched async closure",
            Self::N3095 => "Generator resume type mismatch",
            Self::N3096 => "Generator yield type mismatch",
            Self::N3097 => "Generator return type mismatch",
            Self::N4001 => "Module not found",
            Self::N4002 => "File not found",
            Self::N4003 => "Circular module dependency",
            Self::N4004 => "Symbol not exported",
            Self::N4005 => "Import cycle",
            Self::N4006 => "Ambiguous import",
            Self::N4007 => "Shadowed import",
            Self::N4008 => "Wildcard import conflict",
            Self::N4009 => "Relative import beyond root",
            Self::N4010 => "Invalid module name",
            Self::N4011 => "Module not in search path",
            Self::N4012 => "Module parse error",
            Self::N4013 => "Module type error",
            Self::N4014 => "Dependency not found",
            Self::N4015 => "Dependency cycle",
            Self::N4016 => "Dependency version conflict",
            Self::N4017 => "Broken package",
            Self::N4018 => "Missing manifest",
            Self::N4019 => "Invalid manifest format",
            Self::N4020 => "Manifest syntax error",
            Self::N4021 => "Manifest missing required field",
            Self::N4022 => "Manifest duplicate entry",
            Self::N4023 => "Module compiled with different version",
            Self::N4024 => "Module compiled with incompatible flags",
            Self::N4025 => "Module interface mismatch",
            Self::N4026 => "Recursive module loading",
            Self::N4027 => "Module path too deep",
            Self::N5001 => "Unused variable",
            Self::N5002 => "Unused import",
            Self::N5003 => "Unused assignment",
            Self::N5004 => "Unused function",
            Self::N5005 => "Unused struct",
            Self::N5006 => "Unused type",
            Self::N5007 => "Dead code",
            Self::N5008 => "Unreachable code",
            Self::N5009 => "Empty loop body",
            Self::N5010 => "Suspicious assignment in conditional",
            Self::N5011 => "Lossy implicit conversion",
            Self::N5012 => "Deprecated item",
            Self::N5013 => "Missing documentation",
            Self::N5014 => "Non-standard naming",
            Self::N5015 => "Name shadows outer",
            Self::N5016 => "Unnecessary closure",
            Self::N5017 => "Redundant pattern",
            Self::N5018 => "Missing else branch",
            Self::N5019 => "Deep nesting",
            Self::N5020 => "Complex expression",
            Self::N5021 => "High cognitive complexity",
            Self::N5022 => "High cyclomatic complexity",
            Self::N5023 => "Too many parameters",
            Self::N5024 => "Too many return types",
            Self::N5025 => "Function too long",
            Self::N5026 => "File too long",
            Self::N5027 => "Line too long",
            Self::N5028 => "Inconsistent naming style",
            Self::N5029 => "Non-canonical ordering",
            Self::N5030 => "Unsafe block used",
            Self::N5031 => "Unsafe function",
            Self::N5032 => "Unnecessary unsafe",
            Self::N5033 => "Comparing boolean literal",
            Self::N5034 => "Assigning boolean in conditional",
            Self::N5035 => "Negating boolean literal",
            Self::N5036 => "Nested conditional",
            Self::N5037 => "Constant condition",
            Self::N5038 => "Redundant cast",
            Self::N5039 => "Suspicious comparison",
            Self::N5040 => "Infinite loop detected",
            Self::N5041 => "Missing break in loop",
            Self::N5042 => "Uninitialized variable",
            Self::N5043 => "Possibly uninitialized variable",
            Self::N5044 => "Fallthrough in match",
            Self::N5045 => "Missing case in match",
            Self::N5046 => "Redundant default case",
            Self::N5047 => "Unnecessary else-if",
            Self::N5048 => "Redundant else branch",
            Self::N5049 => "Empty else branch",
            Self::N5050 => "Unnecessary parentheses",
            Self::N5051 => "Unnecessary return",
            Self::N5052 => "Unnecessary semicolon",
            Self::N5053 => "Empty statement",
            Self::N5054 => "Statement with no effect",
            Self::N5055 => "Variable assigned but not used",
            Self::N5056 => "Function argument reassigned",
            Self::N5057 => "Mutable variable could be immutable",
            Self::N5058 => "Redundant field name",
            Self::N5059 => "Unnecessary qualification",
            Self::N5060 => "Module naming convention",
            Self::N5061 => "Non-idiomatic code",
            Self::N6001 => "Code generation failure",
            Self::N6002 => "Unsupported feature for target",
            Self::N6003 => "Linker error",
            Self::N6004 => "Assembly error",
            Self::N6005 => "Target not supported",
            Self::N6006 => "Invalid optimization level",
            Self::N6007 => "Invalid debug info level",
            Self::N6008 => "Inline assembly error",
            Self::N6009 => "Compiler intrinsic error",
            Self::N6010 => "Stack overflow during codegen",
            Self::N6011 => "Global offset overflow",
            Self::N6012 => "Jump table overflow",
            Self::N6013 => "Too many static variables",
            Self::N6014 => "Too many functions",
            Self::N6015 => "Function too large",
            Self::N6016 => "External symbol not found",
            Self::N6017 => "Duplicate symbol export",
            Self::N6018 => "Undefined symbol",
            Self::N6019 => "Relocation overflow",
            Self::N6020 => "TLS not supported",
            Self::N6021 => "ABI mismatch",
            Self::N6022 => "CPU feature not available",
            Self::N6023 => "OS feature not available",
            Self::N6024 => "Inline assembly constraint violation",
            Self::N6025 => "Intrinsic signature mismatch",
            Self::N6026 => "Vector type not supported",
            Self::N6027 => "Atomic not supported",
            Self::N6028 => "SIMD not supported",
            Self::N6029 => "Codegen buffer overflow",
            Self::N6030 => "Unsupported calling convention",
            Self::N6031 => "Too many locals",
            Self::N6032 => "Section attribute conflict",
            Self::N6033 => "Link once group conflict",
            Self::N6034 => "Visibility attribute conflict",
            Self::N7001 => "Program panicked",
            Self::N7002 => "Index out of bounds",
            Self::N7003 => "Stack overflow",
            Self::N7004 => "Arithmetic overflow",
            Self::N7005 => "Division by zero",
            Self::N7006 => "Null pointer dereference",
            Self::N7007 => "Unwrap of None",
            Self::N7008 => "Unwrap of error",
            Self::N7009 => "Out of memory",
            Self::N7010 => "Assertion failed",
            Self::N7011 => "Unreachable code executed",
            Self::N7012 => "TODO encountered at runtime",
            Self::N7013 => "Unimplemented functionality",
            Self::N7014 => "Buffer overflow",
            Self::N7015 => "Invalid UTF-8 sequence",
            Self::N7016 => "Integer conversion overflow",
            Self::N7017 => "Float conversion overflow",
            Self::N7018 => "Negative index",
            Self::N7019 => "Invalid enum discriminant",
            Self::N7020 => "Type cast error at runtime",
            Self::N7021 => "Recursive call overflow",
            Self::N7022 => "Invalid allocator state",
            Self::N7023 => "Double free",
            Self::N7024 => "Use after free",
            Self::N7025 => "Mutex poison",
            Self::N7026 => "Channel closed",
            Self::N7027 => "Timeout",
            Self::N7028 => "IO error",
            Self::N7029 => "Network error",
            Self::N8001 => "Configuration parse error",
            Self::N8002 => "Configuration missing field",
            Self::N8003 => "Configuration invalid value",
            Self::N8004 => "Build target not found",
            Self::N8005 => "Build script error",
            Self::N8006 => "Missing build dependency",
            Self::N8007 => "Invalid build profile",
            Self::N8008 => "Invalid manifest",
            Self::N8009 => "Manifest missing package name",
            Self::N8010 => "Manifest missing version",
            Self::N8011 => "Manifest duplicate entry",
            Self::N8012 => "Manifest invalid dependency",
            Self::N8013 => "Workspace member not found",
            Self::N8014 => "Workspace duplicate member",
            Self::N8015 => "Invalid toolchain",
            Self::N8016 => "Toolchain not installed",
            Self::N8017 => "Invalid target triple",
            Self::N8018 => "Test failed",
            Self::N8019 => "Benchmark failed",
            Self::N8020 => "Missing test configuration",
            Self::N8021 => "Invalid compiler flag",
            Self::N8022 => "Conflicting compiler flags",
            Self::N8023 => "Unsupported flag for target",
            Self::N8024 => "Invalid linker flag",
            Self::N8025 => "Missing linker",
            Self::N8026 => "Missing assembler",
            Self::N8027 => "Output path not writable",
            Self::N8028 => "Cache directory not accessible",
            Self::N8029 => "Concurrent build conflict",
            Self::N8030 => "Build system internal error",
            Self::N8031 => "Invalid package name",
            Self::N8032 => "Package name invalid characters",
            Self::N8033 => "Invalid package version",
            Self::N8034 => "License not recognized",
            Self::N8035 => "Missing package license",
            Self::N8036 => "Missing package description",
            Self::N8037 => "Invalid edition",
            Self::N8038 => "Feature flag not recognized",
            Self::N8039 => "Feature flag conflict",
            Self::N9001 => "Internal compiler error",
            Self::N9002 => "Internal bug — please report",
            Self::N9003 => "Unreachable code path in compiler",
            Self::N9004 => "Unimplemented compiler feature",
            Self::N9005 => "Compiler assertion failure",
            Self::N9006 => "Compiler invariant violation",
            Self::N9007 => "Type checker invariant failure",
            Self::N9008 => "Name resolution invariant failure",
            Self::N9009 => "Codegen invariant failure",
            Self::N9010 => "Compiler data structure corruption",
            Self::N9011 => "Missing compiler pass",
            Self::N9012 => "Compiler pass cycle",
            Self::N9013 => "Compiler query cycle",
            Self::N9014 => "Incremental cache mismatch",
            Self::N9015 => "Incremental fingerprint conflict",
            Self::N9016 => "AST validation failure",
            Self::N9017 => "HIR validation failure",
            Self::N9018 => "MIR validation failure",
            Self::N9019 => "LLVM / backend error",
            Self::N9020 => "Compiler memory allocation failure",
            Self::N9021 => "Thread panic in compiler worker",
            Self::N9022 => "Compiler resource limit exceeded",
            Self::N9023 => "Compiler I/O error",
            Self::N9024 => "Compiler timeout",
            Self::N9025 => "Invalid incremental compilation state",
        }
    }

    /// Numeric code (e.g., 1 for N0001, 1001 for N1001).
    pub fn number(&self) -> u16 {
        // "N0001"[1..] => "0001" => 1
        let s = self.as_str();
        s[1..].parse().unwrap_or(0)
    }

    /// Category name for grouping this error code.
    pub fn category(&self) -> &'static str {
        let n = self.number();
        if n < 100 {
            "Lexer"
        } else if n < 2000 {
            "Parser"
        } else if n < 3000 {
            "Name Resolution"
        } else if n < 4000 {
            "Type System"
        } else if n < 5000 {
            "Module System"
        } else if n < 6000 {
            "Lint"
        } else if n < 7000 {
            "Codegen"
        } else if n < 8000 {
            "Runtime"
        } else if n < 9000 {
            "Build System"
        } else {
            "Internal"
        }
    }

    /// Severity level: Error, Warning, Note, or Bug.
    pub fn severity(&self) -> &'static str {
        let n = self.number();
        // Lint codes are all warnings
        if (5001..=5061).contains(&n) {
            return "Warning";
        }
        // Runtime: N7012 is a note, rest are bugs
        if (7001..=7029).contains(&n) {
            if n == 7012 {
                return "Note";
            }
            return "Bug";
        }
        // Codegen: N6006, N6007 are errors, rest are bugs
        if (6001..=6034).contains(&n) {
            if n == 6006 || n == 6007 {
                return "Error";
            }
            return "Bug";
        }
        // Internal codes are all bugs
        if (9001..=9025).contains(&n) {
            return "Bug";
        }
        // Name resolution warning overrides
        if n == 2017
            || n == 2018
            || n == 2019
            || n == 2031
            || n == 2032
            || n == 2033
            || n == 2034
            || n == 2049
        {
            return "Warning";
        }
        // Type system warning overrides
        if n == 3046 || n == 3061 {
            return "Warning";
        }
        // Module system warning overrides
        if n == 4008 {
            return "Warning";
        }
        // Build system warning overrides
        if n == 8034 || n == 8035 || n == 8036 || n == 8038 {
            return "Warning";
        }
        // Everything else is an error
        "Error"
    }

    /// Colorized ANSI explanation for this error code.
    /// Used by `nimble explain <code>` in the CLI (no markdown).
    pub fn explanation(&self) -> &'static str {
        match self {
            Self::N0001 => {
                "\
\x1b[1;31mN0001\x1b[0m \x1b[1mIllegal tab character\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nNimble uses spaces for indentation and does not allow tab characters (\\t) anywhere in the source code. A tab was encountered while scanning the input. This is a hard error to enforce consistent indentation across all editors and platforms. Configure your editor to insert spaces when you press the Tab key, typically with an indent width of 2 or 4 spaces.\
"
            }
            Self::N0002 => {
                "\
\x1b[1;31mN0002\x1b[0m \x1b[1mUnexpected character\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe lexer encountered a character that is not part of the Nimble language grammar. This typically happens when a non-ASCII, control character, or unsymbolic glyph appears outside of a string or comment context. Check for stray characters, typos, or copy-paste artifacts in your source file.\
"
            }
            Self::N0003 => {
                "\
\x1b[1;31mN0003\x1b[0m \x1b[1mUnmatched closing delimiter\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA closing bracket, parenthesis, or brace was found without a matching opening counterpart. Common causes include deleting an opening delimiter, having too many closing delimiters due to a typo, or incorrect nesting of brackets. Check the surrounding context for mismatched parentheses, braces, or brackets.\
"
            }
            Self::N0004 => {
                "\
\x1b[1;31mN0004\x1b[0m \x1b[1mInvalid float literal\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA floating-point number literal is malformed. Nimble requires floats to have digits on both sides of the decimal point (e.g. 3.14), a single decimal point without surrounding digits is not allowed. Scientific notation must follow the form <digits>e<exponent> or <digits>E<exponent>.\
"
            }
            Self::N0005 => {
                "\
\x1b[1;31mN0005\x1b[0m \x1b[1mInteger literal out of range\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn integer literal exceeds the maximum representable value for the target type. Nimble integers are 64-bit signed by default, with a range of -2^63 to 2^63-1. If you need larger values, consider using a Float or a big integer library.\
"
            }
            Self::N0006 => {
                "\
\x1b[1;31mN0006\x1b[0m \x1b[1mUnterminated string literal\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA string literal was started with a double-quote (\") but the end of file or end of line was reached before the closing double-quote. In Nimble, string literals cannot span multiple lines unless escaped. Add a closing double-quote or escape the newline if you intend a multi-line string.\
"
            }
            Self::N0007 => {
                "\
\x1b[1;31mN0007\x1b[0m \x1b[1mInvalid escape sequence\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA backslash followed by an unrecognized character was found inside a string literal. Valid escape sequences in Nimble include \\n, \\t, \\r, \\0, \\\\, \\\", and \\'. Use only these recognized escape sequences.\
"
            }
            Self::N0008 => {
                "\
\x1b[1;31mN0008\x1b[0m \x1b[1mNewline inside string literal\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn unescaped newline character was detected inside a string literal. Nimble requires all string content to be on a single line unless the newline is explicitly escaped with a backslash at the end of the line.\
"
            }
            Self::N0009 => {
                "\
\x1b[1;31mN0009\x1b[0m \x1b[1mIndentation error\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe indentation level of a line does not match any enclosing block's indent level. Nimble uses Python-style indentation, so every indented block must align consistently. Ensure all lines in the same block have exactly the same indentation.\
"
            }
            Self::N0010 => {
                "\
\x1b[1;31mN0010\x1b[0m \x1b[1mEmpty character literal\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA character literal ('') was found with zero characters between the single quotes. Character literals must contain exactly one character. If you need an empty value, use a string literal (\"\") instead.\
"
            }
            Self::N0011 => {
                "\
\x1b[1;31mN0011\x1b[0m \x1b[1mMulti-byte character literal\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA character literal contains more than one byte. Nimble char literals (inside single quotes) must contain exactly one ASCII character or one Unicode codepoint. Use a string literal if you need multiple characters.\
"
            }
            Self::N0012 => {
                "\
\x1b[1;31mN0012\x1b[0m \x1b[1mUnicode escape in non-unicode context\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA \\u or \\U unicode escape sequence was used in a context that does not support unicode escapes. Unicode escapes are only valid inside string literals.\
"
            }
            Self::N0013 => {
                "\
\x1b[1;31mN0013\x1b[0m \x1b[1mInvalid numeric suffix\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA literal number has an unrecognized suffix. Nimble uses type suffixes like `i32`, `u64`, or `f32` to specify the type of a numeric literal. Only recognized suffixes are allowed.\
"
            }
            Self::N0014 => {
                "\
\x1b[1;31mN0014\x1b[0m \x1b[1mLeading zeros in decimal integer\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA decimal integer literal starts with one or more leading zeros. Decimal numbers must not have leading zeros. If you intend an octal literal, use the 0o prefix; otherwise remove the leading zeros.\
"
            }
            Self::N0015 => {
                "\
\x1b[1;31mN0015\x1b[0m \x1b[1mBinary literal overflow\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA binary literal (0b prefix) contains a value that exceeds the maximum representable value for the target integer type. The number of bits in the binary representation must fit within the type's width.\
"
            }
            Self::N0016 => {
                "\
\x1b[1;31mN0016\x1b[0m \x1b[1mHex literal overflow\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA hexadecimal literal (0x prefix) contains a value that exceeds the maximum representable value for the target integer type.\
"
            }
            Self::N0017 => {
                "\
\x1b[1;31mN0017\x1b[0m \x1b[1mOctal literal overflow\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn octal literal (0o prefix) contains a value that exceeds the maximum representable value for the target integer type.\
"
            }
            Self::N0018 => {
                "\
\x1b[1;31mN0018\x1b[0m \x1b[1mInvalid binary literal format\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA binary literal (0b prefix) contains digits other than 0 and 1. Binary literals may only contain the digits 0 and 1, optionally separated by underscores.\
"
            }
            Self::N0019 => {
                "\
\x1b[1;31mN0019\x1b[0m \x1b[1mInvalid hex literal format\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA hexadecimal literal (0x prefix) contains invalid characters. Hex digits include 0-9, a-f, A-F, and optionally underscores.\
"
            }
            Self::N0020 => {
                "\
\x1b[1;31mN0020\x1b[0m \x1b[1mInvalid octal literal format\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn octal literal (0o prefix) contains digits other than 0-7. Octal digits range from 0 to 7 only.\
"
            }
            Self::N0021 => {
                "\
\x1b[1;31mN0021\x1b[0m \x1b[1mUnterminated block comment\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA block comment (/* ... */) was started but never closed before end of file. Add the closing */ delimiter.\
"
            }
            Self::N0022 => {
                "\
\x1b[1;31mN0022\x1b[0m \x1b[1mUnrecognized token in string interpolation\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA string interpolation expression contains an invalid token. String interpolation allows only expressions and identifiers.\
"
            }
            Self::N0023 => {
                "\
\x1b[1;31mN0023\x1b[0m \x1b[1mInvalid unicode identifier start\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn identifier begins with a unicode character that is not a valid identifier start per Nimble's rules. Identifiers must start with a letter or underscore.\
"
            }
            Self::N0024 => {
                "\
\x1b[1;31mN0024\x1b[0m \x1b[1mNon-printable character in source\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA non-printable control character (outside the printable ASCII range) was found in the source code. Remove or replace the character.\
"
            }
            Self::N0025 => {
                "\
\x1b[1;31mN0025\x1b[0m \x1b[1mByte order mark detected\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA UTF-8 BOM (byte order mark, U+FEFF) was found at the start of the file. Save the file without a BOM.\
"
            }
            Self::N0026 => {
                "\
\x1b[1;31mN0026\x1b[0m \x1b[1mNull character in source\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA null byte (\\0) was found in the source code. Null bytes are not allowed outside of string literals.\
"
            }
            Self::N0027 => {
                "\
\x1b[1;31mN0027\x1b[0m \x1b[1mString literal exceeds maximum length\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA string literal contains more than the maximum allowed number of characters. Break the string into smaller parts and concatenate.\
"
            }
            Self::N0028 => {
                "\
\x1b[1;31mN0028\x1b[0m \x1b[1mEmpty unicode escape sequence\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA unicode escape sequence (\\u, \\U) has no hex digits. Provide at least one hex digit after the \\u or \\U marker.\
"
            }
            Self::N0029 => {
                "\
\x1b[1;31mN0029\x1b[0m \x1b[1mMalformed unicode escape sequence\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA unicode escape sequence contains invalid hex digits or is incorrectly structured. Use the form \\uXXXX or \\UXXXXXXXX.\
"
            }
            Self::N0030 => {
                "\
\x1b[1;31mN0030\x1b[0m \x1b[1mUnicode codepoint out of range\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA unicode escape sequence specifies a codepoint value outside the valid Unicode range (0x0000-0x10FFFF). Use a value within range.\
"
            }
            Self::N0031 => {
                "\
\x1b[1;31mN0031\x1b[0m \x1b[1mReserved keyword used as identifier\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA reserved keyword is being used as an identifier name. Keywords like fn, let, var, if, else, while, return, etc. cannot be used as variable or function names.\
"
            }
            Self::N0032 => {
                "\
\x1b[1;31mN0032\x1b[0m \x1b[1mNon-standard line ending\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA non-standard line ending (CR, CR+LF, or other) was detected. Use standard LF line endings for consistency.\
"
            }
            Self::N0033 => {
                "\
\x1b[1;31mN0033\x1b[0m \x1b[1mMixed tabs and spaces\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nBoth tabs and spaces are used for indentation in the same file. Pick one style (spaces recommended) and use it consistently.\
"
            }
            Self::N0034 => {
                "\
\x1b[1;31mN0034\x1b[0m \x1b[1mDigit separator at wrong position\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn underscore digit separator appears in an invalid position in a numeric literal. Underscores must be placed between digits, not at the start, end, or adjacent to a radix prefix.\
"
            }
            Self::N0035 => {
                "\
\x1b[1;31mN0035\x1b[0m \x1b[1mConsecutive digit separators\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nTwo or more consecutive underscore digit separators appear in a numeric literal. Use at most one underscore between digits.\
"
            }
            Self::N0036 => {
                "\
\x1b[1;31mN0036\x1b[0m \x1b[1mTrailing digit separator\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA numeric literal ends with an underscore digit separator. Remove the trailing underscore.\
"
            }
            Self::N0037 => {
                "\
\x1b[1;31mN0037\x1b[0m \x1b[1mLeading digit separator\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA numeric literal begins with an underscore digit separator. Remove the leading underscore.\
"
            }
            Self::N0038 => {
                "\
\x1b[1;31mN0038\x1b[0m \x1b[1mInvalid digit for radix\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA numeric literal contains a digit that is not valid for its specified radix (e.g. 0b1020 has a 2 in binary). Use only digits valid for the radix.\
"
            }
            Self::N0039 => {
                "\
\x1b[1;31mN0039\x1b[0m \x1b[1mUnterminated raw string literal\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLexer\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA raw string literal (r\"...\") was started but not closed before end of file or line. Add a closing double-quote.\
"
            }
            Self::N1001 => {
                "\
\x1b[1;31mN1001\x1b[0m \x1b[1mExpected specific token\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe parser expected a specific token (e.g. ';', ':', '=') at the current position but found something else. This usually means a syntax error, missing punctuation, or a misplaced construct near the indicated location.\
"
            }
            Self::N1002 => {
                "\
\x1b[1;31mN1002\x1b[0m \x1b[1mUnexpected token\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe parser encountered a token that does not make sense at the current position in the grammar. This is a generic syntax error that indicates a structural problem with the code around the indicated location.\
"
            }
            Self::N1003 => {
                "\
\x1b[1;31mN1003\x1b[0m \x1b[1mExpected expression\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe parser expected an expression (a value-producing construct) but found something else. This can happen after operators, after `=`, after `return`, or in other expression contexts.\
"
            }
            Self::N1004 => {
                "\
\x1b[1;31mN1004\x1b[0m \x1b[1mUnclosed delimiter\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA grouping delimiter (parenthesis, bracket, or brace) was opened but not closed by the expected token. Check the matching delimiter at the reported location.\
"
            }
            Self::N1005 => {
                "\
\x1b[1;31mN1005\x1b[0m \x1b[1mExpected indented block\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA colon (`:`) was followed by an end-of-line without an indented block on the next line. Nimble requires an indented body after function definitions, if/else, while, for, and other colon-terminated constructs.\
"
            }
            Self::N1006 => {
                "\
\x1b[1;31mN1006\x1b[0m \x1b[1mUnexpected indentation\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn indentation change occurs at an unexpected location, typically because a line is indented more than expected relative to surrounding blocks.\
"
            }
            Self::N1007 => {
                "\
\x1b[1;31mN1007\x1b[0m \x1b[1mExpected identifier\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe parser expected an identifier (a name) but found something else. Identifiers are required for variable names, function names, type names, and field access.\
"
            }
            Self::N1008 => {
                "\
\x1b[1;31mN1008\x1b[0m \x1b[1mExpected type name\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA type annotation requires a type name but found something else. Type names come after colons in variable declarations and function signatures.\
"
            }
            Self::N1009 => {
                "\
\x1b[1;31mN1009\x1b[0m \x1b[1mExpected parameter name\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA function or method parameter list contains a construct that is not a valid parameter name.\
"
            }
            Self::N1010 => {
                "\
\x1b[1;31mN1010\x1b[0m \x1b[1mExpected colon\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA colon (`:`) was expected in a type annotation, label, or block-introducing construct but was not found.\
"
            }
            Self::N1011 => {
                "\
\x1b[1;31mN1011\x1b[0m \x1b[1mExpected semicolon\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA semicolon (`;`) was expected but not found. In Nimble, semicolons are optional line separators but required in certain contexts like separating statements on the same line.\
"
            }
            Self::N1012 => {
                "\
\x1b[1;31mN1012\x1b[0m \x1b[1mExpected equals sign\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn equals sign (`=`) was expected in an assignment, variable initializer, or default value but was not found.\
"
            }
            Self::N1013 => {
                "\
\x1b[1;31mN1013\x1b[0m \x1b[1mExpected arrow\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA function arrow (`->`) was expected in a return type annotation or lambda expression but was not found.\
"
            }
            Self::N1014 => {
                "\
\x1b[1;31mN1014\x1b[0m \x1b[1mExpected comma\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA comma (`,`) was expected in a list (parameters, arguments, struct fields) but was not found.\
"
            }
            Self::N1015 => {
                "\
\x1b[1;31mN1015\x1b[0m \x1b[1mExpected dot\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA dot (`.`) was expected for member access or qualified path access but was not found.\
"
            }
            Self::N1016 => {
                "\
\x1b[1;31mN1016\x1b[0m \x1b[1mMissing function body\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA function declaration is missing its body. Functions must have either a body block or be declared as extern.\
"
            }
            Self::N1017 => {
                "\
\x1b[1;31mN1017\x1b[0m \x1b[1mMissing return type or return expression\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA function with a declared return type is missing a return expression in its body, or a non-void function is missing its return type annotation.\
"
            }
            Self::N1018 => {
                "\
\x1b[1;31mN1018\x1b[0m \x1b[1mInvalid function parameter\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA function parameter list contains an invalid parameter. Parameters must have the form `name: Type` or `name`.\
"
            }
            Self::N1019 => {
                "\
\x1b[1;31mN1019\x1b[0m \x1b[1mToo many function parameters\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA function declaration exceeds the maximum number of allowed parameters defined by the compiler limit.\
"
            }
            Self::N1020 => {
                "\
\x1b[1;31mN1020\x1b[0m \x1b[1mToo few arguments in call\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA function call provides fewer arguments than the function signature expects.\
"
            }
            Self::N1021 => {
                "\
\x1b[1;31mN1021\x1b[0m \x1b[1mExpected statement\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe parser expected a statement (declaration, assignment, expression statement, etc.) at the current position.\
"
            }
            Self::N1022 => {
                "\
\x1b[1;31mN1022\x1b[0m \x1b[1mExpected binding\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nExpected a `let` or `var` binding declaration at this position. In Nimble, variables must be explicitly declared with `let` (immutable) or `var` (mutable).\
"
            }
            Self::N1023 => {
                "\
\x1b[1;31mN1023\x1b[0m \x1b[1mExpected keyword\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA specific keyword (`if`, `else`, `while`, `for`, `return`, `fn`, etc.) was expected at this position.\
"
            }
            Self::N1024 => {
                "\
\x1b[1;31mN1024\x1b[0m \x1b[1mInvalid left-hand side of assignment\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe left-hand side of an assignment operator (`=`, `+=`, etc.) is not a valid assignment target. Only variables, mutable fields, and indexed expressions can be assigned to.\
"
            }
            Self::N1025 => {
                "\
\x1b[1;31mN1025\x1b[0m \x1b[1mNested function without closure context\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA function is nested inside another function but the context does not support closures. Nested functions require closure support.\
"
            }
            Self::N1026 => {
                "\
\x1b[1;31mN1026\x1b[0m \x1b[1mDuplicate parameter name\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA function has two parameters with the same name. Parameter names must be unique within a function signature.\
"
            }
            Self::N1027 => {
                "\
\x1b[1;31mN1027\x1b[0m \x1b[1mDefault value before non-default parameter\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA parameter with a default value appears before a parameter without one. All parameters with defaults must come after parameters without defaults.\
"
            }
            Self::N1028 => {
                "\
\x1b[1;31mN1028\x1b[0m \x1b[1mExpected module path\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn import or use statement expects a module path (a sequence of identifiers separated by `::` or dots) but found something else.\
"
            }
            Self::N1029 => {
                "\
\x1b[1;31mN1029\x1b[0m \x1b[1mExpected import symbol\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn import statement expects a symbol name to import from a module.\
"
            }
            Self::N1030 => {
                "\
\x1b[1;31mN1030\x1b[0m \x1b[1mCircular import\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nTwo or more modules directly or indirectly import each other, creating a cycle. Nimble does not allow circular imports.\
"
            }
            Self::N1031 => {
                "\
\x1b[1;31mN1031\x1b[0m \x1b[1mBreak outside of loop\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA `break` statement appears outside of any enclosing loop (while, for). break is only valid within loop bodies.\
"
            }
            Self::N1032 => {
                "\
\x1b[1;31mN1032\x1b[0m \x1b[1mContinue outside of loop\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA `continue` statement appears outside of any enclosing loop. continue is only valid within loop bodies.\
"
            }
            Self::N1033 => {
                "\
\x1b[1;31mN1033\x1b[0m \x1b[1mReturn outside of function\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA `return` statement appears outside of any function body. return is only valid inside functions.\
"
            }
            Self::N1034 => {
                "\
\x1b[1;31mN1034\x1b[0m \x1b[1mYield outside of generator\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA `yield` expression appears outside of a generator function. yield is only valid inside generators.\
"
            }
            Self::N1035 => {
                "\
\x1b[1;31mN1035\x1b[0m \x1b[1mInvalid for-loop binding\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe binding pattern in a for-loop is invalid. For-loops require an identifier or destructuring pattern followed by `in` and an iterable expression.\
"
            }
            Self::N1036 => {
                "\
\x1b[1;31mN1036\x1b[0m \x1b[1mExpected `in` in for-loop\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA for-loop is missing the `in` keyword between the binding and the iterable expression.\
"
            }
            Self::N1037 => {
                "\
\x1b[1;31mN1037\x1b[0m \x1b[1mEmpty struct body\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA struct declaration has no fields. Structs must have at least one field.\
"
            }
            Self::N1038 => {
                "\
\x1b[1;31mN1038\x1b[0m \x1b[1mEmpty interface body\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn interface declaration has no methods. Interfaces must have at least one method signature.\
"
            }
            Self::N1039 => {
                "\
\x1b[1;31mN1039\x1b[0m \x1b[1mMethod declaration outside of interface\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA method-like declaration appears outside of an interface or struct body.\
"
            }
            Self::N1040 => {
                "\
\x1b[1;31mN1040\x1b[0m \x1b[1mDuplicate field name in struct\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA struct has two or more fields with the same name. Field names must be unique within a struct.\
"
            }
            Self::N1041 => {
                "\
\x1b[1;31mN1041\x1b[0m \x1b[1mUnnamed field in struct literal\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA struct literal uses a positional (unnamed) value but the struct expects named fields. Use `field: value` syntax.\
"
            }
            Self::N1042 => {
                "\
\x1b[1;31mN1042\x1b[0m \x1b[1mExpected struct expression\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA struct literal expression is expected but something else was found. Struct literals use `Name { field: value, ... }` syntax.\
"
            }
            Self::N1043 => {
                "\
\x1b[1;31mN1043\x1b[0m \x1b[1mMissing colon in type annotation\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA type annotation is missing the colon separator between the name and its type. Use `name: Type` syntax.\
"
            }
            Self::N1044 => {
                "\
\x1b[1;31mN1044\x1b[0m \x1b[1mUnexpected trailing comma\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA trailing comma appears where it is not syntactically allowed.\
"
            }
            Self::N1045 => {
                "\
\x1b[1;31mN1045\x1b[0m \x1b[1mMalformed string interpolation expression\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA string interpolation expression within a string has invalid syntax. Interpolation expressions must be valid expressions enclosed in the proper delimiters.\
"
            }
            Self::N1046 => {
                "\
\x1b[1;31mN1046\x1b[0m \x1b[1mUnterminated block comment\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA block comment (/*) was started but the closing (*/) was not found before end of file.\
"
            }
            Self::N1047 => {
                "\
\x1b[1;31mN1047\x1b[0m \x1b[1mExpected attribute\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn attribute annotation is expected at this position. Attributes use `#[name]` syntax.\
"
            }
            Self::N1048 => {
                "\
\x1b[1;31mN1048\x1b[0m \x1b[1mInvalid attribute target\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn attribute is applied to a construct that does not support attributes.\
"
            }
            Self::N1049 => {
                "\
\x1b[1;31mN1049\x1b[0m \x1b[1mDuplicate attribute\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe same attribute is applied more than once to the same construct.\
"
            }
            Self::N1050 => {
                "\
\x1b[1;31mN1050\x1b[0m \x1b[1mUnknown attribute\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn unrecognized attribute name was used. Check the spelling of the attribute.\
"
            }
            Self::N1051 => {
                "\
\x1b[1;31mN1051\x1b[0m \x1b[1mExpected type arguments\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA generic type is missing its type argument list. Use `Type<T, U>` syntax.\
"
            }
            Self::N1052 => {
                "\
\x1b[1;31mN1052\x1b[0m \x1b[1mUnclosed type argument list\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA type argument list (`<`) was opened but not closed (`>`) before the expected position.\
"
            }
            Self::N1053 => {
                "\
\x1b[1;31mN1053\x1b[0m \x1b[1mType argument count mismatch\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe number of type arguments provided does not match the number of type parameters defined by the generic.\
"
            }
            Self::N1054 => {
                "\
\x1b[1;31mN1054\x1b[0m \x1b[1mExpected `<` for generics\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA `<` token was expected to open a generic or type argument list.\
"
            }
            Self::N1055 => {
                "\
\x1b[1;31mN1055\x1b[0m \x1b[1mExpected `>` for generics\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA `>` token was expected to close a generic or type argument list.\
"
            }
            Self::N1056 => {
                "\
\x1b[1;31mN1056\x1b[0m \x1b[1mUnterminated lambda body\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA lambda expression has no body or the body is incomplete. Lambdas use `|params| expr` syntax.\
"
            }
            Self::N1057 => {
                "\
\x1b[1;31mN1057\x1b[0m \x1b[1mExpected binding in for-loop\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA for-loop is missing its binding variable. Use `for var in iter: body`.\
"
            }
            Self::N1058 => {
                "\
\x1b[1;31mN1058\x1b[0m \x1b[1mExpected path expression\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA qualified path expression was expected. Paths use `module::name` or `object.field` syntax.\
"
            }
            Self::N1059 => {
                "\
\x1b[1;31mN1059\x1b[0m \x1b[1mExpected literal\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA literal value was expected but something else was found.\
"
            }
            Self::N1060 => {
                "\
\x1b[1;31mN1060\x1b[0m \x1b[1mExpected pattern\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA pattern (used in match arms, destructuring, or binding) was expected but something else was found.\
"
            }
            Self::N1061 => {
                "\
\x1b[1;31mN1061\x1b[0m \x1b[1mExpected guard expression\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA match arm with a guard (`if` clause) is missing the guard expression after `if`.\
"
            }
            Self::N1062 => {
                "\
\x1b[1;31mN1062\x1b[0m \x1b[1mExpected where clause\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nExpected a `where` clause for specifying trait bounds or constraints.\
"
            }
            Self::N1063 => {
                "\
\x1b[1;31mN1063\x1b[0m \x1b[1mExpected semicolon or newline after statement\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA statement must be terminated by either a semicolon or a newline. Two statements cannot appear on the same line without a separator.\
"
            }
            Self::N1064 => {
                "\
\x1b[1;31mN1064\x1b[0m \x1b[1mExpected operator\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn operator (such as `+`, `-`, `*`, `/`, `==`, etc.) was expected at this position, typically in an expression context.\
"
            }
            Self::N1065 => {
                "\
\x1b[1;31mN1065\x1b[0m \x1b[1mInvalid prefix operator\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe prefix operator used is not valid for the type of expression it precedes.\
"
            }
            Self::N1066 => {
                "\
\x1b[1;31mN1066\x1b[0m \x1b[1mInvalid postfix operator\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe postfix operator used is not valid for the type of expression it follows.\
"
            }
            Self::N1067 => {
                "\
\x1b[1;31mN1067\x1b[0m \x1b[1mOperator precedence ambiguity\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe combination of operators in an expression is ambiguous and requires explicit grouping with parentheses.\
"
            }
            Self::N1068 => {
                "\
\x1b[1;31mN1068\x1b[0m \x1b[1mIncompatible comparison chaining\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nComparisons of different types or incompatible operators cannot be chained together. Use parentheses to group comparisons explicitly.\
"
            }
            Self::N1069 => {
                "\
\x1b[1;31mN1069\x1b[0m \x1b[1mExpected tuple element\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA tuple expression or type is missing an element. Use `(a, b, c)` syntax with commas between elements.\
"
            }
            Self::N1070 => {
                "\
\x1b[1;31mN1070\x1b[0m \x1b[1mExpected array element\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn array literal is missing an element. Use `[a, b, c]` syntax with commas between elements.\
"
            }
            Self::N1071 => {
                "\
\x1b[1;31mN1071\x1b[0m \x1b[1mExpected struct field\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA struct literal is missing a field. Use `Struct { field: value }` syntax.\
"
            }
            Self::N1072 => {
                "\
\x1b[1;31mN1072\x1b[0m \x1b[1mExpected enum variant\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn enum variant pattern or constructor was expected but something else was found.\
"
            }
            Self::N1073 => {
                "\
\x1b[1;31mN1073\x1b[0m \x1b[1mExpected match arm\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA match expression must have at least one arm (pattern => expression). Add at least one match arm.\
"
            }
            Self::N1074 => {
                "\
\x1b[1;31mN1074\x1b[0m \x1b[1mExpected `=>` in match arm\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA match arm pattern is not followed by `=>`. Use `pattern => expression` syntax.\
"
            }
            Self::N1075 => {
                "\
\x1b[1;31mN1075\x1b[0m \x1b[1mExpected pattern guard\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA match arm guard (if clause) is missing its condition expression.\
"
            }
            Self::N1076 => {
                "\
\x1b[1;31mN1076\x1b[0m \x1b[1mInvalid doc comment placement\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA doc comment (`///` or `//!`) appears in a position where documentation is not allowed.\
"
            }
            Self::N1077 => {
                "\
\x1b[1;31mN1077\x1b[0m \x1b[1mExpected doc comment\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA documentation comment was expected, typically before a public API item.\
"
            }
            Self::N1078 => {
                "\
\x1b[1;31mN1078\x1b[0m \x1b[1mInvalid visibility modifier\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA visibility modifier (pub, pub(crate), etc.) is being used in an invalid position or with an incorrect syntax.\
"
            }
            Self::N1079 => {
                "\
\x1b[1;31mN1079\x1b[0m \x1b[1mExpected item\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA top-level or nested item (function, struct, interface, constant, etc.) was expected at this position.\
"
            }
            Self::N1080 => {
                "\
\x1b[1;31mN1080\x1b[0m \x1b[1mNested function without body\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA nested function declaration has no body. Nested functions must have an inline body.\
"
            }
            Self::N1081 => {
                "\
\x1b[1;31mN1081\x1b[0m \x1b[1mUnterminated generic list\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA generic parameter list (`<`) was started but not closed (`>`) before a structural end was reached.\
"
            }
            Self::N1082 => {
                "\
\x1b[1;31mN1082\x1b[0m \x1b[1mExpected lifetime parameter\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA lifetime parameter (using `'a` syntax) was expected but not found.\
"
            }
            Self::N1083 => {
                "\
\x1b[1;31mN1083\x1b[0m \x1b[1mExpected const parameter\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA const generic parameter was expected but not found. Const parameters use `const NAME: Type` syntax.\
"
            }
            Self::N1084 => {
                "\
\x1b[1;31mN1084\x1b[0m \x1b[1mAmbiguous literal suffix\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mParser\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA numeric literal has a suffix that could refer to multiple types or is not a recognized suffix. Add an explicit type annotation or use a standard suffix.\
"
            }
            Self::N2001 => {
                "\
\x1b[1;31mN2001\x1b[0m \x1b[1mUndefined variable\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA variable name is used that has not been defined in the current scope. Check the spelling and ensure the variable is declared with `let` or `var` before use.\
"
            }
            Self::N2002 => {
                "\
\x1b[1;31mN2002\x1b[0m \x1b[1mDuplicate definition\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA name is defined more than once in the same scope. Every variable, function, type, and module name must be unique within its scope.\
"
            }
            Self::N2003 => {
                "\
\x1b[1;31mN2003\x1b[0m \x1b[1mUndefined function\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA function call references a function name that has not been defined in any accessible scope.\
"
            }
            Self::N2004 => {
                "\
\x1b[1;31mN2004\x1b[0m \x1b[1mUndefined struct\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA struct type name is used but no matching struct definition is found.\
"
            }
            Self::N2005 => {
                "\
\x1b[1;31mN2005\x1b[0m \x1b[1mUndefined interface\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn interface name is used but no matching interface definition is found.\
"
            }
            Self::N2006 => {
                "\
\x1b[1;31mN2006\x1b[0m \x1b[1mUndefined module\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA module name in an import path cannot be found in the module search path.\
"
            }
            Self::N2007 => {
                "\
\x1b[1;31mN2007\x1b[0m \x1b[1mUndefined type\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA type name is used that has not been defined. This includes built-in types and user-defined types.\
"
            }
            Self::N2008 => {
                "\
\x1b[1;31mN2008\x1b[0m \x1b[1mUndefined macro\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA macro invocation references a macro name that has not been defined.\
"
            }
            Self::N2009 => {
                "\
\x1b[1;31mN2009\x1b[0m \x1b[1mAccess to private item\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA private item (function, type, field) from another module is being accessed outside of its defining module.\
"
            }
            Self::N2010 => {
                "\
\x1b[1;31mN2010\x1b[0m \x1b[1mCyclic module dependency\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nTwo or more modules depend on each other directly or indirectly, forming a cycle. Nimble does not support circular dependencies.\
"
            }
            Self::N2011 => {
                "\
\x1b[1;31mN2011\x1b[0m \x1b[1mCyclic type definition\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA type definition refers to itself directly or indirectly in a way that creates an infinite type.\
"
            }
            Self::N2012 => {
                "\
\x1b[1;31mN2012\x1b[0m \x1b[1mAmbiguous name\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA name resolves to more than one definition in the current scope. Use a qualified path to disambiguate.\
"
            }
            Self::N2013 => {
                "\
\x1b[1;31mN2013\x1b[0m \x1b[1mInvalid visibility qualifier\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA visibility qualifier is used in an invalid context or has incorrect syntax.\
"
            }
            Self::N2014 => {
                "\
\x1b[1;31mN2014\x1b[0m \x1b[1mModule not found\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe module specified in an import statement cannot be found in any of the configured module search paths.\
"
            }
            Self::N2015 => {
                "\
\x1b[1;31mN2015\x1b[0m \x1b[1mSymbol not exported\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn import statement tries to import a symbol that exists in the target module but is not publicly exported.\
"
            }
            Self::N2016 => {
                "\
\x1b[1;31mN2016\x1b[0m \x1b[1mName conflicts with builtin\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA user-defined name conflicts with a built-in name. Choose a different name.\
"
            }
            Self::N2017 => {
                "\
\x1b[1;33mN2017\x1b[0m \x1b[1mName shadows builtin\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA user-defined name shadows a built-in name. This is allowed but can lead to confusion.\
"
            }
            Self::N2018 => {
                "\
\x1b[1;33mN2018\x1b[0m \x1b[1mUnused import\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn import statement introduces a name that is never used in the current file.\
"
            }
            Self::N2019 => {
                "\
\x1b[1;33mN2019\x1b[0m \x1b[1mWildcard import leaks names\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA wildcard import (`use module::*`) brings names into scope that may conflict with other imports.\
"
            }
            Self::N2020 => {
                "\
\x1b[1;31mN2020\x1b[0m \x1b[1m`self` used outside of method\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe `self` keyword is used outside of a method body. `self` is only valid as a method parameter or within method bodies.\
"
            }
            Self::N2021 => {
                "\
\x1b[1;31mN2021\x1b[0m \x1b[1m`super` used outside of class context\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe `super` keyword is used outside of a class or type context. super is only valid for accessing parent type members.\
"
            }
            Self::N2022 => {
                "\
\x1b[1;31mN2022\x1b[0m \x1b[1mInvalid self parameter\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA method's self parameter has an invalid type or position. Self must be the first parameter.\
"
            }
            Self::N2023 => {
                "\
\x1b[1;31mN2023\x1b[0m \x1b[1mMethod without self parameter\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA method in an implementation block has no self parameter. Methods must take `self` (or `&self`, `mut self`) as the first parameter.\
"
            }
            Self::N2024 => {
                "\
\x1b[1;31mN2024\x1b[0m \x1b[1mReturn type mismatch in method impl\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA method implementation's return type does not match the return type declared in the interface or parent definition.\
"
            }
            Self::N2025 => {
                "\
\x1b[1;31mN2025\x1b[0m \x1b[1mMissing required interface method\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA type that claims to implement an interface does not provide an implementation for all required methods.\
"
            }
            Self::N2026 => {
                "\
\x1b[1;31mN2026\x1b[0m \x1b[1mExtra method not in interface\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA type implementing an interface defines additional methods that are not part of the interface. This is not allowed.\
"
            }
            Self::N2027 => {
                "\
\x1b[1;31mN2027\x1b[0m \x1b[1mInvalid method override\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA method override does not match the signature of the method it is overriding. Parameter types and return types must match.\
"
            }
            Self::N2028 => {
                "\
\x1b[1;31mN2028\x1b[0m \x1b[1mOverride without base definition\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA method is marked as override but no matching method exists in any parent type or interface.\
"
            }
            Self::N2029 => {
                "\
\x1b[1;31mN2029\x1b[0m \x1b[1mInconsistent associated type binding\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn associated type in a trait or interface implementation is bound to a type that conflicts with other constraints.\
"
            }
            Self::N2030 => {
                "\
\x1b[1;31mN2030\x1b[0m \x1b[1mCircular trait bound\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nTrait bounds form a cycle (e.g. A: B and B: A). Circular bounds are not allowed.\
"
            }
            Self::N2031 => {
                "\
\x1b[1;33mN2031\x1b[0m \x1b[1mUnused variable\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA variable is declared but never used. Consider removing it or prefixing with an underscore.\
"
            }
            Self::N2032 => {
                "\
\x1b[1;33mN2032\x1b[0m \x1b[1mUnused assignment\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA value is assigned to a variable but the variable is never read afterward.\
"
            }
            Self::N2033 => {
                "\
\x1b[1;33mN2033\x1b[0m \x1b[1mVariable shadows outer variable\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA variable declaration shadows a variable from an outer scope. Consider renaming to avoid confusion.\
"
            }
            Self::N2034 => {
                "\
\x1b[1;33mN2034\x1b[0m \x1b[1mUnreachable pattern\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA pattern in a match expression can never be reached because a previous pattern already covers all matching values.\
"
            }
            Self::N2035 => {
                "\
\x1b[1;31mN2035\x1b[0m \x1b[1mNon-exhaustive patterns\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA match expression does not cover all possible values of the matched type. Add a catch-all pattern (`_ => ...`) or handle all variants.\
"
            }
            Self::N2036 => {
                "\
\x1b[1;31mN2036\x1b[0m \x1b[1mPattern binding conflict\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA pattern binds the same name more than once. Each binding in a pattern must have a unique name.\
"
            }
            Self::N2037 => {
                "\
\x1b[1;31mN2037\x1b[0m \x1b[1mIllegal binding mode in pattern\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA pattern uses an unsupported binding mode (e.g., ref, mut) in a context where it is not allowed.\
"
            }
            Self::N2038 => {
                "\
\x1b[1;31mN2038\x1b[0m \x1b[1mInvalid pattern syntax\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe pattern syntax is invalid. Patterns must follow the allowed grammar (literals, identifiers, destructuring, etc.).\
"
            }
            Self::N2039 => {
                "\
\x1b[1;31mN2039\x1b[0m \x1b[1mUnresolved import\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn import path or symbol cannot be resolved. Check the module path and symbol name for typos.\
"
            }
            Self::N2040 => {
                "\
\x1b[1;31mN2040\x1b[0m \x1b[1mUnresolved re-export\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA re-export (`pub use`) refers to a symbol that cannot be found.\
"
            }
            Self::N2041 => {
                "\
\x1b[1;31mN2041\x1b[0m \x1b[1mPrivate re-export\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA re-export attempts to publicly expose a private symbol from another module.\
"
            }
            Self::N2042 => {
                "\
\x1b[1;31mN2042\x1b[0m \x1b[1mConflicting re-export\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nTwo or more re-exports attempt to expose different items with the same name.\
"
            }
            Self::N2043 => {
                "\
\x1b[1;31mN2043\x1b[0m \x1b[1mRe-export of non-existent symbol\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA re-export refers to a symbol that does not exist in the source module.\
"
            }
            Self::N2044 => {
                "\
\x1b[1;31mN2044\x1b[0m \x1b[1mSelf-import\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA module attempts to import itself. Remove the self-import.\
"
            }
            Self::N2045 => {
                "\
\x1b[1;31mN2045\x1b[0m \x1b[1mInvalid use of `Self` type alias\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe `Self` type alias is used in an invalid context. `Self` is only valid inside trait or interface definitions.\
"
            }
            Self::N2046 => {
                "\
\x1b[1;31mN2046\x1b[0m \x1b[1mExternal crate not found\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn external crate dependency specified in an import cannot be found. Ensure the crate is listed in the manifest and installed.\
"
            }
            Self::N2047 => {
                "\
\x1b[1;31mN2047\x1b[0m \x1b[1mExternal crate version conflict\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nTwo dependencies require different versions of the same external crate, creating a conflict.\
"
            }
            Self::N2048 => {
                "\
\x1b[1;31mN2048\x1b[0m \x1b[1mExternal crate feature not found\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA feature of an external crate requested in an import or config does not exist.\
"
            }
            Self::N2049 => {
                "\
\x1b[1;33mN2049\x1b[0m \x1b[1mUnused extern crate\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mName Resolution\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn external crate is declared as a dependency but never imported or used.\
"
            }
            Self::N3001 => {
                "\
\x1b[1;31mN3001\x1b[0m \x1b[1mType mismatch\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn expression has a type that does not match the expected type in this context. This can happen in assignments, function arguments, return statements, and binary operations. Ensure the expression produces the correct type.\
"
            }
            Self::N3002 => {
                "\
\x1b[1;31mN3002\x1b[0m \x1b[1mAssign to immutable variable\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn attempt is made to assign a new value to a variable declared with `let`. Use `var` instead of `let` if the variable needs to be reassigned.\
"
            }
            Self::N3003 => {
                "\
\x1b[1;31mN3003\x1b[0m \x1b[1mUndefined type\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA type name used in an annotation or expression is not defined in any accessible scope. Check for typos and ensure the type is imported.\
"
            }
            Self::N3004 => {
                "\
\x1b[1;31mN3004\x1b[0m \x1b[1mCall of non-function value\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA call expression attempts to invoke a value that is not a function. Only functions, closures, and callable objects can be called.\
"
            }
            Self::N3005 => {
                "\
\x1b[1;31mN3005\x1b[0m \x1b[1mArgument count mismatch\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA function call provides a different number of arguments than the function signature expects. Check both the count and the presence of default arguments.\
"
            }
            Self::N3006 => {
                "\
\x1b[1;31mN3006\x1b[0m \x1b[1mMissing required method\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA type is used where an interface or trait is expected, but the type does not implement all required methods.\
"
            }
            Self::N3007 => {
                "\
\x1b[1;31mN3007\x1b[0m \x1b[1mRecursive type without indirection\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA type definition is recursive without using indirection (e.g., Box, reference). Direct recursion in value types leads to infinite size.\
"
            }
            Self::N3008 => {
                "\
\x1b[1;31mN3008\x1b[0m \x1b[1mInfinite type\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nType inference produces an infinitely recursive type. This usually happens when a variable is used in a way that creates a cycle in its type.\
"
            }
            Self::N3009 => {
                "\
\x1b[1;31mN3009\x1b[0m \x1b[1mCannot infer type\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe type of an expression cannot be inferred from context. Add an explicit type annotation.\
"
            }
            Self::N3010 => {
                "\
\x1b[1;31mN3010\x1b[0m \x1b[1mType annotation required\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA construct requires an explicit type annotation but none was provided. This is often needed for function parameters and certain variable definitions where inference is ambiguous.\
"
            }
            Self::N3011 => {
                "\
\x1b[1;31mN3011\x1b[0m \x1b[1mExpected concrete type, found abstract\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA concrete type was expected but an abstract type (trait, interface, or type variable) was provided. Use a specific concrete type.\
"
            }
            Self::N3012 => {
                "\
\x1b[1;31mN3012\x1b[0m \x1b[1mTrait bound not satisfied\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA type is used in a context that requires it to implement a specific trait or interface, but the type does not satisfy the bound.\
"
            }
            Self::N3013 => {
                "\
\x1b[1;31mN3013\x1b[0m \x1b[1mAssociated type not specified\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA trait or interface has an associated type that must be specified but has not been provided. Use the `type Assoc = ConcreteType;` syntax.\
"
            }
            Self::N3014 => {
                "\
\x1b[1;31mN3014\x1b[0m \x1b[1mWrong number of type arguments\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA generic type or function is provided with a different number of type arguments than it expects.\
"
            }
            Self::N3015 => {
                "\
\x1b[1;31mN3015\x1b[0m \x1b[1mType argument out of bounds\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA type argument does not satisfy the bounds declared for the corresponding type parameter.\
"
            }
            Self::N3016 => {
                "\
\x1b[1;31mN3016\x1b[0m \x1b[1mConflicting type arguments\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nTwo type arguments to the same generic are in conflict with each other.\
"
            }
            Self::N3017 => {
                "\
\x1b[1;31mN3017\x1b[0m \x1b[1mCross-module type violation\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA type from one module is used in a way that violates the type's invariants across module boundaries.\
"
            }
            Self::N3018 => {
                "\
\x1b[1;31mN3018\x1b[0m \x1b[1mBorrow of immutable as mutable\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn immutable reference is used where a mutable reference is required. Use `mut` on the binding or pass a mutable reference explicitly.\
"
            }
            Self::N3019 => {
                "\
\x1b[1;31mN3019\x1b[0m \x1b[1mBorrow of moved value\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA value that has been moved to another owner is being used. Use a reference instead of moving, or reorder operations.\
"
            }
            Self::N3020 => {
                "\
\x1b[1;31mN3020\x1b[0m \x1b[1mUse after move\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA value is used after it has been moved. The ownership was transferred and the original binding is no longer valid.\
"
            }
            Self::N3021 => {
                "\
\x1b[1;31mN3021\x1b[0m \x1b[1mMultiple mutable borrows\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA value is mutably borrowed more than once at the same time. Only one mutable borrow is allowed at a time.\
"
            }
            Self::N3022 => {
                "\
\x1b[1;31mN3022\x1b[0m \x1b[1mLifetime mismatch\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe lifetimes of two references are incompatible. The borrowed value does not live long enough to satisfy the lifetime constraint.\
"
            }
            Self::N3023 => {
                "\
\x1b[1;31mN3023\x1b[0m \x1b[1mLifetime elision failure\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe compiler cannot infer the lifetime of a reference using the default elision rules. Add explicit lifetime annotations.\
"
            }
            Self::N3024 => {
                "\
\x1b[1;31mN3024\x1b[0m \x1b[1mLifetime bound not satisfied\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA reference's lifetime does not satisfy the bounds required by the context. The data must live at least as long as the constraint requires.\
"
            }
            Self::N3025 => {
                "\
\x1b[1;31mN3025\x1b[0m \x1b[1mLifetime constraint violation\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA lifetime constraint specified in the code is violated by the actual usage. Ensure all borrows respect the declared lifetimes.\
"
            }
            Self::N3026 => {
                "\
\x1b[1;31mN3026\x1b[0m \x1b[1mMissing lifetime annotation\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA function or type signature uses references but is missing required lifetime annotations. Add lifetime parameters like `'a`.\
"
            }
            Self::N3027 => {
                "\
\x1b[1;31mN3027\x1b[0m \x1b[1mInvalid lifetime name\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA lifetime name is not valid. Lifetime names must start with a tick (`'`) followed by an identifier.\
"
            }
            Self::N3028 => {
                "\
\x1b[1;31mN3028\x1b[0m \x1b[1mMismatched mutability in reference\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe mutability of a reference does not match what is required. An immutable reference cannot be used where a mutable reference is needed, and vice versa.\
"
            }
            Self::N3029 => {
                "\
\x1b[1;31mN3029\x1b[0m \x1b[1mDangling reference\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA reference outlives the data it points to. The referenced value has been dropped while the reference still exists.\
"
            }
            Self::N3030 => {
                "\
\x1b[1;31mN3030\x1b[0m \x1b[1mDrop of type with move semantics\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn attempt is made to drop a value that has move semantics in a context that does not support it.\
"
            }
            Self::N3031 => {
                "\
\x1b[1;31mN3031\x1b[0m \x1b[1mBorrow of constant value\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA constant value is being borrowed mutably. Constants are immutable and cannot be borrowed as mutable.\
"
            }
            Self::N3032 => {
                "\
\x1b[1;31mN3032\x1b[0m \x1b[1mNumeric overflow in constant expression\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA constant expression produces a numeric value that overflows the target type. Use a smaller value or a larger type.\
"
            }
            Self::N3033 => {
                "\
\x1b[1;31mN3033\x1b[0m \x1b[1mDivision by zero in constant expression\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA constant expression contains a division by zero. Ensure all divisors are non-zero at compile time.\
"
            }
            Self::N3034 => {
                "\
\x1b[1;31mN3034\x1b[0m \x1b[1mRemainder by zero in constant expression\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA constant expression contains a remainder operation with a zero divisor.\
"
            }
            Self::N3035 => {
                "\
\x1b[1;31mN3035\x1b[0m \x1b[1mNegation of unsigned integer\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe unary negation operator (`-`) is applied to an unsigned integer type. Unsigned types cannot represent negative values.\
"
            }
            Self::N3036 => {
                "\
\x1b[1;31mN3036\x1b[0m \x1b[1mShift exceeds bit width\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA bit shift operation uses a shift amount that is greater than or equal to the bit width of the type.\
"
            }
            Self::N3037 => {
                "\
\x1b[1;31mN3037\x1b[0m \x1b[1mOperator not applicable to types\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn operator is used with operand types that do not support that operation. Check the operator and types are compatible.\
"
            }
            Self::N3038 => {
                "\
\x1b[1;31mN3038\x1b[0m \x1b[1mComparison of unordered values\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA comparison operator is applied to values that cannot be ordered (e.g., floats with NaN, or types that do not implement comparison traits).\
"
            }
            Self::N3039 => {
                "\
\x1b[1;31mN3039\x1b[0m \x1b[1mInvalid unary operator for type\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA unary operator (like `-`, `!`, `~`) is applied to a type that does not support it.\
"
            }
            Self::N3040 => {
                "\
\x1b[1;31mN3040\x1b[0m \x1b[1mInvalid binary operator for types\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA binary operator (like `+`, `-`, `*`, `/`, `==`) is applied to types that do not support it. Check the operand types.\
"
            }
            Self::N3041 => {
                "\
\x1b[1;31mN3041\x1b[0m \x1b[1mNo common operator overload\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe operator overload resolution cannot find a suitable implementation for the given operand types.\
"
            }
            Self::N3042 => {
                "\
\x1b[1;31mN3042\x1b[0m \x1b[1mAmbiguous operator application\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nMore than one operator overload matches the operand types. Use an explicit method call to disambiguate.\
"
            }
            Self::N3043 => {
                "\
\x1b[1;31mN3043\x1b[0m \x1b[1mWrong number of generic type parameters\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA generic type or function is instantiated with the wrong number of type parameters.\
"
            }
            Self::N3044 => {
                "\
\x1b[1;31mN3044\x1b[0m \x1b[1mGeneric bound not satisfied\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA type argument does not satisfy the bounds declared on the generic parameter.\
"
            }
            Self::N3045 => {
                "\
\x1b[1;31mN3045\x1b[0m \x1b[1mMissing generic type annotation\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA generic construct requires a type annotation that cannot be inferred. Provide the type parameters explicitly.\
"
            }
            Self::N3046 => {
                "\
\x1b[1;33mN3046\x1b[0m \x1b[1mGeneric parameter not used\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA generic type parameter is declared but never used in the function signature or body. Consider removing it.\
"
            }
            Self::N3047 => {
                "\
\x1b[1;31mN3047\x1b[0m \x1b[1mConcrete type in abstract context\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA concrete type is used where an abstract type (interface/trait) was expected. The concrete type does not implement the required abstraction.\
"
            }
            Self::N3048 => {
                "\
\x1b[1;31mN3048\x1b[0m \x1b[1mAbstract type in concrete context\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn abstract type is used where a concrete type is required. Abstract types cannot be instantiated or used in value contexts.\
"
            }
            Self::N3049 => {
                "\
\x1b[1;31mN3049\x1b[0m \x1b[1mNon-constant in const context\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA non-constant expression is used where a compile-time constant is required. Only literal values and const functions can be used in const contexts.\
"
            }
            Self::N3050 => {
                "\
\x1b[1;31mN3050\x1b[0m \x1b[1mNon-const call in const context\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA function that is not declared as `const` is called in a const context. Only const functions can be called at compile time.\
"
            }
            Self::N3051 => {
                "\
\x1b[1;31mN3051\x1b[0m \x1b[1mMutable reference in const context\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA mutable reference is used in a const context. Const contexts do not allow mutation.\
"
            }
            Self::N3052 => {
                "\
\x1b[1;31mN3052\x1b[0m \x1b[1mIf condition must be boolean\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe condition in an `if` expression must be of type Bool. The provided expression has a non-boolean type.\
"
            }
            Self::N3053 => {
                "\
\x1b[1;31mN3053\x1b[0m \x1b[1mWhile condition must be boolean\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe condition in a `while` loop must be of type Bool. The provided expression has a non-boolean type.\
"
            }
            Self::N3054 => {
                "\
\x1b[1;31mN3054\x1b[0m \x1b[1mFor-each binding type mismatch\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe binding variable type in a for loop does not match the element type of the iterable expression.\
"
            }
            Self::N3055 => {
                "\
\x1b[1;31mN3055\x1b[0m \x1b[1mReturn type mismatch\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA return expression's type does not match the declared return type of the function. Ensure the returned value has the correct type.\
"
            }
            Self::N3056 => {
                "\
\x1b[1;31mN3056\x1b[0m \x1b[1mMissing return value\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA function with a non-void return type has a code path that does not return a value. Add a return statement to all code paths.\
"
            }
            Self::N3057 => {
                "\
\x1b[1;31mN3057\x1b[0m \x1b[1mExtra return value from void function\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA function declared as returning Void contains a return expression with a value. Remove the value or change the return type.\
"
            }
            Self::N3058 => {
                "\
\x1b[1;31mN3058\x1b[0m \x1b[1mReturn not allowed here\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA return statement appears in a context where it is not valid, such as inside a closure that does not capture the function's return path.\
"
            }
            Self::N3059 => {
                "\
\x1b[1;31mN3059\x1b[0m \x1b[1mMissing async annotation\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA function that uses `await` is not marked as `async`. Add the async keyword to the function signature.\
"
            }
            Self::N3060 => {
                "\
\x1b[1;31mN3060\x1b[0m \x1b[1mCannot await non-future\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe `await` expression is used on a value that is not a Future. Only values of type Future can be awaited.\
"
            }
            Self::N3061 => {
                "\
\x1b[1;33mN3061\x1b[0m \x1b[1mIncompatible implicit conversion\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn implicit type conversion might lose information or is not allowed. Consider an explicit cast.\
"
            }
            Self::N3062 => {
                "\
\x1b[1;31mN3062\x1b[0m \x1b[1mForward declaration type mismatch\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe type of a forward-declared item does not match the full definition. The types must be consistent.\
"
            }
            Self::N3063 => {
                "\
\x1b[1;31mN3063\x1b[0m \x1b[1mMissing forward declaration\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn item that requires a forward declaration (e.g., mutually recursive functions) does not have one.\
"
            }
            Self::N3064 => {
                "\
\x1b[1;31mN3064\x1b[0m \x1b[1mField type mismatch in struct literal\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe type of a value in a struct literal does not match the declared field type.\
"
            }
            Self::N3065 => {
                "\
\x1b[1;31mN3065\x1b[0m \x1b[1mMissing field in struct literal\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA struct literal does not provide a value for all required fields.\
"
            }
            Self::N3066 => {
                "\
\x1b[1;31mN3066\x1b[0m \x1b[1mExtra field in struct literal\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA struct literal contains a field name that does not exist in the struct definition.\
"
            }
            Self::N3067 => {
                "\
\x1b[1;31mN3067\x1b[0m \x1b[1mAmbiguous field in struct literal\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA field name in a struct literal could refer to multiple fields (e.g., due to inheritance or mixins).\
"
            }
            Self::N3068 => {
                "\
\x1b[1;31mN3068\x1b[0m \x1b[1mCyclic struct definition\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA struct definition refers to itself in a way that creates an infinite-sized type. Use a pointer or Box for recursive references.\
"
            }
            Self::N3069 => {
                "\
\x1b[1;31mN3069\x1b[0m \x1b[1mTuple index out of bounds\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA tuple index accesses an element that does not exist. Tuple indices start at 0 and must be less than the tuple's arity.\
"
            }
            Self::N3070 => {
                "\
\x1b[1;31mN3070\x1b[0m \x1b[1mArray index out of bounds\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA constant array index expression has a value outside the array's defined bounds.\
"
            }
            Self::N3071 => {
                "\
\x1b[1;31mN3071\x1b[0m \x1b[1mNon-comptime array index\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn array index must be a compile-time constant expression but a variable expression was used.\
"
            }
            Self::N3072 => {
                "\
\x1b[1;31mN3072\x1b[0m \x1b[1mMismatched array length in type\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn array type annotation specifies a length that does not match the actual array literal.\
"
            }
            Self::N3073 => {
                "\
\x1b[1;31mN3073\x1b[0m \x1b[1mType alias cycle\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA type alias directly or indirectly refers to itself, creating a cycle.\
"
            }
            Self::N3074 => {
                "\
\x1b[1;31mN3074\x1b[0m \x1b[1mType alias uses non-existent type\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA type alias refers to a type that does not exist or is not in scope.\
"
            }
            Self::N3075 => {
                "\
\x1b[1;31mN3075\x1b[0m \x1b[1mUnsized type in field\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn unsized type (type whose size is unknown at compile time) is used as a struct field. Use a pointer or Box.\
"
            }
            Self::N3076 => {
                "\
\x1b[1;31mN3076\x1b[0m \x1b[1mUnsized type in local variable\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn unsized type is used as a local variable. Local variables must have a known size at compile time.\
"
            }
            Self::N3077 => {
                "\
\x1b[1;31mN3077\x1b[0m \x1b[1mUnsized type in parameter\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn unsized type is used as a function parameter. Use a reference or pointer parameter instead.\
"
            }
            Self::N3078 => {
                "\
\x1b[1;31mN3078\x1b[0m \x1b[1mUnsized type in return position\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn unsized type is used as a return type. Return types must have a known size.\
"
            }
            Self::N3079 => {
                "\
\x1b[1;31mN3079\x1b[0m \x1b[1mUnsized type in struct field\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn unsized type appears as a struct field. Wrap it in a pointer type like `Box<T>`.\
"
            }
            Self::N3080 => {
                "\
\x1b[1;31mN3080\x1b[0m \x1b[1mUnexpected type parameter\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA type parameter is provided in a context that does not expect one.\
"
            }
            Self::N3081 => {
                "\
\x1b[1;31mN3081\x1b[0m \x1b[1mExpected type parameter\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA generic type or function expects a type parameter but none was supplied.\
"
            }
            Self::N3082 => {
                "\
\x1b[1;31mN3082\x1b[0m \x1b[1mInvalid enum discriminant type\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe underlying type specified for an enum's discriminant is not valid. Only integer types can be used as discriminant types.\
"
            }
            Self::N3083 => {
                "\
\x1b[1;31mN3083\x1b[0m \x1b[1mDuplicate enum discriminant value\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nTwo or more enum variants have the same discriminant value. Discriminant values must be unique within an enum.\
"
            }
            Self::N3084 => {
                "\
\x1b[1;31mN3084\x1b[0m \x1b[1mEnum discriminant overflow\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn enum discriminant value exceeds the range of the underlying integer type.\
"
            }
            Self::N3085 => {
                "\
\x1b[1;31mN3085\x1b[0m \x1b[1mNon-exhaustive enum match\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA match on an enum does not cover all variants. Add arms for missing variants or a catch-all pattern.\
"
            }
            Self::N3086 => {
                "\
\x1b[1;33mN3086\x1b[0m \x1b[1mUnreachable match arm\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA match arm can never execute because a previous arm already matches all values it would match.\
"
            }
            Self::N3087 => {
                "\
\x1b[1;33mN3087\x1b[0m \x1b[1mOverlapping match patterns\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nTwo or more patterns in a match can match the same value. Reorder or refine the patterns.\
"
            }
            Self::N3088 => {
                "\
\x1b[1;31mN3088\x1b[0m \x1b[1mInvalid ref pattern\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA `ref` pattern modifier is used in an invalid position or on a type that does not support it.\
"
            }
            Self::N3089 => {
                "\
\x1b[1;31mN3089\x1b[0m \x1b[1mInvalid mut pattern\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA `mut` pattern modifier is used in an invalid position or on a type that is not mutable.\
"
            }
            Self::N3090 => {
                "\
\x1b[1;31mN3090\x1b[0m \x1b[1mPattern requires unit type\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA pattern that expects a unit type (empty tuple) is used with a non-unit value.\
"
            }
            Self::N3091 => {
                "\
\x1b[1;31mN3091\x1b[0m \x1b[1mClosure without closure context\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA closure is defined in a context that does not support closures. Closures require the ability to capture variables from the enclosing scope.\
"
            }
            Self::N3092 => {
                "\
\x1b[1;31mN3092\x1b[0m \x1b[1mClosure captures disjoint variables\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA closure captures variables that have disjoint lifetimes, making it impossible to infer a single combined lifetime.\
"
            }
            Self::N3093 => {
                "\
\x1b[1;31mN3093\x1b[0m \x1b[1mNon-copy type in closure by copy\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA closure attempts to capture a non-Copy type by value (copy) in a `Fn` closure context. The type must implement Copy.\
"
            }
            Self::N3094 => {
                "\
\x1b[1;31mN3094\x1b[0m \x1b[1mMismatched async closure\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn async closure is used in a context that expects a sync closure, or vice versa.\
"
            }
            Self::N3095 => {
                "\
\x1b[1;31mN3095\x1b[0m \x1b[1mGenerator resume type mismatch\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA generator's resume argument type does not match what the generator expects.\
"
            }
            Self::N3096 => {
                "\
\x1b[1;31mN3096\x1b[0m \x1b[1mGenerator yield type mismatch\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA generator's yielded value type does not match the expected yield type.\
"
            }
            Self::N3097 => {
                "\
\x1b[1;31mN3097\x1b[0m \x1b[1mGenerator return type mismatch\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mType System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA generator's return type does not match the declared or inferred return type.\
"
            }
            Self::N4001 => {
                "\
\x1b[1;31mN4001\x1b[0m \x1b[1mModule not found\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mModule System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA module referenced in a use/import declaration does not exist in the module search paths. Check the module name and ensure it is properly installed.\
"
            }
            Self::N4002 => {
                "\
\x1b[1;31mN4002\x1b[0m \x1b[1mFile not found\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mModule System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA file referenced in a module path does not exist at the expected location.\
"
            }
            Self::N4003 => {
                "\
\x1b[1;31mN4003\x1b[0m \x1b[1mCircular module dependency\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mModule System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nModules form a circular dependency chain. Nimble requires acyclic module dependencies.\
"
            }
            Self::N4004 => {
                "\
\x1b[1;31mN4004\x1b[0m \x1b[1mSymbol not exported\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mModule System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA symbol imported from a module is not publicly exported by that module. Check the module's public API.\
"
            }
            Self::N4005 => {
                "\
\x1b[1;31mN4005\x1b[0m \x1b[1mImport cycle detected\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mModule System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn import graph cycle was detected. Modules must form a directed acyclic graph.\
"
            }
            Self::N4006 => {
                "\
\x1b[1;31mN4006\x1b[0m \x1b[1mAmbiguous import\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mModule System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn imported symbol resolves to multiple definitions across different imported modules. Use a qualified path to disambiguate.\
"
            }
            Self::N4007 => {
                "\
\x1b[1;31mN4007\x1b[0m \x1b[1mShadowed import\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mModule System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn imported name conflicts with another import. Use an alias or rename one of the imports.\
"
            }
            Self::N4008 => {
                "\
\x1b[1;33mN4008\x1b[0m \x1b[1mWildcard import naming conflict\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mModule System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA wildcard import introduces names that conflict with existing definitions or other imports.\
"
            }
            Self::N4009 => {
                "\
\x1b[1;31mN4009\x1b[0m \x1b[1mRelative import beyond root\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mModule System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA relative import (using `super` or `../`) goes beyond the package root. Relative imports cannot escape the package boundary.\
"
            }
            Self::N4010 => {
                "\
\x1b[1;31mN4010\x1b[0m \x1b[1mInvalid module name\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mModule System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA module name contains invalid characters or does not follow naming conventions. Module names must be valid identifiers.\
"
            }
            Self::N4011 => {
                "\
\x1b[1;31mN4011\x1b[0m \x1b[1mModule not in search path\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mModule System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA module file could not be found in any of the configured import search paths.\
"
            }
            Self::N4012 => {
                "\
\x1b[1;31mN4012\x1b[0m \x1b[1mModule parse error\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mModule System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA module file could not be parsed due to syntax errors. The module must contain valid Nimble source code.\
"
            }
            Self::N4013 => {
                "\
\x1b[1;31mN4013\x1b[0m \x1b[1mModule type error\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mModule System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA module file contains type errors. The module must be type-correct before it can be imported.\
"
            }
            Self::N4014 => {
                "\
\x1b[1;31mN4014\x1b[0m \x1b[1mDependency not found\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mModule System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA package dependency specified in the manifest cannot be resolved. Check that the dependency exists and is accessible.\
"
            }
            Self::N4015 => {
                "\
\x1b[1;31mN4015\x1b[0m \x1b[1mDependency cycle\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mModule System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nTwo or more packages depend on each other, creating a cycle. All dependency graphs must be acyclic.\
"
            }
            Self::N4016 => {
                "\
\x1b[1;31mN4016\x1b[0m \x1b[1mDependency version conflict\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mModule System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nMultiple packages require different incompatible versions of the same dependency.\
"
            }
            Self::N4017 => {
                "\
\x1b[1;31mN4017\x1b[0m \x1b[1mBroken package structure\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mModule System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe package directory structure does not follow the expected convention.\
"
            }
            Self::N4018 => {
                "\
\x1b[1;31mN4018\x1b[0m \x1b[1mMissing manifest\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mModule System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe expected manifest file (e.g., nimble.toml) is missing from the package root.\
"
            }
            Self::N4019 => {
                "\
\x1b[1;31mN4019\x1b[0m \x1b[1mInvalid manifest format\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mModule System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe manifest file has an invalid format or structure and cannot be parsed.\
"
            }
            Self::N4020 => {
                "\
\x1b[1;31mN4020\x1b[0m \x1b[1mManifest syntax error\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mModule System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe manifest file contains a syntax error. Check the manifest syntax.\
"
            }
            Self::N4021 => {
                "\
\x1b[1;31mN4021\x1b[0m \x1b[1mManifest missing required field\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mModule System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe manifest file is missing a required field. Add the required field.\
"
            }
            Self::N4022 => {
                "\
\x1b[1;31mN4022\x1b[0m \x1b[1mManifest duplicate entry\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mModule System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe manifest file contains duplicate entries for the same field or dependency.\
"
            }
            Self::N4023 => {
                "\
\x1b[1;31mN4023\x1b[0m \x1b[1mModule compiled with different version\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mModule System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA pre-compiled module was compiled with a different version of the compiler and is incompatible.\
"
            }
            Self::N4024 => {
                "\
\x1b[1;31mN4024\x1b[0m \x1b[1mModule compiled with incompatible flags\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mModule System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA pre-compiled module was compiled with different compiler flags that affect ABI compatibility.\
"
            }
            Self::N4025 => {
                "\
\x1b[1;31mN4025\x1b[0m \x1b[1mModule interface mismatch\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mModule System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe public interface of a compiled module does not match what is expected from its source.\
"
            }
            Self::N4026 => {
                "\
\x1b[1;31mN4026\x1b[0m \x1b[1mRecursive module loading\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mModule System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA module directly or indirectly triggers loading itself, creating infinite recursion.\
"
            }
            Self::N4027 => {
                "\
\x1b[1;31mN4027\x1b[0m \x1b[1mModule path too deep\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mModule System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA module path exceeds the maximum allowed nesting depth. Flatten your module hierarchy.\
"
            }
            Self::N5001 => {
                "\
\x1b[1;33mN5001\x1b[0m \x1b[1mUnused variable\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA variable is declared but never used. Consider removing it or prefixing the name with an underscore to suppress this warning.\
"
            }
            Self::N5002 => {
                "\
\x1b[1;33mN5002\x1b[0m \x1b[1mUnused import\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn import is not used anywhere in the file. Remove the unused import to keep the code clean.\
"
            }
            Self::N5003 => {
                "\
\x1b[1;33mN5003\x1b[0m \x1b[1mUnused assignment\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA value is assigned to a variable but the variable is never read after the assignment.\
"
            }
            Self::N5004 => {
                "\
\x1b[1;33mN5004\x1b[0m \x1b[1mUnused function\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA function is defined but never called anywhere in the program.\
"
            }
            Self::N5005 => {
                "\
\x1b[1;33mN5005\x1b[0m \x1b[1mUnused struct\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA struct type is defined but never used.\
"
            }
            Self::N5006 => {
                "\
\x1b[1;33mN5006\x1b[0m \x1b[1mUnused type\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA type alias or enum is defined but never used.\
"
            }
            Self::N5007 => {
                "\
\x1b[1;33mN5007\x1b[0m \x1b[1mDead code\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nCode exists that will never execute due to control flow guarantees. Remove it.\
"
            }
            Self::N5008 => {
                "\
\x1b[1;33mN5008\x1b[0m \x1b[1mUnreachable code\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nCode after a return, break, continue, or infinite loop will never execute.\
"
            }
            Self::N5009 => {
                "\
\x1b[1;33mN5009\x1b[0m \x1b[1mEmpty loop body\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA loop has an empty body. Either add code to the body or remove the loop.\
"
            }
            Self::N5010 => {
                "\
\x1b[1;33mN5010\x1b[0m \x1b[1mSuspicious assignment in conditional\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn assignment expression is used in a conditional context. It may be a typo for equality comparison (`==`).\
"
            }
            Self::N5011 => {
                "\
\x1b[1;33mN5011\x1b[0m \x1b[1mLossy implicit conversion\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn implicit type conversion that may lose precision or information. Use an explicit cast to make the conversion clear.\
"
            }
            Self::N5012 => {
                "\
\x1b[1;33mN5012\x1b[0m \x1b[1mDeprecated item\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe code uses a deprecated function, type, or construct. Check the documentation for the recommended replacement.\
"
            }
            Self::N5013 => {
                "\
\x1b[1;33mN5013\x1b[0m \x1b[1mMissing documentation\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA public API item is missing documentation. Add a doc comment (`///`) to describe it.\
"
            }
            Self::N5014 => {
                "\
\x1b[1;33mN5014\x1b[0m \x1b[1mNon-standard naming\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn identifier does not follow Nimble naming conventions. Use snake_case for variables and functions, PascalCase for types.\
"
            }
            Self::N5015 => {
                "\
\x1b[1;33mN5015\x1b[0m \x1b[1mName shadows outer\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA local name shadows a name from an outer scope, which can cause confusion.\
"
            }
            Self::N5016 => {
                "\
\x1b[1;33mN5016\x1b[0m \x1b[1mUnnecessary closure\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA closure that simply wraps another function call can be replaced with a direct reference to the function.\
"
            }
            Self::N5017 => {
                "\
\x1b[1;33mN5017\x1b[0m \x1b[1mRedundant pattern\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA pattern in a match or let binding is unnecessarily complex. Simplify it.\
"
            }
            Self::N5018 => {
                "\
\x1b[1;33mN5018\x1b[0m \x1b[1mMissing else branch\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn if expression without an else branch may not return a value. Add an else branch if needed.\
"
            }
            Self::N5019 => {
                "\
\x1b[1;33mN5019\x1b[0m \x1b[1mDeep nesting\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nCode is nested more than the recommended depth. Refactor to reduce nesting.\
"
            }
            Self::N5020 => {
                "\
\x1b[1;33mN5020\x1b[0m \x1b[1mComplex expression\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn expression is overly complex. Break it into simpler sub-expressions with intermediate variables.\
"
            }
            Self::N5021 => {
                "\
\x1b[1;33mN5021\x1b[0m \x1b[1mHigh cognitive complexity\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe cognitive complexity of a function exceeds the recommended limit. Consider breaking the function into smaller pieces.\
"
            }
            Self::N5022 => {
                "\
\x1b[1;33mN5022\x1b[0m \x1b[1mHigh cyclomatic complexity\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe cyclomatic complexity of a function is too high. Refactor to reduce the number of independent paths.\
"
            }
            Self::N5023 => {
                "\
\x1b[1;33mN5023\x1b[0m \x1b[1mToo many parameters\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA function has more parameters than the recommended maximum. Consider using a struct to group related parameters.\
"
            }
            Self::N5024 => {
                "\
\x1b[1;33mN5024\x1b[0m \x1b[1mToo many return types\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA type has more return type variants than recommended. Simplify the type hierarchy.\
"
            }
            Self::N5025 => {
                "\
\x1b[1;33mN5025\x1b[0m \x1b[1mFunction too long\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA function body exceeds the recommended maximum number of lines. Extract helper functions.\
"
            }
            Self::N5026 => {
                "\
\x1b[1;33mN5026\x1b[0m \x1b[1mFile too long\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe source file exceeds the recommended maximum line count. Split into multiple files.\
"
            }
            Self::N5027 => {
                "\
\x1b[1;33mN5027\x1b[0m \x1b[1mLine too long\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA source line exceeds the maximum allowed column width. Wrap the line.\
"
            }
            Self::N5028 => {
                "\
\x1b[1;33mN5028\x1b[0m \x1b[1mInconsistent naming style\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nIdentifier naming is inconsistent within the codebase. Follow the established naming convention.\
"
            }
            Self::N5029 => {
                "\
\x1b[1;33mN5029\x1b[0m \x1b[1mNon-canonical ordering\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nItems are not ordered according to the project's convention (e.g., imports before definitions, public before private).\
"
            }
            Self::N5030 => {
                "\
\x1b[1;33mN5030\x1b[0m \x1b[1mUnsafe block used\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn unsafe block is used. Ensure that all invariants are manually verified.\
"
            }
            Self::N5031 => {
                "\
\x1b[1;33mN5031\x1b[0m \x1b[1mUnsafe function\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA function is declared as unsafe. Only use unsafe when absolutely necessary and document safety requirements.\
"
            }
            Self::N5032 => {
                "\
\x1b[1;33mN5032\x1b[0m \x1b[1mUnnecessary unsafe\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn unsafe block or function is unnecessary because it contains no actual unsafe operations.\
"
            }
            Self::N5033 => {
                "\
\x1b[1;33mN5033\x1b[0m \x1b[1mComparing boolean literal\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA boolean value is compared to `true` or `false` directly. Use the value directly or use the `!` operator for negation.\
"
            }
            Self::N5034 => {
                "\
\x1b[1;33mN5034\x1b[0m \x1b[1mAssigning boolean in conditional\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn assignment using `=` appears in a condition. This is likely a typo for `==`.\
"
            }
            Self::N5035 => {
                "\
\x1b[1;33mN5035\x1b[0m \x1b[1mNegating boolean literal\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA boolean literal is negated with `!`. Use the opposite literal directly.\
"
            }
            Self::N5036 => {
                "\
\x1b[1;33mN5036\x1b[0m \x1b[1mNested conditional\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA conditional expression is nested inside another conditional in a way that could be simplified.\
"
            }
            Self::N5037 => {
                "\
\x1b[1;33mN5037\x1b[0m \x1b[1mConstant condition\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA condition in an if or while is always true or always false, making the branch redundant.\
"
            }
            Self::N5038 => {
                "\
\x1b[1;33mN5038\x1b[0m \x1b[1mRedundant cast\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA type cast is unnecessary because the source and target types are the same or the conversion is implicit.\
"
            }
            Self::N5039 => {
                "\
\x1b[1;33mN5039\x1b[0m \x1b[1mSuspicious comparison\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA comparison is likely always true or always false (e.g., x == x).\
"
            }
            Self::N5040 => {
                "\
\x1b[1;33mN5040\x1b[0m \x1b[1mInfinite loop detected\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA loop condition is always true and there is no break statement, making the loop potentially infinite.\
"
            }
            Self::N5041 => {
                "\
\x1b[1;33mN5041\x1b[0m \x1b[1mMissing break in loop\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA loop with a constant true condition has no break statement. Add a condition check and break.\
"
            }
            Self::N5042 => {
                "\
\x1b[1;33mN5042\x1b[0m \x1b[1mUninitialized variable\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA variable is used before being assigned a value. Initialize it before use.\
"
            }
            Self::N5043 => {
                "\
\x1b[1;33mN5043\x1b[0m \x1b[1mPossibly uninitialized variable\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA variable may be used without being initialized in all code paths.\
"
            }
            Self::N5044 => {
                "\
\x1b[1;33mN5044\x1b[0m \x1b[1mFallthrough in match\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA match arm falls through to the next arm. Use an explicit `continue` or handle all cases.\
"
            }
            Self::N5045 => {
                "\
\x1b[1;33mN5045\x1b[0m \x1b[1mMissing case in match\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA match on an enum or union type does not cover all variants. Add the missing cases.\
"
            }
            Self::N5046 => {
                "\
\x1b[1;33mN5046\x1b[0m \x1b[1mRedundant default case\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA match has a default case that can never be reached because all variants are already covered.\
"
            }
            Self::N5047 => {
                "\
\x1b[1;33mN5047\x1b[0m \x1b[1mUnnecessary else-if\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn else-if chain can be simplified. Use a match expression instead.\
"
            }
            Self::N5048 => {
                "\
\x1b[1;33mN5048\x1b[0m \x1b[1mRedundant else branch\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe else branch of an if-else is empty or redundant. Remove it.\
"
            }
            Self::N5049 => {
                "\
\x1b[1;33mN5049\x1b[0m \x1b[1mEmpty else branch\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn else branch is empty. Either add code or remove the else.\
"
            }
            Self::N5050 => {
                "\
\x1b[1;33mN5050\x1b[0m \x1b[1mUnnecessary parentheses\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nParentheses around an expression are not needed and reduce readability. Remove them.\
"
            }
            Self::N5051 => {
                "\
\x1b[1;33mN5051\x1b[0m \x1b[1mUnnecessary return\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA return statement at the end of a function is redundant. Remove the return keyword.\
"
            }
            Self::N5052 => {
                "\
\x1b[1;33mN5052\x1b[0m \x1b[1mUnnecessary semicolon\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA semicolon appears in a position where it is not needed. Remove it.\
"
            }
            Self::N5053 => {
                "\
\x1b[1;33mN5053\x1b[0m \x1b[1mEmpty statement\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn empty statement (bare semicolon or empty line) can be removed.\
"
            }
            Self::N5054 => {
                "\
\x1b[1;33mN5054\x1b[0m \x1b[1mStatement with no effect\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn expression statement has no side effects and its result is discarded. It can be removed.\
"
            }
            Self::N5055 => {
                "\
\x1b[1;33mN5055\x1b[0m \x1b[1mVariable assigned but not used\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA variable is assigned a value but is never read afterward. Consider removing the assignment.\
"
            }
            Self::N5056 => {
                "\
\x1b[1;33mN5056\x1b[0m \x1b[1mFunction argument reassigned\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA function parameter is reassigned inside the function body. Use a local variable instead.\
"
            }
            Self::N5057 => {
                "\
\x1b[1;33mN5057\x1b[0m \x1b[1mMutable variable could be immutable\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA variable declared with `var` is never mutated. Use `let` instead.\
"
            }
            Self::N5058 => {
                "\
\x1b[1;33mN5058\x1b[0m \x1b[1mRedundant field name in struct literal\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA struct literal uses the field name even though the value is the same name. The shorthand syntax can be used.\
"
            }
            Self::N5059 => {
                "\
\x1b[1;33mN5059\x1b[0m \x1b[1mUnnecessary qualification\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA module path qualification is unnecessary because the symbol is already in scope.\
"
            }
            Self::N5060 => {
                "\
\x1b[1;33mN5060\x1b[0m \x1b[1mModule naming convention violation\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA module name does not follow the project's naming conventions for modules.\
"
            }
            Self::N5061 => {
                "\
\x1b[1;33mN5061\x1b[0m \x1b[1mNon-idiomatic code pattern\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mLint\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe code uses a pattern that is not idiomatic for Nimble. Consider using a more standard approach.\
"
            }
            Self::N6001 => {
                "\
\x1b[1;35mN6001\x1b[0m \x1b[1mCode generation failure\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe code generator encountered an internal error while generating the output. This is a compiler bug. Report it to the Nimble developers with the source code that triggered it.\
"
            }
            Self::N6002 => {
                "\
\x1b[1;35mN6002\x1b[0m \x1b[1mUnsupported feature for target\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA language feature or construct is not supported on the selected code generation target (e.g., architecture, OS).\
"
            }
            Self::N6003 => {
                "\
\x1b[1;35mN6003\x1b[0m \x1b[1mLinker error\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe linker returned an error while linking the compiled output. Check for missing symbols or library paths.\
"
            }
            Self::N6004 => {
                "\
\x1b[1;35mN6004\x1b[0m \x1b[1mAssembly error\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn error occurred during assembly generation. This typically indicates a compiler bug.\
"
            }
            Self::N6005 => {
                "\
\x1b[1;35mN6005\x1b[0m \x1b[1mTarget not supported\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe specified compilation target is not supported by the code generator.\
"
            }
            Self::N6006 => {
                "\
\x1b[1;31mN6006\x1b[0m \x1b[1mInvalid optimization level\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe specified optimization level is not recognized. Valid levels are 0-3, s, or z.\
"
            }
            Self::N6007 => {
                "\
\x1b[1;31mN6007\x1b[0m \x1b[1mInvalid debug info level\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe specified debug information level is invalid.\
"
            }
            Self::N6008 => {
                "\
\x1b[1;35mN6008\x1b[0m \x1b[1mInline assembly error\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn inline assembly block contains invalid syntax or constraints. Check the assembly template.\
"
            }
            Self::N6009 => {
                "\
\x1b[1;35mN6009\x1b[0m \x1b[1mCompiler intrinsic error\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA compiler intrinsic function was used incorrectly or has an invalid signature.\
"
            }
            Self::N6010 => {
                "\
\x1b[1;35mN6010\x1b[0m \x1b[1mStack overflow during codegen\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe code generator's internal stack overflowed while processing a deeply nested construct. Try simplifying the code.\
"
            }
            Self::N6011 => {
                "\
\x1b[1;35mN6011\x1b[0m \x1b[1mGlobal offset overflow\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe global offset table for the generated code exceeds the maximum allowed size. Reduce the number of global variables or functions.\
"
            }
            Self::N6012 => {
                "\
\x1b[1;35mN6012\x1b[0m \x1b[1mJump table overflow\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA generated jump table (for match/large switches) exceeds the maximum size. Use if-else chains instead.\
"
            }
            Self::N6013 => {
                "\
\x1b[1;35mN6013\x1b[0m \x1b[1mToo many static variables\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe number of static variables in the compilation unit exceeds the target limit. Reduce static variable usage.\
"
            }
            Self::N6014 => {
                "\
\x1b[1;35mN6014\x1b[0m \x1b[1mToo many functions\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe number of functions in the compilation unit exceeds the target module limit. Split the code into multiple files.\
"
            }
            Self::N6015 => {
                "\
\x1b[1;35mN6015\x1b[0m \x1b[1mFunction too large\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA single function exceeds the maximum size that the code generator can handle. Split the function into smaller functions.\
"
            }
            Self::N6016 => {
                "\
\x1b[1;31mN6016\x1b[0m \x1b[1mExternal symbol not found\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA symbol referenced as external (e.g., from a different compilation unit or library) was not found during linking.\
"
            }
            Self::N6017 => {
                "\
\x1b[1;31mN6017\x1b[0m \x1b[1mDuplicate symbol export\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nTwo compilation units export the same symbol. Rename one of the symbols.\
"
            }
            Self::N6018 => {
                "\
\x1b[1;31mN6018\x1b[0m \x1b[1mUndefined symbol\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA symbol referenced in the code has no definition anywhere in the compilation unit or its dependencies.\
"
            }
            Self::N6019 => {
                "\
\x1b[1;35mN6019\x1b[0m \x1b[1mRelocation overflow\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA relocation (address fixup) in the generated object code exceeds the maximum range for the target architecture.\
"
            }
            Self::N6020 => {
                "\
\x1b[1;35mN6020\x1b[0m \x1b[1mTLS not supported\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThread-local storage is not available on the target platform. Remove the `thread_local` annotation.\
"
            }
            Self::N6021 => {
                "\
\x1b[1;31mN6021\x1b[0m \x1b[1mABI mismatch\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe calling convention or ABI of an external function does not match the expected ABI. Ensure consistent ABI declarations.\
"
            }
            Self::N6022 => {
                "\
\x1b[1;31mN6022\x1b[0m \x1b[1mCPU feature not available\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA required CPU feature (e.g., AVX2, SSE4.1) is not available on the target platform.\
"
            }
            Self::N6023 => {
                "\
\x1b[1;31mN6023\x1b[0m \x1b[1mOS feature not available\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA required operating system feature is not available on the target platform.\
"
            }
            Self::N6024 => {
                "\
\x1b[1;31mN6024\x1b[0m \x1b[1mInline assembly constraint violation\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn inline assembly constraint is invalid or incompatible with the operands.\
"
            }
            Self::N6025 => {
                "\
\x1b[1;35mN6025\x1b[0m \x1b[1mIntrinsic signature mismatch\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA compiler intrinsic is called with arguments that do not match its expected signature.\
"
            }
            Self::N6026 => {
                "\
\x1b[1;31mN6026\x1b[0m \x1b[1mVector type not supported\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe target does not support the required SIMD vector type. Use a scalar type or check target features.\
"
            }
            Self::N6027 => {
                "\
\x1b[1;31mN6027\x1b[0m \x1b[1mAtomic not supported\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAtomic operations are not supported on the target platform for the specified data type.\
"
            }
            Self::N6028 => {
                "\
\x1b[1;31mN6028\x1b[0m \x1b[1mSIMD not supported\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nSIMD operations are not supported on the target platform. Disable SIMD features or use a different target.\
"
            }
            Self::N6029 => {
                "\
\x1b[1;35mN6029\x1b[0m \x1b[1mCodegen buffer overflow\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn internal code generator buffer overflowed. This is a compiler bug.\
"
            }
            Self::N6030 => {
                "\
\x1b[1;31mN6030\x1b[0m \x1b[1mUnsupported calling convention\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe specified calling convention is not supported on the target platform.\
"
            }
            Self::N6031 => {
                "\
\x1b[1;35mN6031\x1b[0m \x1b[1mToo many locals\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA function has too many local variables for the code generator to handle. Split the function.\
"
            }
            Self::N6032 => {
                "\
\x1b[1;31mN6032\x1b[0m \x1b[1mSection attribute conflict\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nTwo or more items in the same section have conflicting attributes.\
"
            }
            Self::N6033 => {
                "\
\x1b[1;31mN6033\x1b[0m \x1b[1mLink once group conflict\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nTwo or more items in the same link_once group have conflicting definitions.\
"
            }
            Self::N6034 => {
                "\
\x1b[1;31mN6034\x1b[0m \x1b[1mVisibility attribute conflict\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mCodegen\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nTwo or more symbols with the same name but different visibility are defined.\
"
            }
            Self::N7001 => {
                "\
\x1b[1;35mN7001\x1b[0m \x1b[1mProgram panicked\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe program encountered an unexpected condition and panicked. This is a runtime error that terminates the process. Check the panic message for details about what went wrong.\
"
            }
            Self::N7002 => {
                "\
\x1b[1;35mN7002\x1b[0m \x1b[1mIndex out of bounds\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn array, list, or tuple index was outside the valid range. Ensure the index is less than the container's length and non-negative.\
"
            }
            Self::N7003 => {
                "\
\x1b[1;35mN7003\x1b[0m \x1b[1mStack overflow\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe program's call stack exceeded the maximum allowed size. This usually indicates infinite recursion or excessive stack allocation.\
"
            }
            Self::N7004 => {
                "\
\x1b[1;35mN7004\x1b[0m \x1b[1mArithmetic overflow\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn arithmetic operation overflowed the range of the integer type. Use checked arithmetic or larger types.\
"
            }
            Self::N7005 => {
                "\
\x1b[1;35mN7005\x1b[0m \x1b[1mDivision by zero\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA division or modulo operation had a zero divisor at runtime. Ensure the divisor is non-zero before the operation.\
"
            }
            Self::N7006 => {
                "\
\x1b[1;35mN7006\x1b[0m \x1b[1mNull pointer dereference\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe program tried to dereference a null or invalid pointer. This is a serious bug.\
"
            }
            Self::N7007 => {
                "\
\x1b[1;35mN7007\x1b[0m \x1b[1mUnwrap of None value\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA `None` value was unwrapped, causing a panic. Use pattern matching or safe unwrapping methods to handle the None case.\
"
            }
            Self::N7008 => {
                "\
\x1b[1;35mN7008\x1b[0m \x1b[1mUnwrap of error result\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn `Err` result was unwrapped, causing a panic. Handle errors with pattern matching or propagate them properly.\
"
            }
            Self::N7009 => {
                "\
\x1b[1;35mN7009\x1b[0m \x1b[1mOut of memory\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe program could not allocate the required memory. This is typically a system resource exhaustion issue.\
"
            }
            Self::N7010 => {
                "\
\x1b[1;35mN7010\x1b[0m \x1b[1mAssertion failed\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA runtime assertion (`assert`) failed. The condition evaluated to false. Check the assertion condition and the program logic.\
"
            }
            Self::N7011 => {
                "\
\x1b[1;35mN7011\x1b[0m \x1b[1mUnreachable code executed\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nCode marked as unreachable was executed, indicating a logic error in the program.\
"
            }
            Self::N7012 => {
                "\
\x1b[1;34mN7012\x1b[0m \x1b[1mTODO encountered at runtime\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;34mNote\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA TODO stub was executed at runtime. This indicates incomplete implementation.\
"
            }
            Self::N7013 => {
                "\
\x1b[1;35mN7013\x1b[0m \x1b[1mUnimplemented functionality\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe program reached a code path that is not yet implemented.\
"
            }
            Self::N7014 => {
                "\
\x1b[1;35mN7014\x1b[0m \x1b[1mBuffer overflow\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA buffer write operation exceeded the buffer's capacity. This can be a security vulnerability.\
"
            }
            Self::N7015 => {
                "\
\x1b[1;35mN7015\x1b[0m \x1b[1mInvalid UTF-8 sequence\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn operation encountered an invalid UTF-8 byte sequence. Ensure all string data is valid UTF-8.\
"
            }
            Self::N7016 => {
                "\
\x1b[1;35mN7016\x1b[0m \x1b[1mInteger conversion overflow\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn integer type conversion resulted in an overflow. Use checked conversion or ensure the value fits in the target type.\
"
            }
            Self::N7017 => {
                "\
\x1b[1;35mN7017\x1b[0m \x1b[1mFloat conversion overflow\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA floating-point conversion resulted in an overflow or underflow.\
"
            }
            Self::N7018 => {
                "\
\x1b[1;35mN7018\x1b[0m \x1b[1mNegative index\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA negative index was used in a context that only accepts non-negative indices.\
"
            }
            Self::N7019 => {
                "\
\x1b[1;35mN7019\x1b[0m \x1b[1mInvalid enum discriminant\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn enum value has an invalid discriminant value, indicating memory corruption or unsafe code violation.\
"
            }
            Self::N7020 => {
                "\
\x1b[1;35mN7020\x1b[0m \x1b[1mType cast error at runtime\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA runtime type cast failed because the value's actual type does not match the target type.\
"
            }
            Self::N7021 => {
                "\
\x1b[1;35mN7021\x1b[0m \x1b[1mRecursive call overflow\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe recursion depth exceeded the maximum allowed limit. Use iteration instead of recursion or increase the recursion limit.\
"
            }
            Self::N7022 => {
                "\
\x1b[1;35mN7022\x1b[0m \x1b[1mInvalid allocator state\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe memory allocator detected an inconsistent internal state. This is a serious bug.\
"
            }
            Self::N7023 => {
                "\
\x1b[1;35mN7023\x1b[0m \x1b[1mDouble free detected\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA memory deallocation was attempted twice on the same allocation. This indicates a memory management bug.\
"
            }
            Self::N7024 => {
                "\
\x1b[1;35mN7024\x1b[0m \x1b[1mUse after free\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nMemory was accessed after it was freed. This indicates a memory management bug.\
"
            }
            Self::N7025 => {
                "\
\x1b[1;35mN7025\x1b[0m \x1b[1mMutex poison\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA mutex is in a poisoned state because a previous lock holder panicked while holding the lock.\
"
            }
            Self::N7026 => {
                "\
\x1b[1;35mN7026\x1b[0m \x1b[1mChannel closed\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA channel operation failed because the channel was closed. Check if the channel is still open before operating.\
"
            }
            Self::N7027 => {
                "\
\x1b[1;35mN7027\x1b[0m \x1b[1mTimeout\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn operation timed out before completing. Increase the timeout or optimize the operation.\
"
            }
            Self::N7028 => {
                "\
\x1b[1;35mN7028\x1b[0m \x1b[1mIO error\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn input/output operation failed. Check the file system, permissions, and device availability.\
"
            }
            Self::N7029 => {
                "\
\x1b[1;35mN7029\x1b[0m \x1b[1mNetwork error\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mRuntime\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA network operation failed. Check network connectivity, address correctness, and firewall settings.\
"
            }
            Self::N8001 => {
                "\
\x1b[1;31mN8001\x1b[0m \x1b[1mConfiguration parse error\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA configuration file could not be parsed. Check the file format and syntax.\
"
            }
            Self::N8002 => {
                "\
\x1b[1;31mN8002\x1b[0m \x1b[1mConfiguration missing field\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA configuration file is missing a required field. Add the missing field.\
"
            }
            Self::N8003 => {
                "\
\x1b[1;31mN8003\x1b[0m \x1b[1mConfiguration invalid value\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA configuration field has an invalid value. Check the expected type and valid range.\
"
            }
            Self::N8004 => {
                "\
\x1b[1;31mN8004\x1b[0m \x1b[1mBuild target not found\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe specified build target does not exist in the project configuration.\
"
            }
            Self::N8005 => {
                "\
\x1b[1;31mN8005\x1b[0m \x1b[1mBuild script error\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA build script (pre-build, post-build) exited with a non-zero status.\
"
            }
            Self::N8006 => {
                "\
\x1b[1;31mN8006\x1b[0m \x1b[1mMissing build dependency\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA build-time dependency is not available. Install the required build tools.\
"
            }
            Self::N8007 => {
                "\
\x1b[1;31mN8007\x1b[0m \x1b[1mInvalid build profile\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe specified build profile (e.g., debug, release) is not valid.\
"
            }
            Self::N8008 => {
                "\
\x1b[1;31mN8008\x1b[0m \x1b[1mInvalid manifest\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe project manifest file has an invalid format or content.\
"
            }
            Self::N8009 => {
                "\
\x1b[1;31mN8009\x1b[0m \x1b[1mManifest missing package name\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe package manifest is missing the package name field. Add a name field.\
"
            }
            Self::N8010 => {
                "\
\x1b[1;31mN8010\x1b[0m \x1b[1mManifest missing version\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe package manifest is missing the version field. Add a version string.\
"
            }
            Self::N8011 => {
                "\
\x1b[1;31mN8011\x1b[0m \x1b[1mManifest duplicate entry\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe manifest contains duplicate entries for the same key. Remove the duplicates.\
"
            }
            Self::N8012 => {
                "\
\x1b[1;31mN8012\x1b[0m \x1b[1mManifest invalid dependency\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA dependency specification in the manifest is invalid. Check the dependency format.\
"
            }
            Self::N8013 => {
                "\
\x1b[1;31mN8013\x1b[0m \x1b[1mWorkspace member not found\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA workspace member specified in the manifest does not exist.\
"
            }
            Self::N8014 => {
                "\
\x1b[1;31mN8014\x1b[0m \x1b[1mWorkspace duplicate member\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA workspace member appears more than once in the members list.\
"
            }
            Self::N8015 => {
                "\
\x1b[1;31mN8015\x1b[0m \x1b[1mInvalid toolchain\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe specified toolchain identifier is invalid. Use a recognized toolchain name.\
"
            }
            Self::N8016 => {
                "\
\x1b[1;31mN8016\x1b[0m \x1b[1mToolchain not installed\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe required toolchain is not installed. Install it using the appropriate toolchain manager.\
"
            }
            Self::N8017 => {
                "\
\x1b[1;31mN8017\x1b[0m \x1b[1mInvalid target triple\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe specified target triple (e.g., x86_64-pc-windows-msvc) is not recognized.\
"
            }
            Self::N8018 => {
                "\
\x1b[1;31mN8018\x1b[0m \x1b[1mTest failed\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nOne or more tests failed. Check the test output for details.\
"
            }
            Self::N8019 => {
                "\
\x1b[1;31mN8019\x1b[0m \x1b[1mBenchmark failed\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nOne or more benchmarks failed. Check the benchmark output for details.\
"
            }
            Self::N8020 => {
                "\
\x1b[1;31mN8020\x1b[0m \x1b[1mMissing test configuration\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA test suite is missing its configuration file.\
"
            }
            Self::N8021 => {
                "\
\x1b[1;31mN8021\x1b[0m \x1b[1mInvalid compiler flag\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn invalid compiler flag was specified. Check the available flags.\
"
            }
            Self::N8022 => {
                "\
\x1b[1;31mN8022\x1b[0m \x1b[1mConflicting compiler flags\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nTwo or more compiler flags conflict with each other. Remove the conflicting flags.\
"
            }
            Self::N8023 => {
                "\
\x1b[1;31mN8023\x1b[0m \x1b[1mUnsupported flag for target\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA compiler flag is not supported for the selected target platform.\
"
            }
            Self::N8024 => {
                "\
\x1b[1;31mN8024\x1b[0m \x1b[1mInvalid linker flag\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn invalid linker flag was specified.\
"
            }
            Self::N8025 => {
                "\
\x1b[1;31mN8025\x1b[0m \x1b[1mMissing linker\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe system linker was not found. Install the required linker tool.\
"
            }
            Self::N8026 => {
                "\
\x1b[1;31mN8026\x1b[0m \x1b[1mMissing assembler\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe system assembler was not found. Install the required assembler.\
"
            }
            Self::N8027 => {
                "\
\x1b[1;31mN8027\x1b[0m \x1b[1mOutput path not writable\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe specified output path is not writable. Check permissions.\
"
            }
            Self::N8028 => {
                "\
\x1b[1;31mN8028\x1b[0m \x1b[1mCache directory not accessible\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe compiler cache directory is not accessible. Check permissions.\
"
            }
            Self::N8029 => {
                "\
\x1b[1;31mN8029\x1b[0m \x1b[1mConcurrent build conflict\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAnother build process is using the same output directory. Wait for it to finish or use a different output path.\
"
            }
            Self::N8030 => {
                "\
\x1b[1;31mN8030\x1b[0m \x1b[1mBuild system internal error\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe build system encountered an internal error. Report this bug to the developers.\
"
            }
            Self::N8031 => {
                "\
\x1b[1;31mN8031\x1b[0m \x1b[1mInvalid package name\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe package name does not follow the required naming convention.\
"
            }
            Self::N8032 => {
                "\
\x1b[1;31mN8032\x1b[0m \x1b[1mPackage name invalid characters\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe package name contains invalid characters. Use only alphanumeric characters, hyphens, and underscores.\
"
            }
            Self::N8033 => {
                "\
\x1b[1;31mN8033\x1b[0m \x1b[1mInvalid package version\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe package version string does not follow semantic versioning format.\
"
            }
            Self::N8034 => {
                "\
\x1b[1;33mN8034\x1b[0m \x1b[1mLicense not recognized\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe specified license identifier is not in the recognized license list. Use a standard SPDX identifier.\
"
            }
            Self::N8035 => {
                "\
\x1b[1;33mN8035\x1b[0m \x1b[1mMissing package license\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe package manifest does not specify a license. Add a license field.\
"
            }
            Self::N8036 => {
                "\
\x1b[1;33mN8036\x1b[0m \x1b[1mMissing package description\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe package manifest does not include a description. Add a brief description.\
"
            }
            Self::N8037 => {
                "\
\x1b[1;31mN8037\x1b[0m \x1b[1mInvalid edition\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe specified language edition is not recognized or not supported.\
"
            }
            Self::N8038 => {
                "\
\x1b[1;33mN8038\x1b[0m \x1b[1mFeature flag not recognized\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;33mWarning\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA feature flag specified in the configuration is not recognized. Check available features.\
"
            }
            Self::N8039 => {
                "\
\x1b[1;31mN8039\x1b[0m \x1b[1mFeature flag conflict\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;31mError\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mBuild System\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nTwo or more feature flags conflict and cannot be enabled simultaneously.\
"
            }
            Self::N9001 => {
                "\
\x1b[1;35mN9001\x1b[0m \x1b[1mInternal compiler error\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mInternal\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe compiler encountered an unexpected internal error. This is a compiler bug. Please report it to the Nimble development team with a minimal reproduction of the source code.\
"
            }
            Self::N9002 => {
                "\
\x1b[1;35mN9002\x1b[0m \x1b[1mInternal bug — please report\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mInternal\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe compiler detected a condition that should never occur during correct compilation. This is a bug. Please file a bug report.\
"
            }
            Self::N9003 => {
                "\
\x1b[1;35mN9003\x1b[0m \x1b[1mUnreachable code path in compiler\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mInternal\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe compiler reached a code path that should be unreachable. This indicates a bug in the compiler's logic.\
"
            }
            Self::N9004 => {
                "\
\x1b[1;35mN9004\x1b[0m \x1b[1mUnimplemented compiler feature\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mInternal\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe compiler encountered a construct that is not yet implemented. This feature may be added in a future release.\
"
            }
            Self::N9005 => {
                "\
\x1b[1;35mN9005\x1b[0m \x1b[1mCompiler assertion failure\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mInternal\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn internal compiler assertion failed. This indicates a bug in the compiler that should be reported.\
"
            }
            Self::N9006 => {
                "\
\x1b[1;35mN9006\x1b[0m \x1b[1mCompiler invariant violation\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mInternal\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn internal compiler invariant was violated. This indicates a bug in the compiler that should be reported.\
"
            }
            Self::N9007 => {
                "\
\x1b[1;35mN9007\x1b[0m \x1b[1mType checker invariant failure\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mInternal\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe type checker encountered an inconsistent internal state. This is a compiler bug. Report it with a reproduction case.\
"
            }
            Self::N9008 => {
                "\
\x1b[1;35mN9008\x1b[0m \x1b[1mName resolution invariant failure\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mInternal\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe name resolution pass encountered an inconsistent internal state. This is a compiler bug.\
"
            }
            Self::N9009 => {
                "\
\x1b[1;35mN9009\x1b[0m \x1b[1mCodegen invariant failure\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mInternal\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe code generator encountered an inconsistent internal state. This is a compiler bug.\
"
            }
            Self::N9010 => {
                "\
\x1b[1;35mN9010\x1b[0m \x1b[1mCompiler data structure corruption\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mInternal\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nAn internal compiler data structure has been corrupted. This is a serious compiler bug.\
"
            }
            Self::N9011 => {
                "\
\x1b[1;35mN9011\x1b[0m \x1b[1mMissing compiler pass\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mInternal\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA required compiler analysis or transformation pass is missing from the compilation pipeline.\
"
            }
            Self::N9012 => {
                "\
\x1b[1;35mN9012\x1b[0m \x1b[1mCompiler pass cycle\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mInternal\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nCompiler passes form a dependency cycle. This is a compiler architecture bug.\
"
            }
            Self::N9013 => {
                "\
\x1b[1;35mN9013\x1b[0m \x1b[1mCompiler query cycle\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mInternal\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nCompiler queries formed a dependency cycle during incremental compilation.\
"
            }
            Self::N9014 => {
                "\
\x1b[1;35mN9014\x1b[0m \x1b[1mIncremental cache mismatch\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mInternal\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe incremental compilation cache is inconsistent with the current source code. A clean rebuild may be needed.\
"
            }
            Self::N9015 => {
                "\
\x1b[1;35mN9015\x1b[0m \x1b[1mIncremental fingerprint conflict\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mInternal\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nTwo source files had the same fingerprint, causing a cache conflict. This is a compiler bug.\
"
            }
            Self::N9016 => {
                "\
\x1b[1;35mN9016\x1b[0m \x1b[1mAST validation failure\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mInternal\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe Abstract Syntax Tree (AST) failed validation checks. This indicates a bug in the parser or tree construction.\
"
            }
            Self::N9017 => {
                "\
\x1b[1;35mN9017\x1b[0m \x1b[1mHIR validation failure\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mInternal\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe High-level Intermediate Representation (HIR) failed validation. This indicates a bug in lowering or analysis passes.\
"
            }
            Self::N9018 => {
                "\
\x1b[1;35mN9018\x1b[0m \x1b[1mMIR validation failure\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mInternal\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe Mid-level Intermediate Representation (MIR) failed validation. This indicates a bug in a transformation or optimization pass.\
"
            }
            Self::N9019 => {
                "\
\x1b[1;35mN9019\x1b[0m \x1b[1mLLVM / backend error\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mInternal\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe LLVM backend or alternative backend returned an unexpected error. This may be a compiler compatibility issue.\
"
            }
            Self::N9020 => {
                "\
\x1b[1;35mN9020\x1b[0m \x1b[1mCompiler memory allocation failure\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mInternal\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe compiler could not allocate required memory. Try building with more RAM or reducing parallel compilation jobs.\
"
            }
            Self::N9021 => {
                "\
\x1b[1;35mN9021\x1b[0m \x1b[1mThread panic in compiler worker\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mInternal\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nA compiler worker thread panicked. This is a compiler bug. Try re-running the compilation.\
"
            }
            Self::N9022 => {
                "\
\x1b[1;35mN9022\x1b[0m \x1b[1mCompiler resource limit exceeded\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mInternal\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe compiler exceeded a resource limit (e.g., too many types, too many files). Try splitting the project.\
"
            }
            Self::N9023 => {
                "\
\x1b[1;35mN9023\x1b[0m \x1b[1mCompiler I/O error\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mInternal\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe compiler encountered an I/O error while reading or writing files. Check disk space, permissions, and path validity.\
"
            }
            Self::N9024 => {
                "\
\x1b[1;35mN9024\x1b[0m \x1b[1mCompiler timeout\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mInternal\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe compilation exceeded the maximum allowed time. The code may be too large or the compiler may be stuck in a loop.\
"
            }
            Self::N9025 => {
                "\
\x1b[1;35mN9025\x1b[0m \x1b[1mInvalid incremental compilation state\x1b[0m\n\x1b[1mSeverity\x1b[0m  : \x1b[1;35mBug\x1b[0m\n\x1b[1mCategory\x1b[0m  : \x1b[1;36mInternal\x1b[0m\n\x1b[38;5;244m│\x1b[0m\nThe incremental compilation cache was in an invalid state. Perform a clean build to resolve this.\
"
            }
        }
    }
}
