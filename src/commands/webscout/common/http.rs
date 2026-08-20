use crate::commands::webscout::common::constants::{
  ERROR_HTTP_CLIENT_BUILD, ERROR_HTTP_READ_BODY, ERROR_HTTP_REQUEST, HTTP_USER_AGENT,
};
use reqwest::blocking::{Client, Response};
use reqwest::header::HeaderMap;
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct HttpResponse {
  pub status: u16,
  pub status_text: String,
  pub headers: HashMap<String, String>,
  pub body: String,
  pub duration_ms: u64,
  pub content_type: String,
}

pub struct HttpClient {
  client: Client,
  client_no_redirect: Client,
}

impl HttpClient {
  pub fn new(timeout_secs: u64, accept_invalid_certs: bool) -> Result<Self, String> {
    let build = |redirect: reqwest::redirect::Policy| {
      Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .danger_accept_invalid_certs(accept_invalid_certs)
        .redirect(redirect)
        .user_agent(HTTP_USER_AGENT)
        .build()
    };

    let client = build(reqwest::redirect::Policy::limited(10))
      .map_err(|e| format!("{ERROR_HTTP_CLIENT_BUILD}{e}"))?;
    let client_no_redirect = build(reqwest::redirect::Policy::none())
      .map_err(|e| format!("{ERROR_HTTP_CLIENT_BUILD}{e}"))?;

    Ok(Self {
      client,
      client_no_redirect,
    })
  }

  pub fn get_no_redirect(&self, url: &str) -> Result<HttpResponse, String> {
    let start = Instant::now();
    let response = self
      .client_no_redirect
      .get(url)
      .send()
      .map_err(|e| format!("{ERROR_HTTP_REQUEST}{e}"))?;
    self.read_response(response, start)
  }

  pub fn post_form(&self, url: &str, fields: &[(&str, &str)]) -> Result<HttpResponse, String> {
    let start = Instant::now();

    let response = self
      .client
      .post(url)
      .form(fields)
      .send()
      .map_err(|e| format!("{ERROR_HTTP_REQUEST}{e}"))?;

    self.read_response(response, start)
  }

  pub fn post_json(&self, url: &str, body: &serde_json::Value) -> Result<HttpResponse, String> {
    let start = Instant::now();

    let response = self
      .client
      .post(url)
      .json(body)
      .send()
      .map_err(|e| format!("{ERROR_HTTP_REQUEST}{e}"))?;

    self.read_response(response, start)
  }

  pub fn get_with_params(
    &self,
    url: &str,
    params: &[(&str, &str)],
  ) -> Result<HttpResponse, String> {
    let start = Instant::now();

    let response = self
      .client
      .get(url)
      .query(params)
      .send()
      .map_err(|e| format!("{ERROR_HTTP_REQUEST}{e}"))?;

    self.read_response(response, start)
  }

  pub fn get(&self, url: &str) -> Result<HttpResponse, String> {
    let start = Instant::now();
    let response = self
      .client
      .get(url)
      .send()
      .map_err(|e| format!("{ERROR_HTTP_REQUEST}{e}"))?;
    self.read_response(response, start)
  }

  fn read_response(&self, response: Response, start: Instant) -> Result<HttpResponse, String> {
    let duration_ms = start.elapsed().as_millis() as u64;
    let status = response.status().as_u16();
    let status_text = response
      .status()
      .canonical_reason()
      .unwrap_or("Unknown")
      .to_string();
    let headers = flatten_headers(response.headers());
    let content_type = headers.get("content-type").cloned().unwrap_or_default();

    let body = response
      .text()
      .map_err(|e| format!("{ERROR_HTTP_READ_BODY}{e}"))?;

    Ok(HttpResponse {
      status,
      status_text,
      headers,
      body,
      duration_ms,
      content_type,
    })
  }
}

fn flatten_headers(map: &HeaderMap) -> HashMap<String, String> {
  map
    .iter()
    .map(|(k, v)| {
      (
        k.as_str().to_lowercase(),
        v.to_str().unwrap_or("").to_string(),
      )
    })
    .collect()
}
