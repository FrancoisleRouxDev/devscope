// duplicates.rs - finds files with identical content.
// Cheap filters first: group by size, then hash only the files that share a
// size, then confirm candidates byte-for-byte so a hash collision can never
// report two different files as the same. Read-only.

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::Hasher;
use std::io::Read;
use std::path::{Path, PathBuf};

use rayon::prelude::*;
use walkdir::WalkDir;

use crate::types::DuplicateGroup;

const CHUNK: usize = 64 * 1024;
const MAX_GROUPS: usize = 100;

fn hash_file(path: &Path) -> Option<u64> {
    let mut f = File::open(path).ok()?;
    let mut hasher = DefaultHasher::new(); // fixed keys, so the same content always hashes the same
    let mut buf = vec![0u8; CHUNK];
    loop {
        let n = f.read(&mut buf).ok()?;
        if n == 0 {
            return Some(hasher.finish());
        }
        hasher.write(&buf[..n]);
    }
}

/// Full byte-for-byte comparison.
fn same_content(a: &Path, b: &Path) -> bool {
    let (Ok(mut fa), Ok(mut fb)) = (File::open(a), File::open(b)) else { return false };
    let (mut ba, mut bb) = (vec![0u8; CHUNK], vec![0u8; CHUNK]);
    loop {
        let Ok(na) = fa.read(&mut ba) else { return false };
        let mut filled = 0;
        while filled < na {
            match fb.read(&mut bb[filled..na]) {
                Ok(0) | Err(_) => return false,
                Ok(n) => filled += n,
            }
        }
        if na == 0 {
            // `a` is exhausted: equal only if `b` is too
            return fb.read(&mut bb).map_or(false, |n| n == 0);
        }
        if ba[..na] != bb[..na] {
            return false;
        }
    }
}

/// Split same-hash files into groups that are truly identical.
fn confirm(size: u64, mut paths: Vec<PathBuf>) -> Vec<DuplicateGroup> {
    let mut groups = Vec::new();
    while paths.len() > 1 {
        let base = paths.remove(0);
        let (same, rest): (Vec<_>, Vec<_>) = paths.into_iter().partition(|p| same_content(&base, p));
        paths = rest;
        if !same.is_empty() {
            let copies = same.len() as u64 + 1;
            let mut all = vec![base.to_string_lossy().into_owned()];
            all.extend(same.iter().map(|p| p.to_string_lossy().into_owned()));
            groups.push(DuplicateGroup { size, paths: all, wasted: size * (copies - 1) });
        }
    }
    groups
}

pub fn find(root: &Path, min_size: u64) -> Vec<DuplicateGroup> {
    let mut by_size: HashMap<u64, Vec<PathBuf>> = HashMap::new();
    for e in WalkDir::new(root).follow_links(false).into_iter().filter_map(Result::ok) {
        if !e.file_type().is_file() {
            continue;
        }
        if let Ok(m) = e.metadata() {
            if m.len() >= min_size.max(1) {
                by_size.entry(m.len()).or_default().push(e.into_path());
            }
        }
    }

    let candidates: Vec<(u64, Vec<PathBuf>)> = by_size.into_iter().filter(|(_, v)| v.len() > 1).collect();

    let mut groups: Vec<DuplicateGroup> = candidates
        .par_iter()
        .flat_map_iter(|(size, paths)| {
            let mut by_hash: HashMap<u64, Vec<PathBuf>> = HashMap::new();
            for p in paths {
                if let Some(h) = hash_file(p) {
                    by_hash.entry(h).or_default().push(p.clone());
                }
            }
            by_hash.into_values().filter(|v| v.len() > 1).flat_map(|v| confirm(*size, v)).collect::<Vec<_>>()
        })
        .collect();

    groups.sort_by(|a, b| b.wasted.cmp(&a.wasted));
    groups.truncate(MAX_GROUPS);
    groups
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn finds_only_true_duplicates() {
        let root = std::env::temp_dir().join(format!("devscope_dup_{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("x")).unwrap();
        fs::write(root.join("a.bin"), b"hello world").unwrap();
        fs::write(root.join("x/copy.bin"), b"hello world").unwrap();
        fs::write(root.join("same_size.bin"), b"hello there").unwrap(); // same length, different content
        fs::write(root.join("unique.bin"), b"abc").unwrap();

        let groups = find(&root, 1);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].paths.len(), 2);
        assert_eq!(groups[0].wasted, 11);
        let _ = fs::remove_dir_all(&root);
    }
}
