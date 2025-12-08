# NetWatch

NetWatch is a network inspection and traffic analysis tool designed to capture packets, scan ports, and detect suspicious communications on a target host or network.

```
netwatch <command> [global options] [specific options]
```

The tool follows the syntax below, where a **specific command is required** (only one per analysis), and may be combined with **global and/or specific options** to refine the inspection:

```bash
netwatch capture -i eth0 --pcap --timeout 30
```

| option                  | description                      | default                     |
| ----------------------- | -------------------------------- | --------------------------- |
| `--help`                | Display help                     |                             |
| `-i, --interface <int>` | Network interface to analyze     | System default interface    |
| `--target <ip>`         | Target host (for scan/detection) | Local network               |
| `--timeout <sec>`       | Maximum scan duration            | `no-timeout`                |
| `--json`                | Output in JSON format            |                             |
| `--yaml`                | Output in YAML format            |                             |
| `--csv`                 | Export as CSV format             |                             |
| `--export <file>`       | Export results                   | `netwatch-<cmd>-<date>.txt` |
| `--zip`                 | Compress exported output         |                             |

### `capture`

Captures and inspects live network traffic. If no option: real-time display of packets on the selected interface.

| option      | description                                  |
| ----------- | -------------------------------------------- |
| `--pcap`    | Save raw traffic in PCAP format              |
| `--filter`  | BPF filter (port, protocol, host, etc.)      |
| `--verbose` | Display detailed information for each packet |

### `scan`

Performs TCP or UDP port scanning. If no option: fast TCP scan on common service ports.

| option        | description                      |
| ------------- | -------------------------------- |
| `--udp`       | Enable UDP scan                  |
| `--range <r>` | Port range to scan (e.g. 1-1024) |
| `--stealth`   | Stealth TCP scan (SYN only)      |
| `--top <n>`   | Scan top n most used ports       |

### `detect`

Identifies suspicious or abnormal network behavior. If no option: global anomaly detection on active traffic.

| option          | description                                          |
| --------------- | ---------------------------------------------------- |
| `--outbound`    | Highlight unusual outbound connections               |
| `--frequency`   | Alert on excessive traffic on a port/service         |
| `--new-service` | Detect newly opened services or unexpected listeners |
| `--dns-leak`    | Check outbound DNS queries for leak indicators       |