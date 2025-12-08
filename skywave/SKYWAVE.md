# SkyWave

SkyWave is a Wi-Fi security assessment tool used to scan, analyze, and test the security of wireless networks, detecting vulnerable access points, weak configurations, and connected clients.

```
skywave <command> [global options] [specific options]
```

The tool follows the syntax below, where a **specific command is required** (only one per analysis), and may be combined with **global and/or specific options** to refine the inspection:

```bash
skywave scan -i wlan0 --hidden --json
```

| option                  | description              | default                    |
| ----------------------- | ------------------------ | -------------------------- |
| `--help`                | Display help             |                            |
| `-i, --interface <int>` | Network interface to use | System default             |
| `--channel <num>`       | Scan a specific channel  | All available              |
| `--timeout <sec>`       | Maximum scan duration    | `no-timeout`               |
| `--json`                | Output in JSON format    |                            |
| `--yaml`                | Output in YAML format    |                            |
| `--csv`                 | Export as CSV format     |                            |
| `--export <file>`       | Export results           | `skywave-<cmd>-<date>.txt` |
| `--zip`                 | Compress exported output |                            |

### `scan`

Detects nearby Wi-Fi networks. If no option: discover all visible and hidden access points within range.

| option           | description               |
| ---------------- | ------------------------- |
| `--ssid <name>`  | Filter by SSID            |
| `--hidden`       | Detect hidden networks    |
| `--bssid <mac>`  | Filter by BSSID           |
| `--signal <lvl>` | Filter by signal strength |

### `audit`

Analyzes wireless security protocols and vulnerabilities. If no option: report all detected security features/weaknesses.

| option           | description                                 |
| ---------------- | ------------------------------------------- |
| `--wep`          | Detect WEP networks                         |
| `--wpa`          | Detect WPA/WPA2 networks                    |
| `--wpa3`         | Detect WPA3 networks                        |
| `--default-pass` | Test for default credentials                |
| `--pmkid`        | Attempt PMKID extraction for offline attack |

### `clients`

Enumerates clients connected to access points. If no option: list all visible clients around the interface.

| option         | description             |
| -------------- | ----------------------- |
| `--ap <bssid>` | Target specific AP      |
| `--ip-only`    | Show only IP addresses  |
| `--mac-only`   | Show only MAC addresses |

### `test`

Performs Wi-Fi intrusion testing techniques (offensive operations). If no option: requires at least a SSID or BSSID target.

| option                | description                                      |
| --------------------- | ------------------------------------------------ |
| `--deauth <bssid>`    | Send deauthentication frames to target AP        |
| `--evil-twin`         | Clone and broadcast a rogue network              |
| `--handshake-capture` | Capture WPA/WPA2 handshake                       |
| `--wps`               | Test WPS security (bruteforce, info disclosure…) |
| `--ssid <name>`       | Target SSID (required for multiple operations)   |
| `--bssid <mac>`       | Target BSSID (recommended)                       |