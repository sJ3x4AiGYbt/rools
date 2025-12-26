# MalScan

MalScan is a static analysis tool designed to inspect suspicious files in a legal and non-intrusive manner. It provides indicators such as entropy scoring, potential packer detection, and simplified YARA signature scanning.

```bash
malscan <path> [options]
```

* If `<path>` is not mentionned or doesn't exist → `--help`
* By default: File → performs a detailed analysis
* By default: Directory → performs a scan across multiple files

| option                | description                                                    |
| --------------------- | -------------------------------------------------------------- |
| `--help`              | Display help, current working directory, and a contextual directory overview. For root paths (/ or drive root), shows a summarized directory tree with high-level descriptions to help select a scan target |
| `--timeout <sec>`     | Maximum command duration (default: `no-timeout`)               |
| `--entropy`           | Compute entropy to detect packers / encrypted payloads         |
| `--upx`               | Basic detection of UPX-packed binaries                         |
| `--yara <rules.yara>` | Apply custom YARA signatures to find known threats             |
| `--metadata`          | Extract metadata (size, timestamps, sections, imports)         |
| `--deep`              | Force deep analysis on all files (recommended for directories) |
| `--recursive`         | Recursively scan directories and subfolders                    |
| `--flags`             | Show only files with suspicious indicators (for directories)   |
| `--json`              | Output in JSON format                                          |
| `--yaml`              | Output in YAML format                                          |
| `--csv`               | Output in CSV format                                           |
| `--export <file>`     | Export results (default: `malscan-<date>.txt`)                 |
| `--zip`               | Compress exported output                                       |

## Notes

* Combining `--deep` with `--recursive` may significantly increase scan time.
* When scanning directories without `--deep`, MalScan focuses on lightweight indicators.
* Output format options (`--json`, `--yaml`, `--csv`) are mutually exclusive.

## Exemples 

```bash
malscan sample.exe --entropy --yara rules.yara --json
```