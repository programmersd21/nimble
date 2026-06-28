# Compilation Pipeline

## Pipeline Stages

### 1. Lexing

`Lexer::new(source)` tokenizes the source string into a flat stream of `Token` values with positional `Span` information. Indentation is tracked via an internal stack, producing `Indent` / `Dedent` tokens for blocks. Supports full UTF-8 Unicode identifiers (XID_Start/XID_Continue). Structured `LexError` recovery via `drain_errors()` - the lexer never hard-panics on malformed input.

```
Source chars -> Lexer -> Token stream + Vec<LexError>
```

### 2. Parsing

`Parser::new(source)` consumes the token stream and builds an AST (`Program`). Expression parsing uses Pratt-style precedence climbing. Features **panic-mode error recovery**: when a statement fails to parse, the error is recorded, the parser skips to the next sync token (`Newline`, `Dedent`, or `Eof`), and parsing continues. All accumulated errors are available via `drain_parse_errors()`.

```
Token stream -> Parser -> AST (Program) + Vec<ParseError>
```

### 3. HIR Lowering

`lower_program(&program)` transforms the AST into a High-level Intermediate Representation (`HirProgram`). The HIR removes semantically transparent wrappers (e.g. `Grouping` expressions) while preserving all `Span` information for diagnostics and codegen. Every AST node type has a corresponding HIR variant.

```
AST -> HIR Lowering -> HirProgram
```

### 4. Name Resolution

`Resolver::resolve(&program)` performs two-pass name resolution:
- **Pass 1 (collect_definitions)**: Walks the AST, assigns a unique `DefId` to every definition (functions, variables, structs, interfaces, parameters, builtins), and tracks lexical scopes with proper nesting for `if`/`while`/`for`/`defer`/function bodies.
- **Pass 2 (resolve_references)**: Rebuilds scope chains and resolves every identifier reference to its `DefId`. Reports `UndefinedVariable` and `DuplicateDefinition` errors.

Returns a `ResolvedProgram` with:
- `resolved: HashMap<usize, DefId>` - maps byte_index to definition
- `definitions: Vec<Def>` - all definitions with name, kind, span, mutability
- `lookup(name)` / `lookup_by_span(span)` / `get_def(id)` - query APIs

```
AST -> Resolver -> ResolvedProgram + Vec<ResolveError>
```

### 5. Type Checking

`TypeChecker::check_program(program, resolved)` performs semantic analysis using the resolved definitions:
- Hindley-Milner type inference with unification
- Generic function monomorphization (fresh type variables per instantiation)
- Enum variant resolution and pattern matching type checking
- Method call desugaring (`obj.method(args)` -> `method(obj, args)`)
- Closure capture analysis
- Macro expansion
- Interface conformance checking
- Ownership tracking scaffold

Returns an `Environment` mapping every name to its resolved `Symbol`.

```
ResolvedProgram -> TypeChecker -> Environment + Subst + Vec<TypeError>
```

### 6. Code Generation

`Codegen::generate(program, env)` emits textual LLVM IR (`.ll` format). Built-in type mapping:

| Nimble | LLVM IR |
|--------|---------|
| `Int`  | `i64` |
| `Float` | `double` |
| `Bool` | `i1` |
| `String` | `ptr` |
| `Void` | `void` |
| `&T` / `&mut T` | `ptr` |
| `struct` | LLVM named `%T` |
| `enum` | `{i64, i64}` tagged union |

The codegen also handles:
- Enum tagged unions with tag check and payload extraction
- Lambda trampolines for captured closures
- Function pointer codegen for non-capturing closures
- `defer` scope-exit emission via a defer stack
- `?` operator tag-check + early-return IR
- Optional LLVM debug info (`!DILocation`, `DISubprogram`, `DICompileUnit`)

```
AST + Environment -> Codegen -> LLVM IR (.ll)
```

### 5. Object Code

The driver writes the IR to a temporary file and invokes `clang -c` (LLVM's integrated assembler) to produce an object file.

```
LLVM IR -> clang -c -> .o / .obj
```

### 6. Linking

The `smelt` driver auto-discovers a system linker (`cc`, `clang`, `gcc`, `cl.exe`, or `link.exe`) and links the object file with the `ember` runtime library.

```
.o/.obj + ember.a -> linker -> executable
```

## Pipeline Configuration

`PipelineConfig` controls LLVM optimization passes via `clang` flags:

| Field | Default | Effect |
|-------|---------|--------|
| `opt_level` | `Aggressive` | `-O0` through `-O3` |
| `vectorize_slp` | `true` | `-vectorize-slp` |
| `vectorize_loop` | `true` | `-vectorize-loops` |
| `gvn` | `true` | Reserved for future fine-grained control |
| `sroa` | `true` | Reserved for future fine-grained control |
| `licm` | `true` | Reserved for future fine-grained control |
| `slsr` | `true` | Reserved for future fine-grained control |
| `merge_functions` | `true` | Reserved for future fine-grained control |
| `target_cpu` | `None` | `-mcpu=<cpu>` |
| `target_features` | `None` | `-mattr=<features>` |
| `reloc_model` | `"pic"` | `-relocation-model=<model>` |

`native_host_args()` returns `["-mcpu", "native"]` for host-optimized code.

## Compiler Driver API

```
use nimble::{compile, CompileOptions};

let opts = CompileOptions {
    output_path: "output.obj".into(),
    source_path: Some("src/main.nbl".into()),
    emit_llvm: false,
    ..Default::default()
};
compile(source, &opts).unwrap();
```

## Build Profiles

The workspace defines these Cargo profiles:

### release
- `lto = "fat"`, `codegen-units = 1`, `panic = "abort"`
- `strip = "symbols"`, `debug = false`, `incremental = false`
- `opt-level = 3`

### bench
- `lto = "fat"`, `codegen-units = 1`
- `opt-level = 3`, `debug = false`
