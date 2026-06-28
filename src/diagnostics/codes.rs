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

    /// Full markdown explanation for this error code.
    /// Used by `nimble explain <code>`.
    pub fn explanation(&self) -> &'static str {
        match self {
            Self::N0001 => {
                "\
### Error N0001: Illegal Tab Character\n\
\n\
**What happened?** A tab character (`\\t`) was found in the source code. \
Nimble requires spaces for indentation.\n\
\n\
**Why it happened?** Your editor likely inserted a raw tab instead of spaces.\n\
\n\
**How to fix?** Configure your editor to expand tabs to spaces \
(usually 4 spaces per indent level).\n\
"
            }
            _ => {
                "\
### Error Explanation\n\
\n\
**What happened?** See the error title for a summary.\n\
\n\
**How to fix?** Refer to the diagnostic message for specific guidance.\n\
"
            }
        }
    }
}
