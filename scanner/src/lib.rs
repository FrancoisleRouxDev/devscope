// lib.rs - crate root and PyO3 module definition.
// Declares the submodules and exposes the public Python API
// (e.g. scan_project(path, options) -> ScanResult) as the
// devscope_scanner Python module. Orchestration only: walk, count in
// parallel, aggregate. (Example logic.)

mod counter;
mod discovery;
mod languages;
mod secrets;
mod types;
mod walker;

use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use rayon::prelude::*;

use types::{FileStats, LanguageStats, ProjectInfo, ScanOptions, ScanResult, SecretFinding};

const LARGEST_FILES_LIMIT: usize = 10;

/// Scan `path` and return aggregated stats. Releases the GIL while working.
#[pyfunction]
#[pyo3(signature = (path, options=None))]
fn scan_project(py: Python<'_>, path: &str, options: Option<ScanOptions>) -> PyResult<ScanResult> {
    let root = Path::new(path);
    if !root.is_dir() {
        return Err(PyValueError::new_err(format!("not a directory: {path}")));
    }
    let opts = options.unwrap_or_else(|| ScanOptions {
        extra_ignores: Vec::new(),
        max_file_size: 5_000_000,
    });

    Ok(py.allow_threads(|| scan(root, &opts)))
}

/// Find projects under `root` (a workspace folder) and measure their size.
#[pyfunction]
#[pyo3(signature = (root, max_depth=4))]
fn discover_projects(py: Python<'_>, root: &str, max_depth: usize) -> PyResult<Vec<ProjectInfo>> {
    let root = Path::new(root);
    if !root.is_dir() {
        return Err(PyValueError::new_err(format!("not a directory: {}", root.display())));
    }
    Ok(py.allow_threads(|| discovery::discover(root, max_depth)))
}

/// Look for possible leaked secrets inside one project folder.
#[pyfunction]
fn find_secrets(py: Python<'_>, path: &str) -> PyResult<Vec<SecretFinding>> {
    let root = Path::new(path);
    if !root.is_dir() {
        return Err(PyValueError::new_err(format!("not a directory: {path}")));
    }
    let opts = ScanOptions { extra_ignores: Vec::new(), max_file_size: 5_000_000 };
    Ok(py.allow_threads(|| secrets::find_secrets(root, &opts)))
}

/// Pure-Rust scan, kept separate from the PyO3 wrapper so it is easy to test.
fn scan(root: &Path, opts: &ScanOptions) -> ScanResult {
    let start = Instant::now();

    // 1. walk  2. count files in parallel (rayon)
    let files = walker::collect_files(root, opts);
    let mut per_file: Vec<FileStats> = files
        .par_iter()
        .filter_map(|p| {
            let lang = languages::detect(p)?;
            counter::count_file(p, lang)
        })
        .collect();

    // 3. aggregate per language
    let mut by_lang: HashMap<String, LanguageStats> = HashMap::new();
    let mut result = ScanResult { root: root.to_string_lossy().into_owned(), ..Default::default() };
    for f in &per_file {
        let entry = by_lang.entry(f.language.clone()).or_insert_with(|| LanguageStats {
            name: f.language.clone(),
            ..Default::default()
        });
        entry.files += 1;
        entry.code += f.code;
        entry.comments += f.comments;
        entry.blanks += f.blanks;

        result.total_files += 1;
        result.code += f.code;
        result.comments += f.comments;
        result.blanks += f.blanks;
    }
    result.total_lines = result.code + result.comments + result.blanks;

    result.languages = by_lang.into_values().collect();
    result.languages.sort_by(|a, b| b.code.cmp(&a.code));

    per_file.sort_by_key(|f| std::cmp::Reverse(f.total_lines()));
    per_file.truncate(LARGEST_FILES_LIMIT);
    result.largest_files = per_file;

    result.duration_ms = start.elapsed().as_millis() as u64;
    result
}

/// The `devscope_scanner` Python module.
#[pymodule]
fn devscope_scanner(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ScanOptions>()?;
    m.add_class::<FileStats>()?;
    m.add_class::<LanguageStats>()?;
    m.add_class::<ScanResult>()?;
    m.add_class::<ProjectInfo>()?;
    m.add_class::<SecretFinding>()?;
    m.add_function(wrap_pyfunction!(scan_project, m)?)?;
    m.add_function(wrap_pyfunction!(discover_projects, m)?)?;
    m.add_function(wrap_pyfunction!(find_secrets, m)?)?;
    Ok(())
}
