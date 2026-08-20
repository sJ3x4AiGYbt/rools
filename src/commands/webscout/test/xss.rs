use std::collections::HashSet;

use crate::commands::webscout::common::constants::MODULE_XSS;
use crate::commands::webscout::common::http;
use crate::commands::webscout::crawl::parser;
use crate::commands::webscout::test::probe;

const PAYLOADS: &[&str] = &[
  "<script>alert('xss')</script>",
  "\"><script>alert('xss')</script>",
  "'><script>alert('xss')</script>",
  "\"><img src=x onerror=alert('xss')>",
  "<svg onload=alert('xss')>",
  "javascript:alert('xss')",
  "<script>alert(1)</script>",
  "\"><script>alert(1)</script>",
  "'><script>alert(1)</script>",
  "\"><img src=x onerror=alert(1)>",
  "<svg onload=alert(1)>",
  "javascript:alert(1)",
];

const INDICATORS: &[(&str, &str)] = &[
  ("<script>alert", "<script>alert"),
  ("\"><script>", "\"><script>"),
  ("'><script>", "'><script>"),
  ("onerror=alert", "onerror=alert"),
  ("<svg onload=alert", "<svg onload=alert"),
  ("javascript:alert", "javascript:alert"),
];

pub fn run(
  forms: &[parser::FormInfo],
  url: &str,
  client: &http::HttpClient,
  json_body: Option<&serde_json::Value>,
) -> probe::ModuleReport {
  let mut findings = Vec::new();
  let mut tested = Vec::new();
  let mut seen: HashSet<(String, String, String)> = HashSet::new();

  for payload in PAYLOADS {
    for result in probe::probe_url_params(client, url, payload) {
      if let Some(f) = check(&result)
        && seen.insert((f.url.clone(), f.param.clone(), f.payload.clone()))
      {
        findings.push(f);
      }
      tested.push(probe::TestedItem {
        url: result.url.clone(),
        param: result.param.clone(),
        payload: result.payload.clone(),
      });
    }

    for form in forms {
      for result in probe::probe_form(client, form, payload) {
        if let Some(f) = check(&result)
          && seen.insert((f.url.clone(), f.param.clone(), f.payload.clone()))
        {
          findings.push(f);
        }
        tested.push(probe::TestedItem {
          url: result.url.clone(),
          param: result.param.clone(),
          payload: result.payload.clone(),
        });
      }
    }

    if let Some(template) = json_body {
      for result in probe::probe_json_body(client, url, template, payload) {
        if let Some(f) = check(&result)
          && seen.insert((f.url.clone(), f.param.clone(), f.payload.clone()))
        {
          findings.push(f);
        }
        tested.push(probe::TestedItem {
          url: result.url.clone(),
          param: result.param.clone(),
          payload: result.payload.clone(),
        });
      }
    }
  }

  probe::ModuleReport { findings, tested }
}

fn check(result: &probe::ProbeResult) -> Option<probe::Finding> {
  let body = &result.response.body;

  let evidence = INDICATORS
    .iter()
    .find(|(indicator, _)| body.contains(indicator))
    .map(|(_, label)| *label)?;

  Some(probe::Finding {
    module: MODULE_XSS,
    url: result.url.clone(),
    param: result.param.clone(),
    payload: result.payload.clone(),
    evidence: extract_context(body, evidence),
  })
}

fn extract_context(body: &str, indicator: &str) -> String {
  let pos = match body.find(indicator) {
    Some(p) => p,
    None => return indicator.to_string(),
  };
  let start = pos.saturating_sub(40);
  let end = (pos + indicator.len() + 40).min(body.len());

  let before = body[start..pos].replace('\n', " ");
  let matched = &body[pos..pos + indicator.len()];
  let after = body[pos + indicator.len()..end].replace('\n', " ");

  format!("{}»{}«{}", before.trim_start(), matched, after.trim_end())
}
