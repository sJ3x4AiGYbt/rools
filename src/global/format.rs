#[derive(Debug, Copy, Clone)]
pub enum OutputFormat {
  Txt,
  Csv,
  Json,
  Yaml,
}

impl OutputFormat {
  pub fn extension(&self) -> &'static str {
    match self {
      OutputFormat::Csv => "csv",
      OutputFormat::Json => "json",
      OutputFormat::Yaml => "yaml",
      OutputFormat::Txt => "txt",
    }
  }

  pub fn from_flags(csv: bool, json: bool, yaml: bool) -> Self {
    if csv {
      OutputFormat::Csv
    } else if json {
      OutputFormat::Json
    } else if yaml {
      OutputFormat::Yaml
    } else {
      OutputFormat::Txt
    }
  }
}

pub fn escape_csv(s: &str) -> String {
  s.replace('"', "\"\"")
}
