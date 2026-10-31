// types.rs - shared data structures.
// Defines ScanOptions, FileStats, LanguageStats and ScanResult, with
// PyO3 conversions so they cross the Rust -> Python boundary cleanly.
// (Example logic: field set mirrors the planned data contract in the README.)

use pyo3::prelude::*;

/// Options passed in from Python. Every field has a default.
#[pyclass(get_all, set_all)]
#[derive(Clone, Debug)]
pub struct ScanOptions {
    /// Extra directory/file names to skip, on top of .gitignore.
    pub extra_ignores: Vec<String>,
    /// Skip files larger than this many bytes (avoids huge generated files).
    pub max_file_size: u64,
}

#[pymethods]
impl ScanOptions {
    #[new]
    #[pyo3(signature = (extra_ignores=Vec::new(), max_file_size=5_000_000))]
    fn new(extra_ignores: Vec<String>, max_file_size: u64) -> Self {
        Self { extra_ignores, max_file_size }
    }
}

/// Line counts for a single file.
#[pyclass(get_all)]
#[derive(Clone, Debug, Default)]
pub struct FileStats {
    pub path: String,
    pub language: String,
    pub code: u64,
    pub comments: u64,
    pub blanks: u64,
}

impl FileStats {
    pub fn total_lines(&self) -> u64 {
        self.code + self.comments + self.blanks
    }
}

/// Aggregated counts for one language.
#[pyclass(get_all)]
#[derive(Clone, Debug, Default)]
pub struct LanguageStats {
    pub name: String,
    pub files: u64,
    pub code: u64,
    pub comments: u64,
    pub blanks: u64,
}

/// A project folder found under a workspace root.
#[pyclass(get_all)]
#[derive(Clone, Debug, Default)]
pub struct ProjectInfo {
    pub path: String,
    pub name: String,
    /// What marked it as a project: "git", "node", "rust", "python", ...
    pub kinds: Vec<String>,
    pub size_bytes: u64,
    /// Bytes inside rebuildable folders (node_modules, target, venv, ...).
    pub junk_bytes: u64,
    pub junk_dirs: Vec<String>,
    /// Newest modification time (unix seconds) of files outside junk and .git.
    pub last_modified: i64,
}

/// One possible leaked secret. The value itself is never returned.
#[pyclass(get_all)]
#[derive(Clone, Debug, Default)]
pub struct SecretFinding {
    pub path: String,
    /// 1-based line number; 0 for whole-file findings (e.g. a private key file).
    pub line: u64,
    pub kind: String,
}

/// Everything the API needs to answer one scan request.
#[pyclass(get_all)]
#[derive(Clone, Debug, Default)]
pub struct ScanResult {
    pub root: String,
    pub total_files: u64,
    pub total_lines: u64,
    pub code: u64,
    pub comments: u64,
    pub blanks: u64,
    pub duration_ms: u64,
    pub languages: Vec<LanguageStats>,
    pub largest_files: Vec<FileStats>,
}
