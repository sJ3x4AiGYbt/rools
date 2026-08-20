# rools

**R**ust **O**ffensive & **O**bservability **L**ightweight **S**uite — a cross-platform (Linux, macOS, Windows) security and observability toolkit written in Rust.

## Details

- Bundles two independent tools: **SysViz** (kernel-level syscall/process monitor) and **WebScout** (web recon + vulnerability scanner).
- SysViz backend is OS-specific: `strace` (Linux), `eslogger`/Endpoint Security (macOS), ETW via FFI (Windows).
- WebScout `crawl` maps the attack surface: internal links, HTML forms, technology detection from headers/meta tags.
- WebScout `test` fires targeted payloads to probe for SQL injection, reflected XSS, missing CSRF protection and parameter fuzzing.
- Results print to stdout or export to JSON/YAML/CSV, optionally zipped.
- SysViz requires elevated privileges (root on Linux/macOS, administrator on Windows) to access the kernel event stream.
- Requirements:
  - **Linux**: GCC, `pkg-config`, `libssl` (dev headers)
  - **macOS**: Xcode Command Line Tools
  - **Windows**: MSYS2 (GNU toolchain, `gcc`/`binutils`)

## Usage

Prebuilt binaries are available on the [Releases](https://github.com/sJ3x4AiGYbt/rools/releases) page.

Unified syntax: `rools <tool> <command> [options]`

Full help for each command: `rools <tool> --help`.

## Limitations

- Only Linux, macOS and Windows are supported — other targets fail to compile.
- **SysViz (Linux)** parses `strace`'s text output, which varies subtly across `strace` versions/flags — the parser has only been validated by static review, not by running it end-to-end on Linux. If output looks empty or misattributed, please open an issue with your `strace --version`.
- SysViz without `--pid` on Linux attaches `strace` to every PID under `/proc`, which can be heavy on busy systems and noisy (kernel threads can't be ptrace-attached and fail silently).
- SysViz requires the `strace` binary to be installed on Linux — not bundled, not always present by default on minimal distros.
- WebScout does not render JavaScript — HTML is parsed statically, so links/forms from SPAs are only reachable by testing the backend API directly via `test --body`.
- `crawl --recursion` only follows same-host links; external links are recorded but never followed.
- `webscout test` only fetches the single `--url` given — it does not crawl the site itself, so pages discovered by `crawl --recursion` are not automatically fed into `test`; each URL of interest must be tested manually.
- `webscout test` findings are **heuristic, not proof of exploitability** — expect both false positives and false negatives:
  - SQLi boolean-based relies on response-length diffing; SQLi time-based relies on artificial delay detection (`SLEEP`) and is sensitive to network jitter.
  - XSS only checks for raw payload reflection in the HTML body, not actual script execution — a reflection in a non-executable context (JSON, comment, escaped elsewhere) can still be flagged.
  - CSRF is a static heuristic (token field name, cookie flags) — it does not verify the server actually validates the token.
  - Fuzzing's error/boundary/overflow checks rely on HTTP 5xx status with no control request, so unrelated server errors can be misattributed.
  - Always review findings manually before treating them as confirmed vulnerabilities.
- `--json`, `--yaml` and `--csv` are mutually exclusive; only one output file per run.
