mod cli;
pub mod parser;
mod render;
mod util;
mod xml;

use anyhow::{Context, Result};
use clap::Parser;
use std::path::{Path, PathBuf};

use cli::{Cli, Format};
use render::{render_compact, render_json, render_toon};
use util::{estimate_tokens_approx, walk_inputs};
use xml::{FlattenOptions, flatten_xml_file};

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    let inputs = walk_inputs(&cli.input, cli.recursive, &cli.glob)
        .with_context(|| format!("Failed to enumerate input path(s): {}", cli.input.display()))?;

    if inputs.is_empty() {
        anyhow::bail!(
            "No input files matched (input={}, glob={})",
            cli.input.display(),
            cli.glob
        );
    }

    let mut outputs: Vec<(PathBuf, String)> = Vec::with_capacity(inputs.len());

    for path in inputs {
        let out = process_one(&path, &cli)?;
        outputs.push((path, out));
    }

    let final_text = if outputs.len() == 1 && !cli.always_wrap {
        outputs.into_iter().next().unwrap().1
    } else {
        // Multi-doc wrapper (keeps format valid/usable).
        match cli.format {
            Format::Json => render_json::wrap_many(outputs, cli.json_mode, cli.minify)?,
            Format::Toon => render_toon::wrap_many(outputs),
            Format::Compact => render_compact::wrap_many(outputs),
        }
    };

    if cli.show_tokens {
        let tokens = estimate_tokens_approx(&final_text);
        eprintln!("Approx output tokens: {}", tokens);
    }

    match &cli.output {
        Some(out_path) => {
            std::fs::write(out_path, final_text)
                .with_context(|| format!("Failed to write output file: {}", out_path.display()))?;
        }
        None => {
            print!("{final_text}");
        }
    }

    Ok(())
}

fn process_one(path: &Path, cli: &Cli) -> Result<String> {
    if !path.exists() {
        anyhow::bail!("Input file does not exist: {}", path.display());
    }

    // Flatten any XML into generic (path, kind, value) entries.
    // If parsing fails, you can decide to hard-fail or skip; we hard-fail by default.
    let opts = FlattenOptions {
        include_attributes: cli.include_attributes,
        include_text: true,
        include_cdata: true,
        strip_namespace_prefix: !cli.keep_ns_prefix,
        max_text_len: cli.max_text_len,
        path_style: cli.path_style,
    };

    let flat = flatten_xml_file(path, &opts)
        .with_context(|| format!("XML flatten failed: {}", path.display()))?;

    let rendered = match cli.format {
        Format::Json => render_json::render(&flat, cli.json_mode, cli.minify)?,
        Format::Toon => render_toon::render(&flat),
        Format::Compact => render_compact::render(&flat),
    };

    if cli.show_tokens_per_file {
        let tokens = estimate_tokens_approx(&rendered);
        eprintln!("{} -> approx tokens: {}", path.display(), tokens);
    }

    Ok(rendered)
}
