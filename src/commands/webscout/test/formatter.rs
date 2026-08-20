use crate::commands::webscout::common::constants::{
  FIELD_EVIDENCE, FIELD_FINDINGS, FIELD_FORMS, FIELD_MODULE, FIELD_PARAM, FIELD_PAYLOAD,
  FIELD_SCAN, FIELD_TARGET, FIELD_TXT_FORMS_FOUND, FIELD_URL,
};
use crate::commands::webscout::test::probe;
use crate::global::format;
use crate::global::writer;
use std::sync::atomic::{AtomicBool, Ordering};

pub trait Formatter {
  fn header(&self, _url: &str, _forms_found: usize) {}
  fn scan_start(&self, _module: &'static str) {}
  fn finding(&self, f: &probe::Finding);
  fn no_findings(&self) {}
  fn footer(&self) {}
}

pub struct Txt {
  pub writer: writer::OutputWriter,
}

impl Formatter for Txt {
  fn header(&self, url: &str, forms_found: usize) {
    self.writer.write(format_args!("\n[{FIELD_TARGET}]  {url}"));
    self.writer.write(format_args!(
      "[{FIELD_FORMS}]   {forms_found} {FIELD_TXT_FORMS_FOUND}"
    ));
  }

  fn scan_start(&self, module: &'static str) {
    self
      .writer
      .write(format_args!("\n[{FIELD_SCAN}]    {module}"));
  }

  fn finding(&self, f: &probe::Finding) {
    self.writer.write(format_args!(
      "   {}, {}: {}, {}:{:?}\n   {}: {}",
      f.url, FIELD_PARAM, f.param, FIELD_PAYLOAD, f.payload, FIELD_EVIDENCE, f.evidence
    ));
  }

  fn no_findings(&self) {
    self.writer.write(format_args!("   no findings",)); //**
  }
}

pub struct Csv {
  pub writer: writer::OutputWriter,
  header_done: AtomicBool,
}

impl Csv {
  pub fn new(writer: writer::OutputWriter) -> Self {
    Self {
      writer,
      header_done: AtomicBool::new(false),
    }
  }
}

impl Formatter for Csv {
  fn finding(&self, f: &probe::Finding) {
    if !self.header_done.swap(true, Ordering::SeqCst) {
      self.writer.write(format_args!(
        "\n{FIELD_MODULE},{FIELD_URL},{FIELD_PARAM},{FIELD_PAYLOAD},{FIELD_EVIDENCE}",
      ));
    }
    self.writer.write(format_args!(
      "{},{},{},{},{}",
      format::escape_csv(f.module),
      format::escape_csv(&f.url),
      format::escape_csv(&f.param),
      format::escape_csv(&f.payload),
      format::escape_csv(&f.evidence),
    ));
  }
}

pub struct Json {
  pub writer: writer::OutputWriter,
  first: AtomicBool,
}

impl Json {
  pub fn new(writer: writer::OutputWriter) -> Self {
    Self {
      writer,
      first: AtomicBool::new(true),
    }
  }
}

impl Formatter for Json {
  fn header(&self, url: &str, forms_found: usize) {
    self.writer.write(format_args!(
            "\n{{\n\"{FIELD_TARGET}\":\"{url}\",\n\"{FIELD_FORMS}\":\"{forms_found}\",\n\"{FIELD_FINDINGS}\":["
        ));
  }

  fn finding(&self, f: &probe::Finding) {
    let sep = if self.first.swap(false, Ordering::SeqCst) {
      ""
    } else {
      ","
    };
    let obj = serde_json::json!({
        FIELD_MODULE:   f.module,
        FIELD_URL:      f.url,
        FIELD_PARAM:    f.param,
        FIELD_PAYLOAD:  f.payload,
        FIELD_EVIDENCE: f.evidence,
    });
    self.writer.write(format_args!(
      "{}{}",
      sep,
      serde_json::to_string(&obj).unwrap_or_default()
    ));
  }

  fn footer(&self) {
    self.writer.write(format_args!("]\n}}"));
  }
}

pub struct Yaml {
  pub writer: writer::OutputWriter,
}

impl Formatter for Yaml {
  fn header(&self, url: &str, forms_found: usize) {
    self.writer.write(format_args!("\n---"));
    self.writer.write(format_args!("{FIELD_TARGET}: \"{url}\""));
    self
      .writer
      .write(format_args!("{FIELD_FORMS}: \"{forms_found}\""));
    self.writer.write(format_args!("{FIELD_FINDINGS}:"));
  }

  fn finding(&self, f: &probe::Finding) {
    self
      .writer
      .write(format_args!("  - {}:   \"{}\"", FIELD_MODULE, f.module));
    self
      .writer
      .write(format_args!("    {}:      \"{}\"", FIELD_URL, f.url));
    self
      .writer
      .write(format_args!("    {}:    \"{}\"", FIELD_PARAM, f.param));
    self
      .writer
      .write(format_args!("    {}:  {:?}", FIELD_PAYLOAD, f.payload));
    self
      .writer
      .write(format_args!("    {}: \"{}\"", FIELD_EVIDENCE, f.evidence));
  }

  fn footer(&self) {
    self.writer.write(format_args!("..."));
  }
}
