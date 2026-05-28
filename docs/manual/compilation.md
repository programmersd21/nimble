# Compilation Pipeline

## Pipeline Stages

### 1. Lexing

`Lexer::new(source)` tokenizes the source string into a flat stream of `Token` values with positional `Span` information. Indentation is tracked via an internal stack, producing `Indent` / `Dedent` tokens for blocks.

```
Source chars -> Lexer -> Token stream
```

### 2. Parsing

`Parser::new(source)` consumes the token stream and builds an AST (`Program`). Expression parsing uses Pratt-style precedence climbing.

```
Token stream -> Parser -> AST (Program)
```

### 3. Type Checking

`TypeChecker::check_program(program)` performs semantic analysis in two passes:
- **Pass 1**: Register all top-level function signatures.
- **Pass 2**: Type-check all function bodies against registered signatures.

Returns an `Environment` mapping every name to its resolved `Symbol`.

```
AST -> TypeChecker -> Environment + Subst
```

### 4. Code Generation

`Codegen::generate(program, env)` emits textual LLVM IR (`.ll` format). Built-in type mapping:

| Nimble | LLVM IR |
|--------|---------|
| `Int`  | `i64` |
| `Float` | `double` |
| `Bool` | `i1` |
| `String` | `ptr` |
| `Void` | `void` |

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
