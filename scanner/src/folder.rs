// folder.rs - general analysis of any folder (Downloads, Documents, a drive).
// One parallel pass computes totals per file category, the largest files and
// subfolders, how old the files are, and likely clutter (installers and
// "name (1).ext" download copies). Read-only; symlinks are not followed.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use rayon::prelude::*;
use walkdir::WalkDir;

use crate::discovery;
use crate::types::{AgeBucket, CategoryStats, DirStat, FolderFile, FolderReport};

const TOP_FILES: usize = 25;
const TOP_CLUTTER: usize = 50;
const AGE_LABELS: [&str; 5] = ["Last 30 days", "1 to 6 months", "6 to 12 months", "1 to 2 years", "Over 2 years"];

/// File category from the extension.
pub fn category(path: &Path) -> &'static str {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
    match ext.as_str() {
        "jpg" | "jpeg" | "png" | "gif" | "webp" | "svg" | "bmp" | "heic" | "tif" | "tiff" | "ico" | "raw" => "Images",
        "mp4" | "mkv" | "mov" | "avi" | "webm" | "wmv" | "flv" | "m4v" => "Video",
        "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" | "wma" => "Audio",
        "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "txt" | "rtf" | "odt" | "csv" | "md" | "epub" => "Documents",
        "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" | "iso" => "Archives",
        "exe" | "msi" | "msix" | "appx" | "dmg" | "pkg" | "deb" | "rpm" | "appimage" | "apk" => "Installers",
        "rs" | "py" | "js" | "ts" | "svelte" | "html" | "css" | "json" | "toml" | "yml" | "yaml" | "java" | "c" | "cpp"
        | "h" | "go" | "php" | "rb" | "sh" | "sql" => "Code",
        _ => "Other",
    }
}

/// True for names like "report (1)" or "setup (12)" that browsers add to repeat downloads.
fn is_copy_name(stem: &str) -> bool {
    match stem.rfind(" (") {
        Some(i) if stem.ends_with(')') => {
            let digits = &stem[i + 2..stem.len() - 1];
            !digits.is_empty() && digits.chars().all(|c| c.is_ascii_digit())
        }
        _ => false,
    }
}

/// True for files inside dependency, build or git folders (node_modules, .venv, target, .git ...).
/// Their .exe and archive files belong to tools, so they are never "forgotten downloads".
/// Only the part below `root` is checked, so analysing e.g. C:\work\target\Downloads still works.
fn in_tool_dir(path: &Path, root: &Path) -> bool {
    path.strip_prefix(root).unwrap_or(path).components().any(|c| {
        c.as_os_str()
            .to_str()
            .map_or(false, |s| s == ".git" || discovery::ANYWHERE.contains(&s))
    })
}

fn now_secs() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64)
}

fn age_bucket(modified: i64, now: i64) -> usize {
    if modified <= 0 {
        return 4; // unknown age is grouped with the oldest
    }
    match (now - modified) / 86_400 {
        d if d < 30 => 0,
        d if d < 182 => 1,
        d if d < 365 => 2,
        d if d < 730 => 3,
        _ => 4,
    }
}

fn file_entry(path: &Path, meta: &fs::Metadata) -> FolderFile {
    let modified = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map_or(0, |d| d.as_secs() as i64);
    FolderFile {
        path: path.to_string_lossy().into_owned(),
        size: meta.len(),
        modified,
        category: category(path).to_string(),
        reason: String::new(),
    }
}

/// Keep a list from growing without bound while many threads add to it.
fn trim(v: &mut Vec<FolderFile>, keep: usize) {
    if v.len() > keep * 4 {
        v.sort_by(|a, b| b.size.cmp(&a.size));
        v.truncate(keep);
    }
}

fn finalize(v: &mut Vec<FolderFile>, keep: usize) {
    v.sort_by(|a, b| b.size.cmp(&a.size));
    v.truncate(keep);
}

/// Running totals for one part of the tree; parts are merged at the end.
#[derive(Default)]
struct Acc {
    root: PathBuf,
    files: u64,
    bytes: u64,
    cats: HashMap<&'static str, (u64, u64)>,
    ages: [(u64, u64); 5],
    biggest: Vec<FolderFile>,
    clutter: Vec<FolderFile>,
}

impl Acc {
    fn add(&mut self, mut f: FolderFile, now: i64) {
        self.files += 1;
        self.bytes += f.size;

        let cat = category(Path::new(&f.path));
        let c = self.cats.entry(cat).or_default();
        c.0 += 1;
        c.1 += f.size;

        let a = &mut self.ages[age_bucket(f.modified, now)];
        a.0 += 1;
        a.1 += f.size;

        let stem = Path::new(&f.path).file_stem().and_then(|s| s.to_str()).unwrap_or("");
        let reason = if in_tool_dir(Path::new(&f.path), &self.root) {
            "" // part of a project's tooling, not clutter
        } else if cat == "Installers" {
            "installer"
        } else if is_copy_name(stem) {
            "copy"
        } else {
            ""
        };
        if !reason.is_empty() {
            let mut c = f.clone();
            c.reason = reason.to_string();
            self.clutter.push(c);
            trim(&mut self.clutter, TOP_CLUTTER);
        }

        f.reason.clear();
        self.biggest.push(f);
        trim(&mut self.biggest, TOP_FILES);
    }

    fn merge(mut self, o: Acc) -> Acc {
        self.files += o.files;
        self.bytes += o.bytes;
        for (k, (f, b)) in o.cats {
            let c = self.cats.entry(k).or_default();
            c.0 += f;
            c.1 += b;
        }
        for i in 0..5 {
            self.ages[i].0 += o.ages[i].0;
            self.ages[i].1 += o.ages[i].1;
        }
        self.biggest.extend(o.biggest);
        self.clutter.extend(o.clutter);
        trim(&mut self.biggest, TOP_FILES);
        trim(&mut self.clutter, TOP_CLUTTER);
        self
    }
}

/// Walk one top-level entry of the root: a single file, or a whole subfolder.
fn scan_child(p: &Path, root: &Path, now: i64) -> (Acc, Option<DirStat>) {
    let mut acc = Acc { root: root.to_path_buf(), ..Default::default() };
    let Ok(meta) = fs::symlink_metadata(p) else { return (acc, None) };

    if meta.is_file() {
        acc.add(file_entry(p, &meta), now);
        return (acc, None);
    }
    if !meta.is_dir() {
        return (acc, None); // symlinks and special files are skipped
    }

    let mut dir = DirStat {
        path: p.to_string_lossy().into_owned(),
        name: p.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default(),
        ..Default::default()
    };
    for e in WalkDir::new(p).follow_links(false).into_iter().filter_map(Result::ok) {
        if !e.file_type().is_file() {
            continue;
        }
        if let Ok(m) = e.metadata() {
            dir.files += 1;
            dir.bytes += m.len();
            acc.add(file_entry(e.path(), &m), now);
        }
    }
    (acc, Some(dir))
}

pub fn analyze(root: &Path) -> FolderReport {
    let start = Instant::now();
    let now = now_secs();

    let children: Vec<PathBuf> = fs::read_dir(root)
        .map(|rd| rd.filter_map(Result::ok).map(|e| e.path()).collect())
        .unwrap_or_default();

    // Each top-level entry is scanned on its own thread, then merged.
    let parts: Vec<(Acc, Option<DirStat>)> = children.par_iter().map(|p| scan_child(p, root, now)).collect();

    let mut total = Acc { root: root.to_path_buf(), ..Default::default() };
    let mut dirs = Vec::new();
    for (acc, dir) in parts {
        total = total.merge(acc);
        dirs.extend(dir);
    }
    finalize(&mut total.biggest, TOP_FILES);
    finalize(&mut total.clutter, TOP_CLUTTER);
    dirs.sort_by(|a, b| b.bytes.cmp(&a.bytes));
    dirs.truncate(TOP_FILES);

    let mut categories: Vec<CategoryStats> = total
        .cats
        .iter()
        .map(|(name, (files, bytes))| CategoryStats { name: name.to_string(), files: *files, bytes: *bytes })
        .collect();
    categories.sort_by(|a, b| b.bytes.cmp(&a.bytes));

    FolderReport {
        root: root.to_string_lossy().into_owned(),
        total_files: total.files,
        total_bytes: total.bytes,
        duration_ms: start.elapsed().as_millis() as u64,
        project_count: discovery::count(root, 3),
        categories,
        largest_files: total.biggest,
        largest_dirs: dirs,
        age_buckets: AGE_LABELS
            .iter()
            .zip(total.ages.iter())
            .map(|(l, (f, b))| AgeBucket { label: l.to_string(), files: *f, bytes: *b })
            .collect(),
        clutter: total.clutter,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn categorises_and_detects_copies() {
        assert_eq!(category(Path::new("a/Photo.JPG")), "Images");
        assert_eq!(category(Path::new("setup.exe")), "Installers");
        assert_eq!(category(Path::new("noext")), "Other");
        assert!(is_copy_name("report (1)"));
        assert!(is_copy_name("report (12)"));
        assert!(!is_copy_name("report (final)"));
        assert!(!is_copy_name("report"));
    }

    #[test]
    fn analyzes_a_folder() {
        let root = std::env::temp_dir().join(format!("devscope_folder_{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("sub")).unwrap();
        fs::write(root.join("a.jpg"), vec![0u8; 300]).unwrap();
        fs::write(root.join("setup.exe"), vec![0u8; 200]).unwrap();
        fs::write(root.join("doc (1).pdf"), vec![0u8; 50]).unwrap();
        fs::write(root.join("sub/b.mp4"), vec![0u8; 1000]).unwrap();
        // an .exe shipped inside node_modules is tooling, not a forgotten installer
        fs::create_dir_all(root.join("app/node_modules/electron")).unwrap();
        fs::write(root.join("app/node_modules/electron/electron.exe"), vec![0u8; 20]).unwrap();

        let r = analyze(&root);
        assert_eq!((r.total_files, r.total_bytes), (5, 1570));
        assert_eq!(r.clutter.len(), 2); // setup.exe and "doc (1).pdf" only
        assert_eq!(r.categories[0].name, "Video"); // largest category first
        assert_eq!(r.largest_files[0].size, 1000);
        assert_eq!(r.largest_dirs[0].name, "sub");
        assert_eq!(r.age_buckets[0].files, 5); // all just created
        let reasons: Vec<&str> = r.clutter.iter().map(|f| f.reason.as_str()).collect();
        assert!(reasons.contains(&"installer") && reasons.contains(&"copy"));
        let _ = fs::remove_dir_all(&root);
    }
}
