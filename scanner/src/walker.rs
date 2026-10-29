//! # walker.rs — find the files worth scanning
//!
//! PURPOSE
//!   Recursively walk the scan root and yield candidate file paths, fast and
//!   safely. This module decides WHAT gets scanned; `counter.rs` decides HOW.
//!
//! BEHAVIOUR
//!   * Respects `.gitignore`, `.ignore`, and global git excludes (toggleable).
//!   * Skips hidden entries by default (`.git/`, `.venv/`, `.idea/`).
//!   * Always skips well-known heavy dirs even without a .gitignore:
//!     `node_modules`, `target`, `dist`, `build`, `__pycache__`, `.venv`, `venv`.
//!   * Does NOT follow symlinks by default → no infinite loops, no escaping the
//!     project folder.
//!   * Walks in parallel across CPU cores.
//!
//! CROSS-PLATFORM NOTES
//!   * Windows: paths > 260 chars need the `\\?\` prefix; the `ignore` crate and
//!     std handle this when given absolute, canonicalised paths. Avoid
//!     `canonicalize()` output in *user-facing* strings (it adds `\\?\`) —
//!     use the `dunce` crate or strip it at the output boundary.
//!   * macOS: APFS is case-insensitive by default; don't dedupe by lowercase.
//!   * Linux: filenames may be arbitrary bytes — keep them as `PathBuf`.

// ============================================================================
// LOGIC SKETCH (commented out — not implemented yet)
// ============================================================================
//
// use ignore::{WalkBuilder, WalkState};
// use std::path::{Path, PathBuf};
//
// const ALWAYS_SKIP: &[&str] = &[
//     "node_modules", "target", "dist", "build", "__pycache__", ".venv", "venv", ".git",
// ];
//
// pub struct WalkOptions {
//     pub respect_gitignore: bool,
//     pub include_hidden: bool,
//     pub follow_symlinks: bool,     // default false
//     pub threads: usize,            // 0 = number of logical CPUs
// }
//
// /// Calls `on_file` for every candidate file, from multiple threads.
// /// Streaming (callback) instead of returning Vec<PathBuf> keeps memory flat
// /// and lets counting start before the walk finishes.
// pub fn walk<F>(root: &Path, opts: &WalkOptions, on_file: F)
// where
//     F: Fn(PathBuf) + Sync,
// {
//     WalkBuilder::new(root)
//         .git_ignore(opts.respect_gitignore)
//         .hidden(!opts.include_hidden)
//         .follow_links(opts.follow_symlinks)
//         .threads(opts.threads)
//         .filter_entry(|e| !is_always_skipped(e.file_name()))
//         .build_parallel()
//         .run(|| Box::new(|entry| {
//             if let Ok(e) = entry {
//                 if e.file_type().is_some_and(|t| t.is_file()) { on_file(e.into_path()); }
//             }
//             // Err(_) → one unreadable dir; keep going, don't abort the walk.
//             WalkState::Continue
//         }));
// }

// ============================================================================
// JUNIOR vs SENIOR — walking a directory tree
// ============================================================================
//
// ── JUNIOR ──────────────────────────────────────────────────────────────────
//
//     pub fn walk(dir: &str, out: &mut Vec<String>) {
//         for entry in std::fs::read_dir(dir).unwrap() {
//             let path = entry.unwrap().path();
//             if path.is_dir() {
//                 walk(path.to_str().unwrap(), out);          // recursion
//             } else {
//                 out.push(path.to_str().unwrap().to_string());
//             }
//         }
//     }
//
//   Problems:
//     * `path.is_dir()` follows symlinks → a link to `..` recurses forever
//       (or until stack overflow).
//     * Descends into `node_modules` / `target` / `.git` — a typical JS project
//       goes from 300 files to 300,000, and the stats are meaningless.
//     * Single-threaded; on an SSD the walk is CPU-bound and leaves cores idle.
//     * Collects every path into one Vec before doing anything.
//     * Three `unwrap()`s: one permission-denied folder aborts everything.
//
// ── SENIOR ──────────────────────────────────────────────────────────────────
//
//   * Don't reinvent it: `ignore::WalkBuilder` (the engine behind ripgrep) is
//     battle-tested on every OS, parallel, and .gitignore-aware.
//   * No symlink following by default; opt-in only.
//   * Prune heavy directories with `filter_entry` so they're never even opened.
//   * Stream paths to a callback/channel → bounded memory, pipelined work.
//   * Per-entry errors are recorded and skipped, not fatal.
//   * Options struct instead of a growing list of bool parameters.
