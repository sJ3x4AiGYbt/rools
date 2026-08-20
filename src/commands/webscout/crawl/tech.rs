use crate::commands::webscout::http;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TechHit {
  pub name: String,
  pub category: TechCategory,
  pub source: DetectionSource,
  pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TechCategory {
  WebServer,
  Language,
  Framework,
  Cms,
  Cache,
  Security,
  Analytics,
  Cdn,
  JavaScript,
}

#[derive(Debug, Clone)]
pub enum DetectionSource {
  Header(String),
  HtmlMeta,
  HtmlBody,
  Cookie(String),
}

impl std::fmt::Display for DetectionSource {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      DetectionSource::Header(name) => write!(f, "header:{name}"),
      DetectionSource::Cookie(name) => write!(f, "cookie:{name}"),
      DetectionSource::HtmlMeta => write!(f, "html-meta"),
      DetectionSource::HtmlBody => write!(f, "html-body"),
    }
  }
}

impl std::fmt::Display for TechCategory {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let s = match self {
      TechCategory::WebServer => "web server",
      TechCategory::Language => "language",
      TechCategory::Framework => "framework",
      TechCategory::Cms => "cms",
      TechCategory::Cache => "cache",
      TechCategory::Security => "security",
      TechCategory::Analytics => "analytics",
      TechCategory::Cdn => "cdn",
      TechCategory::JavaScript => "javascript",
    };
    f.pad(s)
  }
}

const HEADER_FINGERPRINTS: &[(&str, &str, &str, TechCategory)] = &[
  ("server", "apache", "Apache", TechCategory::WebServer),
  ("server", "nginx", "Nginx", TechCategory::WebServer),
  ("server", "iis", "IIS", TechCategory::WebServer),
  ("server", "lighttpd", "Lighttpd", TechCategory::WebServer),
  ("server", "caddy", "Caddy", TechCategory::WebServer),
  ("server", "openresty", "OpenResty", TechCategory::WebServer),
  (
    "server",
    "werkzeug",
    "Werkzeug (Flask dev)",
    TechCategory::WebServer,
  ),
  ("server", "cloudflare", "Cloudflare", TechCategory::Cdn),
  ("server", "awselb", "AWS ELB", TechCategory::Cdn),
  ("server", "netlify", "Netlify", TechCategory::Cdn),
  ("x-powered-by", "php", "PHP", TechCategory::Language),
  ("x-powered-by", "asp.net", "ASP.NET", TechCategory::Language),
  (
    "x-powered-by",
    "express",
    "Express.js",
    TechCategory::Framework,
  ),
  (
    "x-powered-by",
    "next.js",
    "Next.js",
    TechCategory::Framework,
  ),
  ("x-powered-by", "django", "Django", TechCategory::Framework),
  ("x-generator", "drupal", "Drupal", TechCategory::Cms),
  ("x-drupal-cache", "", "Drupal", TechCategory::Cms),
  ("x-joomla", "", "Joomla", TechCategory::Cms),
  ("x-aspnet-version", "", "ASP.NET", TechCategory::Language),
  (
    "x-aspnetmvc-version",
    "",
    "ASP.NET MVC",
    TechCategory::Framework,
  ),
  ("x-cache", "cloudfront", "CloudFront", TechCategory::Cdn),
  ("via", "cloudfront", "CloudFront", TechCategory::Cdn),
  ("x-varnish", "", "Varnish", TechCategory::Cache),
  ("cf-ray", "", "Cloudflare", TechCategory::Cdn),
  ("x-fastly", "", "Fastly", TechCategory::Cdn),
  (
    "strict-transport-security",
    "",
    "HSTS",
    TechCategory::Security,
  ),
  ("content-security-policy", "", "CSP", TechCategory::Security),
  ("x-frame-options", "", "X-Frame", TechCategory::Security),
];

const COOKIE_FINGERPRINTS: &[(&str, &str, TechCategory)] = &[
  ("phpsessid", "PHP", TechCategory::Language),
  ("jsessionid", "Java / Spring", TechCategory::Language),
  ("asp.net_sessionid", "ASP.NET", TechCategory::Language),
  ("laravel_session", "Laravel", TechCategory::Framework),
  ("django", "Django", TechCategory::Framework),
  ("rails", "Ruby on Rails", TechCategory::Framework),
  ("wp-", "WordPress", TechCategory::Cms),
  ("drupal", "Drupal", TechCategory::Cms),
  ("_ga", "Google Analytics", TechCategory::Analytics),
  ("_gid", "Google Analytics", TechCategory::Analytics),
];

const BODY_FINGERPRINTS: &[(&str, &str, TechCategory)] = &[
  ("/wp-content/", "WordPress", TechCategory::Cms),
  ("/wp-includes/", "WordPress", TechCategory::Cms),
  ("Drupal.settings", "Drupal", TechCategory::Cms),
  ("/sites/default/files", "Drupal", TechCategory::Cms),
  ("/components/com_", "Joomla", TechCategory::Cms),
  ("Joomla!", "Joomla", TechCategory::Cms),
  ("typo3", "TYPO3", TechCategory::Cms),
  ("shopify", "Shopify", TechCategory::Cms),
  ("cdn.shopify.com", "Shopify", TechCategory::Cms),
  ("squarespace", "Squarespace", TechCategory::Cms),
  ("wix.com", "Wix", TechCategory::Cms),
  ("__NEXT_DATA__", "Next.js", TechCategory::Framework),
  ("__nuxt", "Nuxt.js", TechCategory::Framework),
  ("ng-version", "Angular", TechCategory::JavaScript),
  ("data-reactroot", "React", TechCategory::JavaScript),
  ("data-v-app", "Vue.js", TechCategory::JavaScript),
  ("ember-application", "Ember.js", TechCategory::JavaScript),
  ("svelte", "Svelte", TechCategory::JavaScript),
  ("jquery", "jQuery", TechCategory::JavaScript),
  ("bootstrap", "Bootstrap", TechCategory::JavaScript),
  ("tailwindcss", "Tailwind CSS", TechCategory::JavaScript),
  (
    "google-analytics.com",
    "Google Analytics",
    TechCategory::Analytics,
  ),
  ("gtag(", "Google Tag Mgr", TechCategory::Analytics),
  ("hotjar", "Hotjar", TechCategory::Analytics),
  ("segment.com", "Segment", TechCategory::Analytics),
  ("id=\"root\"", "React (SPA)", TechCategory::JavaScript),
  ("id=\"app\"", "Vue.js (SPA)", TechCategory::JavaScript),
  ("id=\"__nuxt\"", "Nuxt.js (SPA)", TechCategory::JavaScript),
  ("id=\"ng-app\"", "Angular (SPA)", TechCategory::JavaScript),
];

pub fn detect(res: &http::HttpResponse) -> Vec<TechHit> {
  let mut hits: Vec<TechHit> = Vec::new();
  let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

  let mut push = |hit: TechHit| {
    if seen.insert(hit.name.clone()) {
      hits.push(hit);
    }
  };

  detect_headers(&res.headers, &mut push);
  detect_cookies(&res.headers, &mut push);
  detect_meta(&res.body, &mut push);
  detect_body(&res.body, &mut push);

  hits
}

fn detect_headers(headers: &HashMap<String, String>, push: &mut impl FnMut(TechHit)) {
  for (header_key, pattern, tech_name, category) in HEADER_FINGERPRINTS {
    if let Some(value) = headers.get(*header_key) {
      let lower = value.to_lowercase();
      if pattern.is_empty() || lower.contains(pattern) {
        let version = if *header_key == "server" || *header_key == "x-powered-by" {
          extract_version(value)
        } else {
          None
        };
        push(TechHit {
          name: tech_name.to_string(),
          category: category.clone(),
          source: DetectionSource::Header(header_key.to_string()),
          version,
        });
      }
    }
  }
}

fn detect_cookies(headers: &HashMap<String, String>, push: &mut impl FnMut(TechHit)) {
  let cookie_header = headers.get("set-cookie").cloned().unwrap_or_default();
  let lower = cookie_header.to_lowercase();

  for (cookie_pattern, tech_name, category) in COOKIE_FINGERPRINTS {
    if lower.contains(cookie_pattern) {
      push(TechHit {
        name: tech_name.to_string(),
        category: category.clone(),
        source: DetectionSource::Cookie(cookie_pattern.to_string()),
        version: None,
      });
    }
  }
}

fn detect_meta(body: &str, push: &mut impl FnMut(TechHit)) {
  if let Some(start) = body.to_lowercase().find("<meta name=\"generator\"") {
    let slice = &body[start..];
    if let Some(content_start) = slice.to_lowercase().find("content=\"") {
      let after = &slice[content_start + 9..];
      if let Some(end) = after.find('"') {
        let generator = &after[..end];
        let lower = generator.to_lowercase();

        let tech = if lower.contains("wordpress") {
          Some(("WordPress", TechCategory::Cms))
        } else if lower.contains("drupal") {
          Some(("Drupal", TechCategory::Cms))
        } else if lower.contains("joomla") {
          Some(("Joomla", TechCategory::Cms))
        } else if lower.contains("hugo") {
          Some(("Hugo", TechCategory::Framework))
        } else if lower.contains("gatsby") {
          Some(("Gatsby", TechCategory::Framework))
        } else if lower.contains("jekyll") {
          Some(("Jekyll", TechCategory::Framework))
        } else if lower.contains("next.js") {
          Some(("Next.js", TechCategory::Framework))
        } else {
          None
        };

        if let Some((name, category)) = tech {
          push(TechHit {
            name: name.to_string(),
            category,
            source: DetectionSource::HtmlMeta,
            version: extract_version(generator),
          });
        }
      }
    }
  }
}

fn detect_body(body: &str, push: &mut impl FnMut(TechHit)) {
  let lower = body.to_lowercase();
  for (pattern, tech_name, category) in BODY_FINGERPRINTS {
    if lower.contains(&pattern.to_lowercase()) {
      push(TechHit {
        name: tech_name.to_string(),
        category: category.clone(),
        source: DetectionSource::HtmlBody,
        version: None,
      });
    }
  }
}

fn extract_version(value: &str) -> Option<String> {
  let mut chars = value.chars().peekable();
  while let Some(c) = chars.next() {
    if c == '/' || c == '@' || c == ' ' || c == '-' {
      if chars
        .peek()
        .map(|c| *c == 'v' || *c == 'V')
        .unwrap_or(false)
      {
        chars.next();
      }
      if chars.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
        let version: String = chars
          .by_ref()
          .take_while(|c| c.is_ascii_digit() || *c == '.')
          .collect();
        if !version.is_empty() {
          return Some(version);
        }
      }
    }
  }
  None
}
