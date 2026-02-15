use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn walk_inputs(input: &Path, recursive: bool, glob: &str) -> Result<Vec<PathBuf>> {
    if input.is_file() {
        return Ok(vec![input.to_path_buf()]);
    }

    if !input.is_dir() {
        return Ok(vec![]);
    }

    // Minimal glob: only supports "*.xml" / "*.*" / "*suffix"
    let suffix = glob.strip_prefix('*');

    let mut out = Vec::new();
    let walker = if recursive {
        WalkDir::new(input)
    } else {
        WalkDir::new(input).max_depth(1)
    };

    for e in walker.into_iter().filter_map(|e| e.ok()) {
        if !e.file_type().is_file() {
            continue;
        }
        let p = e.path();
        if let Some(suf) = suffix {
            if suf.is_empty()
                || p.file_name()
                    .and_then(|x| x.to_str())
                    .is_some_and(|name| name.ends_with(suf))
            {
                out.push(p.to_path_buf());
            }
        } else {
            // if glob doesn't start with '*', fall back to all files
            out.push(p.to_path_buf());
        }
    }

    out.sort();
    Ok(out)
}
