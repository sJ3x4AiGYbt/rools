use crate::commands::webscout::common::constants::{
  EVIDENCE_CSRF_NO_SAMESITE, EVIDENCE_CSRF_NO_SECURE, EVIDENCE_CSRF_NO_TOKEN,
  EVIDENCE_CSRF_NO_TOKEN_SUFFIX, MODULE_CSRF,
};
use crate::commands::webscout::common::http;
use crate::commands::webscout::crawl::parser;
use crate::commands::webscout::test::probe;

const TOKEN_FIELD_NAMES: &[&str] = &[
  "csrf_token",
  "_token",
  "authenticity_token",
  "__requestverificationtoken",
  "csrf",
  "_csrf",
  "xsrf_token",
  "csrfmiddlewaretoken",
  "_csrf_token",
  "form_token",
];

const CSRF_HEADERS: &[&str] = &["x-csrf-token", "x-xsrf-token", "x-frame-options"];

pub fn run(
  forms: &[parser::FormInfo],
  url: &str,
  response: &http::HttpResponse,
) -> probe::ModuleReport {
  let mut findings = Vec::new();
  let mut tested = Vec::new();

  let has_csrf_header = CSRF_HEADERS
    .iter()
    .any(|h| response.headers.contains_key(*h));

  let post_forms: Vec<&parser::FormInfo> = forms.iter().filter(|f| f.method == "POST").collect();

  for form in &post_forms {
    let has_token = form.fields.iter().any(|field| {
      let name_lower = field.name.to_lowercase();
      TOKEN_FIELD_NAMES.iter().any(|t| name_lower.contains(t))
    });

    tested.push(probe::TestedItem {
      url: form.action.clone(),
      param: "(form)".to_string(),
      payload: String::new(),
    });

    if !has_token && !has_csrf_header {
      findings.push(probe::Finding {
        module: MODULE_CSRF,
        url: form.action.clone(),
        param: "(form)".to_string(),
        payload: String::new(),
        evidence: format!(
          "{}{} {}",
          EVIDENCE_CSRF_NO_TOKEN,
          form.fields.len(),
          EVIDENCE_CSRF_NO_TOKEN_SUFFIX
        ),
      });
    }
  }

  if let Some(cookie) = response.headers.get("set-cookie") {
    let lower = cookie.to_lowercase();
    let has_samesite = lower.contains("samesite=strict") || lower.contains("samesite=lax");

    if !post_forms.is_empty() {
      tested.push(probe::TestedItem {
        url: url.to_string(),
        param: "set-cookie".to_string(),
        payload: String::new(),
      });

      if !has_samesite {
        findings.push(probe::Finding {
          module: MODULE_CSRF,
          url: url.to_string(),
          param: "set-cookie".to_string(),
          payload: String::new(),
          evidence: format!("{EVIDENCE_CSRF_NO_SAMESITE}{cookie}"),
        });
      }
    }

    if url.starts_with("https://") {
      tested.push(probe::TestedItem {
        url: url.to_string(),
        param: "set-cookie".to_string(),
        payload: String::new(),
      });

      if !lower.contains("secure") {
        findings.push(probe::Finding {
          module: MODULE_CSRF,
          url: url.to_string(),
          param: "set-cookie".to_string(),
          payload: String::new(),
          evidence: format!("{EVIDENCE_CSRF_NO_SECURE}{cookie}"),
        });
      }
    }
  }

  probe::ModuleReport { findings, tested }
}
