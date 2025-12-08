# SysViz

SysViz is a system call inspection and monitoring tool designed to analyze syscalls executed by a process, their frequency, latency, and real-time behavioral characteristics.

```
sysviz <command> [global options] [specific options]
```

The tool follows the syntax below, where a **specific command is required** (only one per analysis), and may be combined with **global and/or specific options** to refine the inspection:

```bash
sysviz trace --pid 1234 --live --stats --json
```

| option            | description                  | default                       |
| ----------------- | ---------------------------- | ----------------------------- |
| `--help`          | Display help                 |                               |
| `--pid <id>`      | PID of the target process    | Required for process analysis |
| `--timeout <sec>` | Maximum duration of analysis | `no-timeout`                  |
| `--json`          | Output in JSON format        |                               |
| `--yaml`          | Output in YAML format        |                               |
| `--csv`           | Export as CSV format         |                               |
| `--export <file>` | Export results               | `sysviz-<cmd>-<date>.txt`     |
| `--zip`           | Compress exported output     |                               |

### `trace`

Analyzes system calls executed by a process. If no option: list all detected syscalls with basic information.

| option         | description                                   |
| -------------- | --------------------------------------------- |
| `--stats`      | Display total syscall count and frequency     |
| `--latency`    | Measure average execution latency per syscall |
| `--live`       | Real-time mode — continuous syscall tracking  |
| `--filter <s>` | Filter by syscall name or pattern             |

### `monitor`

Monitors system-wide syscall activity in real time. If no option: display global syscall stream.

| option        | description                                       |
| ------------- | ------------------------------------------------- |
| `--top <n>`   | Show top N most frequent syscalls                 |
| `--proc`      | Show PID of the calling process                   |
| `--alert <s>` | Trigger alert when a specific syscall is observed |