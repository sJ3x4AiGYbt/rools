use crate::commands::webscout::common::constants::{
  FIELD_ACTION, FIELD_CATEGORY, FIELD_CONTENT_TYPE, FIELD_DURATION_MS, FIELD_FIELDS, FIELD_FORMS,
  FIELD_FORMS_FOUND, FIELD_FOUND, FIELD_ID, FIELD_INDEX, FIELD_LINKS, FIELD_LINKS_FOUND,
  FIELD_METHOD, FIELD_NAME, FIELD_NO, FIELD_RECURSION, FIELD_REQUIRED, FIELD_SEARCH, FIELD_SIZE,
  FIELD_SOURCE, FIELD_STATUS, FIELD_TARGET, FIELD_TECH, FIELD_TECHNOLOGIES, FIELD_TIME,
  FIELD_TXT_FORMS_FOUND, FIELD_TYPE, FIELD_URL, FIELD_VERSION, FIELD_YES, TXT_FORMS_FOUND,
  TXT_FORMS_NONE, TXT_SEARCH_FOUND, TXT_SEARCH_NONE, TXT_TECH_FOUND, TXT_TECH_NONE,
};
use crate::commands::webscout::crawl::parser;
use crate::commands::webscout::crawl::search;
use crate::commands::webscout::crawl::tech;
use crate::global::format;
use crate::global::writer;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct PageEvent<'a> {
  pub index: usize,
  pub url: &'a str,
  pub status: u16,
  pub size: usize,
  pub status_text: Option<&'a str>,
  pub duration_ms: Option<u64>,
  pub content_type: Option<&'a str>,
  pub links_found: usize,
  pub forms_enabled: bool,
  pub forms_found: usize,
}

pub trait Formatter {
  fn header(&self) {}
  fn page(&self, event: &PageEvent);
  fn header_recursion(&self);
  fn recursion(&self, event: &PageEvent);
  fn technologies(&self, hits: &[tech::TechHit]);
  fn forms(&self, forms: &[parser::FormInfo]);
  fn search(&self, hits: &[search::SearchHit]);
  fn footer(&self) {}
}

pub struct Txt {
  pub writer: writer::OutputWriter,
}

impl Formatter for Txt {
  fn page(&self, e: &PageEvent) {
    self
      .writer
      .write(format_args!("\n[{}]  {}", FIELD_TARGET, e.url));
    self.writer.write(format_args!(
      "   {}     : {} {}",
      FIELD_STATUS,
      e.status,
      e.status_text.unwrap_or("")
    ));
    self
      .writer
      .write(format_args!("   {}       : {} bytes", FIELD_SIZE, e.size));
    self.writer.write(format_args!(
      "   {}       : {}ms",
      FIELD_TIME,
      e.duration_ms.unwrap_or(0)
    ));
    self.writer.write(format_args!(
      "   {}       : {}",
      FIELD_TYPE,
      e.content_type.unwrap_or("")
    ));
    self.writer.write(format_args!(
      "   {}      : {} {}",
      FIELD_LINKS, e.links_found, FIELD_FOUND
    ));
  }

  fn header_recursion(&self) {
    self.writer.write(format_args!("\n[{FIELD_RECURSION}]"));
  }

  fn recursion(&self, e: &PageEvent) {
    if e.forms_enabled {
      self.writer.write(format_args!(
        "   {:<5}: {:>3} {:<100} \n          {}ms, {} bytes, {}, {} {}, {} {}",
        e.index,
        e.status,
        e.url,
        e.duration_ms.unwrap_or(0),
        e.size,
        e.content_type.unwrap_or(""),
        e.links_found,
        FIELD_LINKS_FOUND,
        e.forms_found,
        FIELD_TXT_FORMS_FOUND
      ));
    } else {
      self.writer.write(format_args!(
        "   {:<5}: {:>3} {:<100} \n          {}ms, {} bytes, {}, {} {}",
        e.index,
        e.status,
        e.url,
        e.duration_ms.unwrap_or(0),
        e.size,
        e.content_type.unwrap_or(""),
        e.links_found,
        FIELD_LINKS_FOUND
      ));
    }
  }

  fn technologies(&self, hits: &[tech::TechHit]) {
    if hits.is_empty() {
      self
        .writer
        .write(format_args!("\n[{FIELD_TECH}] {TXT_TECH_NONE}"));
      return;
    }
    self
      .writer
      .write(format_args!("\n[{FIELD_TECH}] {TXT_TECH_FOUND}:"));
    for hit in hits {
      let version = hit
        .version
        .as_deref()
        .map(|v| format!(" {v}"))
        .unwrap_or_default();
      self.writer.write(format_args!(
        "   {:<10} : {}{}",
        hit.category, hit.name, version
      ));
    }
  }

  fn forms(&self, forms: &[parser::FormInfo]) {
    if forms.is_empty() {
      self
        .writer
        .write(format_args!("\n[{FIELD_FORMS}] {TXT_FORMS_NONE}"));
      return;
    }
    self.writer.write(format_args!(
      "\n[{}] {} {}:",
      FIELD_FORMS,
      forms.len(),
      TXT_FORMS_FOUND
    ));
    for form in forms {
      self
        .writer
        .write(format_args!("   {} ({})", form.action, form.method));
      for field in &form.fields {
        let req = if field.required { FIELD_YES } else { FIELD_NO };
        self.writer.write(format_args!(
          "      {}={}, {}={}, {}={}",
          FIELD_NAME, field.name, FIELD_TYPE, field.field_type, FIELD_REQUIRED, req
        ));
      }
    }
  }

  fn search(&self, hits: &[search::SearchHit]) {
    if hits.is_empty() {
      self
        .writer
        .write(format_args!("\n[{FIELD_SEARCH}] {TXT_SEARCH_NONE}"));
      return;
    }
    self.writer.write(format_args!(
      "\n[{}] {} {}:",
      FIELD_SEARCH,
      hits.len(),
      TXT_SEARCH_FOUND
    ));
    for hit in hits {
      self.writer.write(format_args!(
        "   {:<11}: {} {}   ({} bytes)",
        hit.category, hit.status, hit.url, hit.size
      ));
    }
  }
}

pub struct Csv {
  pub writer: writer::OutputWriter,
  first_page: AtomicBool,
  first_recursion: AtomicBool,
}

impl Csv {
  pub fn new(writer: writer::OutputWriter) -> Self {
    Self {
      writer,
      first_page: AtomicBool::new(true),
      first_recursion: AtomicBool::new(true),
    }
  }
}

impl Formatter for Csv {
  fn page(&self, e: &PageEvent) {
    self.writer.write(format_args!(
      "\n# {} — {}",
      FIELD_TARGET,
      format::escape_csv(e.url)
    ));
    if self.first_page.swap(false, Ordering::SeqCst) {
      self.writer.write(format_args!(
                "{FIELD_URL},{FIELD_STATUS},{FIELD_SIZE},{FIELD_DURATION_MS},{FIELD_CONTENT_TYPE},{FIELD_LINKS_FOUND}",
            ));
    }
    self.writer.write(format_args!(
      "{},{},{},{},{},{}",
      format::escape_csv(e.url),
      e.status,
      e.size,
      e.duration_ms.unwrap_or(0),
      format::escape_csv(e.content_type.unwrap_or("")),
      e.links_found,
    ));
  }

  fn header_recursion(&self) {
    self.writer.write(format_args!("\n# {FIELD_RECURSION}"));
  }

  fn recursion(&self, e: &PageEvent) {
    if self.first_recursion.swap(false, Ordering::SeqCst) {
      if e.forms_enabled {
        self.writer.write(format_args!(
                    "{FIELD_INDEX},{FIELD_URL},{FIELD_STATUS},{FIELD_SIZE},{FIELD_DURATION_MS},{FIELD_CONTENT_TYPE},{FIELD_LINKS_FOUND},{FIELD_FORMS_FOUND}"
                ));
      } else {
        self.writer.write(format_args!(
                    "{FIELD_INDEX},{FIELD_URL},{FIELD_STATUS},{FIELD_SIZE},{FIELD_DURATION_MS},{FIELD_CONTENT_TYPE},{FIELD_LINKS_FOUND}",
                ));
      }
    }
    if e.forms_enabled {
      self.writer.write(format_args!(
        "{},{},{},{},{},{},{},{}",
        e.index,
        format::escape_csv(e.url),
        e.status,
        e.size,
        e.duration_ms.unwrap_or(0),
        e.content_type.unwrap_or(""),
        e.links_found,
        e.forms_found,
      ));
    } else {
      self.writer.write(format_args!(
        "{},{},{},{},{},{},{}",
        e.index,
        format::escape_csv(e.url),
        e.status,
        e.size,
        e.duration_ms.unwrap_or(0),
        e.content_type.unwrap_or(""),
        e.links_found,
      ));
    }
  }

  fn technologies(&self, hits: &[tech::TechHit]) {
    if hits.is_empty() {
      return;
    }
    self.writer.write(format_args!(
      "\n# {FIELD_TECHNOLOGIES}\n{FIELD_NAME},{FIELD_CATEGORY},{FIELD_VERSION},{FIELD_SOURCE}"
    ));
    for hit in hits {
      self.writer.write(format_args!(
        "{},{},{},{}",
        format::escape_csv(&hit.name),
        format::escape_csv(&format!("{}", hit.category)),
        format::escape_csv(hit.version.as_deref().unwrap_or("")),
        format::escape_csv(&format!("{}", hit.source)),
      ));
    }
  }

  fn forms(&self, forms: &[parser::FormInfo]) {
    self.writer.write(format_args!(
            "\n# {FIELD_FORMS}\n{FIELD_ID},{FIELD_ACTION},{FIELD_METHOD},{FIELD_NAME},{FIELD_TYPE},{FIELD_REQUIRED}"
        ));
    for (id, form) in forms.iter().enumerate() {
      for field in &form.fields {
        self.writer.write(format_args!(
          "{},{},{},{},{},{}",
          id + 1,
          format::escape_csv(&form.action),
          format::escape_csv(&form.method),
          format::escape_csv(&field.name),
          format::escape_csv(&field.field_type),
          field.required,
        ));
      }
    }
  }

  fn search(&self, hits: &[search::SearchHit]) {
    self.writer.write(format_args!(
      "\n# {FIELD_SEARCH}\n{FIELD_URL},{FIELD_CATEGORY},{FIELD_STATUS},{FIELD_SIZE}_bytes"
    ));
    for hit in hits {
      self.writer.write(format_args!(
        "{},{},{},{}",
        format::escape_csv(&hit.url),
        format::escape_csv(&hit.category),
        hit.status,
        hit.size,
      ));
    }
  }
}

pub struct Json {
  pub writer: writer::OutputWriter,
  has_recursion: AtomicBool,
  has_search: AtomicBool,
}

impl Json {
  pub fn new(writer: writer::OutputWriter) -> Self {
    Self {
      writer,
      has_recursion: AtomicBool::new(false),
      has_search: AtomicBool::new(false),
    }
  }
}

impl Formatter for Json {
  fn header(&self) {
    self.writer.write(format_args!("{{"));
  }

  fn page(&self, e: &PageEvent) {
    let obj = serde_json::json!({
        FIELD_URL:          e.url,
        FIELD_STATUS:       e.status,
        FIELD_SIZE:         e.size,
        FIELD_DURATION_MS:  e.duration_ms.unwrap_or(0),
        FIELD_CONTENT_TYPE: e.content_type.unwrap_or(""),
        FIELD_LINKS_FOUND:  e.links_found,
    });
    self.writer.write(format_args!(
      "\"{}\": {}",
      FIELD_TARGET,
      serde_json::to_string(&obj).unwrap_or_default()
    ));
  }

  fn header_recursion(&self) {
    self.has_recursion.store(true, Ordering::SeqCst);
    self.writer.write(format_args!(",\"{FIELD_RECURSION}\": ["));
  }

  fn recursion(&self, e: &PageEvent) {
    let mut obj = serde_json::json!({
        FIELD_INDEX:        e.index,
        FIELD_URL:          e.url,
        FIELD_STATUS:       e.status,
        FIELD_SIZE:         e.size,
        FIELD_DURATION_MS:  e.duration_ms.unwrap_or(0),
        FIELD_CONTENT_TYPE: e.content_type.unwrap_or(""),
        FIELD_LINKS_FOUND:  e.links_found,
    });

    if e.forms_enabled {
      obj["forms_found"] = serde_json::json!(e.forms_found);
    }

    self.writer.write(format_args!(
      "{}",
      serde_json::to_string(&obj).unwrap_or_default()
    ));
  }

  fn technologies(&self, hits: &[tech::TechHit]) {
    let arr: Vec<_> = hits
      .iter()
      .map(|h| {
        serde_json::json!({
            FIELD_NAME:     h.name,
            FIELD_CATEGORY: format!("{}", h.category),
            FIELD_VERSION:  h.version,
            FIELD_SOURCE:   format!("{}", h.source),
        })
      })
      .collect();
    self.writer.write(format_args!(
      ",\"{}\": {}",
      FIELD_TECHNOLOGIES,
      serde_json::to_string(&arr).unwrap_or_default()
    ));
  }

  fn forms(&self, forms: &[parser::FormInfo]) {
    let arr: Vec<_> = forms
      .iter()
      .map(|f| {
        serde_json::json!({
            FIELD_ACTION: f.action,
            FIELD_METHOD: f.method,
            FIELD_FIELDS: f.fields.iter().map(|field| serde_json::json!({
                FIELD_NAME:       field.name,
                FIELD_TYPE:       field.field_type,
                FIELD_REQUIRED:   field.required,
            })).collect::<Vec<_>>(),
        })
      })
      .collect();
    self.writer.write(format_args!(
      ",\"{}\": {}",
      FIELD_FORMS,
      serde_json::to_string(&arr).unwrap_or_default()
    ));
  }

  fn search(&self, hits: &[search::SearchHit]) {
    self.has_search.store(true, Ordering::SeqCst);
    if self.has_recursion.load(Ordering::SeqCst) {
      self.writer.write(format_args!("]"));
    }
    let arr: Vec<_> = hits
      .iter()
      .map(|h| {
        serde_json::json!({
            FIELD_URL:      h.url,
            FIELD_CATEGORY: h.category,
            FIELD_STATUS:   h.status,
            FIELD_SIZE:     h.size,
        })
      })
      .collect();
    self.writer.write(format_args!(
      ",\"{}\": {}",
      FIELD_SEARCH,
      serde_json::to_string(&arr).unwrap_or_default()
    ));
  }

  fn footer(&self) {
    if self.has_recursion.load(Ordering::SeqCst) && !self.has_search.load(Ordering::SeqCst) {
      self.writer.write(format_args!("]"));
    }
    self.writer.write(format_args!("}}"));
  }
}

pub struct Yaml {
  pub writer: writer::OutputWriter,
}

impl Formatter for Yaml {
  fn header(&self) {
    self.writer.write(format_args!("---"));
  }

  fn page(&self, e: &PageEvent) {
    self.writer.write(format_args!("{FIELD_TARGET}:"));
    self
      .writer
      .write(format_args!("  - {}: \"{}\"", FIELD_URL, e.url));
    self
      .writer
      .write(format_args!("    {}: {}", FIELD_STATUS, e.status));
    self
      .writer
      .write(format_args!("    {}: {}", FIELD_SIZE, e.size));
    self.writer.write(format_args!(
      "    {}: {}",
      FIELD_DURATION_MS,
      e.duration_ms.unwrap_or(0)
    ));
    self.writer.write(format_args!(
      "    {}: \"{}\"",
      FIELD_CONTENT_TYPE,
      e.content_type.unwrap_or("")
    ));
    self
      .writer
      .write(format_args!("    {}: {}", FIELD_LINKS_FOUND, e.links_found));
  }

  fn header_recursion(&self) {
    self.writer.write(format_args!("{FIELD_RECURSION}:"));
  }

  fn recursion(&self, e: &PageEvent) {
    self
      .writer
      .write(format_args!("  - {}: {}", FIELD_INDEX, e.index));
    self
      .writer
      .write(format_args!("    {}: \"{}\"", FIELD_URL, e.url));
    self
      .writer
      .write(format_args!("    {}: {}", FIELD_STATUS, e.status));
    self
      .writer
      .write(format_args!("    {}: {}", FIELD_SIZE, e.size));
    self.writer.write(format_args!(
      "    {}: {}",
      FIELD_DURATION_MS,
      e.duration_ms.unwrap_or(0)
    ));
    self.writer.write(format_args!(
      "    {}: {}",
      FIELD_CONTENT_TYPE,
      e.content_type.unwrap_or("")
    ));
    self
      .writer
      .write(format_args!("    {}: {}", FIELD_LINKS_FOUND, e.links_found));
    if e.forms_enabled {
      self
        .writer
        .write(format_args!("    {}: {}", FIELD_FORMS_FOUND, e.forms_found));
    }
  }

  fn technologies(&self, hits: &[tech::TechHit]) {
    self.writer.write(format_args!("{FIELD_TECHNOLOGIES}:"));
    if hits.is_empty() {
      return;
    }
    for hit in hits {
      self
        .writer
        .write(format_args!("  - {}: \"{}\"", FIELD_NAME, hit.name));
      self
        .writer
        .write(format_args!("    {}: \"{}\"", FIELD_CATEGORY, hit.category));
      if let Some(v) = &hit.version {
        self
          .writer
          .write(format_args!("    {FIELD_VERSION}: \"{v}\""));
      }
    }
  }

  fn forms(&self, forms: &[parser::FormInfo]) {
    self.writer.write(format_args!("{FIELD_FORMS}:"));
    if forms.is_empty() {
      return;
    }
    for form in forms {
      self
        .writer
        .write(format_args!("  - {}: \"{}\"", FIELD_ACTION, form.action));
      self
        .writer
        .write(format_args!("    {}: \"{}\"", FIELD_METHOD, form.method));
      self.writer.write(format_args!("    {FIELD_FIELDS}:"));
      for field in &form.fields {
        self
          .writer
          .write(format_args!("      - {}: \"{}\"", FIELD_NAME, field.name));
        self.writer.write(format_args!(
          "        {}: \"{}\"",
          FIELD_TYPE, field.field_type
        ));
        self.writer.write(format_args!(
          "        {}: {}",
          FIELD_REQUIRED, field.required
        ));
      }
    }
  }

  fn search(&self, hits: &[search::SearchHit]) {
    self.writer.write(format_args!("{FIELD_SEARCH}:"));
    if hits.is_empty() {
      return;
    }
    for hit in hits {
      self
        .writer
        .write(format_args!("  - {}: \"{}\"", FIELD_URL, hit.url));
      self
        .writer
        .write(format_args!("    {}: \"{}\"", FIELD_CATEGORY, hit.category));
      self
        .writer
        .write(format_args!("    {}: {}", FIELD_STATUS, hit.status));
      self
        .writer
        .write(format_args!("    {}: {}", FIELD_SIZE, hit.size));
    }
  }

  fn footer(&self) {
    self.writer.write(format_args!("..."));
  }
}
