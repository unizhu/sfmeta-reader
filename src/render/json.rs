use crate::cli::JsonMode;
use crate::xml::FlatEntry;
use anyhow::Result;
use serde_json::{Map, Value, json};
use std::path::PathBuf;

pub fn render(entries: &[FlatEntry], mode: JsonMode, minify: bool) -> Result<String> {
    let v = match mode {
        JsonMode::List => serde_json::to_value(entries)?,
        JsonMode::Map => {
            // last-write-wins if duplicate paths; use List if you need lossless output
            let mut m = Map::new();
            for e in entries {
                m.insert(e.p.clone(), Value::String(e.v.clone()));
            }
            Value::Object(m)
        }
    };

    Ok(if minify {
        serde_json::to_string(&v)?
    } else {
        serde_json::to_string_pretty(&v)?
    })
}

pub fn wrap_many(docs: Vec<(PathBuf, String)>, mode: JsonMode, minify: bool) -> Result<String> {
    // docs already rendered as strings; but for JSON wrapper we re-embed as JSON values.
    // If you want fully structured, call flatten+render per file into Value instead.
    let mut arr = Vec::with_capacity(docs.len());
    for (path, body) in docs {
        arr.push(json!({
            "file": path.to_string_lossy(),
            "data": match mode {
                JsonMode::List => serde_json::from_str::<Value>(&body).unwrap_or(Value::String(body)),
                JsonMode::Map => serde_json::from_str::<Value>(&body).unwrap_or(Value::String(body)),
            }
        }));
    }

    Ok(if minify {
        serde_json::to_string(&arr)?
    } else {
        serde_json::to_string_pretty(&arr)?
    })
}
