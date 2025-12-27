# MemTrace

MemTrace is a lightweight user-land memory scanning tool designed to read, search, and extract data from a target process with proper access permissions.

```bash
memtrace <pid> [options]
```

* If `<pid>` is not mentionned or doesn't exist → `--help`
* By default: display target process memory layout (mapped regions, permissions, sizes) without reading or extracting memory contents.

| option                | description                                                |
| --------------------- | ---------------------------------------------------------- |
| `--help`              | Display help and a list of accessible processes (PID, name, owner), based on current user permissions |
| `--timeout <sec>`     | Maximum command duration (default: `no-timeout`)           |
| `--pattern <hex>`     | Hex string to search in memory (e.g. `90 90`)              |
| `--signature <sig>`   | Use predefined malware or process signatures               |
| `--fast`              | Reduced-scope scan for increased speed                     |
| `--range <addr-addr>` | Extract a specific memory range (e.g. `0x400000-0x500000`) |
| `--auto`              | Automatically extract memory regions matching scan results |
| `--json`              | Output in JSON format                                      |
| `--yaml`              | Output in YAML format                                      |
| `--csv`               | Output in CSV format                                       |
| `--bin`               | Output in binary format                                    |
| `--hex`               | Output in hexadecimal format                               |
| `--export <file>`     | Export results (default: `memtrace-<date>.txt`)            |
| `--zip`               | Compress exported output                                   |

## Notes

* `--export` enables memory extraction; without it, MemTrace never writes to disk.
* `--range` can be used alone to extract memory without pattern scanning.
* Combining scan options with `--auto` may significantly increase execution time.
* Output format options (`--json`, `--yaml`, `--csv`, `--hex`) are mutually exclusive.
* MemTrace operates strictly in user-land and does not attempt to bypass OS-level protections.

## Exemples 

```bash
memtrace 1234 --pattern "90 90" --export matches.bin
```

<!-- 
memtrace
| `-h, --host <ip>`       | Target host (remote execution) | Local only                  |
| `-u, --username <user>` | Username for authentication    | Current user                |
| `-p, --password <pass>` | Password for authentication    | Prompt                      | 

sentryproc
| `-h, --host <ip>`       | Target host (remote execution) | Local only                    |
| `-u, --username <user>` | Username for authentication    | Current user                  |
| `-p, --password <pass>` | Password for authentication    | Prompt                        |
-->