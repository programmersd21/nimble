# Lexer Errors

This page documents all error codes in the **Lexer** category.

## Summary

Total codes: 39
- 39 error code(s)

## Error Codes

---

### N0001: Illegal tab character

- **Severity:** Error
- **Code:** `N0001`

Nimble uses spaces for indentation and does not allow tab characters (\t) anywhere in the source code. A tab was encountered while scanning the input. This is a hard error to enforce consistent indentation across all editors and platforms. Configure your editor to insert spaces when you press the Tab key, typically with an indent width of 2 or 4 spaces.

---

### N0002: Unexpected character

- **Severity:** Error
- **Code:** `N0002`

The lexer encountered a character that is not part of the Nimble language grammar. This typically happens when a non-ASCII, control character, or unsymbolic glyph appears outside of a string or comment context. Check for stray characters, typos, or copy-paste artifacts in your source file.

---

### N0003: Unmatched closing delimiter

- **Severity:** Error
- **Code:** `N0003`

A closing bracket, parenthesis, or brace was found without a matching opening counterpart. Common causes include deleting an opening delimiter, having too many closing delimiters due to a typo, or incorrect nesting of brackets. Check the surrounding context for mismatched parentheses, braces, or brackets.

---

### N0004: Invalid float literal

- **Severity:** Error
- **Code:** `N0004`

A floating-point number literal is malformed. Nimble requires floats to have digits on both sides of the decimal point (e.g. 3.14), a single decimal point without surrounding digits is not allowed. Scientific notation must follow the form <digits>e<exponent> or <digits>E<exponent>.

---

### N0005: Integer literal out of range

- **Severity:** Error
- **Code:** `N0005`

An integer literal exceeds the maximum representable value for the target type. Nimble integers are 64-bit signed by default, with a range of -2^63 to 2^63-1. If you need larger values, consider using a Float or a big integer library.

---

### N0006: Unterminated string literal

- **Severity:** Error
- **Code:** `N0006`

A string literal was started with a double-quote (") but the end of file or end of line was reached before the closing double-quote. In Nimble, string literals cannot span multiple lines unless escaped. Add a closing double-quote or escape the newline if you intend a multi-line string.

---

### N0007: Invalid escape sequence

- **Severity:** Error
- **Code:** `N0007`

A backslash followed by an unrecognized character was found inside a string literal. Valid escape sequences in Nimble include \n, \t, \r, \0, \\, \", and \'. Use only these recognized escape sequences.

---

### N0008: Newline inside string literal

- **Severity:** Error
- **Code:** `N0008`

An unescaped newline character was detected inside a string literal. Nimble requires all string content to be on a single line unless the newline is explicitly escaped with a backslash at the end of the line.

---

### N0009: Indentation error

- **Severity:** Error
- **Code:** `N0009`

The indentation level of a line does not match any enclosing block's indent level. Nimble uses Python-style indentation, so every indented block must align consistently. Ensure all lines in the same block have exactly the same indentation.

---

### N0010: Empty character literal

- **Severity:** Error
- **Code:** `N0010`

A character literal ('') was found with zero characters between the single quotes. Character literals must contain exactly one character. If you need an empty value, use a string literal ("") instead.

---

### N0011: Multi-byte character literal

- **Severity:** Error
- **Code:** `N0011`

A character literal contains more than one byte. Nimble char literals (inside single quotes) must contain exactly one ASCII character or one Unicode codepoint. Use a string literal if you need multiple characters.

---

### N0012: Unicode escape in non-unicode context

- **Severity:** Error
- **Code:** `N0012`

A \u or \U unicode escape sequence was used in a context that does not support unicode escapes. Unicode escapes are only valid inside string literals.

---

### N0013: Invalid numeric suffix

- **Severity:** Error
- **Code:** `N0013`

A literal number has an unrecognized suffix. Nimble uses type suffixes like `i32`, `u64`, or `f32` to specify the type of a numeric literal. Only recognized suffixes are allowed.

---

### N0014: Leading zeros in decimal integer

- **Severity:** Error
- **Code:** `N0014`

A decimal integer literal starts with one or more leading zeros. Decimal numbers must not have leading zeros. If you intend an octal literal, use the 0o prefix; otherwise remove the leading zeros.

---

### N0015: Binary literal overflow

- **Severity:** Error
- **Code:** `N0015`

A binary literal (0b prefix) contains a value that exceeds the maximum representable value for the target integer type. The number of bits in the binary representation must fit within the type's width.

---

### N0016: Hex literal overflow

- **Severity:** Error
- **Code:** `N0016`

A hexadecimal literal (0x prefix) contains a value that exceeds the maximum representable value for the target integer type.

---

### N0017: Octal literal overflow

- **Severity:** Error
- **Code:** `N0017`

An octal literal (0o prefix) contains a value that exceeds the maximum representable value for the target integer type.

---

### N0018: Invalid binary literal format

- **Severity:** Error
- **Code:** `N0018`

A binary literal (0b prefix) contains digits other than 0 and 1. Binary literals may only contain the digits 0 and 1, optionally separated by underscores.

---

### N0019: Invalid hex literal format

- **Severity:** Error
- **Code:** `N0019`

A hexadecimal literal (0x prefix) contains invalid characters. Hex digits include 0-9, a-f, A-F, and optionally underscores.

---

### N0020: Invalid octal literal format

- **Severity:** Error
- **Code:** `N0020`

An octal literal (0o prefix) contains digits other than 0-7. Octal digits range from 0 to 7 only.

---

### N0021: Unterminated block comment

- **Severity:** Error
- **Code:** `N0021`

A block comment (/* ... */) was started but never closed before end of file. Add the closing */ delimiter.

---

### N0022: Unrecognized token in string interpolation

- **Severity:** Error
- **Code:** `N0022`

A string interpolation expression contains an invalid token. String interpolation allows only expressions and identifiers.

---

### N0023: Invalid unicode identifier start

- **Severity:** Error
- **Code:** `N0023`

An identifier begins with a unicode character that is not a valid identifier start per Nimble's rules. Identifiers must start with a letter or underscore.

---

### N0024: Non-printable character in source

- **Severity:** Error
- **Code:** `N0024`

A non-printable control character (outside the printable ASCII range) was found in the source code. Remove or replace the character.

---

### N0025: Byte order mark detected

- **Severity:** Error
- **Code:** `N0025`

A UTF-8 BOM (byte order mark, U+FEFF) was found at the start of the file. Save the file without a BOM.

---

### N0026: Null character in source

- **Severity:** Error
- **Code:** `N0026`

A null byte (\0) was found in the source code. Null bytes are not allowed outside of string literals.

---

### N0027: String literal exceeds maximum length

- **Severity:** Error
- **Code:** `N0027`

A string literal contains more than the maximum allowed number of characters. Break the string into smaller parts and concatenate.

---

### N0028: Empty unicode escape sequence

- **Severity:** Error
- **Code:** `N0028`

A unicode escape sequence (\u, \U) has no hex digits. Provide at least one hex digit after the \u or \U marker.

---

### N0029: Malformed unicode escape sequence

- **Severity:** Error
- **Code:** `N0029`

A unicode escape sequence contains invalid hex digits or is incorrectly structured. Use the form \uXXXX or \UXXXXXXXX.

---

### N0030: Unicode codepoint out of range

- **Severity:** Error
- **Code:** `N0030`

A unicode escape sequence specifies a codepoint value outside the valid Unicode range (0x0000-0x10FFFF). Use a value within range.

---

### N0031: Reserved keyword used as identifier

- **Severity:** Error
- **Code:** `N0031`

A reserved keyword is being used as an identifier name. Keywords like fn, let, var, if, else, while, return, etc. cannot be used as variable or function names.

---

### N0032: Non-standard line ending

- **Severity:** Error
- **Code:** `N0032`

A non-standard line ending (CR, CR+LF, or other) was detected. Use standard LF line endings for consistency.

---

### N0033: Mixed tabs and spaces

- **Severity:** Error
- **Code:** `N0033`

Both tabs and spaces are used for indentation in the same file. Pick one style (spaces recommended) and use it consistently.

---

### N0034: Digit separator at wrong position

- **Severity:** Error
- **Code:** `N0034`

An underscore digit separator appears in an invalid position in a numeric literal. Underscores must be placed between digits, not at the start, end, or adjacent to a radix prefix.

---

### N0035: Consecutive digit separators

- **Severity:** Error
- **Code:** `N0035`

Two or more consecutive underscore digit separators appear in a numeric literal. Use at most one underscore between digits.

---

### N0036: Trailing digit separator

- **Severity:** Error
- **Code:** `N0036`

A numeric literal ends with an underscore digit separator. Remove the trailing underscore.

---

### N0037: Leading digit separator

- **Severity:** Error
- **Code:** `N0037`

A numeric literal begins with an underscore digit separator. Remove the leading underscore.

---

### N0038: Invalid digit for radix

- **Severity:** Error
- **Code:** `N0038`

A numeric literal contains a digit that is not valid for its specified radix (e.g. 0b1020 has a 2 in binary). Use only digits valid for the radix.

---

### N0039: Unterminated raw string literal

- **Severity:** Error
- **Code:** `N0039`

A raw string literal (r"...") was started but not closed before end of file or line. Add a closing double-quote.

---

