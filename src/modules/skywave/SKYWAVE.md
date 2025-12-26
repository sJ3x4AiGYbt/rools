# SkyWave

SkyWave is a wireless security assessment tool designed to scan, analyze, and evaluate the security posture of nearby Wi-Fi networks, access points, and associated clients.

```bash
skywave <interface> <command> [options]
```

* If `<interface>` is not specified → system default interface is used
* If `<interface>` does not exist → `--help`
* Only **one command** may be used per run
* Commands may be combined with **global options** and **command-specific options** to refine the assessment scope

## Global options

| option                  | description                                          |
| ----------------------- | ---------------------------------------------------- |
| `--help`                | Display help and usage examples                      |
| `--channel <num>`       | Restrict scan to a specific channel                  |
| `--timeout <sec>`       | Maximum command duration (default: `no-timeout`)     |
| `--json`                | Output in JSON format                                |
| `--yaml`                | Output in YAML format                                |
| `--csv`                 | Output in CSV format                                 |
| `--export <file>`       | Export results (default: `skywave-<cmd>-<date>.txt`) |
| `--zip`                 | Compress exported output                             |

## Command: `scan`

Discovers nearby wireless networks and access points.

* If no option is specified → discover all visible and hidden access points within range.

| option           | description                              |
| ---------------- | ---------------------------------------- |
| `--ssid <name>`  | Filter results by SSID                   |
| `--bssid <mac>`  | Filter results by BSSID                  |
| `--hidden`       | Attempt discovery of hidden networks     |
| `--signal <lvl>` | Filter by minimum signal strength (RSSI) |

## Command: `audit`

Analyzes detected networks for security configuration, protocol usage, and known weaknesses.

* If no option is specified → report all detected security features and misconfigurations.

| option           | description                                     |
| ---------------- | ----------------------------------------------- |
| `--wep`          | Identify networks using WEP                     |
| `--wpa`          | Identify WPA / WPA2-secured networks            |
| `--wpa3`         | Identify WPA3-secured networks                  |
| `--default-pass` | Check for common/default credentials indicators |
| `--pmkid`        | Attempt PMKID capture for offline analysis      |

## Command: `clients`

Enumerates wireless clients observed communicating with access points.

* If no option is specified → list all detected clients in range.

| option         | description                              |
| -------------- | ---------------------------------------- |
| `--ap <bssid>` | Restrict enumeration to a specific AP    |
| `--ip-only`    | Display only IP addresses (if available) |
| `--mac-only`   | Display only MAC addresses               |

## Command: `test`

Performs **controlled wireless intrusion testing techniques**.

* This command requires explicit targets and appropriate authorization.

| option                | description                                |
| --------------------- | ------------------------------------------ |
| `--ssid <name>`       | Target SSID (required for most operations) |
| `--bssid <mac>`       | Target BSSID (recommended)                 |
| `--deauth`            | Send deauthentication frames to target AP  |
| `--evil-twin`         | Deploy a rogue access point                |
| `--handshake-capture` | Capture WPA/WPA2 authentication handshake  |
| `--wps`               | Test WPS configuration and exposure        |

## Notes

* SkyWave operates in passive mode for `scan`, `audit`, and `clients` commands.
* The `test` command performs active operations and may disrupt wireless communications.
* Wireless scan accuracy depends on interface capabilities and driver support.
* Using `--timeout` is recommended during active or continuous operations.
* Output format options (`--json`, `--yaml`, `--csv`) are mutually exclusive.

## Example

```bash
skywave wlan0 scan --hidden --json
```
