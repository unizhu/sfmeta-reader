---
name: sfmeta-reader
description: Read and analyze Salesforce Metadata API XML files by converting them into compact, LLM-friendly text (TOON, Compact, or JSON). Use this skill when the user asks you to analyze, summarize, compare, or understand Salesforce metadata files such as Custom Objects, Profiles, Permission Sets, Flows, Layouts, Apex classes, or any other Salesforce XML metadata. Prefer to use TOON.
---

# sfmeta-reader

A CLI tool that converts verbose Salesforce metadata XML into compact, token-efficient representations for analysis.

## When to Use

Use sfmeta-reader whenever you need to:
- **Analyze** Salesforce metadata XML files (objects, fields, profiles, flows, etc.)
- **Summarize** what a metadata component does
- **Compare** two versions of metadata (before/after)
- **Audit** permission sets, profiles, or security settings
- **Review** flow logic, validation rules, or field configurations

## Quick Start

Run the wrapper script from this skill's directory:

```bash
# Unix (macOS / Linux)
./scripts/run.sh --input <PATH_TO_XML> --format toon

# Windows (PowerShell)
.\scripts\run.ps1 --input <PATH_TO_XML> --format toon
```

## Output Format Selection

Choose the format based on your task:

| Format | Flag | Best For |
|---|---|---|
| **TOON** | `--format toon` | General analysis, summaries, readable output (default) |
| **Compact** | `--format compact` | Minimal tokens, embedding, batch processing |
| **JSON** | `--format json` | Structured downstream processing, programmatic use |

## Common Workflows

### 1. Analyze a Single Metadata File
```bash
./scripts/run.sh --input path/to/Account.object-meta.xml --format toon
```
Then analyze the output to summarize fields, validation rules, record types, etc.

### 2. Scan an Entire SFDX Project
```bash
./scripts/run.sh --input force-app/main/default --recursive true --glob "*.xml" --format toon
```

### 3. Compare Metadata Changes
```bash
./scripts/run.sh --input before.xml --format toon > /tmp/before.toon
./scripts/run.sh --input after.xml  --format toon > /tmp/after.toon
diff -u /tmp/before.toon /tmp/after.toon
```

### 4. Audit Permissions
```bash
./scripts/run.sh --input path/to/Admin.profile-meta.xml --format toon
```
Review the output for object permissions, field permissions, and application visibilities.

### 5. Token Budget Estimation
```bash
./scripts/run.sh --input path/to/file.xml --format toon --show-tokens
```

## Key CLI Options

| Option | Description |
|---|---|
| `--input <PATH>` | File or directory to process |
| `--format <toon\|compact\|json>` | Output format |
| `--recursive true` | Recurse into directories |
| `--glob "<pattern>"` | File filter when scanning directories (e.g. `"*.xml"`) |
| `--json-mode <list\|map>` | JSON sub-mode (`list` = lossless, `map` = smaller) |
| `--include-attributes` | Include XML attributes in output |
| `--max-text-len <N>` | Truncate long text nodes (0 = unlimited) |
| `--show-tokens` | Print approximate token count |

For the full CLI reference, see [resources/reference.md](resources/reference.md).
