use crate::commands::webscout::common::constants::{
  EVIDENCE_TRANSPORT_NO_HSTS, EVIDENCE_TRANSPORT_NO_HTTPS, EVIDENCE_TRANSPORT_NO_REDIRECT,
  EVIDENCE_TRANSPORT_NO_REDIRECT_SUFFIX, MODULE_TRANSPORT,
};
use crate::commands::webscout::common::http;
use crate::commands::webscout::test::probe;

pub fn run(url: &str, client: &http::HttpClient, res: &http::HttpResponse) -> probe::ModuleReport {
  let mut findings = Vec::new();
  let mut tested = Vec::new();

  if let Some(http_url) = url.strip_prefix("https://") {
    check_https_target(
      url,
      client,
      res,
      &format!("http://{http_url}"),
      &mut findings,
      &mut tested,
    );
  } else if let Some(https_host) = url.strip_prefix("http://") {
    check_http_target(
      url,
      client,
      &format!("https://{https_host}"),
      &mut findings,
      &mut tested,
    );
  }

  probe::ModuleReport { findings, tested }
}

fn check_https_target(
  url: &str,
  client: &http::HttpClient,
  res: &http::HttpResponse,
  http_url: &str,
  findings: &mut Vec<probe::Finding>,
  tested: &mut Vec<probe::TestedItem>,
) {
  tested.push(probe::TestedItem {
    url: url.to_string(),
    param: "hsts".to_string(),
    payload: String::new(),
  });

  if !res.headers.contains_key("strict-transport-security") {
    findings.push(probe::Finding {
      module: MODULE_TRANSPORT,
      url: url.to_string(),
      param: "hsts".to_string(),
      payload: String::new(),
      evidence: EVIDENCE_TRANSPORT_NO_HSTS.to_string(),
    });
  }

  let Ok(http_res) = client.get_no_redirect(http_url) else {
    return;
  };

  tested.push(probe::TestedItem {
    url: http_url.to_string(),
    param: "downgrade".to_string(),
    payload: String::new(),
  });

  let redirects_to_https = (300..400).contains(&http_res.status)
    && http_res
      .headers
      .get("location")
      .map(|loc| loc.starts_with("https://"))
      .unwrap_or(false);

  if !redirects_to_https {
    findings.push(probe::Finding {
      module: MODULE_TRANSPORT,
      url: http_url.to_string(),
      param: "downgrade".to_string(),
      payload: String::new(),
      evidence: format!(
        "{}{}) {}",
        EVIDENCE_TRANSPORT_NO_REDIRECT, http_res.status, EVIDENCE_TRANSPORT_NO_REDIRECT_SUFFIX
      ),
    });
  }
}

fn check_http_target(
  url: &str,
  client: &http::HttpClient,
  https_url: &str,
  findings: &mut Vec<probe::Finding>,
  tested: &mut Vec<probe::TestedItem>,
) {
  tested.push(probe::TestedItem {
    url: https_url.to_string(),
    param: "no-https".to_string(),
    payload: String::new(),
  });

  if client.get(https_url).is_err() {
    findings.push(probe::Finding {
      module: MODULE_TRANSPORT,
      url: url.to_string(),
      param: "no-https".to_string(),
      payload: String::new(),
      evidence: EVIDENCE_TRANSPORT_NO_HTTPS.to_string(),
    });
  }
}
