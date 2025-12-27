# SentryProc

SentryProc is a system auditing tool designed to monitor active processes, inspect their permissions, and detect anomalies or suspicious behavior.

```bash
sentryproc [options]
```

* By default: list all detected processes with basic metadata (PID, name, owner, privilege level, execution path).

| option            | description                                       |
| ----------------- | ------------------------------------------------- |
| `--help`          | Display help                                      |
| `--timeout <sec>` | Maximum command duration (default: `no-timeout`)  |
| `--user <name>`   | Filter by user owner                              |
| `--pid <id>`      | Filter by specific PID                            |
| `--running`       | Only active/running processes                     |
| `--service`       | Only system services/background                   |
| `--sensitive`     | Only high-privileged or security-critical procs   |
| `--file-access`   | Track file usage by listed processes              |
| `--unsigned`      | Identify unsigned/unverified executables          |
| `--hidden`        | Detect hidden or masked processes                 |
| `--high-cpu`      | Spot excessive CPU-usage patterns                 |
| `--network`       | Detect unauthorized network-active processes      |
| `--json`          | Output in JSON format                             |
| `--yaml`          | Output in YAML format                             |
| `--csv`           | Output in CSV format                              |
| `--export <file>` | Export results (default: `sentryproc-<date>.txt`) |
| `--zip`           | Compress exported output                          |

## Notes 
* SentryProc performs passive inspection and does not interfere with running processes.
* Some detection features (unsigned, hidden, network) rely on OS-specific capabilities.
* Using multiple behavioral filters may increase execution time.
* Output format options (`--json`, `--yaml`, `--csv`) are mutually exclusive.
* Results reflect the system state at scan time and may change rapidly.

## Exemples 

```bash
sentryproc --sensitive --unsigned --network --json
```

