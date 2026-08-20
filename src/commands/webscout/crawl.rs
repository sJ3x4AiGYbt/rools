pub mod formatter;
pub mod parser;
pub mod queue;
pub mod search;
pub mod tech;

use crate::commands::webscout::common::config::{self as cfg, WebScoutRunner};
use crate::commands::webscout::common::constants::{
  ERROR_HTTP_CLIENT, ERROR_INVALID_URL, FIELD_ERROR, FIELD_INPUT, FIELD_ROOT, WARN_HTTPS_FALLBACK,
  WARN_HTTPS_FALLBACK_MID, WARN_MISSING_SCHEME,
};
use crate::commands::webscout::common::http;
use crate::commands::webscout::common::url;
use crate::global::export;
use crate::global::format;
use crate::global::shutdown as shutdown_mod;
use crate::global::writer as writer_mod;

pub struct CrawlRunner;

impl CrawlRunner {
  pub fn new() -> Self {
    Self
  }
}

impl WebScoutRunner for CrawlRunner {
  fn run(&mut self, config: &cfg::WebScoutConfig) {
    let crawl = match &config.action {
      cfg::WebScoutAction::Crawl(c) => c,
      _ => return,
    };

    let shutdown = shutdown_mod::Shutdown::new(config.timeout);
    let writer = writer_mod::OutputWriter::new(config.export_path.clone(), true);

    let mut parsed = match url::parse_and_fix(&crawl.url) {
      Ok(p) => p,
      Err(e) => {
        eprintln!("{ERROR_INVALID_URL}{e}");
        return;
      }
    };

    writer.write(format_args!("[{}]   {}", FIELD_INPUT, crawl.url));

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

    let fmt = get_formatter(config);
    fmt.header();

    let target = if crawl.recursion {
      parsed.root.clone()
    } else {
      parsed.normalized.clone()
    };

    let (res, fallback) =
      match fetch_with_https_fallback(&client, &target, parsed.was_fixed, &writer) {
        Ok(v) => v,
        Err(()) => return,
      };

    if fallback.is_some() {
      parsed.root = parsed.root.replacen("https://", "http://", 1);
      parsed.normalized = parsed.normalized.replacen("https://", "http://", 1);
    }

    if crawl.recursion {
      crawl_recursive(res, &parsed, crawl, &client, &shutdown, fmt.as_ref())
    } else {
      crawl_single(res, &parsed, crawl, fmt.as_ref())
    };

    fmt.footer();
    export::Export::finalize(&config.export_path, config.zip).ok();
  }
}

fn fetch_with_https_fallback(
  client: &http::HttpClient,
  target: &str,
  was_fixed: bool,
  writer: &writer_mod::OutputWriter,
) -> Result<(http::HttpResponse, Option<String>), ()> {
  match client.get(target) {
    Ok(r) => Ok((r, None)),
    Err(e) if was_fixed && target.starts_with("https://") => {
      let fallback = target.replacen("https://", "http://", 1);
      writer.write(format_args!(
        "{WARN_HTTPS_FALLBACK} ({e}), {WARN_HTTPS_FALLBACK_MID}: {fallback}"
      ));
      match client.get(&fallback) {
        Ok(r) => Ok((r, Some(fallback))),
        Err(e2) => {
          eprintln!("[{FIELD_ERROR}]   {e2}");
          Err(())
        }
      }
    }
    Err(e) => {
      eprintln!("[{FIELD_ERROR}]   {e}");
      Err(())
    }
  }
}

fn crawl_single(
  res: http::HttpResponse,
  parsed: &url::ParsedUrl,
  config: &cfg::CrawlConfig,
  fmt: &dyn formatter::Formatter,
) {
  let url = parsed.normalized.as_str();

  fmt.page(&formatter::PageEvent {
    index: 1,
    url,
    status: res.status,
    size: res.body.len(),
    status_text: Some(&res.status_text),
    duration_ms: Some(res.duration_ms),
    content_type: Some(&res.content_type),
    links_found: parser::parse_links(&res.body, url).len(),
    forms_found: 0,
    forms_enabled: config.forms,
  });

  fmt.technologies(&tech::detect(&res));

  if config.forms {
    fmt.forms(&parser::parse_forms(&res.body, url));
  }
}

fn crawl_recursive(
  res: http::HttpResponse,
  parsed: &url::ParsedUrl,
  config: &cfg::CrawlConfig,
  client: &http::HttpClient,
  shutdown: &shutdown_mod::Shutdown,
  fmt: &dyn formatter::Formatter,
) {
  fmt.page(&formatter::PageEvent {
    index: 1,
    url: &parsed.root,
    status: res.status,
    size: res.body.len(),
    status_text: Some(&res.status_text),
    duration_ms: Some(res.duration_ms),
    content_type: Some(&res.content_type),
    links_found: parser::parse_links(&res.body, &parsed.root).len(),
    forms_found: 0,
    forms_enabled: config.forms,
  });

  fmt.technologies(&tech::detect(&res));

  let mut queue = queue::CrawlQueue::new(&parsed.normalized, &parsed.host, 1000);

  let sitemap_url = format!("{}/sitemap.xml", parsed.root.trim_end_matches('/'));
  if let Ok(res) = client.get(&sitemap_url) {
    let urls = parser::parse_sitemap(&res.body, &parsed.normalized);
    queue.enqueue_links(&urls);
  }

  let robots_url = format!("{}/robots.txt", parsed.root.trim_end_matches('/'));
  if let Ok(res) = client.get(&robots_url) {
    let paths = parser::parse_robots(&res.body, &parsed.normalized);
    queue.enqueue_links(&paths);
  }

  fmt.header_recursion();

  while let Some(url) = queue.next() {
    if !shutdown.is_running() {
      break;
    }

    let res = match client.get(&url) {
      Ok(r) => r,
      Err(e) => {
        eprintln!("{FIELD_ERROR}{e}");
        continue;
      }
    };

    fmt.recursion(&formatter::PageEvent {
      index: queue.visited_count(),
      url: &url,
      status: res.status,
      size: res.body.len(),
      status_text: None,
      duration_ms: Some(res.duration_ms),
      content_type: Some(&res.content_type),
      links_found: {
        let links = parser::parse_links(&res.body, &url);
        queue.enqueue_links(&links);
        if url.ends_with(".js") {
          let js_routes = parser::parse_js_routes(&res.body, &parsed.normalized);
          queue.enqueue_links(&js_routes);
        }
        links.len()
      },
      forms_enabled: config.forms,
      forms_found: {
        let forms = parser::parse_forms(&res.body, &url);
        forms.len()
      },
    });
  }

  if config.search && shutdown.is_running() {
    let hits = search::run(&parsed.root, client, queue.visited());
    fmt.search(&hits);
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
