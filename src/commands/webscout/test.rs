pub mod csrf;
pub mod formatter;
pub mod fuzzing;
pub mod probe;
pub mod sqli;
pub mod transport;
pub mod xss;

use crate::commands::webscout::common::config::{self as cfg, WebScoutRunner};
use crate::commands::webscout::common::constants::{
  ERROR_HTTP_CLIENT, ERROR_INVALID_URL, ERROR_TEST_INVALID_JSON, EVIDENCE_NOT_VULNERABLE,
  FIELD_ERROR, FIELD_INPUT, FIELD_ROOT, MODULE_CSRF, MODULE_FUZZ, MODULE_SQLI, MODULE_TRANSPORT,
  MODULE_XSS, WARN_HTTPS_FALLBACK, WARN_HTTPS_FALLBACK_MID, WARN_MISSING_SCHEME,
};
use crate::commands::webscout::common::http;
use crate::commands::webscout::common::url;
use crate::commands::webscout::crawl::parser;
use crate::global::export;
use crate::global::format;
use crate::global::shutdown as shutdown_mod;
use crate::global::writer as writer_mod;

pub struct TestRunner;

impl TestRunner {
  pub fn new() -> Self {
    Self
  }
}

impl WebScoutRunner for TestRunner {
  fn run(&mut self, config: &cfg::WebScoutConfig) {
    let test = match &config.action {
      cfg::WebScoutAction::Test(c) => c,
      _ => return,
    };

    let shutdown = shutdown_mod::Shutdown::new(config.timeout);
    let writer = writer_mod::OutputWriter::new(config.export_path.clone(), true);

    let mut parsed = match url::parse_and_fix(&test.url) {
      Ok(p) => p,
      Err(e) => {
        eprintln!("{ERROR_INVALID_URL}{e}");
        return;
      }
    };

    writer.write(format_args!("[{}]   {}", FIELD_INPUT, test.url));

    if parsed.was_fixed {
      writer.write(format_args!("{}{}", WARN_MISSING_SCHEME, parsed.normalized));
    }

    writer.write(format_args!("[{}]    {}", FIELD_ROOT, parsed.root));

    let timeout = config.timeout.unwrap_or(10);
    let client = match http::HttpClient::new(timeout, false) {
      Ok(c) => c,
      Err(e) => {
        eprintln!("{ERROR_HTTP_CLIENT}{e}");
        return;
      }
    };

    let res = match client.get(&parsed.normalized) {
      Ok(r) => r,
      Err(e) if parsed.was_fixed && parsed.normalized.starts_with("https://") => {
        let fallback = parsed.normalized.replacen("https://", "http://", 1);
        writer.write(format_args!(
          "{WARN_HTTPS_FALLBACK} ({e}), {WARN_HTTPS_FALLBACK_MID}: {fallback}"
        ));
        match client.get(&fallback) {
          Ok(r) => {
            parsed.normalized = fallback;
            parsed.root = parsed.root.replacen("https://", "http://", 1);
            r
          }
          Err(e2) => {
            eprintln!("[{FIELD_ERROR}]   {e2}");
            return;
          }
        }
      }
      Err(e) => {
        eprintln!("[{FIELD_ERROR}]   {e}");
        return;
      }
    };

    let forms = parser::parse_forms(&res.body, &parsed.normalized);
    let fmt = get_formatter(config);

    let json_body: Option<serde_json::Value> = match &test.body {
      Some(raw) => match serde_json::from_str(raw) {
        Ok(v) => Some(v),
        Err(e) => {
          eprintln!("{ERROR_TEST_INVALID_JSON}{e}");
          return;
        }
      },
      None => None,
    };

    fmt.header(&parsed.normalized, forms.len());

    let run_all = !test.sql && !test.xss && !test.csrf && !test.fuzz && !test.transport;

    if (run_all || test.sql) && shutdown.is_running() {
      fmt.scan_start(MODULE_SQLI);
      let report = sqli::run(&forms, &parsed.normalized, &client, json_body.as_ref());
      if report.findings.is_empty() {
        if report.tested.is_empty() {
          fmt.no_findings();
        } else {
          for t in &report.tested {
            fmt.finding(&probe::Finding {
              module: MODULE_SQLI,
              url: t.url.clone(),
              param: t.param.clone(),
              payload: t.payload.clone(),
              evidence: EVIDENCE_NOT_VULNERABLE.to_string(),
            });
          }
        }
      }
      for f in &report.findings {
        fmt.finding(f);
      }
    }

    if (run_all || test.xss) && shutdown.is_running() {
      fmt.scan_start(MODULE_XSS);
      let report = xss::run(&forms, &parsed.normalized, &client, json_body.as_ref());
      if report.findings.is_empty() {
        if report.tested.is_empty() {
          fmt.no_findings();
        } else {
          for t in &report.tested {
            fmt.finding(&probe::Finding {
              module: MODULE_XSS,
              url: t.url.clone(),
              param: t.param.clone(),
              payload: t.payload.clone(),
              evidence: EVIDENCE_NOT_VULNERABLE.to_string(),
            });
          }
        }
      }
      for f in &report.findings {
        fmt.finding(f);
      }
    }

    if (run_all || test.csrf) && shutdown.is_running() {
      fmt.scan_start(MODULE_CSRF);
      let report = csrf::run(&forms, &parsed.normalized, &res);
      if report.findings.is_empty() {
        if report.tested.is_empty() {
          fmt.no_findings();
        } else {
          for t in &report.tested {
            fmt.finding(&probe::Finding {
              module: MODULE_CSRF,
              url: t.url.clone(),
              param: t.param.clone(),
              payload: t.payload.clone(),
              evidence: EVIDENCE_NOT_VULNERABLE.to_string(),
            });
          }
        }
      }
      for f in &report.findings {
        fmt.finding(f);
      }
    }

    if (run_all || test.fuzz) && shutdown.is_running() {
      fmt.scan_start(MODULE_FUZZ);
      let report = fuzzing::run(
        &forms,
        &parsed.normalized,
        &client,
        &res.body,
        json_body.as_ref(),
      );
      if report.findings.is_empty() {
        if report.tested.is_empty() {
          fmt.no_findings();
        } else {
          for t in &report.tested {
            fmt.finding(&probe::Finding {
              module: MODULE_FUZZ,
              url: t.url.clone(),
              param: t.param.clone(),
              payload: t.payload.clone(),
              evidence: EVIDENCE_NOT_VULNERABLE.to_string(),
            });
          }
        }
      }
      for f in &report.findings {
        fmt.finding(f);
      }
    }

    if (run_all || test.transport) && shutdown.is_running() {
      fmt.scan_start(MODULE_TRANSPORT);
      let report = transport::run(&parsed.normalized, &client, &res);
      if report.findings.is_empty() {
        if report.tested.is_empty() {
          fmt.no_findings();
        } else {
          for t in &report.tested {
            fmt.finding(&probe::Finding {
              module: MODULE_TRANSPORT,
              url: t.url.clone(),
              param: t.param.clone(),
              payload: t.payload.clone(),
              evidence: EVIDENCE_NOT_VULNERABLE.to_string(),
            });
          }
        }
      }
      for f in &report.findings {
        fmt.finding(f);
      }
    }

    fmt.footer();
    export::Export::finalize(&config.export_path, config.zip).ok();
  }
}

fn get_formatter(config: &cfg::WebScoutConfig) -> Box<dyn formatter::Formatter> {
  let writer = writer_mod::OutputWriter::new(config.export_path.clone(), true);
  match config.output_format {
    format::OutputFormat::Json => Box::new(formatter::Json::new(writer)),
    format::OutputFormat::Csv => Box::new(formatter::Csv::new(writer)),
    format::OutputFormat::Yaml => Box::new(formatter::Yaml { writer }),
    format::OutputFormat::Txt => Box::new(formatter::Txt { writer }),
  }
}
