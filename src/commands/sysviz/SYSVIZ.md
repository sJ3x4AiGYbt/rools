# SysViz

SysViz is a real-time system-level monitor built for security auditing and behavioral analysis. It hooks into the kernel event stream to capture syscalls, process lifecycle events and execution flows as they happen.

* On macOS it relies on `eslogger` (Endpoint Security).
* On Linux it relies on `strace`.
* On Windows it relies on ETW (Event Tracing for Windows).

```bash
sysviz [options]
```

* By default: displays a live global event stream — timestamp, event name, PID, responsible PID, user and process name for every captured event.

| option            | description                                              |
| ----------------- | ---------------------------------------------------------- |
| `--help`          | Display help                                             |
| `--timeout <sec>` | Maximum command duration (default: `no-timeout`)         |
| `--pid <id>`      | Filter by a specific Process ID                          |
| `--name <s>`      | Filter by event/syscall name (platform-dependent format) |
| `--stats`         | Display total event count and frequency                  |
| `--top <n>`       | Show top N most frequent events (requires `--stats`)      |
| `--alert <s>`     | Trigger a visual alert when a specific event is observed  |
| `--json`          | Output in JSON format                                    |
| `--yaml`          | Output in YAML format                                    |
| `--csv`           | Output in CSV format                                     |
| `--export <file>` | Export results (default: `sysviz-<date>.txt`)            |
| `--zip`           | Compress exported output                                 |

## Platform-specific behavior

SysViz uses a different backend per OS, so `--pid` and `--name` behave differently:

| platform | backend    | `--pid` when omitted                                    | `--name` accepted value                                                                 |
| -------- | ---------- | -------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| Linux    | `strace`   | Attaches to every PID found under `/proc` (can be heavy on busy systems) | One or more comma-separated syscall names (e.g. `open,read,execve`) |
| macOS    | `eslogger` | Monitors the full system-wide Endpoint Security stream, filtered per-event afterwards | A single Endpoint Security event, e.g. `exec`, `exit`, `uipc_connect`, `uipc_bind`, `xpc_connect`, `open`, `create`, `rename`, `unlink`, `mmap` |
| Windows  | ETW        | Monitors the full system-wide ETW syscall stream, filtered per-event afterwards | A single NT syscall name (e.g. `NtCreateFile`), matched case-insensitively |

Each captured event reports: timestamp, event/syscall name, PID, responsible PID (the process that triggered the action — notably meaningful on macOS), user, and process name.

## Notes

* SysViz runs in live monitoring mode; without `--timeout`, execution continues until manually interrupted (Ctrl+C).
* Using `--timeout` is strongly recommended for automated analysis or scripting.
* `--alert` works independently of `--name`: the full event stream keeps flowing while matching events are additionally highlighted.
* Root / Administrator privileges are required on Linux and macOS. On Windows, ETW session creation typically also requires Administrator rights, even though this isn't currently enforced by the tool itself before starting.
* Combining `--stats` or `--top` with a high-volume event stream may introduce additional overhead.
* Output format options (`--json`, `--yaml`, `--csv`) are mutually exclusive.
* Syscall visibility and granularity depend on OS capabilities and permissions.

## Examples

```bash
# Monitor all default system events
sysviz

# Track a specific process
sysviz --pid 1234

# Linux: monitor several syscalls at once, with statistics
sysviz --name "open,read,execve" --stats --top 10

# macOS: monitor a single Endpoint Security event
sysviz --name exec --stats --top 10

# Windows: monitor a single NT syscall
sysviz --name NtCreateFile

# Export to JSON with compression
sysviz --json --export output.json --zip

# Alert on a critical operation
sysviz --alert exec --timeout 60
```