// walker.rs - recursive, parallel directory traversal.
// Walks a project tree, honouring .gitignore and a built-in ignore list
// (node_modules, target, .git, venv, dist, ...), and yields candidate
// source files to the counter.
// (Example logic: uses the `ignore` crate, the same walker ripgrep uses.)

use std::path::{Path, PathBuf};

use ignore::WalkBuilder;

use crate::types::ScanOptions;

/// Directories that are never worth scanning, even without a .gitignore.
const ALWAYS_SKIP: &[&str] = &[
    ".git", "node_modules", "target", "dist", "build", "venv", ".venv", "__pycache__",
];

/// Like `collect_files` but ignores .gitignore (still skips junk folders and .git).
/// Used by the secrets check: a gitignored .env still deserves a mention.
pub fn collect_all(root: &Path, opts: &ScanOptions) -> Vec<PathBuf> {
    let skip: Vec<String> = ALWAYS_SKIP.iter().map(|s| s.to_string()).collect();
    let max_size = opts.max_file_size;

    WalkBuilder::new(root)
        .hidden(false) // dotfiles such as .env are exactly what we want to see
        .git_ignore(false)
        .git_global(false)
        .git_exclude(false)
        .filter_entry(move |e| e.file_name().to_str().map_or(true, |n| !skip.iter().any(|s| s == n)))
        .build()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().map_or(false, |t| t.is_file()))
        .filter(|e| e.metadata().map_or(false, |m| m.len() <= max_size))
        .map(|e| e.into_path())
        .collect()
}

/// Collect every regular file under `root` that passes the ignore rules and size limit.
pub fn collect_files(root: &Path, opts: &ScanOptions) -> Vec<PathBuf> {
    let skip: Vec<String> = ALWAYS_SKIP
        .iter()
        .map(|s| s.to_string())
        .chain(opts.extra_ignores.iter().cloned())
        .collect();
    let max_size = opts.max_file_size;

    WalkBuilder::new(root)
        .hidden(true) // skip dotfiles/dirs
        .filter_entry(move |e| {
            e.file_name()
                .to_str()
                .map(|name| !skip.iter().any(|s| s == name))
                .unwrap_or(true)
        })
        .build()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().map_or(false, |t| t.is_file()))
        .filter(|e| e.metadata().map_or(false, |m| m.len() <= max_size))
        .map(|e| e.into_path())
        .collect()
}
