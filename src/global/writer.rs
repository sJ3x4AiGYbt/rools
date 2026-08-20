use crate::global::constants::ERROR_FORMATTER_FILE;
use std::io::{self, Write};

#[derive(Clone)]
pub struct OutputWriter {
  path: Option<String>,
  stdout: bool,
}

impl OutputWriter {
  pub fn new(file_path: Option<String>, stdout: bool) -> Self {
    Self {
      path: file_path,
      stdout,
    }
  }

  pub fn write(&self, args: std::fmt::Arguments) {
    if let Some(ref path) = self.path {
      match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
      {
        Ok(mut f) => {
          if let Err(e) = writeln!(f, "{args}") {
            eprintln!("{ERROR_FORMATTER_FILE} {path}: {e}");
          }
        }
        Err(e) => eprintln!("{ERROR_FORMATTER_FILE} {path}: {e}"),
      }
    }
    if self.stdout {
      println!("{args}");
    }
  }

  pub fn flush(&self) {
    if self.stdout {
      io::stdout().flush().unwrap();
    }
  }
}
