use crate::args;
use crate::global::format;
use chrono::Local;

#[derive(Debug, Clone)]
pub enum WebScoutAction {
  Crawl(CrawlConfig),
  Test(TestConfig),
}

#[derive(Debug, Clone)]
pub struct CrawlConfig {
  pub url: String,
  pub forms: bool,
  pub recursion: bool,
  pub search: bool,
}

#[derive(Debug, Clone)]
pub struct TestConfig {
  pub url: String,
  pub sql: bool,
  pub xss: bool,
  pub csrf: bool,
  pub fuzz: bool,
  pub transport: bool,
  pub body: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WebScoutConfig {
  pub action: WebScoutAction,
  pub timeout: Option<u64>,
  pub output_format: format::OutputFormat,
  pub export_path: Option<String>,
  pub zip: bool,
}

pub trait WebScoutRunner {
  fn run(&mut self, config: &WebScoutConfig);
}

impl WebScoutConfig {
  pub fn new(args: args::WebScoutArgs) -> Self {
    let (action, global) = match args.action {
      args::WebScoutCommands::Crawl {
        url,
        recursion,
        search,
        forms,
        global,
      } => (
        WebScoutAction::Crawl(CrawlConfig {
          url,
          recursion,
          search,
          forms,
        }),
        global,
      ),
      args::WebScoutCommands::Test {
        url,
        sql,
        xss,
        csrf,
        fuzz,
        transport,
        body,
        global,
      } => (
        WebScoutAction::Test(TestConfig {
          url,
          sql,
          xss,
          csrf,
          fuzz,
          transport,
          body,
        }),
        global,
      ),
    };

    let output_format = format::OutputFormat::from_flags(global.csv, global.json, global.yaml);

    let prefix = match &action {
      WebScoutAction::Crawl(_) => "crawl",
      WebScoutAction::Test(_) => "test",
    };

    let export_path = global.export.map(|filename| {
      if filename.is_empty() {
        let date = Local::now().format("%Y-%m-%d_%H-%M-%S");
        let ext = output_format.extension();
        format!("webscout-{prefix}-{date}.{ext}")
      } else if filename.contains('.') {
        filename
      } else {
        format!("{}.{}", filename, output_format.extension())
      }
    });

    Self {
      action,
      timeout: global.timeout,
      output_format,
      export_path,
      zip: global.zip,
    }
  }
}
