# WebScout

WebScout is a lightweight web application vulnerability scanner designed to detect common weaknesses through automated crawling, input discovery, and targeted payload testing.

```bash
webscout <url> <command> [options]
```

* If `<url>` is not mentionned or is unreachable → `--help`
* Only **one command** may be used per run
* Commands may be combined with **global options** and **command-specific options** to refine the analysis scope

## Global options

| option            | description                                           |
| ----------------- | ----------------------------------------------------- |
| `--help`          | Display help and usage examples                       |
| `--timeout <sec>` | Maximum command duration (default: `no-timeout`)      |
| `--json`          | Output in JSON format                                 |
| `--yaml`          | Output in YAML format                                 |
| `--csv`           | Output in CSV format                                  |
| `--export <file>` | Export results (default: `webscout-<cmd>-<date>.txt`) |
| `--zip`           | Compress exported output                              |

## Command: `test`

Tests the target application for common web vulnerabilities using targeted payloads and heuristics.

* If no option is specified → run a **baseline vulnerability assessment** (SQLi, XSS, CSRF, directory exposure).

| option      | description                              |
| ----------- | ---------------------------------------- |
| `--sql`     | Detect SQL injection weaknesses          |
| `--xss`     | Detect reflected XSS vulnerabilities     |
| `--csrf`    | Check for missing CSRF protections       |
| `--dirlist` | Identify directory listing exposures     |
| `--fuzz`    | Parameter fuzzing for extended detection |

## Command: `crawl`

Crawls the target to discover application structure, entry points, and navigation paths.

* If no option is specified → enumerate accessible pages and map navigation flow.

| option        | description                       |
| ------------- | --------------------------------- |
| `--depth <n>` | Maximum crawling depth            |
| `--forms`     | Include HTML form discovery       |
| `--links`     | Restrict to hyperlink enumeration |

## Notes

* WebScout performs non-intrusive testing and avoids destructive payloads by design.
* Crawling depth and fuzzing may significantly increase execution time.
* Using `--timeout` is recommended for large or dynamic applications.
* Output format options (`--json`, `--yaml`, `--csv`) are mutually exclusive.
* Scan coverage depends on authentication state and application access controls.

## Example

```bash
webscout https://target.local test --xss --sql --json
```
