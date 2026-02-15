use crate::xml::FlatEntry;
use std::path::PathBuf;

pub fn render(entries: &[FlatEntry]) -> String {
    // TOON: path = value (one per line)
    // This is intentionally generic and complete (all elements/attrs present in entries).
    let mut s = String::new();
    for e in entries {
        s.push_str(&e.p);
        s.push_str(" = ");
        s.push_str(&escape_value(&e.v));
        s.push('\n');
    }
    s
}

pub fn wrap_many(docs: Vec<(PathBuf, String)>) -> String {
    let mut s = String::new();
    for (i, (path, body)) in docs.into_iter().enumerate() {
        if i > 0 {
            s.push('\n');
        }
        s.push_str("### file: ");
        s.push_str(&path.to_string_lossy());
        s.push('\n');
        s.push_str(&body);
    }
    s
}

fn escape_value(v: &str) -> String {
    // Keep it readable; avoid adding many extra tokens.
    v.replace('\n', "\\n").replace('\r', "\\r")
}
