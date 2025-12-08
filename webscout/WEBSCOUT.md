# WebScout

WebScout is a lightweight web application vulnerability scanner designed to detect common weaknesses by performing automated crawling and payload fuzzing.

```
webscout <command> [global options] [specific options]
```

The tool follows the syntax below, where a **specific command is required** (only one per analysis), and may be combined with **global and/or specific options** to refine the inspection:

```bash
webscout test --url https://target.local --xss --sql --json
```

| option            | description              | default                     |
| ----------------- | ------------------------ | --------------------------- |
| `--help`          | Display help             |                             |
| `--url <target>`  | Base target URL          | Required                    |
| `--timeout <sec>` | Maximum request duration | `no-timeout`                |
| `--json`          | Output in JSON format    |                             |
| `--yaml`          | Output in YAML format    |                             |
| `--csv`           | Export as CSV format     |                             |
| `--export <file>` | Export results           | `webscout-<cmd>-<date>.txt` |
| `--zip`           | Compress exported output |                             |

### `test`

Tests the target for common web vulnerabilities. If no option: run crawler first, then basic vulnerability checks.

| option      | description                              |
| ----------- | ---------------------------------------- |
| `--sql`     | Detect SQL injection weaknesses          |
| `--xss`     | Detect reflected XSS vulnerabilities     |
| `--csrf`    | Check for missing CSRF protections       |
| `--dirlist` | Identify directory listing exposures     |
| `--fuzz`    | Parameter fuzzing for extended detection |

### `crawl`

Crawls the target to discover structure and user input areas. If no option: enumerate accessible pages and map navigation flow.

| option        | description                       |
| ------------- | --------------------------------- |
| `--depth <n>` | Max crawling depth                |
| `--forms`     | Include HTML form discovery       |
| `--links`     | Restrict to hyperlink enumeration |