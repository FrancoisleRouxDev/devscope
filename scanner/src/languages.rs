// languages.rs - language detection table.
// Maps file extensions / filenames to a language name and its comment
// syntax (line + block markers), used by the counter.
// (Example logic: a handful of languages; the real table will be much larger.)

use std::path::Path;

pub struct Language {
    pub name: &'static str,
    pub extensions: &'static [&'static str],
    pub line_comments: &'static [&'static str],
    pub block_comment: Option<(&'static str, &'static str)>,
}

const C_STYLE: Option<(&str, &str)> = Some(("/*", "*/"));

pub static LANGUAGES: &[Language] = &[
    Language { name: "Rust", extensions: &["rs"], line_comments: &["//"], block_comment: C_STYLE },
    Language { name: "Python", extensions: &["py"], line_comments: &["#"], block_comment: None },
    Language { name: "JavaScript", extensions: &["js", "mjs", "cjs"], line_comments: &["//"], block_comment: C_STYLE },
    Language { name: "TypeScript", extensions: &["ts", "tsx"], line_comments: &["//"], block_comment: C_STYLE },
    Language { name: "Svelte", extensions: &["svelte"], line_comments: &["//"], block_comment: Some(("<!--", "-->")) },
    Language { name: "HTML", extensions: &["html", "htm"], line_comments: &[], block_comment: Some(("<!--", "-->")) },
    Language { name: "CSS", extensions: &["css"], line_comments: &[], block_comment: C_STYLE },
    Language { name: "TOML", extensions: &["toml"], line_comments: &["#"], block_comment: None },
    Language { name: "Markdown", extensions: &["md"], line_comments: &[], block_comment: None },
];

/// Look up a language by file extension (case-insensitive). None = not source we track.
pub fn detect(path: &Path) -> Option<&'static Language> {
    let ext = path.extension()?.to_str()?.to_ascii_lowercase();
    LANGUAGES.iter().find(|l| l.extensions.contains(&ext.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_by_extension() {
        assert_eq!(detect(Path::new("src/lib.rs")).unwrap().name, "Rust");
        assert_eq!(detect(Path::new("App.SVELTE")).unwrap().name, "Svelte");
        assert!(detect(Path::new("image.png")).is_none());
    }
}
