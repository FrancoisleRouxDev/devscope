//! # stats.rs — the scanner's output data model
//!
//! PURPOSE
//!   Plain data types that describe a scan result, plus pure functions to
//!   merge them. No I/O, no threads, no PyO3 here — which makes this the
//!   easiest module to unit-test and the stable "contract" that the Python
//!   stubs (`devscope_scanner.pyi`), the Pydantic schemas
//!   (`api/devscope_api/schemas.py`) and the TS types
//!   (`dashboard/src/lib/api/types.ts`) all mirror.
//!
//! PATH CONVENTION AT THE BOUNDARY
//!   `FileStats::path` is stored RELATIVE to the scan root and normalised to
//!   `/` separators, so the dashboard renders `src/main.rs` identically on
//!   Windows, macOS and Linux.

// ============================================================================
// LOGIC SKETCH (commented out — not implemented yet)
// ============================================================================
//
// use crate::language::Language;
// use std::collections::HashMap;
//
// #[derive(Debug, Clone, Default, PartialEq, Eq)]
// pub struct LineCounts { pub code: u64, pub comments: u64, pub blanks: u64 }
//
// impl LineCounts {
//     pub fn total(&self) -> u64 { self.code + self.comments + self.blanks }
// }
//
// impl std::ops::AddAssign for LineCounts { /* field-wise += */ }
//
// #[derive(Debug, Clone)]
// pub struct FileStats {
//     pub path: String,          // relative, "/"-separated
//     pub language: Language,
//     pub lines: LineCounts,
//     pub bytes: u64,
// }
//
// #[derive(Debug, Clone, Default)]
// pub struct LanguageStats { pub files: u64, pub lines: LineCounts, pub bytes: u64 }
//
// #[derive(Debug, Clone)]
// pub struct ScanReport {
//     pub root: String,
//     pub languages: Vec<(Language, LanguageStats)>,   // sorted by code lines desc
//     pub largest_files: Vec<FileStats>,               // top N by total lines
//     pub skipped: u64,
//     pub duration_ms: u64,
// }
//
// /// Per-thread accumulator. Each rayon worker owns one (no locks), and they
// /// are merged pairwise at the end (`reduce`). Merge must be associative.
// #[derive(Default)]
// pub struct Accumulator {
//     by_lang: HashMap<Language, LanguageStats>,
//     top_files: TopN<FileStats>,   // a small bounded min-heap, not a full sort
//     skipped: u64,
// }
//
// impl Accumulator {
//     pub fn add(&mut self, f: FileStats) { ... }
//     pub fn merge(mut self, other: Self) -> Self { ... }
//     pub fn finish(self, root: String, duration_ms: u64) -> ScanReport { ... }
// }

// ============================================================================
// JUNIOR vs SENIOR — aggregating results from many threads
// ============================================================================
//
// ── JUNIOR ──────────────────────────────────────────────────────────────────
//
//     lazy_static! { static ref TOTALS: Mutex<HashMap<String, u64>> = ...; }
//
//     files.par_iter().for_each(|f| {
//         let n = count_lines(f);
//         *TOTALS.lock().unwrap().entry(lang_of(f)).or_insert(0) += n;
//     });
//
//     // later: collect every FileStats into a Vec and sort it to get the top 20
//
//   Problems:
//     * Global mutable state: a second scan (the API serves many!) sees the
//       first scan's numbers unless someone remembers to clear it.
//     * Every file grabs the same Mutex → threads queue up; 16 cores perform
//       like 2.
//     * Sorting 500k FileStats to keep 20 wastes memory and time.
//
// ── SENIOR ──────────────────────────────────────────────────────────────────
//
//     files.par_iter()
//          .fold(Accumulator::default, |mut acc, f| { acc.add(count(f)); acc })
//          .reduce(Accumulator::default, Accumulator::merge)
//          .finish(root, elapsed)
//
//   * No global state: each scan builds its own value and returns it.
//   * fold/reduce gives every thread a private accumulator → zero contention.
//   * Bounded TopN heap keeps memory O(N), not O(files).
//   * `merge` is a pure, associative function → property-testable.
