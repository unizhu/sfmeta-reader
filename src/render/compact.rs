use crate::xml::FlatEntry;
use std::path::PathBuf;

pub fn render(entries: &[FlatEntry]) -> String {
    // One-liner: p=v|p=v|...
    // Minimal escaping to stay machine-parseable.
    let mut parts: Vec<String> = Vec::with_capacity(entries.len());
    for e in entries {
        parts.push(format!("{}={}", e.p, esc(&e.v)));
    }
    parts.join("|")
}

pub fn wrap_many(docs: Vec<(PathBuf, String)>) -> String {
    // Still compact, but keep doc boundaries.
    let mut parts = Vec::with_capacity(docs.len());
    for (path, body) in docs {
        parts.push(format!("file={};{}", esc(&path.to_string_lossy()), body));
    }
    parts.join("\n")
}

fn esc(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('|', "\\|")
        .replace('=', "\\=")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}
