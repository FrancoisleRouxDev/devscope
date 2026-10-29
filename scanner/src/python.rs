//! # python.rs — PyO3 bindings (compiled only with `--features python`)
//!
//! PURPOSE
//!   Expose a tiny, Pythonic surface over the Rust core:
//!
//!   ```python
//!   import devscope_scanner
//!   report = devscope_scanner.scan("/path/to/project", top_n_files=20)
//!   report.languages[0].language   # "Rust"
//!   ```
//!
//!   This is the ONLY file that knows Python exists. It converts:
//!     * Python args (str, kwargs)   → `Path` + `ScanOptions`
//!     * `ScanReport`                → `#[pyclass]` objects (read-only attributes)
//!     * `ScanError`                 → `devscope_scanner.ScanError` (a Python exception)
//!
//!   The contract is documented in `scanner/devscope_scanner.pyi`.
//!
//! BUILD
//!   `maturin develop --release` from /scanner (with the /api venv active).

// ============================================================================
// LOGIC SKETCH (commented out — not implemented yet)
// ============================================================================
//
// use pyo3::{create_exception, exceptions::PyException, prelude::*};
// use std::path::PathBuf;
//
// create_exception!(devscope_scanner, ScanError, PyException);
//
// #[pyclass(frozen, name = "ScanReport", module = "devscope_scanner")]
// pub struct PyScanReport { #[pyo3(get)] root: String, /* ... */ }
//
// #[pyfunction]
// #[pyo3(signature = (root, *, respect_gitignore=true, include_hidden=false,
//                     max_file_size=5_000_000, top_n_files=20))]
// fn scan(py: Python<'_>, root: PathBuf, respect_gitignore: bool, include_hidden: bool,
//         max_file_size: u64, top_n_files: usize) -> PyResult<PyScanReport> {
//     let opts = crate::scan::ScanOptions { respect_gitignore, include_hidden,
//                                           max_file_size, top_n_files, ..Default::default() };
//
//     // Release the GIL while Rust works: FastAPI's other threads (health
//     // checks, a second request) keep running during a 3-second scan.
//     let result = py.detach(|| crate::scan::scan(&root, &opts));
//
//     result.map(PyScanReport::from).map_err(|e| ScanError::new_err(e.to_string()))
// }
//
// #[pymodule]
// fn devscope_scanner(m: &Bound<'_, PyModule>) -> PyResult<()> {
//     m.add("__version__", env!("CARGO_PKG_VERSION"))?;
//     m.add("ScanError", m.py().get_type::<ScanError>())?;
//     m.add_class::<PyScanReport>()?;
//     m.add_function(wrap_pyfunction!(scan, m)?)?;
//     Ok(())
// }

// ============================================================================
// JUNIOR vs SENIOR — writing PyO3 bindings
// ============================================================================
//
// ── JUNIOR ──────────────────────────────────────────────────────────────────
//
//     #[pyfunction]
//     fn scan(path: String) -> PyResult<String> {
//         let report = crate::scan::scan(Path::new(&path), &Default::default()).unwrap();
//         Ok(serde_json::to_string(&report).unwrap())   // Python then json.loads() it
//     }
//
//   Problems:
//     * Holds the GIL for the whole scan → the entire FastAPI process freezes,
//       including `/health`, so Electron thinks the backend died.
//     * `unwrap()` → a Rust panic surfaces as `pyo3_runtime.PanicException`,
//       which isn't a normal `Exception` subclass and dodges `except Exception`.
//     * Serialise-to-JSON-then-parse doubles the work and loses typing; Python
//       callers get `dict[str, Any]`.
//     * Business logic creeps into the binding layer over time.
//
// ── SENIOR ──────────────────────────────────────────────────────────────────
//
//   * `py.detach(..)` (called `allow_threads` in older PyO3) around all CPU/I/O work.
//   * A dedicated `ScanError` Python exception; every `ScanError` variant is
//     mapped here, in one place.
//   * Return `#[pyclass(frozen)]` objects — typed, immutable, cheap.
//   * Keyword-only args with defaults (`*` in the signature) mirror the
//     `ScanOptions` defaults, so Python callers can't mix up positional bools.
//   * This file stays a thin adapter: zero scanning logic, just conversion.
//   * A `.pyi` stub ships with the wheel so mypy and editors understand it.
