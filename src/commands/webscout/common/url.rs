use crate::commands::webscout::common::constants::{
  ERROR_URL_EMPTY_HOST, ERROR_URL_INVALID, ERROR_URL_NO_HOST,
};
use url::Url;

pub struct ParsedUrl {
  pub normalized: String,
  pub root: String,
  pub host: String,
  pub was_fixed: bool,
}

pub fn parse_and_fix(input: &str) -> Result<ParsedUrl, String> {
  let (raw, was_fixed) = if input.contains("://") {
    (input.to_string(), false)
  } else {
    (format!("https://{input}"), true)
  };

  let parsed = Url::parse(&raw).map_err(|e| format!("{ERROR_URL_INVALID}{e}"))?;

  let host = parsed.host_str().ok_or(ERROR_URL_NO_HOST)?.to_string();

  if host.is_empty() {
    return Err(ERROR_URL_EMPTY_HOST.to_string());
  }

  let root = format!("{}://{}", parsed.scheme(), parsed.host_str().unwrap_or(""));

  let root = if let Some(port) = parsed.port() {
    format!("{root}:{port}")
  } else {
    root
  };

  Ok(ParsedUrl {
    normalized: raw,
    root,
    host,
    was_fixed,
  })
}

pub fn same_domain(url: &str, base_host: &str) -> bool {
  Url::parse(url)
    .ok()
    .and_then(|u| u.host_str().map(|h| h.to_string()))
    .map(|h| h == base_host || h.ends_with(&format!(".{base_host}")))
    .unwrap_or(false)
}

pub fn resolve(base: &str, href: &str) -> Option<String> {
  let base = Url::parse(base).ok()?;
  base.join(href).ok().map(|u| {
    let mut clean = u.clone();
    clean.set_fragment(None);
    clean.to_string()
  })
}
