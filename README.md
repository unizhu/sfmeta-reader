# sfmeta-reader

[![CI](https://github.com/unizhu/sfmeta-reader/actions/workflows/ci.yml/badge.svg)](https://github.com/unizhu/sfmeta-reader/actions/workflows/ci.yml)
[![Release](https://github.com/unizhu/sfmeta-reader/actions/workflows/release.yml/badge.svg)](https://github.com/unizhu/sfmeta-reader/actions/workflows/release.yml)

sfmeta-reader is a cross-platform CLI that reads Salesforce Metadata API XML files and converts them into smaller, LLM-friendly text representations by flattening XML into normalized path/value entries. Salesforce Metadata API includes hundreds of metadata types (Objects/Fields, Flows, Profiles, Permission Sets, Page Layouts, FlexiPages, LWC/Aura bundles, etc.), so this tool is intentionally type-agnostic and works on any metadata XML you point it at.

## Why this exists

Salesforce metadata XML is verbose (namespaces, repeated wrappers, deep nesting), which increases token usage and makes it harder for an LLM to focus on what matters. sfmeta-reader reduces that overhead by producing a compact, consistent representation that you can feed into an LLM, search/index, diff, or post-process with scripts.

## Features

- Cross-platform: Linux, macOS (Apple Silicon and Intel), Windows
- Reads a single XML file or recursively scans a directory of XML files
- **Generic XML flattening** — type-agnostic, works on any metadata XML
- **Salesforce-aware parser** — structured parsing for 15+ metadata types including `CustomObject`, `Field`, `Profile`, `PermissionSet`, `Flow`, `ApexClass`, `ApexTrigger`, `Layout`, `Report`, `Dashboard`, and more
- **Agent Skill** — publishable skill package for LLM agents (Claude Code, Cursor, etc.) with cross-platform wrapper scripts
- Multiple output formats:
  - TOON: line-oriented `path = value` (good for LLMs and diffs)
  - Compact: one-line `path=value|path=value|...` (smallest)
  - JSON: list or map output for downstream tooling
- Optional inclusion of attributes
- Text truncation controls to keep tokens bounded
- **CI/CD** — GitHub Actions workflows for lint/test and cross-platform release builds (6 targets)

## Requirements

- Rust edition 2024 (set via `edition = "2024"` in `Cargo.toml`). 

## Install

### Pre-built binaries (fastest)

Download a binary for your platform from the [GitHub Releases](https://github.com/unizhu/sfmeta-reader/releases) page:

| Platform | Binary |
|---|---|
| macOS Intel | `sfmeta-reader-darwin-x86_64` |
| macOS ARM (Apple Silicon) | `sfmeta-reader-darwin-aarch64` |
| Linux AMD64 | `sfmeta-reader-linux-x86_64` |
| Linux aarch64 | `sfmeta-reader-linux-aarch64` |
| Windows AMD64 | `sfmeta-reader-windows-x86_64.exe` |
| Windows aarch64 | `sfmeta-reader-windows-aarch64.exe` |

### From source
```bash
git clone <your-repo-url>
cd sfmeta-reader
cargo build --release
```

Binary locations:
- Linux/macOS: `./target/release/sfmeta-reader`
- Windows: `.\target\release\sfmeta-reader.exe`

### Install with Cargo
```bash
cargo install --path .
```

## Quick start

### Convert one metadata XML file to TOON
```bash
sfmeta-reader --input path/to/SomeComponent.xml --format toon > out.toon
```

### Convert to compact format
```bash
sfmeta-reader --input path/to/SomeComponent.xml --format compact > out.compact.txt
```

### Convert to JSON (lossless list mode)
```bash
sfmeta-reader --input path/to/SomeComponent.xml --format json --json-mode list > out.json
```

### Process an entire metadata directory
Example for SFDX source format:
```bash
sfmeta-reader --input force-app/main/default --recursive true --glob "*.xml" --format toon > repo.toon
```

## Output formats

### TOON (default-friendly for LLMs)
Each XML text node and attribute becomes a flattened entry:
```
/CustomObject/fields/fullName/#text = Revenue [toolpix.pythonanywhere](https://toolpix.pythonanywhere.com/blog/json-vs-xml)
/CustomObject/fields/type/#text = Currency [toolpix.pythonanywhere](https://toolpix.pythonanywhere.com/blog/json-vs-xml)
/CustomObject/fields/@xsi:type = SomeType [toolpix.pythonanywhere](https://toolpix.pythonanywhere.com/blog/json-vs-xml)
```

Notes:
- Paths include element indexes (`fields[3]`) to preserve ordering and duplicates.
- `#text` is used for text nodes; attributes are prefixed with `@`.

### Compact
Same information as TOON, but in a single line:
```
/CustomObject/fields/fullNalName/#text=Revenue|/CustomObject/fields [toolpix.pythonanywhere](https://toolpix.pythonanywhere.com/blog/json-vs-xml)/type/#text=Currency|...
```

### JSON
Two modes:
- `--json-mode list` (lossless): `[{ "p": "...", "k": "...", "v": "..." }, ...]`
- `--json-mode map` (smaller but can overwrite duplicates): `{ "/path": "value", ... }`

## Common workflows

### Feed reduced output to an LLM
1. Convert to TOON or Compact.
2. Add a short instruction prompt (what you want the LLM to do).
3. Send only the reduced representation, not the raw XML.

Example:
```bash
sfmeta-reader --input path/to/Flow.flow-meta.xml --format toon > flow.toon
```

Then prompt your LLM with something like:
- “Summarize what this Flow does. List triggers, entry conditions, and major actions.”

### Compare changes in metadata
TOON is line-oriented, which makes diffs readable:
```bash
sfmeta-reader --input before.xml --format toon > before.toon
sfmeta-reader --input after.xml  --format toon > after.toon
diff -u before.toon after.toon
```

## CLI options (high-level)

- `--input <PATH>`: file or directory
- `--output <FILE>`: write to file instead of stdout
- `--recursive <true|false>`: recurse directories
- `--glob "<pattern>"`: file match when input is a directory (example: `"*.xml"`)
- `--format <json|toon|compact>`
- `--minify`: minify JSON output
- `--json-mode <list|map>`
- `--include-attributes`: include XML attributes as entries
- `--keep-ns-prefix`: do not strip namespace prefixes from tag names
- `--max-text-len <N>`: truncate long text nodes (0 = unlimited)
- `--path-style <slash|dot>`: path separator choice
- `--show-tokens`: print approximate token count for final output
- `--show-tokens-per-file`: print per-file token estimates when processing directories

## Agent Skill (for LLM agents)

sfmeta-reader ships as a publishable **Agent Skill** that LLM coding assistants (Claude Code, Cursor, etc.) can discover and use automatically.

### Skill folder structure

```text
skills/sfmeta-reader/
├── SKILL.md              # Skill definition (YAML frontmatter + instructions)
├── scripts/
│   ├── run.sh            # Unix wrapper (auto-detects OS/arch)
│   └── run.ps1           # Windows PowerShell wrapper
├── resources/
│   └── reference.md      # CLI reference + Salesforce metadata cheat sheet
└── bin/                  # Pre-built binaries (populated by CI release)
```

### How it works

1. The LLM reads `SKILL.md` and learns when and how to invoke sfmeta-reader.
2. Wrapper scripts detect the OS and architecture, then run the correct binary from `bin/`.
3. The LLM analyzes the TOON/Compact/JSON output to summarize, audit, or compare metadata.

### Install the skill (one-liner)

**macOS / Linux:**
```bash
curl -fsSL https://raw.githubusercontent.com/unizhu/sfmeta-reader/main/install.sh | bash
```

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/unizhu/sfmeta-reader/main/install.ps1 | iex
```

This will:
1. Detect your OS and architecture automatically
2. Download the correct binary from the latest GitHub Release
3. Install the full skill package to `~/.claude/skills/sfmeta-reader/`

To install to a custom directory, set `SFMETA_INSTALL_DIR` before running:
```bash
SFMETA_INSTALL_DIR=~/my-skills/sfmeta-reader curl -fsSL https://raw.githubusercontent.com/unizhu/sfmeta-reader/main/install.sh | bash
```

## Project layout

```text
sfmeta-reader/
├── Cargo.toml
├── .github/workflows/
│   ├── ci.yml                  # Lint, test, build on push/PR
│   └── release.yml             # Cross-platform release (6 targets)
├── skills/sfmeta-reader/       # Agent Skill package
│   ├── SKILL.md
│   ├── scripts/
│   └── resources/
└── src/
    ├── main.rs
    ├── cli.rs
    ├── lib.rs
    ├── parser/                 # Salesforce-aware structured parser
    │   ├── mod.rs              # SalesforceMetadata root struct + re-exports
    │   ├── parse.rs            # parse_salesforce_xml() function
    │   ├── format.rs           # to_toon_format(), to_compact_format()
    │   └── types/              # Metadata type definitions by category
    │       ├── mod.rs
    │       ├── object.rs       # Field, ValidationRule, Workflow, RecordType
    │       ├── security.rs     # Profile, PermissionSet, ObjectPermission, etc.
    │       ├── ui.rs           # Layout, LayoutSection, Tab
    │       ├── automation.rs   # Flow, ApexClass, ApexTrigger
    │       └── analytics.rs    # Report, Dashboard, LightningComponent, EmailTemplate
    ├── xml/                    # Generic XML flattening engine
    │   ├── mod.rs
    │   ├── flatten.rs
    │   └── options.rs
    ├── render/
    │   ├── mod.rs
    │   ├── json.rs
    │   ├── toon.rs
    │   └── compact.rs
    └── util/
        ├── mod.rs
        ├── fs_walk.rs
        └── token.rs
```

## Limitations and notes

- The **generic XML flattener** (`xml/`) is type-agnostic and reduces verbosity without interpreting Salesforce semantics.
- The **Salesforce-aware parser** (`parser/`) provides structured parsing for common metadata types but does not yet cover all 300+ Salesforce metadata types. New types can be added by creating a struct in the appropriate `parser/types/` submodule.
- `--json-mode map` is not lossless if multiple nodes share the same flattened path; use `list` mode for lossless output.

## License

MIT (or update this section to match your repository).