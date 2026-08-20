# WebScout

WebScout is a lightweight web application reconnaissance and vulnerability scanner. It combines automated crawling with targeted payload testing to surface common weaknesses before attackers do.

```bash
webscout <command> --url <url> [options]
```

* `--url`/`-u` is **required** on every command. The scheme is optional: `example.com` is accepted and automatically normalized to `https://example.com` (a warning is printed if the scheme was missing).
* Only **one command** may be used per run.
* Commands may be combined with **global options** and **command-specific options** to refine the analysis scope.

## Global options

| option            | description                                           |
| ----------------- | ------------------------------------------------------ |
| `--help`          | Display help and usage examples                       |
| `--timeout <sec>` | Maximum command duration (default: `no-timeout`)      |
| `--json`          | Output in JSON format                                 |
| `--yaml`          | Output in YAML format                                 |
| `--csv`           | Output in CSV format                                  |
| `--export <file>` | Export results (default: `webscout-<cmd>-<date>.txt`) |
| `--zip`           | Compress exported output                              |

## Command: `crawl`

Explores the target to discover pages, assets, technologies and potential entry points.

* In static mode (default) → analyzes only the initial page.
* In recursive mode (`--recursion`) → follows all internal links and builds a full site map. Technologies are detected from HTTP headers and HTML meta tags on every visited page.

| option              | description                                              |
| -------------------- | --------------------------------------------------------- |
| `--url <url>`, `-u`   | Target URL to crawl (required)                          |
| `--recursion`, `-r`   | Follow all internal links and map the entire site       |
| `--search`, `-s`      | Probe known sensitive paths after crawl (requires `--recursion`) |
| `--forms`, `-f`       | Detect and analyze HTML forms on visited pages           |

`--search` probes a built-in wordlist against the target root, grouped by category: `admin`, `auth`, `api`, `config`, `vcs`, `backup`, `info`, `server`, `files`. Paths already visited during recursion are skipped, soft-404s are filtered, and only 2xx/3xx responses are reported.

## Command: `test`

Performs application security analysis using targeted payloads and heuristics to detect the most common web vulnerabilities.

* If no flags are specified → runs a **baseline assessment** (every module: SQLi, XSS, CSRF, fuzzing, transport security).

| option              | description                                                                    |
| -------------------- | -------------------------------------------------------------------------------- |
| `--url <url>`, `-u`   | Target URL to test (required)                                                   |
| `--sql`, `-s`         | Check for SQL injection vulnerabilities                                         |
| `--xss`, `-x`         | Search for reflected XSS vulnerabilities                                        |
| `--csrf`, `-c`        | Verify CSRF protection mechanisms                                               |
| `--fuzz`, `-f`        | Perform fuzzing on identified parameters                                        |
| `--transport`, `-t`   | Check transport security posture (HSTS, HTTP downgrade, HTTPS support)          |
| `--body <json>`, `-b` | JSON body template to test as an API endpoint (single-page apps, REST)          |

## Notes

* `--url` accepts a bare domain or a full URL; if the scheme is guessed and HTTPS is unreachable, WebScout automatically retries over HTTP.
* `--body` targets JSON APIs (e.g. single-page apps) whose forms are rendered client-side and never appear in the raw HTML — each string field is fuzzed one at a time while the others keep their original value.
* `--search` on `crawl` requires `--recursion` and only runs after the recursive crawl completes.
* WebScout performs non-intrusive testing and avoids destructive payloads by design.
* `--timeout` bounds both the HTTP client and the whole scan — time-based SQLi checks alone can take over a minute, so a short timeout may cause later modules to be silently skipped.
* Recursive crawling may significantly increase execution time; `--timeout` is recommended for large or dynamic applications.
* Output format options (`--json`, `--yaml`, `--csv`) are mutually exclusive.
* Scan coverage depends on authentication state and application access controls.

## Examples

```bash
# Analyze a single page and detect technologies
webscout crawl --url example.com

# Recursive crawl with form detection
webscout crawl --url example.com --recursion --forms

# Full recon: crawl + sensitive-path probing + JSON export
webscout --json crawl --url example.com --recursion --search

# Run baseline security tests
webscout test --url https://example.com

# Specific SQLi and XSS testing
webscout test --url https://example.com --sql --xss

# Full audit with fuzzing and YAML export
webscout --yaml test --url https://example.com --sql --xss --csrf --fuzz

# Fuzz a JSON API endpoint
webscout test --url https://example.com/api/login --body '{"email":"a@b.com","password":"x"}'
```