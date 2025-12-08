# MalScan

MalScan is a static analysis tool designed to inspect suspicious files in a legal and non-intrusive manner. It provides indicators such as entropy scoring, potential packer detection, and simplified YARA signature scanning.

```
malscan <command> [global options] [specific options]
```

The tool follows the syntax below, where a **specific command is required** (only one per analysis), and may be combined with **global and/or specific options** to refine the inspection:

```bash
malscan analyze sample.exe --entropy --yara rules.yara --json
```

| option            | description                                | default                    |
| ----------------- | ------------------------------------------ | -------------------------- |
| `--help`          | Display help                               |                            |
| `--path <path>`   | File or directory to analyze (required)    |                            |
| `--timeout <sec>` | Maximum scan duration                      | `no-timeout`               |
| `--json`          | Output in JSON format                      |                            |
| `--yaml`          | Output in YAML format                      |                            |
| `--csv`           | Export in CSV format                       |                            |
| `--export <file>` | Export results to a file                   | `malscan-<cmd>-<date>.txt` |
| `--zip`           | Compress exported file                     |                            |

### `analyze`

Performs detailed static analysis on a single file. If no option: output general file information only (format, type, hashes).

| option                | description                                             |
| --------------------- | ------------------------------------------------------- |
| `--entropy`           | Compute entropy to detect packers / encrypted payloads  |
| `--upx`               | Basic detection of UPX-packed binaries                  |
| `--yara <rules.yara>` | Apply custom YARA signatures to find known threats      |
| `--metadata`          | Extract metadata (size, timestamps, sections, imports)  |

### `scan`

Searches for suspicious elements across multiple files. If no option: detect only files with high-risk indicators.

| option        | description                                 |
| ------------- | ------------------------------------------- |
| `--recursive` | Recursively scan directories and subfolders |
| `--flags`     | Show only suspicious files                  |