use crate::args;
use crate::commands::sysviz::common::formatter;
use crate::global::format;
use crate::global::writer;
use chrono::Local;

#[derive(Debug, Clone)]
pub struct SysVizConfig {
  pub timeout: Option<u64>,
  pub pid_filter: Option<u32>,
  pub name_filter: Option<String>,
  pub stats: bool,
  pub top: Option<usize>,
  pub alert: Option<String>,
  pub output_format: format::OutputFormat,
  pub export_path: Option<String>,
  pub zip: bool,
}

pub trait SystemMonitor {
  fn run(&mut self, config: &SysVizConfig);
  fn default_syscalls(&self) -> Vec<&'static str>;
  fn start_message(&self) -> &'static str;
}

impl SysVizConfig {
  pub fn new(args: args::SysVizArgs) -> Self {
    let global = args.global;
    let output_format = format::OutputFormat::from_flags(global.csv, global.json, global.yaml);

    let export_path = global.export.map(|filename: String| {
      if filename.is_empty() {
        let date = Local::now().format("%Y-%m-%d_%H-%M-%S");
        let ext = output_format.extension();
        format!("sysviz-{date}.{ext}")
      } else if filename.contains('.') {
        filename
      } else {
        format!("{}.{}", filename, output_format.extension())
      }
    });

    Self {
      timeout: global.timeout,
      pid_filter: args.pid,
      name_filter: args.name,
      stats: args.stats,
      top: args.top,
      alert: args.alert,
      output_format,
      export_path,
      zip: global.zip,
    }
  }

  pub fn get_formatter(&self) -> Box<dyn formatter::Formatter> {
    let writer = writer::OutputWriter::new(self.export_path.clone(), true);

    match self.output_format {
      format::OutputFormat::Txt => Box::new(formatter::Txt { writer }),
      format::OutputFormat::Csv => Box::new(formatter::Csv { writer }),
      format::OutputFormat::Json => Box::new(formatter::Json::new(writer)),
      format::OutputFormat::Yaml => Box::new(formatter::Yaml { writer }),
    }
  }
}
