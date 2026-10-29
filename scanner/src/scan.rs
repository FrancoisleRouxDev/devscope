//! # scan.rs — the orchestrator (the crate's main entry point)
//!
//! PURPOSE
//!   `scan(root, &ScanOptions) -> Result<ScanReport>` wires the other modules
//!   together: validate root → walk → detect language → count → aggregate.
//!   Both the CLI (`bin/devscope-scan.rs`) and the Python binding
//!   (`python.rs`) call ONLY this function — they never touch walker/counter
//!   directly. That keeps both front-ends behaving identically.
//!
//! FUTURE: PROGRESS + CANCELLATION
//!   Large monorepos take seconds. A later version accepts an optional
//!   progress callback (files seen so far) and an `AtomicBool` cancel flag the
//!   API can flip when the user starts another scan or closes the window.

// ============================================================================
// LOGIC SKETCH (commented out — not implemented yet)
// ============================================================================
//
// use crate::{error::{Result, ScanError}, stats::{Accumulator, ScanReport}, walker};
// use std::{path::Path, time::Instant};
//
// #[derive(Debug, Clone)]
// pub struct ScanOptions {
//     pub respect_gitignore: bool,   // default true
//     pub include_hidden: bool,      // default false
//     pub max_file_size: u64,        // default 5 MB
//     pub top_n_files: usize,        // default 20
//     pub threads: usize,            // default 0 (= all cores)
// }
//
// impl Default for ScanOptions { fn default() -> Self { /* the defaults above */ } }
//
// pub fn scan(root: &Path, opts: &ScanOptions) -> Result<ScanReport> {
//     let started = Instant::now();
//     validate_root(root)?;                         // RootNotFound / RootNotADirectory
//
//     // Option A (simplest): walk into a Vec, then rayon fold/reduce over it.
//     // Option B (lower latency/memory): walker sends paths over a bounded
//     //          crossbeam channel; rayon workers consume and count.
//     // Start with A, measure, move to B only if profiling says so.
//
//     let acc: Accumulator = /* walk → par_iter → fold(add) → reduce(merge) */;
//     Ok(acc.finish(display_root(root), started.elapsed().as_millis() as u64))
// }

// ============================================================================
// JUNIOR vs SENIOR — API shape of the entry point
// ============================================================================
//
// ── JUNIOR ──────────────────────────────────────────────────────────────────
//
//     pub fn scan(path: String, gitignore: bool, hidden: bool, max: u64, top: usize)
//         -> HashMap<String, HashMap<String, u64>>
//
//   Problems:
//     * Five positional args — `scan(p, true, false, 20, 5_000_000)` silently
//       swaps max/top and compiles fine.
//     * Takes `String` by value → forces callers to allocate/clone.
//     * Returns nested maps: no documentation, no type safety, every consumer
//       re-discovers the keys.
//     * Can't fail → so it either panics or returns empty maps on bad input.
//
// ── SENIOR ──────────────────────────────────────────────────────────────────
//
//     pub fn scan(root: &Path, opts: &ScanOptions) -> Result<ScanReport>
//
//   * Borrow inputs (`&Path`) — accepts &str, String, PathBuf via `AsRef<Path>`
//     if desired, without forcing allocation.
//   * Options struct with `Default` → call sites read
//     `ScanOptions { top_n_files: 50, ..Default::default() }`; new options
//     don't break existing callers.
//   * A named, documented return type; a typed error.
//   * Timing measured inside the library so every front-end reports the same.
