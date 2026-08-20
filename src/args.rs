use crate::commands::sysviz::common::constants::BANNER_SYSVIZ;
use crate::commands::webscout::constants::BANNER_WEBSCOUT;
use crate::global::constants::{APP_AUTHOR, APP_NAME, APP_VERSION, BANNER_ROOLS};

use clap::{ArgGroup, Args as ClapArgs, Parser, Subcommand};

#[derive(Parser)]
#[command(name = APP_NAME)]
#[command(author = APP_AUTHOR)]
#[command(version = APP_VERSION)]
#[command(before_help = BANNER_ROOLS)]
/// Rools (Rust Offensive & Observability Lightweight Suite)
#[command(
  about = "A toolkit for system observability and security auditing.",
  after_help = "Use 'rools help <command>' for more information on a specific tool.\n"
)]
pub struct Args {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(ClapArgs)]
#[command(group(
    ArgGroup::new("format")
        .args(["json", "yaml", "csv"])
        .multiple(false)
))]
pub struct GlobalOptions {
  #[arg(
    long,
    value_name = "SEC",
    help = "Stop monitoring after specified duration in seconds.",
    long_help = "Stop monitoring after specified duration in seconds.\n\n\
            By default, monitoring runs indefinitely until manually interrupted (Ctrl+C).\
        "
  )]
  pub timeout: Option<u64>,

  #[arg(
    long,
    help = "Output events in JSON format.",
    long_help = "Output events in JSON format.\n\n\
            Mutually exclusive with --yaml and --csv.\
        "
  )]
  pub json: bool,

  #[arg(
    long,
    help = "Output events in YAML format.",
    long_help = "Output events in YAML format.\n\n\
            Mutually exclusive with --json and --csv.\
        "
  )]
  pub yaml: bool,

  #[arg(
    long,
    help = "Output events in CSV format.",
    long_help = "Output events in CSV format.\n\n\
            Mutually exclusive with --json and --yaml.\
        "
  )]
  pub csv: bool,

  #[arg(
        long,
        value_name = "FILE", 
        default_missing_value = "",
        num_args = 0..=1,
        help = "Save output to the specified file.",
        long_help = "Save output to the specified file.\n\n\
            Export captured events to a file instead of (or in addition to) stdout.\n\
            If no filename is provided, generates a timestamped filename automatically\n\
            (e.g., tools-2025-01-03_14-30-00.txt). The file extension is automatically\n\
            determined by the selected format (--json, --yaml, --csv).\
        "
    )]
  pub export: Option<String>,

  #[arg(
    long,
    requires = "export",
    help = "Compress the exported file into a ZIP archive (requires --export).",
    long_help = "Compress the exported file into a ZIP archive (requires --export).\n\n\
            Automatically compress the output file after monitoring completes. The original\n\
            uncompressed file is removed and a timestamped ZIP archive is created.\
        "
  )]
  pub zip: bool,
}

#[derive(Subcommand)]
#[command(rename_all = "lowercase")]
pub enum Commands {
  #[command(
        before_help = BANNER_SYSVIZ,
        about = "Explores system calls and execution integrity.",
        long_about = "Explores system calls and execution integrity\n\n\
            SysViz is a real-time system-level monitor built for security auditing and\n\
            behavioral analysis. It hooks into the kernel event stream to capture\n\
            syscalls, process lifecycle events and execution flows as they happen.\n\n\
            On macOS it relies on eslogger (Endpoint Security), on Linux on strace\n\
            and on Windows on ETW (Event Tracing for Windows).\
        ",
        after_help = "\x1b[1;4mExamples:\x1b[0m\n  \
            # Monitor all default system events\n  \
            sysviz\n\n  \
            # Track a specific process\n  \
            sysviz --pid 1234\n\n  \
            # Monitor specific events with statistics\n  \
            sysviz --name exec --stats --top 10\n\n  \
            # Export to JSON with compression\n  \
            sysviz --json --export output.json --zip\n\n  \
            # Alert on critical operations\n  \
            sysviz --alert exec --timeout 60\
        "
    )]
  SysViz(SysVizArgs),

  #[command(
        before_help = BANNER_WEBSCOUT,
        about = "Crawls and tests web applications for common vulnerabilities.",
        long_about = "Crawls and tests web applications for common vulnerabilities\n\n\
            WebScout is a lightweight web application reconnaissance and vulnerability\n\
            scanner. It combines automated crawling with targeted payload testing to\n\
            surface common weaknesses before attackers do.\n\n\
            Two commands drive the workflow: crawl maps the attack surface by following\n\
            links, detecting technologies and collecting forms; test fires payloads\n\
            against discovered inputs to probe for SQL injection, reflected XSS, missing\n\
            CSRF protections and extended parameter anomalies via fuzzing.\
        ",
    )]
  WebScout(WebScoutArgs),
}

// SYSVIZ
#[derive(ClapArgs)]
#[command(disable_help_subcommand = true)]
pub struct SysVizArgs {
  #[arg(
    long,
    short = 'p',
    value_name = "ID",
    help = "Filter results by a specific Process ID (PID).",
    long_help = "Filter results by a specific Process ID (PID).\n\n\
            If omitted, all processes are monitored. On Linux this means strace\n\
            is attached to every PID found under /proc, which can be heavy on\n\
            busy systems; on macOS and Windows the system-wide event stream\n\
            (eslogger / ETW) is monitored and filtered afterwards.\
        "
  )]
  pub pid: Option<u32>,

  #[arg(
    long,
    short = 'n',
    value_name = "NAME",
    help = "Filter system calls by event name.",
    long_help = "Filter system calls by event name. By default, all events are monitored.\n\n\
            The accepted value depends on the platform backend:\n\n\
            - Linux (strace): one or more comma-separated syscall names\n\
            (e.g. \"open,read,execve\"), passed directly to strace -e trace=.\n\
            - macOS (eslogger): a single Endpoint Security event, e.g.\n\
            exec, exit, uipc_connect, uipc_bind, xpc_connect, open, create, rename, unlink, mmap.\n\
            - Windows (ETW): a single NT syscall name (e.g. \"NtCreateFile\"), matched\n\
            case-insensitively against the captured event after the fact.\
        "
  )]
  pub name: Option<String>,

  #[arg(
    long,
    short = 's',
    help = "Display frequency statistics for captured events.",
    long_help = "Display frequency statistics for captured events.\n\n\
            Show a summary of event counts at the end of the monitoring session.\n\
            Helps identify the most common operations and system behavior patterns.\
        "
  )]
  pub stats: bool,

  #[arg(
    long,
    short = 't',
    value_name = "N",
    help = "Limit statistics to the top N most frequent events (requires --stats)."
  )]
  pub top: Option<usize>,

  #[arg(
    long,
    short = 'a',
    value_name = "NAME",
    help = "Trigger a visual alert when a specific event is detected.",
    long_help = "Trigger a visual alert when a specific event is detected.\n\n\
            Compares each observed event name against this value for an exact\n\
            match and prints a highlighted alert line when it occurs. Works\n\
            independently of --name, so the full event stream keeps flowing\n\
            while only matching events are additionally flagged.\
        "
  )]
  pub alert: Option<String>,

  #[command(flatten)]
  pub global: GlobalOptions,
}

// WEBSCOUT
#[derive(Parser)]
pub struct WebScoutArgs {
  #[command(subcommand)]
  pub action: WebScoutCommands,
}

#[derive(Subcommand)]
#[command(disable_help_subcommand = true)]
pub enum WebScoutCommands {
  #[command(
        before_help = BANNER_WEBSCOUT,
        about = "Map the structure and attack surface of a web application.",
        long_about = "Explores the target to discover pages, assets, technologies\n\
            and potential entry points.\n\n\
            In static mode (default), analyzes only the initial page.\n\
            In recursive mode (--recursion), follows all internal links\n\
            and builds a full site map. Technologies are detected from\n\
            HTTP headers and HTML meta tags on every visited page.",
        after_help = "\x1b[1;4mExamples:\x1b[0m\n  \
            # Analyze a single page and detect technologies\n  \
            webscout crawl --url example.com\n\n  \
            # Recursive crawl with form detection\n  \
            webscout crawl --url example.com --recursion --forms\n\n  \
            # Full recon: crawl + path probing + JSON export\n  \
            webscout --json crawl --url example.com --recursion --search\n\n  \
            # Save results to file\n  \
            webscout --json --output report.json crawl --url example.com --recursion"
    )]
  Crawl {
    #[arg(
      long,
      short = 'u',
      required = true,
      value_name = "URL",
      help = "Target URL (e.g., https://example.com).",
      long_help = "Target URL to crawl.\n\n\
                The scheme is optional: example.com is accepted and automatically\n\
                normalized to https://example.com. A warning is printed if the\n\
                scheme was missing."
    )]
    url: String,

    #[arg(
      long,
      short = 'r',
      help = "Follow all internal links and map the entire site.",
      long_help = "Recursively follows every link found on each visited page,\n\
                as long as the link stays on the same host.\n\n\
                External links are recorded in the output but never followed.\n\
                The crawler also fetches sitemap.xml and robots.txt to seed\n\
                the queue with additional paths before starting.\n\n\
                Each visited page is reported with its status code, size, \n\
                response time, content type, number of links found and optionally\n\
                its forms (see --forms)."
    )]
    recursion: bool,

    #[arg(
      long,
      short = 's',
      requires = "recursion",
      help = "Probe known sensitive paths on the target (requires --recursion).",
      long_help = "After the recursive crawl completes, probes a built-in wordlist\n\
                of known paths against the target root. Categories include:\n\n\
                admin     — admin panels, dashboards, control interfaces\n\
                auth      — login, register, password reset endpoints\n\
                api       — REST, GraphQL, Swagger, OpenAPI endpoints\n\
                config    — .env, config files, phpinfo, web.config\n\
                vcs       — .git, .svn, .hg repositories\n\
                backup    — backup archives, SQL dumps\n\
                info      — robots.txt, sitemap.xml, security.txt\n\
                server    — server-status, .htaccess, nginx.conf\n\
                files     — uploads, static, assets, media directories\n\n\
                Paths already visited during recursion are skipped.\n\
                A soft-404 detection is applied to filter false positives.\n\
                Only 2xx and 3xx responses are reported."
    )]
    search: bool,

    #[arg(
      long,
      short = 'f',
      help = "Detect and analyze HTML forms on visited pages.",
      long_help = "Parses every HTML page visited and extracts all <form> elements.\n\n\
                For each form, reports:\n\
                - action URL and HTTP method (GET/POST)\n\
                - each input field: name, type and whether it is required\n\n\
                In recursion mode, the form count per page is shown in the\n\
                live progress output. All forms are aggregated in the final report."
    )]
    forms: bool,

    #[command(flatten)]
    global: GlobalOptions,
  },

  #[command(
        before_help = BANNER_WEBSCOUT,
        about = "Execute security modules to detect common web vulnerabilities.",
        long_about = "Performs application security analysis using targeted payloads\n\
            and heuristics to detect the most common web vulnerabilities. If no\n\
            flags are specified, a baseline assessment is performed.",
        after_help = "\x1b[1;4mExamples:\x1b[0m\n  \
            # Run baseline security tests\n  \
            webscout https://example.com test\n\n  \
            # Specific SQLi and XSS testing\n  \
            webscout https://example.com test --sql --xss\n\n  \
            # Full audit with fuzzing and YAML export\n  \
            webscout https://example.com --yaml test --sql --xss --csrf --fuzz"
    )]
  Test {
    #[arg(
      long,
      short = 'u',
      required = true,
      value_name = "URL",
      help = "Target URL (e.g., https://example.com).",
      long_help = "Target URL to crawl.\n\n\
                The scheme is optional: example.com is accepted and automatically\n\
                normalized to https://example.com. A warning is printed if the\n\
                scheme was missing."
    )]
    url: String,

    #[arg(
      long,
      short = 's',
      help = "Check for SQL injection vulnerabilities.",
      long_help = "Tests for SQL injection by identifying inputs that are\n\
                insufficiently protected against database query manipulation."
    )]
    sql: bool,

    #[arg(
      long,
      short = 'x',
      help = "Search for reflected XSS vulnerabilities.",
      long_help = "Identifies injection points that allow client-side script\n\
                execution through reflected Cross-Site Scripting."
    )]
    xss: bool,

    #[arg(
      long,
      short = 'c',
      help = "Verify CSRF protection mechanisms.",
      long_help = "Checks for missing or insufficient CSRF tokens on sensitive\n\
                actions to prevent Cross-Site Request Forgery attacks."
    )]
    csrf: bool,

    #[arg(
      long,
      short = 'f',
      help = "Perform fuzzing on identified parameters.",
      long_help = "Extends detection by testing unexpected or malformed inputs\n\
                against all identified application parameters."
    )]
    fuzz: bool,

    #[arg(
      long,
      short = 't',
      help = "Check the transport security posture (HTTPS/HTTP).",
      long_help = "Probes the scheme opposite to the one that answered --url:\n\
                missing Strict-Transport-Security header, plain HTTP still\n\
                serving content instead of redirecting to HTTPS, or a target\n\
                that does not support HTTPS at all."
    )]
    transport: bool,

    #[arg(
      long,
      short = 'b',
      value_name = "JSON",
      help = "JSON body template to test as an API endpoint.",
      long_help = "JSON object sent as a POST body to --url, used as a template\n\
                for injection: each string field is fuzzed one at a time while the\n\
                others keep their original value.\n\n\
                Use this for single-page apps (Angular/React/Vue) whose forms are\n\
                rendered client-side and never appear in the raw HTML, but whose\n\
                backend API accepts JSON, e.g.:\n\
                --body '{\"email\":\"a@b.com\",\"password\":\"x\"}'"
    )]
    body: Option<String>,

    #[command(flatten)]
    global: GlobalOptions,
  },
}
