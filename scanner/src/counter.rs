// counter.rs - per-file line counting.
// Classifies each line as code / comment / blank and returns totals for
// one file. Must be fast and tolerant of binary or non-UTF-8 files.
// (Example logic: simple line-prefix classifier; ignores comment markers
// inside string literals, which the real version should handle.)

use std::fs;
use std::path::Path;

use crate::languages::Language;
use crate::types::FileStats;

/// Count lines in `path`. Returns None if the file can't be read.
pub fn count_file(path: &Path, lang: &Language) -> Option<FileStats> {
    let bytes = fs::read(path).ok()?;
    // Lossy decode so non-UTF-8 files don't abort the scan.
    let text = String::from_utf8_lossy(&bytes);

    let mut stats = count_text(&text, lang);
    stats.path = path.to_string_lossy().into_owned();
    stats.language = lang.name.to_string();
    Some(stats)
}

/// Classify every line of `text`. Split out from `count_file` so it can be unit-tested.
pub fn count_text(text: &str, lang: &Language) -> FileStats {
    let mut stats = FileStats::default();
    let mut in_block = false;

    for line in text.lines() {
        let line = line.trim();

        if line.is_empty() {
            stats.blanks += 1;
        } else if in_block {
            stats.comments += 1;
            if let Some((_, end)) = lang.block_comment {
                if line.contains(end) {
                    in_block = false;
                }
            }
        } else if lang.line_comments.iter().any(|m| line.starts_with(m)) {
            stats.comments += 1;
        } else if let Some((start, end)) = lang.block_comment.filter(|(s, _)| line.starts_with(s)) {
            stats.comments += 1;
            // Block continues unless it also closes on the same line.
            in_block = !line[start.len()..].contains(end);
        } else {
            stats.code += 1;
        }
    }
    stats
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::languages::LANGUAGES;

    #[test]
    fn counts_rust_lines() {
        let rust = &LANGUAGES[0];
        let src = "// hi\nfn main() {}\n\n/* a\nb */\nlet x = 1;\n";
        let s = count_text(src, rust);
        assert_eq!((s.code, s.comments, s.blanks), (2, 3, 1));
    }
}
