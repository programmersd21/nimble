# Examples

This directory mirrors the language guidance in the docs. Each `basic/` subtree highlights a particular language topic while the `stdlib/` folders demonstrate the public helpers that ships with Nimble.

## Basic topics
- **Control**
  - `basic/control/conditionals.nmb` – conditional branches and early exits.
  - `basic/control/loops.nmb` – `for`, `while`, and range iteration patterns.
- **Data**
  - `basic/data/collections.nmb` – list manipulation plus map aggregation helpers.
- **Functions**
  - `basic/functions/recursion.nmb` – recursion patterns expressed in Nimble style.
- **Errors**
  - `basic/errors/errors.nmb` – structured errors and `?` propagation.
  - `basic/errors/propagation.nmb` – practical file IO example with `?`.

## Standard library modules
Each subdirectory highlights the idioms for a single standard library module. Run any sample with `cargo run --release -- run examples/<path>`.

- **io**
  - `file_ops.nmb` – basic file lifecycle (write/read/exist).
  - `stream_tools.nmb` – bytes/lines operations plus copy helpers.
- **json**
  - `config.nmb` – configuration extraction and formatting.
  - `roundtrip.nmb` – parse/modify/pretty-print JSON payloads.
- **list**
  - `sort_slice.nmb` – sort/slice helpers.
  - `filters.nmb` – filtering, reversing, and push/pop idioms.
- **map**
  - `keys_values.nmb` – iterate keys and values.
  - `merge.nmb` – merging configuration maps.
- **math**
  - `random_lookup.nmb` – random helpers and lookups.
  - `statistics.nmb` – averages and std dev exploration.
- **net**
  - `http_fetch.nmb` – `net.http_get` and simple fetch logging.
- **os**
  - `env.nmb` – inspect runtime arguments.
- **path**
  - `resolve.nmb` – `path.join` and normalization.
- **process**
  - `pipeline.nmb` – shelling out via `process.run`.
- **regex**
  - `extract.nmb` – matches/find helpers.
- **string**
  - `formatting.nmb` – formatting, uppercasing, and padding.
- **time**
  - `countdown.nmb` – sleeping + measuring elapsed time.
