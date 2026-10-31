// secrets.rs - flags possible leaked secrets in a project.
// Looks for private-key files and well-known token formats (AWS, GitHub,
// Slack, PEM keys). Matched values are never returned, only where they are.
// Heuristic, so expect some false positives; it is meant to prompt a review.

use std::fs;
use std::path::Path;

use rayon::prelude::*;

use crate::types::{ScanOptions, SecretFinding};
use crate::walker;

const MAX_FILE: u64 = 1_000_000;

/// True if `line` contains `prefix` followed by at least `min` chars satisfying `ok`.
fn token_after(line: &str, prefix: &str, min: usize, ok: impl Fn(char) -> bool) -> bool {
    line.match_indices(prefix)
        .any(|(i, _)| line[i + prefix.len()..].chars().take_while(|c| ok(*c)).count() >= min)
}

/// Classify one line; returns the kind of secret it looks like.
pub fn check_line(line: &str) -> Option<&'static str> {
    if line.contains("-----BEGIN") && line.contains("PRIVATE KEY-----") {
        Some("private-key")
    } else if token_after(line, "AKIA", 16, |c| c.is_ascii_uppercase() || c.is_ascii_digit()) {
        Some("aws-access-key")
    } else if token_after(line, "ghp_", 36, |c| c.is_ascii_alphanumeric()) {
        Some("github-token")
    } else if token_after(line, "xoxb-", 10, |c| c.is_ascii_alphanumeric() || c == '-')
        || token_after(line, "xoxp-", 10, |c| c.is_ascii_alphanumeric() || c == '-')
    {
        Some("slack-token")
    } else {
        None
    }
}

/// Files that should rarely sit in a project folder.
fn sensitive_name(path: &Path) -> Option<&'static str> {
    let name = path.file_name()?.to_str()?;
    match name {
        ".env" => Some("env-file"),
        "id_rsa" | "id_ed25519" => Some("private-key-file"),
        _ if name.ends_with(".pem") || name.ends_with(".p12") => Some("key-file"),
        _ => None,
    }
}

pub fn find_secrets(root: &Path, opts: &ScanOptions) -> Vec<SecretFinding> {
    // .gitignore is NOT honoured here: a gitignored .env still deserves a mention.
    walker::collect_all(root, opts)
        .par_iter()
        .flat_map_iter(|path| {
            let mut out = Vec::new();
            let shown = path.to_string_lossy().into_owned();
            if let Some(kind) = sensitive_name(path) {
                out.push(SecretFinding { path: shown.clone(), line: 0, kind: kind.into() });
            }
            let small = fs::metadata(path).map_or(false, |m| m.len() <= MAX_FILE);
            if let (true, Ok(bytes)) = (small, fs::read(path)) {
                if !bytes.contains(&0) {
                    // not binary
                    for (i, line) in String::from_utf8_lossy(&bytes).lines().enumerate() {
                        if let Some(kind) = check_line(line) {
                            out.push(SecretFinding { path: shown.clone(), line: i as u64 + 1, kind: kind.into() });
                        }
                    }
                }
            }
            out
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_known_formats() {
        assert_eq!(check_line("key = AKIAABCDEFGHIJKLMNOP"), Some("aws-access-key"));
        assert_eq!(check_line("-----BEGIN RSA PRIVATE KEY-----"), Some("private-key"));
        assert_eq!(check_line(&format!("t = ghp_{}", "a".repeat(36))), Some("github-token"));
        assert_eq!(check_line("let akia = 1;"), None);
        assert_eq!(check_line("AKIA short"), None);
    }
}
