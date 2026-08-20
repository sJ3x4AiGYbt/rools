use crate::commands::webscout::common::http;
use crate::commands::webscout::crawl::parser;
use url::Url;

#[derive(Debug, Clone)]
pub struct Finding {
  pub module: &'static str,
  pub url: String,
  pub param: String,
  pub payload: String,
  pub evidence: String,
}

pub struct ProbeResult {
  pub url: String,
  pub param: String,
  pub payload: String,
  pub response: http::HttpResponse,
}

pub struct TestedItem {
  pub url: String,
  pub param: String,
  pub payload: String,
}

pub struct ModuleReport {
  pub findings: Vec<Finding>,
  pub tested: Vec<TestedItem>,
}

pub fn probe_form(
  client: &http::HttpClient,
  form: &parser::FormInfo,
  payload: &str,
) -> Vec<ProbeResult> {
  let mut results = Vec::new();

  for target_field in &form.fields {
    let fields: Vec<(String, String)> = form
      .fields
      .iter()
      .map(|f| {
        let value = if f.name == target_field.name {
          payload.to_string()
        } else {
          neutral_value(&f.field_type)
        };
        (f.name.clone(), value)
      })
      .collect();

    let pairs: Vec<(&str, &str)> = fields
      .iter()
      .map(|(k, v)| (k.as_str(), v.as_str()))
      .collect();

    let result = if form.method == "POST" {
      client.post_form(&form.action, &pairs)
    } else {
      client.get_with_params(&form.action, &pairs)
    };

    if let Ok(response) = result {
      results.push(ProbeResult {
        url: form.action.clone(),
        param: target_field.name.clone(),
        payload: payload.to_string(),
        response,
      });
    }
  }

  results
}

pub fn probe_url_params(client: &http::HttpClient, url: &str, payload: &str) -> Vec<ProbeResult> {
  let parsed = match Url::parse(url) {
    Ok(u) => u,
    Err(_) => return vec![],
  };

  let params: Vec<(String, String)> = parsed
    .query_pairs()
    .map(|(k, v)| (k.into_owned(), v.into_owned()))
    .collect();

  if params.is_empty() {
    return vec![];
  }

  let mut results = Vec::new();

  for (i, (target_key, _)) in params.iter().enumerate() {
    let injected: Vec<(String, String)> = params
      .iter()
      .enumerate()
      .map(|(j, (k, v))| {
        if j == i {
          (k.clone(), payload.to_string())
        } else {
          (k.clone(), v.clone())
        }
      })
      .collect();

    let pairs: Vec<(&str, &str)> = injected
      .iter()
      .map(|(k, v)| (k.as_str(), v.as_str()))
      .collect();

    let base = base_url(url);
    if let Ok(response) = client.get_with_params(&base, &pairs) {
      results.push(ProbeResult {
        url: base.clone(),
        param: target_key.clone(),
        payload: payload.to_string(),
        response,
      });
    }
  }

  results
}

pub fn probe_json_body(
  client: &http::HttpClient,
  url: &str,
  template: &serde_json::Value,
  payload: &str,
) -> Vec<ProbeResult> {
  let mut results = Vec::new();

  let Some(obj) = template.as_object() else {
    return results;
  };

  for (target_key, target_value) in obj {
    if !target_value.is_string() {
      continue;
    }

    let mut injected = template.clone();
    injected[target_key] = serde_json::Value::String(payload.to_string());

    if let Ok(response) = client.post_json(url, &injected) {
      results.push(ProbeResult {
        url: url.to_string(),
        param: target_key.clone(),
        payload: payload.to_string(),
        response,
      });
    }
  }

  results
}

fn neutral_value(field_type: &str) -> String {
  match field_type {
    "email" => "test@example.com".to_string(),
    "number" => "1".to_string(),
    "tel" => "0600000000".to_string(),
    _ => "test".to_string(),
  }
}

fn base_url(url: &str) -> String {
  match url.split_once('?') {
    Some((base, _)) => base.to_string(),
    None => url.to_string(),
  }
}
