# NetWatch

NetWatch is a network inspection and traffic analysis tool designed to capture packets, perform port scanning, and identify suspicious or abnormal network activity.

```bash
netwatch <interface> <command> [options]
```

* If `<interface>` is not specified → system default interface is used
* If `<interface>` does not exist → `--help`
* Only **one command** may be used per run
* Commands may be combined with **global options** and **command-specific options** to refine the inspection scope

## Global options

| option            | description                                           |
| ----------------- | ----------------------------------------------------- |
| `--help`          | Display help and list available network interfaces    |
| `--timeout <sec>` | Maximum command duration (default: `no-timeout`)      |
| `--json`          | Output in JSON format                                 |
| `--yaml`          | Output in YAML format                                 |
| `--csv`           | Output in CSV format                                  |
| `--export <file>` | Export results (default: `netwatch-<cmd>-<date>.txt`) |
| `--zip`           | Compress exported output                              |

## Command: `capture`

Captures live network traffic on the selected interface and optionally applies real-time behavioral detection.

* If no option is specified → display live traffic with basic packet metadata.

| option          | description                                          |
| --------------- | ---------------------------------------------------- |
| `--pcap`        | Save raw traffic in PCAP format                      |
| `--filter`      | Apply BPF filter (port, protocol, host, etc)         |
| `--verbose`     | Display detailed packet information                  |
| `--outbound`    | Highlight unusual outbound connections               |
| `--frequency`   | Alert on excessive traffic on a port or service      |
| `--new-service` | Detect newly opened listeners or unexpected services |
| `--dns-leak`    | Detect suspicious or abnormal DNS queries            |

## Command: `scan`

Performs **active** TCP or UDP port scanning against the local or observed network.

* If no option is specified → perform a fast TCP scan on common service ports.

| option        | description                      |
| ------------- | -------------------------------- |
| `--udp`       | Enable UDP scan                  |
| `--range <r>` | Port range to scan (e.g. 1-1024) |
| `--stealth`   | Stealth TCP scan (SYN only)      |
| `--top <n>`   | Scan top N most used ports       |

## Notes

* NetWatch operates in passive mode when using `capture` and does not alter network traffic.
* Detection options extend live capture with real-time behavioral analysis.
* Live capture runs indefinitely unless interrupted or limited with `--timeout`.
* Active scanning (`scan`) may generate detectable network traffic.
* Packet visibility and capture capabilities depend on OS-level permissions.
* Output format options (`--json`, `--yaml`, `--csv`) are mutually exclusive.

## Example

```bash
netwatch eth0 capture --pcap --timeout 30
```