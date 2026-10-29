//! # language.rs — "what language is this file, and how does it comment?"
//!
//! PURPOSE
//!   * Map a path to a `Language` (by extension, then by well-known filename
//!     like `Dockerfile` / `Makefile`, optionally by shebang `#!/usr/bin/env python`).
//!   * Describe each language's comment syntax so `counter.rs` can classify
//!     lines without knowing anything language-specific itself.
//!
//! CROSS-PLATFORM NOTES
//!   * Extension matching must be case-insensitive: Windows users often have
//!     `Main.CS` or `README.MD`.
//!   * Use `Path::extension()` / `file_name()` — never split on '/' or '\\'.

// ============================================================================
// LOGIC SKETCH (commented out — not implemented yet)
// ============================================================================
//
// use std::path::Path;
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
// #[derive(serde::Serialize)]            // for the CLI's --json output
// pub enum Language {
//     Rust, Python, JavaScript, TypeScript, Svelte, Html, Css,
//     Go, Java, C, Cpp, CSharp, Shell, Markdown, Json, Toml, Yaml,
//     // ...extend as needed
// }
//
// /// Comment syntax a language uses. `&'static` because the table is built
// /// into the binary — no allocation per file.
// pub struct CommentSyntax {
//     pub line:  &'static [&'static str],                  // e.g. ["//"] or ["#"]
//     pub block: &'static [(&'static str, &'static str)],  // e.g. [("/*", "*/")]
// }
//
// impl Language {
//     pub fn detect(path: &Path) -> Option<Self> {
//         // 1. Exact filename matches first ("Dockerfile", "Makefile").
//         // 2. Then the extension, lower-cased once.
//         let ext = path.extension()?.to_str()?.to_ascii_lowercase();
//         Some(match ext.as_str() {
//             "rs" => Self::Rust,
//             "py" | "pyi" => Self::Python,
//             "js" | "mjs" | "cjs" => Self::JavaScript,
//             "ts" | "mts" | "cts" => Self::TypeScript,
//             "svelte" => Self::Svelte,
//             // ...
//             _ => return None,     // unknown → caller decides (count as "Other" or skip)
//         })
//     }
//
//     pub const fn comment_syntax(self) -> CommentSyntax { /* match self { ... } */ }
//
//     /// Stable display name used across the Python/JSON boundary.
//     pub const fn name(self) -> &'static str { /* "Rust", "Python", ... */ }
// }

// ============================================================================
// JUNIOR vs SENIOR — modelling a closed set of values
// ============================================================================
//
// ── JUNIOR ──────────────────────────────────────────────────────────────────
//
//     pub fn detect(path: &str) -> String {
//         if path.ends_with(".rs") { return "rust".to_string(); }
//         if path.ends_with(".py") { return "Python".to_string(); }  // inconsistent casing
//         if path.ends_with(".PY") { return "python".to_string(); }  // patch for one bug report
//         "unknown".to_string()
//     }
//
//   Problems:
//     * Stringly-typed: a typo ("pyhton") compiles fine and silently creates a
//       new "language" in the dashboard.
//     * Allocates a fresh String for *every file* in a 500k-file monorepo.
//     * `ends_with(".rs")` on a `&str` path also matches a *directory* called
//       `foo.rs/` and breaks on non-UTF-8 paths.
//     * Comment syntax ends up as another giant if/else elsewhere — two lists
//       that must be kept in sync by hand.
//
// ── SENIOR ──────────────────────────────────────────────────────────────────
//
//   * A `Copy` enum: zero allocation, exhaustively `match`ed — adding a variant
//     makes the compiler point at every place that needs updating.
//   * Data lives next to the type (`comment_syntax()`, `name()`), so there's
//     one source of truth.
//   * Input is `&Path`, using `extension()` → correct on every OS.
//   * The string form is produced only at the boundary (`name()`), once.
