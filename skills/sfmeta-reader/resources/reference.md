# sfmeta-reader CLI Reference

## Synopsis

```
sfmeta-reader [OPTIONS]
```

## Options

| Option | Type | Default | Description |
|---|---|---|---|
| `--input <PATH>` | Path | *(required)* | File or directory to process |
| `--output <FILE>` | Path | stdout | Write output to file instead of stdout |
| `--format <FORMAT>` | Enum | `toon` | Output format: `toon`, `compact`, or `json` |
| `--recursive <BOOL>` | Bool | `false` | Recurse into subdirectories |
| `--glob <PATTERN>` | String | `*.xml` | Glob filter when processing directories |
| `--json-mode <MODE>` | Enum | `list` | JSON sub-mode: `list` (lossless) or `map` (compact) |
| `--minify` | Flag | — | Minify JSON output |
| `--include-attributes` | Flag | — | Include XML attributes in output |
| `--keep-ns-prefix` | Flag | — | Preserve namespace prefixes on tag names |
| `--max-text-len <N>` | Integer | `0` | Truncate text nodes longer than N chars (0 = unlimited) |
| `--path-style <STYLE>` | Enum | `slash` | Path separator: `slash` or `dot` |
| `--show-tokens` | Flag | — | Print approximate token count for output |
| `--show-tokens-per-file` | Flag | — | Print per-file token estimates |

## Output Formats

### TOON (Text Object-Oriented Notation)

Line-oriented `path = value` format. Best for LLM analysis and `diff`.

```
/CustomObject/fields[0]/fullName/#text = AccountNumber
/CustomObject/fields[0]/type/#text = Text
/CustomObject/fields[0]/required/#text = false
/CustomObject/fields[1]/fullName/#text = Revenue
/CustomObject/fields[1]/type/#text = Currency
```

### Compact

Single-line `path=value|path=value|...` for minimum tokens.

### JSON

Structured output in two modes:

- **list** (lossless): `[{ "p": "/path", "k": "key", "v": "value" }, ...]`
- **map** (compact): `{ "/path/key": "value", ... }` — may overwrite duplicates

## Salesforce Metadata Types Cheat Sheet

| Metadata Type | Common File Suffix | Key Elements |
|---|---|---|
| Custom Object | `.object-meta.xml` | `fields`, `validationRules`, `recordTypes`, `listViews` |
| Profile | `.profile-meta.xml` | `objectPermissions`, `fieldPermissions`, `applicationVisibilities` |
| Permission Set | `.permissionset-meta.xml` | `objectPermissions`, `fieldPermissions`, `userPermissions` |
| Flow | `.flow-meta.xml` | `processType`, `start`, `decisions`, `recordCreates` |
| Apex Class | `.cls-meta.xml` | `apiVersion`, `status` |
| Apex Trigger | `.trigger-meta.xml` | `apiVersion`, `status` |
| Layout | `.layout-meta.xml` | `layoutSections`, `layoutItems` |
| Validation Rule | (inside object) | `fullName`, `active`, `errorConditionFormula` |
| Workflow Rule | `.workflow-meta.xml` | `rules`, `alerts`, `fieldUpdates` |
| Lightning Component | `.js-meta.xml` | `apiVersion`, `isExposed`, `targets` |
| Report | `.report-meta.xml` | `reportType`, `columns`, `filters` |
| Dashboard | `.dashboard-meta.xml` | `dashboardType`, `components` |

## Exit Codes

| Code | Meaning |
|---|---|
| `0` | Success |
| `1` | Error (invalid input, file not found, XML parse failure) |
