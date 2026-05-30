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
fn  let  var  if  elif  else  struct  interface  enum  match  pub
return  while  break  continue  for  in  extern  load  as  true  false
defer  macro  mut  async  await  ref
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
| `.` | Member access, method call |
| `(` `)` | Grouping, function call, parameter list |
| `[` `]` | Generic type argument list |
| `{` `}` | Struct literal fields |

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

### Other

| Operator | Meaning |
|----------|---------|
| `?` | Error propagation (postfix on Result/Option) |
| `&` | Reference operator (prefix) |

## Precedence (lowest to highest)

| Level | Operators | Assoc |
|-------|-----------|-------|
| 1 | `?` | left |
| 2 | `=` `+=` `-=` `*=` `/=` `%=` | right |
| 3 | `||` | left |
| 4 | `&&` | left |
| 5 | `==` `!=` | left |
| 6 | `<` `>` `<=` `>=` | left |
| 7 | `+` `-` | left |
| 8 | `*` `/` `%` | left |
| 9 | unary `-` `!` `&` | right (prefix) |
| 10 | `()` call `.` method | left |

## Expressions

Supported expression forms include:

- Literals: integers, floats, strings, booleans
- Identifiers
- Binary and unary operators
- Function calls
- Assignment expressions
- Method call expressions (`obj.method(args)`)
- Parenthesized grouping
- Member access chains
- Struct literals
- Explicit casts using `as`
- Postfix `?` for error propagation
- Lambda expressions (`fn(params): body`)
- Match expressions
- Macro invocations
- Enum variant construction (`Enum.Variant(args)`)

Not yet implemented as first-class syntax:

- List, map, and tuple literals
- Comprehensions
- Range literals
