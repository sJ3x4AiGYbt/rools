use crate::commands::webscout::url;
use scraper::{Html, Selector};

#[derive(Debug, Clone)]
pub struct FormInfo {
  pub action: String,
  pub method: String,
  pub fields: Vec<FieldInfo>,
}

#[derive(Debug, Clone)]
pub struct FieldInfo {
  pub name: String,
  pub field_type: String,
  pub required: bool,
}

pub fn parse_links(html: &str, base_url: &str) -> Vec<String> {
  let document = Html::parse_document(html);

  let sources: &[(&str, &str)] = &[
    ("a[href]", "href"),
    ("link[href]", "href"),
    ("script[src]", "src"),
    ("form[action]", "action"),
  ];

  let mut links: Vec<String> = sources
    .iter()
    .flat_map(|(selector, attr)| {
      let sel = Selector::parse(selector).expect("valid selector");
      document
        .select(&sel)
        .filter_map(|el| el.value().attr(attr))
        .filter_map(|href| url::resolve(base_url, href))
        .collect::<Vec<_>>()
    })
    .filter(|url| is_crawlable(url))
    .collect();

  links.sort_unstable();
  links.dedup();
  links
}

fn is_crawlable(url: &str) -> bool {
  let lower = url.to_lowercase();
  !lower.starts_with("javascript:")
    && !lower.starts_with("mailto:")
    && !lower.starts_with("tel:")
    && !lower.starts_with("data:")
    && (lower.starts_with("http://") || lower.starts_with("https://"))
}

pub fn parse_forms(html: &str, base_url: &str) -> Vec<FormInfo> {
  let document = Html::parse_document(html);

  let form_sel = Selector::parse("form").expect("valid selector");
  let input_sel = Selector::parse("input").expect("valid selector");
  let textarea_sel = Selector::parse("textarea").expect("valid selector");
  let select_sel = Selector::parse("select").expect("valid selector");

  document
    .select(&form_sel)
    .map(|form| {
      let raw_action = form.value().attr("action").unwrap_or("");
      let action = url::resolve(base_url, raw_action).unwrap_or_else(|| base_url.to_string());

      let method = form
        .value()
        .attr("method")
        .map(|m| m.to_uppercase())
        .unwrap_or_else(|| "GET".to_string());

      let mut fields: Vec<FieldInfo> = Vec::new();

      for input in form.select(&input_sel) {
        let field_type = input.value().attr("type").unwrap_or("text").to_lowercase();

        if matches!(field_type.as_str(), "submit" | "button" | "reset" | "image") {
          continue;
        }

        let name = input
          .value()
          .attr("name")
          .or_else(|| input.value().attr("id"))
          .unwrap_or("(unnamed)")
          .to_string();

        fields.push(FieldInfo {
          name,
          field_type,
          required: input.value().attr("required").is_some(),
        });
      }

      for textarea in form.select(&textarea_sel) {
        let name = textarea
          .value()
          .attr("name")
          .or_else(|| textarea.value().attr("id"))
          .unwrap_or("(unnamed)")
          .to_string();

        fields.push(FieldInfo {
          name,
          field_type: "textarea".to_string(),
          required: textarea.value().attr("required").is_some(),
        });
      }

      for select in form.select(&select_sel) {
        let name = select
          .value()
          .attr("name")
          .or_else(|| select.value().attr("id"))
          .unwrap_or("(unnamed)")
          .to_string();

        fields.push(FieldInfo {
          name,
          field_type: "select".to_string(),
          required: select.value().attr("required").is_some(),
        });
      }

      FormInfo {
        action,
        method,
        fields,
      }
    })
    .filter(|f| !f.fields.is_empty())
    .collect()
}

pub fn parse_sitemap(xml: &str, base_url: &str) -> Vec<String> {
  let mut urls = Vec::new();
  for line in xml.lines() {
    let trimmed = line.trim();
    if trimmed.starts_with("<loc>") && trimmed.ends_with("</loc>") {
      let loc = trimmed
        .trim_start_matches("<loc>")
        .trim_end_matches("</loc>")
        .trim();
      if let Some(resolved) = url::resolve(base_url, loc) {
        urls.push(resolved);
      }
    }
  }
  urls
}

pub fn parse_robots(body: &str, base_url: &str) -> Vec<String> {
  body
    .lines()
    .filter_map(|line| {
      let l = line.trim();
      if l.starts_with("Disallow:") || l.starts_with("Allow:") {
        let path = l.split_once(':')?.1.trim();
        if path.is_empty() || path == "/" {
          return None;
        }
        url::resolve(base_url, path)
      } else if l.starts_with("Sitemap:") {
        let loc = l.split_once(':')?.1.trim();
        Some(loc.to_string())
      } else {
        None
      }
    })
    .collect()
}

pub fn parse_js_routes(js: &str, base_url: &str) -> Vec<String> {
  let mut routes = Vec::new();
  let mut chars = js.chars().peekable();

  while let Some(c) = chars.next() {
    if c == '"' || c == '\'' || c == '`' {
      let quote = c;
      let mut buf = String::new();
      for ch in chars.by_ref() {
        if ch == quote {
          break;
        }
        buf.push(ch);
        if buf.len() > 80 {
          break;
        }
      }
      if buf.starts_with('/')
        && buf.len() > 1
        && buf.len() < 60
        && !buf.contains(' ')
        && !buf.contains('\n')
        && buf
          .chars()
          .all(|c| c.is_ascii_alphanumeric() || "/-_.:".contains(c))
        && let Some(resolved) = url::resolve(base_url, &buf)
        && !routes.contains(&resolved)
      {
        routes.push(resolved);
      }
    }
  }
  routes
}
