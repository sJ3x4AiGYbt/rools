use crate::commands::sysviz::common::constants::{
  CSV_STATS_HEADER, FIELD_OPERATION, FIELD_PID, FIELD_PROCESS_PATH, FIELD_RESPONSIBLE_PID,
  FIELD_TIMESTAMP, FIELD_USER, TXT_HEADER_COUNT, TXT_HEADER_OPERATION, TXT_HEADER_PID,
  TXT_HEADER_PROCESS_PATH, TXT_HEADER_RESPONSIBLE_PID, TXT_HEADER_SIGNING_ID, TXT_HEADER_TIMESTAMP,
  WARN_JSON,
};
use crate::global::format;
use crate::global::writer;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug)]
pub struct Event {
  pub timestamp: String,
  pub source: String,
  pub pid: u32,
  pub responsible_pid: u32,
  pub user: String,
  pub process_name: String,
}

pub trait Formatter {
  fn header(&self);
  fn format(&self, event: &Event);
  fn footer(&self);
  fn stats(&self, counts: &HashMap<String, u64>, top_n: Option<usize>);
}

pub struct Txt {
  pub writer: writer::OutputWriter,
}

impl Formatter for Txt {
  fn header(&self) {
    self.writer.write(format_args!("{:-<175}", ""));
    self.writer.write(format_args!(
            "{TXT_HEADER_TIMESTAMP:<30} {TXT_HEADER_PID:<8} {TXT_HEADER_RESPONSIBLE_PID:<10} {TXT_HEADER_SIGNING_ID:<35} {TXT_HEADER_OPERATION:<15} {TXT_HEADER_PROCESS_PATH:<48}",
        ));
    self.writer.write(format_args!("{:-<175}", ""));
    self.writer.flush();
  }

  fn format(&self, e: &Event) {
    let r_pid = if e.pid != e.responsible_pid {
      e.responsible_pid.to_string()
    } else {
      "-".to_string()
    };

    self.writer.write(format_args!(
      "{:<30} {:<8} {:<10} {:<35} {:<15} {:<48}",
      e.timestamp, e.pid, r_pid, e.user, e.source, e.process_name
    ));
    self.writer.flush();
  }

  fn footer(&self) {
    self.writer.write(format_args!("{:-<175}", ""));
    self.writer.flush();
  }

  fn stats(&self, counts: &HashMap<String, u64>, top_n: Option<usize>) {
    if counts.is_empty() {
      return;
    }

    eprintln!("\n{TXT_HEADER_OPERATION} ({TXT_HEADER_COUNT})");
    eprintln!("{:-<20}", "");

    let mut stats_vec: Vec<(&String, &u64)> = counts.iter().collect();
    stats_vec.sort_by(|a, b| b.1.cmp(a.1));

    let limit = top_n.unwrap_or(stats_vec.len());

    for (name, count) in stats_vec.iter().take(limit) {
      eprintln!("{name} ({count})");
    }
  }
}

pub struct Csv {
  pub writer: writer::OutputWriter,
}

impl Formatter for Csv {
  fn header(&self) {
    self.writer.write(format_args!(
            "{FIELD_TIMESTAMP},{FIELD_PID},{FIELD_RESPONSIBLE_PID},{FIELD_USER},{FIELD_OPERATION},{FIELD_PROCESS_PATH}"
        ));
    self.writer.flush();
  }

  fn format(&self, e: &Event) {
    self.writer.write(format_args!(
      "\"{}\",{},{},\"{}\",\"{}\",\"{}\"",
      format::escape_csv(&e.timestamp),
      e.pid,
      e.responsible_pid,
      format::escape_csv(&e.user),
      format::escape_csv(&e.source),
      format::escape_csv(&e.process_name)
    ));
    self.writer.flush();
  }

  fn footer(&self) {
    self.writer.flush();
  }

  fn stats(&self, counts: &HashMap<String, u64>, top_n: Option<usize>) {
    if counts.is_empty() {
      return;
    }

    eprintln!("\n{CSV_STATS_HEADER}");

    let mut stats_vec: Vec<(&String, &u64)> = counts.iter().collect();
    stats_vec.sort_by(|a, b| b.1.cmp(a.1));

    let limit = top_n.unwrap_or(stats_vec.len());

    for (name, count) in stats_vec.iter().take(limit) {
      eprintln!("\"{}\",{}", format::escape_csv(name), count);
    }
  }
}

pub struct Json {
  pub writer: writer::OutputWriter,
  first_event: AtomicBool,
}

impl Json {
  pub fn new(writer: writer::OutputWriter) -> Self {
    Self {
      writer,
      first_event: AtomicBool::new(true),
    }
  }
}

impl Formatter for Json {
  fn header(&self) {
    self.first_event.store(true, Ordering::SeqCst);
    self.writer.write(format_args!("["));
    self.writer.flush();
  }

  fn format(&self, e: &Event) {
    let json_data = serde_json::json!({
        FIELD_TIMESTAMP: e.timestamp,
        FIELD_PID: e.pid,
        FIELD_RESPONSIBLE_PID: e.responsible_pid,
        FIELD_USER: e.user,
        FIELD_OPERATION: e.source,
        FIELD_PROCESS_PATH: e.process_name
    });

    match serde_json::to_string(&json_data) {
      Ok(json_str) => {
        let is_first = self.first_event.swap(false, Ordering::SeqCst);
        let prefix = if is_first { "  " } else { "  ," };
        self.writer.write(format_args!("{prefix}{json_str}"));
        self.writer.flush();
      }
      Err(e) => {
        eprintln!("{WARN_JSON} {e}");
      }
    }
  }

  fn footer(&self) {
    self.writer.write(format_args!("]"));
    self.writer.flush();
  }

  fn stats(&self, counts: &HashMap<String, u64>, top_n: Option<usize>) {
    if counts.is_empty() {
      eprintln!("{{}}");
      return;
    }

    let mut stats_vec: Vec<(&String, &u64)> = counts.iter().collect();
    stats_vec.sort_by(|a, b| b.1.cmp(a.1));

    let limit = top_n.unwrap_or(stats_vec.len());

    let stats_map: HashMap<&str, u64> = stats_vec
      .iter()
      .take(limit)
      .map(|(name, count)| (name.as_str(), **count))
      .collect();

    let json_output = serde_json::to_string_pretty(&stats_map).unwrap();
    eprintln!("{json_output}");
  }
}

pub struct Yaml {
  pub writer: writer::OutputWriter,
}

impl Formatter for Yaml {
  fn header(&self) {
    self.writer.write(format_args!("---"));
  }

  fn format(&self, e: &Event) {
    self
      .writer
      .write(format_args!("- {}: \"{}\"", FIELD_TIMESTAMP, e.timestamp));
    self
      .writer
      .write(format_args!("  {}: {}", FIELD_PID, e.pid));
    self.writer.write(format_args!(
      "  {}: {}",
      FIELD_RESPONSIBLE_PID, e.responsible_pid
    ));
    self
      .writer
      .write(format_args!("  {}: \"{}\"", FIELD_USER, e.user));
    self
      .writer
      .write(format_args!("  {}: \"{}\"", FIELD_OPERATION, e.source));
    self.writer.write(format_args!(
      "  {}: \"{}\"",
      FIELD_PROCESS_PATH, e.process_name
    ));
    self.writer.flush();
  }

  fn footer(&self) {
    self.writer.write(format_args!("..."));
    self.writer.flush();
  }

  fn stats(&self, counts: &HashMap<String, u64>, top_n: Option<usize>) {
    if counts.is_empty() {
      return;
    }

    eprintln!("---");

    let mut stats_vec: Vec<(&String, &u64)> = counts.iter().collect();
    stats_vec.sort_by(|a, b| b.1.cmp(a.1));

    let limit = top_n.unwrap_or(stats_vec.len());

    for (name, count) in stats_vec.iter().take(limit) {
      eprintln!("  {name}: {count}");
    }
    eprintln!("...");
  }
}
