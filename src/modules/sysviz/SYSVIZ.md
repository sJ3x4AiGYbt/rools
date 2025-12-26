# SysViz

SysViz is a system call inspection and monitoring tool designed to analyze syscalls executed by a process, their frequency, latency, and real-time behavioral characteristics.

```bash
sysviz [options]
```

* By default: display a live global syscall stream with basic information (syscall name, PID, timestamp).

| option            | description                                       |
| ----------------- | ------------------------------------------------- |
| `--help`          | Display help                                      |
| `--timeout <sec>` | Maximum command duration (default: `no-timeout`)  |
| `--pid <id>`      | Filter by specific PID                            |
| `--name <s>`      | Filter by syscall name                            |
| `--stats`         | Display total syscall count and frequency         |
| `--latency`       | Measure average execution latency per syscall     |
| `--top <n>`       | Show top N most frequent syscalls                 |
| `--alert <s>`     | Trigger alert when a specific syscall is observed |
| `--json`          | Output in JSON format                             |
| `--yaml`          | Output in YAML format                             |
| `--csv`           | Output in CSV format                              |
| `--export <file>` | Export results (default: `sysviz-<date>.txt`)     |
| `--zip`           | Compress exported output                          |

## Notes 

* SysViz runs in live monitoring mode; without `--timeout`, execution continues until manually interrupted.
* Using `--timeout` is strongly recommended for automated analysis or scripting.
* Combining `--latency`, `--stats`, or `--top` may introduce additional overhead.
* Output format options (`--json`, `--yaml`, `--csv`) are mutually exclusive.
* Syscall visibility and granularity depend on OS capabilities and permissions.

## Exemples 

```bash
sysviz --pid 1234 --stats --json --timeout 30
```