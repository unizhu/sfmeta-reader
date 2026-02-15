use crate::cli::PathStyle;
use crate::xml::FlattenOptions;
use anyhow::{Context, Result};
use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlatEntry {
    pub p: String,    // path
    pub k: ValueKind, // kind
    pub v: String,    // value
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ValueKind {
    Text,
    Attr,
}

#[derive(Debug)]
struct Frame {
    name: String,
    idx: usize,
}

pub fn flatten_xml_file(path: &Path, opts: &FlattenOptions) -> Result<Vec<FlatEntry>> {
    // Read full file (simple and cross-platform); switch to streaming-bytes if you want
    // extremely large XML files.
    let mut f = BufReader::new(File::open(path)?);
    let mut s = String::new();
    f.read_to_string(&mut s)
        .with_context(|| format!("Failed reading file as UTF-8 text: {}", path.display()))?;

    flatten_xml_str(&s, opts).with_context(|| format!("Failed flattening: {}", path.display()))
}

pub fn flatten_xml_str(xml: &str, opts: &FlattenOptions) -> Result<Vec<FlatEntry>> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut out: Vec<FlatEntry> = Vec::new();
    let mut buf: Vec<u8> = Vec::new();

    let mut stack: Vec<Frame> = Vec::new();
    let mut child_counts: Vec<HashMap<String, usize>> = Vec::new();
    child_counts.push(HashMap::new()); // root-level counts

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                push_element(&mut stack, &mut child_counts, &e, opts)?;
                if opts.include_attributes {
                    push_attrs(&mut out, &stack, &e, opts, &reader)?;
                }
            }
            Ok(Event::Empty(e)) => {
                push_element(&mut stack, &mut child_counts, &e, opts)?;
                if opts.include_attributes {
                    push_attrs(&mut out, &stack, &e, opts, &reader)?;
                }
                // Pop immediately (empty element)
                stack.pop();
                child_counts.pop();
            }
            Ok(Event::Text(e)) => {
                if opts.include_text {
                    let mut text = e.decode()?.to_string();
                    text = normalize_text(text, opts.max_text_len);
                    if !text.is_empty() {
                        out.push(FlatEntry {
                            p: make_path(&stack, opts.path_style, Some("#text")),
                            k: ValueKind::Text,
                            v: text,
                        });
                    }
                }
            }
            Ok(Event::CData(e)) => {
                if opts.include_cdata {
                    let mut text = String::from_utf8_lossy(e.as_ref()).to_string();
                    text = normalize_text(text, opts.max_text_len);
                    if !text.is_empty() {
                        out.push(FlatEntry {
                            p: make_path(&stack, opts.path_style, Some("#cdata")),
                            k: ValueKind::Text,
                            v: text,
                        });
                    }
                }
            }
            Ok(Event::End(_e)) => {
                stack.pop();
                child_counts.pop();
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => {
                return Err(anyhow::anyhow!(
                    "XML parse error at position {}: {}",
                    reader.error_position(),
                    err
                ));
            }
        }
        buf.clear();
    }

    Ok(out)
}

fn push_element(
    stack: &mut Vec<Frame>,
    child_counts: &mut Vec<HashMap<String, usize>>,
    e: &BytesStart<'_>,
    opts: &FlattenOptions,
) -> Result<()> {
    let binding = e.name();
    let raw = binding.as_ref();
    let mut name = String::from_utf8_lossy(raw).to_string();

    if opts.strip_namespace_prefix
        && let Some(i) = name.rfind(':')
    {
        name = name[i + 1..].to_string();
    }

    let parent_map = child_counts
        .last_mut()
        .expect("child_counts always has at least one frame");
    let idx = parent_map.entry(name.clone()).or_insert(0usize);
    let current_idx = *idx;
    *idx += 1;

    stack.push(Frame {
        name,
        idx: current_idx,
    });
    child_counts.push(HashMap::new());
    Ok(())
}

fn push_attrs(
    out: &mut Vec<FlatEntry>,
    stack: &[Frame],
    e: &BytesStart<'_>,
    opts: &FlattenOptions,
    reader: &Reader<&[u8]>,
) -> Result<()> {
    for a in e.attributes().with_checks(false) {
        let a = a?;
        let key = String::from_utf8_lossy(a.key.as_ref()).to_string();
        let val = a.decode_and_unescape_value(reader.decoder())?.to_string();
        let val = normalize_text(val, opts.max_text_len);

        if val.is_empty() {
            continue;
        }

        out.push(FlatEntry {
            p: make_path(stack, opts.path_style, Some(&format!("@{key}"))),
            k: ValueKind::Attr,
            v: val,
        });
    }
    Ok(())
}

fn make_path(stack: &[Frame], style: PathStyle, leaf: Option<&str>) -> String {
    let sep = match style {
        PathStyle::Slash => "/",
        PathStyle::Dot => ".",
    };

    let mut s = String::new();
    for (i, f) in stack.iter().enumerate() {
        if i == 0 {
            // avoid leading '.' but keep leading '/'
            if style == PathStyle::Slash {
                s.push('/');
            }
        } else {
            s.push_str(sep);
        }
        s.push_str(&f.name);
        s.push('[');
        s.push_str(&f.idx.to_string());
        s.push(']');
    }

    if let Some(leaf) = leaf {
        s.push_str(sep);
        s.push_str(leaf);
    }

    s
}

fn normalize_text(mut s: String, max_len: usize) -> String {
    // trim already handled by quick-xml config, but keep safe
    s = s.trim().to_string();
    if s.is_empty() {
        return s;
    }
    if max_len > 0 && s.len() > max_len {
        s.truncate(max_len);
    }
    s
}
