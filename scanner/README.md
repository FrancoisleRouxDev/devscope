# scanner — Rust core

Fast, parallel, `.gitignore`-aware file and line counter. Builds as both a
Rust library/CLI and a Python extension module (`devscope_scanner`) via
[PyO3](https://pyo3.rs) + [maturin](https://www.maturin.rs).

See the root [README](../README.md) for how this fits into devscope.

## Layout

| Path                         | Purpose                                                       |
|------------------------------|---------------------------------------------------------------|
| `Cargo.toml`                 | Crate manifest; `python` feature gates PyO3                   |
| `pyproject.toml`             | maturin config → builds the `devscope-scanner` Python wheel   |
| `devscope_scanner.pyi`       | Python type stubs — the Rust⇄Python contract                  |
| `src/lib.rs`                 | Crate root, module map, public re-exports                     |
| `src/error.rs`               | `ScanError` enum + `Result` alias                             |
| `src/language.rs`            | Language detection and comment syntax table                   |
| `src/counter.rs`             | Per-file code/comment/blank line classification               |
| `src/walker.rs`              | Parallel, gitignore-aware directory walk                      |
| `src/stats.rs`               | Output data model + lock-free aggregation                     |
| `src/scan.rs`                | Orchestrator: `scan(root, &ScanOptions)`                      |
| `src/python.rs`              | PyO3 bindings (only with `--features python`)                 |
| `src/bin/devscope-scan.rs`   | Debug/benchmark CLI                                           |
| `tests/`                     | Integration tests + fixtures                                  |

## Commands

```bash
cargo check                               # fast type-check, no Python needed
cargo test                                # unit + integration tests
cargo clippy --all-targets -- -D warnings # lint
cargo run --release --bin devscope-scan -- ../   # CLI (once implemented)

# Python extension (activate the /api venv first):
maturin develop --release                 # build + install into the venv
maturin build --release                   # wheel → target/wheels/
```
