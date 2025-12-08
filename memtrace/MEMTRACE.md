# MemTrace

MemTrace is a lightweight user-land memory scanning tool designed to read, search, and extract data from a target process with proper access permissions.

```
memtrace <command> [global options] [specific options]
```

The tool follows the syntax below, where a **specific command is required** (only one per analysis), and may be combined with **global and/or specific options** to refine the inspection:

```bash
memtrace scan --pid 1234 --pattern "90 90" --export memdump.txt
```

| option                  | description                    | default                     |
| ----------------------- | ------------------------------ | --------------------------- |
| `--help`                | Display help                   |                             |
| `-h, --host <ip>`       | Target host (remote execution) | Local only                  |
| `-u, --username <user>` | Username for authentication    | Current user                |
| `-p, --password <pass>` | Password for authentication    | Prompt                      |
| `--timeout <sec>`       | Maximum scan duration          | `no-timeout`                |
| `--json`                | Output in JSON format          |                             |
| `--yaml`                | Output in YAML format          |                             |
| `--csv`                 | Export as CSV format           |                             |
| `--export <file>`       | Export results                 | `memtrace-<cmd>-<date>.txt` |
| `--zip`                 | Compress exported output       |                             |

### `scan`

Searches memory for signatures or byte patterns. If no option: scan all accessible memory regions of the target process.

| option              | description                                   |
| ------------------- | --------------------------------------------- |
| `--pid <id>`        | Target process ID                             |
| `--pattern <hex>`   | Hex string to search in memory (e.g. "90 90") |
| `--signature <sig>` | Use predefined malware/process signatures     |
| `--fast`            | Reduced-scope scan for increased speed        |

### — `dump`

Extracts the raw memory content of a running process. If no option: dump full accessible memory with contextual offsets.

| option        | description                            |
| ------------- | -------------------------------------- |
| `--pid <id>`  | Target process ID                      |
| `--range <r>` | Memory range (ex: `0x400000-0x500000`) |
| `--hex`       | Export dump as raw hexadecimal format  |