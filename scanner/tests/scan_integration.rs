//! # tests/scan_integration.rs — end-to-end tests of the public scanner API
//!
//! PURPOSE
//!   Unit tests live next to the code (`#[cfg(test)] mod tests` in each
//!   module). THIS file tests the crate from the outside, exactly as the CLI
//!   and the Python binding use it: build a real directory tree, call
//!   `devscope_scanner::scan`, assert on the `ScanReport`.
//!
//! STRATEGY
//!   * Static fixtures in `tests/fixtures/` for known-good expected counts.
//!   * `tempfile::TempDir` for generated cases (symlink loops, binary files,
//!     huge files, non-UTF-8 names, CRLF files) so nothing OS-specific is
//!     committed to git.
//!   * Mark Unix-only cases with `#[cfg(unix)]` (e.g. symlink creation needs
//!     admin/dev-mode on Windows).

// ============================================================================
// PLANNED TESTS (commented out — not implemented yet)
// ============================================================================
//
// #[test] fn counts_fixture_project_exactly() { /* compare against fixtures/expected.json */ }
// #[test] fn missing_root_returns_root_not_found() { }
// #[test] fn file_as_root_returns_root_not_a_directory() { }
// #[test] fn respects_gitignore_by_default() { }
// #[test] fn skips_node_modules_even_without_gitignore() { }
// #[test] fn binary_files_are_skipped_and_counted_as_skipped() { }
// #[test] fn crlf_and_lf_files_produce_identical_counts() { }
// #[test] fn non_utf8_file_content_does_not_fail_the_scan() { }
// #[test] fn paths_in_report_are_relative_and_forward_slashed() { }
// #[cfg(unix)]
// #[test] fn symlink_loop_terminates() { }
