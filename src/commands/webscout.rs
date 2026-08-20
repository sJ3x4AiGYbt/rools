pub mod common;
pub use common::config;
pub use common::constants;
pub use common::http;
pub use common::url;

mod crawl;
mod test;

use crate::args;
use crate::commands::webscout::config::WebScoutRunner;
use crate::commands::webscout::constants::BANNER_WEBSCOUT;
use crate::global::system;

pub fn run(args: args::WebScoutArgs) {
  println!("{BANNER_WEBSCOUT}");
  system::info();

  let config = config::WebScoutConfig::new(args);

  match &config.action {
    config::WebScoutAction::Crawl(_) => {
      let mut runner = crawl::CrawlRunner::new();
      runner.run(&config);
    }
    config::WebScoutAction::Test(_) => {
      let mut runner = test::TestRunner::new();
      runner.run(&config);
    }
  }
}
