# SentryProc

SentryProc is a system auditing tool designed to monitor active processes, inspect their permissions, and detect anomalies or suspicious behavior on a target host.

```
sentryproc <command> [global options] [specific options]
```

The tool follows the syntax below, where a **specific command is required** (only one per analysis), and may be combined with **global and/or specific options** to refine the inspection:

```bash
sentryproc list --local --running --json
```

| option                  | description                    | default                       |
| ----------------------- | ------------------------------ | ----------------------------- |
| `--help`                | Display help                   |                               |
| `-h, --host <ip>`       | Target host (remote execution) | Local only                    |
| `-u, --username <user>` | Username for authentication    | Current user                  |
| `-p, --password <pass>` | Password for authentication    | Prompt                        |
| `--local`               | Audit the local system         | Auto if no host specified     |
| `--timeout <sec>`       | Maximum request duration       | `no-timeout`                  |
| `--json`                | Output in JSON format          |                               |
| `--yaml`                | Output in YAML format          |                               |
| `--csv`                 | Export as CSV format           |                               |
| `--export <file>`       | Export results                 | `sentryproc-<cmd>-<date>.txt` |
| `--zip`                 | Compress exported output       |                               |

### `list`

Enumerates processes on the target system. If no option: list all processes with basic metadata.

| option          | description                     |
| --------------- | ------------------------------- |
| `--running`     | Only active/running processes   |
| `--user <name>` | Filter by user owner            |
| `--pid <id>`    | Filter by specific PID          |
| `--service`     | Only system services/background |

### `perms`

Analyzes permissions and privilege levels of processes. If no option: display all permission-related data.

| option           | description                                     |
| ---------------- | ----------------------------------------------- |
| `--sensitive`    | Only high-privileged or security-critical procs |
| `--owner <user>` | Filter by owner                                 |
| `--file-access`  | Track file usage by listed processes            |

### `suspicious`

Detects anomalous or malicious process behavior. If no option: run global anomaly detection.

| option       | description                                  |
| ------------ | -------------------------------------------- |
| `--unsigned` | Identify unsigned/unverified executables     |
| `--hidden`   | Detect hidden or masked processes            |
| `--high-cpu` | Spot excessive CPU-usage patterns            |
| `--network`  | Detect unauthorized network-active processes |