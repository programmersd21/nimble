# Package Management

Nimble ships with `nim`, a decentralized, URI-driven package manager that mirrors the Go/Axiom model: packages are identified by their full Git host path and version tag, with no central registry required.

## Global storage layout

All packages and binaries are stored under the user's home directory:

```
~/.nimble/
├── bin/                                    ← installed executables (add to $PATH)
└── pkgs/
    └── {domain}/
        └── {username}/
            └── {repo}@{version}/          ← cached library source trees
```

On Windows the root is `%USERPROFILE%\.nimble\`.

Add `~/.nimble/bin` to your shell's `$PATH` to use installed binaries directly.

## Commands

### `nim fetch [path]`

Reads the `[dependencies]` table from the local `nimble.toml` and ensures every declared package is cloned and cached under `~/.nimble/pkgs/`. Returns the resolved source paths for the compiler's module search path.

```sh
nim fetch          # uses current directory
nim fetch ./myapp  # explicit project root
```

**nimble.toml dependency format:**

```toml
[project]
name    = "myapp"
version = "0.1.0"

[dependencies]
"github.com/soumalya/http-server" = "v1.2.0"
"github.com/user/utils"           = "main"
```

---

### `nim pkg install <uri>@<version>`

Manually cache a library package globally without requiring a local project or `nimble.toml`. Useful for pre-warming the cache or inspecting a package.

```sh
nim pkg install github.com/soumalya/http-server@v1.2.0
nim pkg install github.com/user/utils@main
```

The package is cloned into `~/.nimble/pkgs/{domain}/{user}/{repo}@{version}/` and is immediately available for `import` resolution by the compiler.

---

### `nim install <uri>@<version>`

Clone, compile, and install a standalone executable binary from a remote Nimble project repository.

```sh
nim install github.com/soumalya/kairo@v1.0.5
```

**Pipeline:**

1. Clone the repository at the given tag into an isolated temp directory.
2. Verify a `nimble.toml` is present (confirms it is a Nimble project).
3. Invoke `smelt` to compile the entry point into a native binary.
4. Move the binary to `~/.nimble/bin/{repo_name}[.exe]`.

After installation, run the binary directly if `~/.nimble/bin` is on your `$PATH`:

```sh
kairo --help
```

---

## URI format

All package URIs follow the pattern:

```
{domain}/{username}/{repo}
```

The package manager maps this to a Git clone URL automatically:

| URI | Git URL |
|-----|---------|
| `github.com/user/repo` | `https://github.com/user/repo.git` |
| `gitlab.com/org/lib`   | `https://gitlab.com/org/lib.git`   |

Any Git host that serves HTTPS clones is supported.

## Compiler integration

After `nim fetch` resolves dependencies, the cached source paths are injected into the compiler's module search path. This allows `import` statements to resolve remote packages transparently:

```nimble
import github.com/soumalya/http-server/router
```

The compiler resolves this to `~/.nimble/pkgs/github.com/soumalya/http-server@v1.2.0/router`.

## Requirements

- `git` must be available on `$PATH` for cloning.
- `smelt` must be available on `$PATH` for binary installation (`nim install`).
