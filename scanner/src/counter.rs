//! # counter.rs — count the lines of ONE file
//!
//! PURPOSE
//!   Given a path and its `Language`, return `FileStats { code, comments,
//!   blanks, bytes }`. This is the hottest loop in devscope — it runs once per
//!   file, millions of lines per scan — so it is written for throughput.
//!
//! RULES
//!   * Never load a whole huge file into memory: honour `max_file_size` and
//!     skip anything larger (it's almost always generated/minified/vendored).
//!   * Never assume UTF-8. Source files in the wild are Latin-1, UTF-16 with a
//!     BOM, or contain stray bytes. We work on **bytes**, and only the
//!     comment markers (which are ASCII) need to be recognised.
//!   * Detect binary files cheaply: a NUL byte in the first 8 KiB ⇒ binary ⇒ skip.
//!   * Line endings: count `\n`. That handles `\n` (Unix/macOS) and `\r\n`
//!     (Windows) identically; trim a trailing `\r` before classifying a line.
//!     (Classic-Mac `\r`-only files are rare enough to treat as one line.)

// ============================================================================
// LOGIC SKETCH (commented out — not implemented yet)
// ============================================================================
//
// use std::{fs::File, io::{BufRead, BufReader, Read}, path::Path};
// use crate::{error::Result, language::Language, stats::FileStats};
//
// const BINARY_SNIFF_BYTES: usize = 8 * 1024;
//
// pub enum CountOutcome {
//     Counted(FileStats),
//     Skipped(SkipReason),          // Binary | TooLarge | Unreadable
// }
//
// pub fn count_file(path: &Path, lang: Language, max_size: u64) -> CountOutcome {
//     // 1. metadata().len() > max_size        → Skipped(TooLarge)   (no read at all)
//     // 2. open + BufReader::with_capacity(64 KiB)
//     // 3. peek first 8 KiB via fill_buf(); contains 0u8 → Skipped(Binary)
//     // 4. for each line (read_until(b'\n', &mut buf), reusing ONE Vec<u8>):
//     //        let line = trim_ascii(strip_trailing_cr(&buf));
//     //        match classify(line, &syntax, &mut in_block_comment) {
//     //            Blank   => stats.blanks   += 1,
//     //            Comment => stats.comments += 1,
//     //            Code    => stats.code     += 1,
//     //        }
//     //        buf.clear();
//     // 5. any io::Error mid-file   → Skipped(Unreadable), never panic.
// }
//
// /// Pure function → unit-testable with byte literals, no files needed.
// fn classify(line: &[u8], syntax: &CommentSyntax, in_block: &mut bool) -> LineKind { ... }
//
// #[cfg(test)]
// mod tests {
//     // classify(b"   ", ..)            == Blank
//     // classify(b"// hi", ..)          == Comment
//     // classify(b"let x = 1; // hi", ..) == Code   (code wins on mixed lines)
//     // multi-line /* ... */ spanning 3 lines → 3 × Comment
//     // CRLF input counts the same as LF input
// }

// ============================================================================
// JUNIOR vs SENIOR — reading and counting a file
// ============================================================================
//
// ── JUNIOR ──────────────────────────────────────────────────────────────────
//
//     pub fn count_lines(path: &str) -> usize {
//         let content = std::fs::read_to_string(path).unwrap();
//         content.lines().count()
//     }
//
//   Problems:
//     * `read_to_string` fails (→ panic via unwrap) on ANY non-UTF-8 byte —
//       one Latin-1 file kills the whole scan.
//     * Loads a 2 GB log file or minified bundle fully into RAM.
//     * Binary files (images, .exe) get "counted" as garbage lines.
//     * Only total lines; no code/comment/blank split.
//     * `lines()` allocates nothing here, but the junior's *next* step is
//       usually `content.lines().map(|l| l.to_string()).collect::<Vec<_>>()`,
//       which allocates once per line.
//
// ── SENIOR ──────────────────────────────────────────────────────────────────
//
//   * Check size from metadata *before* opening; skip oversized files.
//   * Stream with a `BufReader` and ONE reusable `Vec<u8>` line buffer —
//     constant memory regardless of file size, zero per-line allocation.
//   * Work on `&[u8]`; comment markers are ASCII so no decoding is needed.
//   * Sniff for NUL bytes to skip binaries.
//   * Return an outcome enum (`Counted` / `Skipped(reason)`) so the report can
//     say *why* files were skipped, instead of silently dropping them.
//   * Keep `classify()` pure so the tricky cases (block comments, CRLF,
//     mixed code+comment lines) are covered by fast unit tests.
//   * Measure before micro-optimising: a `memchr`-based newline scan is the
//     next step *only* if benchmarks show the reader is the bottleneck.
