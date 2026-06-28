# Package Management

Nimble ships with a registry-less, Git-native package manager. Packages are identified by Git repository URLs with optional version tags, branches, or commit hashes - no central registry required.

## Global storage layout

All packages and binaries are stored under the user's home directory:

```
~/.nimble/
├── bin/               ← installed executables (add to $PATH)
└── cache/
    ├── repos/         ← bare/shallow Git clones per URL
    └── pkgs/          ← expanded library packages
```

On Windows the root is `%USERPROFILE%\.nimble\`.

Add `~/.nimble/bin` to your shell's `$PATH` to use installed binaries directly.

## Commands

### `nimble fetch [path]`

Reads the `[dependencies]`, `[dev-dependencies]`, and `[build-dependencies]` tables from `nimble.toml`, clones or fetches every Git dependency in parallel, resolves semver constraints, performs cycle detection and topological sort, and writes a `nimble.lock` with commit hashes and SHA-256 checksums.

```sh
nimble fetch          # uses current directory
nimble fetch ./myapp  # explicit project root
```

**nimble.toml dependency format:**

```toml
[project]
name    = "myapp"
version = "0.1.0"

[dependencies]
json   = { git = "https://github.com/user/json", tag = "v1.2.0" }
http   = { git = "https://github.com/user/http", branch = "main" }
utils  = { path = "../utils" }

[dev-dependencies]
test-lib = { git = "https://github.com/user/test", rev = "abc123" }
```

Bare version strings like `json = "1.2.0"` are stored as constraints but require an explicit source URL to resolve.

---

### `nimble install <uri>@<version>`

Clone, compile, and install a standalone executable binary from a remote Nimble project repository.

```sh
nimble install https://github.com/user/kairo@v1.0.5
```

**Pipeline:**

1. Clone the repository at the given tag into the shared repo cache.
2. Verify a `nimble.toml` is present with a valid entry point.
3. Compile the entry point into a native binary via `smelt::driver::compile`.
4. Move the binary to `~/.nimble/bin/{name}[.exe]`.

After installation, run the binary directly if `~/.nimble/bin` is on your `$PATH`.

---

### `nimble uninstall <name>`

Remove a previously installed binary from `~/.nimble/bin/`.

---

### `nimble upgrade <uri>@<version>`

Re-install a binary at the specified version (uninstalls old, installs new).

---

### `nimble pkg install <uri>@<version>`

Manually cache a library package globally without requiring a local project. The package is cloned into `~/.nimble/cache/pkgs/{name}@{version}/`.

```sh
nimble pkg install https://github.com/user/http-server@v1.2.0
```

---

### `nimble pkg uninstall <uri>@<version>`

Remove a cached library package.

---

### `nimble pkg upgrade <uri>@<version>`

Re-clone a cached library package.

## Source types

| Type | Example | Description |
|------|---------|-------------|
| `git` | `{ git = "https://github.com/user/repo", tag = "v1.0" }` | Git repository with optional tag, branch, or rev |
| `path` | `{ path = "../local-lib" }` | Local filesystem path |
| `version` | `"1.2.0"` | Semver constraint (requires lockfile entry or explicit source) |

## Lockfile

After `fetch`, a `nimble.lock` is generated that pins every dependency to a specific commit with a SHA-256 checksum. The lockfile records:

- Package name, version, source URL, commit hash, checksum
- Transitive dependency list
- Feature flags
- Dependency kind (normal / dev / build)

## Compiler integration

After `nimble fetch` resolves dependencies, the cached source paths are available on the module search path for `import` resolution.

## Requirements

- `git` must be available on `$PATH` for cloning and fetching.
- The Nimble compiler must be available to build installed binaries.
- LLVM / `clang` is required for compilation.
