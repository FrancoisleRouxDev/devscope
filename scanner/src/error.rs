//! # error.rs — the crate's single error type
//!
//! PURPOSE
//!   Every fallible function in the scanner returns `Result<T, ScanError>`.
//!   One enum means callers (the CLI, the PyO3 layer) handle a closed, known
//!   set of failures, and `python.rs` can map each variant to a Python
//!   exception in exactly one place.
//!
//! WHAT IS *NOT* AN ERROR
//!   A single unreadable / binary / oversized file is **not** an error — the
//!   scan skips it and increments `ScanReport::skipped`. Only problems that
//!   make the *whole* scan meaningless (root missing, root not a directory)
//!   become `ScanError`s.

// ============================================================================
// LOGIC SKETCH (commented out — not implemented yet)
// ============================================================================
//
// use std::path::PathBuf;
//
// #[derive(Debug, thiserror::Error)]
// #[non_exhaustive]                       // we may add variants without a breaking change
// pub enum ScanError {
//     #[error("scan root does not exist: {0}")]
//     RootNotFound(PathBuf),
//
//     #[error("scan root is not a directory: {0}")]
//     RootNotADirectory(PathBuf),
//
//     #[error("I/O error at {path}: {source}")]
//     Io {
//         path: PathBuf,
//         #[source]
//         source: std::io::Error,
//     },
// }
//
// /// Crate-wide alias so signatures stay short: `fn scan(..) -> Result<ScanReport>`.
// pub type Result<T, E = ScanError> = std::result::Result<T, E>;

// ============================================================================
// JUNIOR vs SENIOR — error handling in Rust
// ============================================================================
//
// ── JUNIOR ──────────────────────────────────────────────────────────────────
//
//     pub fn read_root(path: &str) -> Vec<String> {
//         let entries = std::fs::read_dir(path).unwrap();     // panics on a typo
//         entries.map(|e| e.unwrap().path().to_str().unwrap().to_string()).collect()
//         //                 ^ panics on a permission error
//         //                                   ^ panics on a non-UTF-8 path (Windows/Linux!)
//     }
//
//   Or the "slightly better" version that still loses information:
//
//     pub fn read_root(path: &str) -> Result<Vec<String>, String> {
//         std::fs::read_dir(path).map_err(|e| e.to_string())?;  // stringly-typed
//         ...
//     }
//
//   Problems:
//     * `unwrap()` turns a user mistake into a crash. Across PyO3 that crash
//       becomes a `PanicException` and can leave the API in a bad state.
//     * `String` errors can't be matched on — the caller can't tell "not found"
//       from "permission denied" without parsing text.
//     * The failing *path* is lost, so the message says "No such file" but not
//       *which* file.
//
// ── SENIOR ──────────────────────────────────────────────────────────────────
//
//     pub fn read_root(root: &Path) -> Result<ReadDir> {
//         if !root.exists()  { return Err(ScanError::RootNotFound(root.into())); }
//         if !root.is_dir()  { return Err(ScanError::RootNotADirectory(root.into())); }
//         std::fs::read_dir(root).map_err(|source| ScanError::Io { path: root.into(), source })
//     }
//
//   Why it's better:
//     * Typed variants → callers `match` on them; python.rs maps each to a
//       specific Python exception / HTTP status.
//     * The path travels with the error, so the dashboard can show it.
//     * `#[source]` keeps the original io::Error chain for debugging.
//     * `#[non_exhaustive]` lets the enum grow without breaking downstream.
//     * `unwrap()` is reserved for true invariants, with a comment saying why
//       it can't fail — or replaced by `expect("reason")`.
