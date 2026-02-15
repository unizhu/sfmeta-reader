use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "sfmeta-reader")]
#[command(about = "Generic XML reducer for Salesforce metadata (works with ALL 300+ types)")]
pub struct Cli {
    /// Input file or directory
    #[arg(short, long, value_name = "PATH")]
    pub input: PathBuf,

    #[arg(short, long)]
    pub output: Option<PathBuf>,

    #[arg(short, long, default_value_t = true)]
    pub recursive: bool,

    #[arg(long, default_value = "*.xml")]
    pub glob: String,

    #[arg(short, long, value_enum, default_value_t = Format::Toon)]
    pub format: Format,

    #[arg(long)]
    pub minify: bool,

    #[arg(long, value_enum, default_value_t = JsonMode::List)]
    pub json_mode: JsonMode,

    #[arg(long)]
    pub include_attributes: bool,

    #[arg(long)]
    pub keep_ns_prefix: bool,

    #[arg(long, default_value_t = 2048)]
    pub max_text_len: usize,

    #[arg(long, value_enum, default_value_t = PathStyle::Slash)]
    pub path_style: PathStyle,

    #[arg(long)]
    pub always_wrap: bool,

    #[arg(short = 't', long)]
    pub show_tokens: bool,

    #[arg(long)]
    pub show_tokens_per_file: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Format {
    Json,
    Toon,
    Compact,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum JsonMode {
    List,
    Map,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum PathStyle {
    Slash,
    Dot,
}
