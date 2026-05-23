# Nimble Syntax Reference

## Lexical Structure

### Comments

Full-line comments begin with `#`. Inline comments are not supported.

```
# this is a comment
let x = 42
```

### Identifiers

Identifiers start with a letter or underscore, followed by zero or more alphanumeric characters or underscores.

```
foo  _bar  baz123  _  my_var
```

### Literals

| Literal | Examples |
|---------|----------|
| Integer | `0`, `42`, `-5` |
| Float | `0.0`, `3.14`, `-2.5` |
| String | `"hello"`, `"line1\nline2"` |
| Boolean | `true`, `false` |

String escape sequences: `\n` (newline), `\t` (tab), `\r` (carriage return), `\0` (null), `\\` (backslash), `\"` (quote), `\'` (single quote).

### Keywords

```
fn  let  var  if  elif  else  struct  interface  pub
return  while  for  in  extern  true  false
```

## Indentation

Nimble uses Python-style significant indentation. Blocks are denoted by a colon `:` followed by a newline and indented statements.

```
fn main() -> Int:
    let x = 42
    return x
```

- Use **spaces only** for indentation. Tabs are illegal.
- Standard convention is 4 spaces per indent level.
- Dedent to the level of an outer block to close the current block.
- Inside parentheses `()`, brackets `[]`, or braces `{}`, indentation tokens are suppressed (implicit continuation).

## Delimiters

| Token | Meaning |
|-------|---------|
| `:` | Block start, type annotation separator |
| `->` | Return type arrow |
| `,` | Separator in parameter/argument lists |
| `.` | Dot (reserved for member access) |
| `(` `)` | Grouping, function call, parameter list |
| `[` `]` | Array indexing (reserved) |
| `{` `}` | Struct literal (reserved) |

## Operators

### Arithmetic

| Operator | Meaning |
|----------|---------|
| `+` | Addition |
| `-` | Subtraction (binary) / Negation (unary) |
| `*` | Multiplication |
| `/` | Division |
| `%` | Modulo / Remainder |

### Comparison

| Operator | Meaning |
|----------|---------|
| `==` | Equal |
| `!=` | Not equal |
| `<` | Less than |
| `>` | Greater than |
| `<=` | Less than or equal |
| `>=` | Greater than or equal |

### Logical

| Operator | Meaning |
|----------|---------|
| `&&` | Logical AND |
| `||` | Logical OR |
| `!` | Logical NOT (unary prefix) |

### Assignment

| Operator | Meaning |
|----------|---------|
| `=` | Assignment |
| `+=` | Add and assign |
| `-=` | Subtract and assign |
| `*=` | Multiply and assign |
| `/=` | Divide and assign |
| `%=` | Modulo and assign |

## Precedence (lowest to highest)

| Level | Operators | Assoc |
|-------|-----------|-------|
| 1 | `=` `+=` `-=` `*=` `/=` `%=` | right |
| 2 | `||` | left |
| 3 | `&&` | left |
| 4 | `==` `!=` | left |
| 5 | `<` `>` `<=` `>=` | left |
| 6 | `+` `-` | left |
| 7 | `*` `/` `%` | left |
| 8 | unary `-` `!` | right (prefix) |
| 9 | `()` call | left |
