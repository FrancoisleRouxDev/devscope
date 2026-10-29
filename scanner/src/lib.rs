//! # devscope-scanner — crate root
//!
//! The fast core of devscope. Given a directory, it produces a [`ScanReport`]:
//! how many files and lines a project has, broken down by language, plus the
//! largest files. Everything else in devscope (the FastAPI service, the
//! Electron/Svelte dashboard) only *displays* what this crate computes.
//!
//! ## Module map
//!
//! | module       | responsibility                                               |
//! |--------------|--------------------------------------------------------------|
//! | `error`      | One typed error enum for the whole crate                     |
//! | `language`   | File → `Language` detection + each language's comment syntax |
//! | `counter`    | Classify the lines of ONE file into code / comment / blank   |
//! | `walker`     | Recursively list candidate files (gitignore-aware, parallel) |
//! | `stats`      | Plain data types (`FileStats`, `LanguageStats`, `ScanReport`)|
//! | `scan`       | Orchestrator: walker → counter → stats aggregation           |
//! | `python`     | PyO3 bindings (compiled only with `--features python`)       |
//!
//! ## Data flow
//!
//! ```text
//!  root path ──▶ walker ──(paths)──▶ language::detect ──▶ counter ──▶ stats::merge ──▶ ScanReport
//!                  │ parallel                                 │ parallel (rayon)
//!                  └─ skips .git, node_modules, target, …     └─ skips binary / huge files
//! ```
//!
//! ## Design rules for this crate
//!
//! * **No panics on user data.** A weird file is *skipped and counted*, never
//!   a crash. Panics across the PyO3 boundary would take down the API process.
//! * **No I/O in `stats`.** Data types are pure, so they are trivial to test.
//! * **Cross-platform paths.** Use `Path`/`PathBuf` internally; only convert to
//!   `/`-separated strings at the output boundary (see `stats`).
//! * **The library never prints.** Output is the CLI's / Python's job.

// Module declarations are *structure*, not logic — each file is currently a
// documented placeholder that compiles to nothing.
pub mod counter;
pub mod error;
pub mod language;
pub mod scan;
pub mod stats;
pub mod walker;

#[cfg(feature = "python")]
mod python;

// --- Planned public API (uncomment when the modules are implemented) --------
//
// Re-export the handful of items callers actually need so they can write
// `devscope_scanner::scan(..)` instead of `devscope_scanner::scan::scan(..)`.
//
// pub use error::{ScanError, Result};
// pub use language::Language;
// pub use scan::{scan, ScanOptions};
// pub use stats::{FileStats, LanguageStats, ScanReport};
